use crate::write_keys::{self, KeyWriter};
use evdev::Key;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::{self, Debug};
use std::time::SystemTime;
use std::{
    sync::mpsc::{channel, Sender},
    thread, time,
};

pub type State = u64;

pub type Transition = u64;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct PairHotkeyEntry {
    pub cond: State,
    pub input: [Key; 2],
    pub output_keys: Vec<KeyInput>,
    pub transition: Option<Transition>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum KeyInputKind {
    Press,
    Release,
}

impl From<KeyInputKind> for i32 {
    fn from(k: KeyInputKind) -> Self {
        match k {
            KeyInputKind::Press => 1,
            KeyInputKind::Release => 0,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct KeyInput(pub Key, pub KeyInputKind);

impl KeyInput {
    pub fn press(key: Key) -> KeyInput {
        Self(key, KeyInputKind::Press)
    }

    pub fn release(key: Key) -> KeyInput {
        Self(key, KeyInputKind::Release)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum KeyInputKindWithRepeat {
    Press,
    Release,
    Repeat,
}

impl From<i32> for KeyInputKindWithRepeat {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::Release,
            1 => Self::Press,
            2 => Self::Repeat,
            _ => panic!("unknown input_event value"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct KeyInputWithRepeat(pub Key, pub KeyInputKindWithRepeat);

impl Debug for KeyInputWithRepeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use KeyInputKindWithRepeat::*;
        let KeyInputWithRepeat(key, input_kind) = self;
        write!(
            f,
            "{:?} {}",
            key,
            match input_kind {
                Press => "↓",
                Release => "↑",
                Repeat => "↺",
            }
        )
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct SingleHotkeyEntry {
    pub cond: State,
    pub input: KeyInput,
    pub output: Vec<KeyInput>,
    pub transition: Option<Transition>,
    pub input_canceler: Vec<KeyInput>,
}

impl From<KeyInputWithRepeat> for KeyInput {
    fn from(KeyInputWithRepeat(name, kind): KeyInputWithRepeat) -> Self {
        match kind {
            KeyInputKindWithRepeat::Press | KeyInputKindWithRepeat::Repeat => {
                Self(name, KeyInputKind::Press)
            }
            KeyInputKindWithRepeat::Release => Self(name, KeyInputKind::Release),
        }
    }
}

impl From<KeyInput> for KeyInputWithRepeat {
    fn from(KeyInput(name, kind): KeyInput) -> Self {
        match kind {
            KeyInputKind::Press => Self(name, KeyInputKindWithRepeat::Press),
            KeyInputKind::Release => Self(name, KeyInputKindWithRepeat::Release),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct KeyConfigUnit {
    pub pair_hotkeys: Vec<PairHotkeyEntry>,
    pub single_hotkeys: Vec<SingleHotkeyEntry>,
    pub layer_name: &'static str,
}

impl fmt::Debug for KeyConfigUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "pair_hotkeys: ")?;
        for e in &self.pair_hotkeys {
            writeln!(f, "    {:?}", e)?;
        }
        write!(f, "single_hotkeys: ")?;
        for e in &self.single_hotkeys {
            write!(f, "\n    {:?}", e)?;
        }
        Ok(())
    }
}

impl fmt::Debug for PairHotkeyEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}, {:?}, {:?}, {:?}",
            self.cond, self.input, self.output_keys, self.transition
        )
    }
}

impl fmt::Debug for SingleHotkeyEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}, {:?}, {:?}, {:?}, {:?}",
            self.cond, self.input, self.output, self.transition, self.input_canceler
        )
    }
}

type KeyEv = (Key, SystemTime);

#[derive(Debug)]
enum KeyRecorderBehavior {
    FireSpecificWaitingKey(KeyEv),
    SendKey((KeyInputWithRepeat, SystemTime)),
}

fn fire_waiting_key_delay(key: (Key, SystemTime), tx: Sender<KeyRecorderBehavior>) {
    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(50));
        tx.send(KeyRecorderBehavior::FireSpecificWaitingKey(key))
            .unwrap();
    });
}

pub struct KeyRecorderUnit {
    tx: Sender<KeyRecorderBehavior>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Action {
    pub output_keys: Vec<KeyInput>,
    pub transition: Option<Transition>,
    pub input_canceler: Vec<KeyInput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct PressingPair<'a> {
    pub pair: BTreeSet<Key>,
    pub action: Option<&'a Action>,
}

trait KeyReceiver: Send {
    fn send_key(&mut self, key: KeyInputWithRepeat, time: SystemTime);
}

fn perform_action(
    action: &Action,
    time: SystemTime,
    key_receiver: &mut impl KeyReceiver,
    state: &mut State,
    input_canceler: &mut HashSet<KeyInput>,
) {
    for output_key in &action.output_keys {
        key_receiver.send_key((*output_key).into(), time);
    }
    if let Some(t) = action.transition {
        log::debug!("state : {} -> {}", *state, t);
        *state = t;
    }
    for c in &action.input_canceler {
        input_canceler.insert(*c);
    }
}

fn fire_key_input(
    key: KeyInput,
    time: SystemTime,
    single_hotkeys: &HashMap<(KeyInput, State), Action>,
    key_receiver: &mut impl KeyReceiver,
    state: &mut State,
    input_canceler: &mut HashSet<KeyInput>,
) {
    if let Some(action) = single_hotkeys.get(&(key, state.clone())) {
        perform_action(action, time, key_receiver, state, input_canceler);
    } else {
        key_receiver.send_key(key.into(), time);
    }
}

fn fire_specific_waiting_key_handler(
    waiting_key: &mut Option<(Key, SystemTime)>,
    key: Key,
    time: SystemTime,
    single_hotkeys: &HashMap<(KeyInput, State), Action>,
    key_receiver: &mut impl KeyReceiver,
    state: &mut State,
    input_canceler: &mut HashSet<KeyInput>,
) {
    if Some((key, time)) == *waiting_key {
        *waiting_key = None;
        fire_key_input(
            KeyInput::press(key),
            time,
            single_hotkeys,
            key_receiver,
            state,
            input_canceler,
        );
    }
}

fn fire_waiting_key(
    waiting_key: &mut Option<(Key, SystemTime)>,
    single_hotkeys: &HashMap<(KeyInput, State), Action>,
    key_receiver: &mut impl KeyReceiver,
    state: &mut State,
    input_canceler: &mut HashSet<KeyInput>,
) {
    if let Some((key, time)) = *waiting_key {
        *waiting_key = None;
        fire_key_input(
            KeyInput::press(key),
            time,
            single_hotkeys,
            key_receiver,
            state,
            input_canceler,
        );
    }
}

fn send_key_handler<'a>(
    waiting_key: &mut Option<(Key, SystemTime)>,
    key: KeyInputWithRepeat,
    time: SystemTime,
    pair_hotkeys_map: &'a HashMap<(BTreeSet<Key>, State), Action>,
    key_receiver: &mut impl KeyReceiver,
    pair_input_keys: &HashSet<(Key, State)>,
    single_hotkeys_map: &HashMap<(KeyInput, State), Action>,
    state: &mut State,
    input_canceler: &mut HashSet<KeyInput>,
    pressing_pair: &mut PressingPair<'a>,
    tx: &Sender<KeyRecorderBehavior>,
) {
    if !input_canceler.remove(&key.into()) {
        match key {
            KeyInputWithRepeat(key_name, KeyInputKindWithRepeat::Press)
                if pair_input_keys.contains(&(key_name, state.clone())) =>
            {
                match *waiting_key {
                    Some((waiting_key_kind, _)) => {
                        let key_set = [waiting_key_kind, key_name];
                        let key_set = key_set.iter().copied().collect::<BTreeSet<Key>>();
                        if let Some(action) =
                            pair_hotkeys_map.get(&(key_set.clone(), state.clone()))
                        {
                            *waiting_key = None;
                            *pressing_pair = PressingPair {
                                pair: key_set,
                                action: Some(action),
                            };
                            perform_action(action, time, key_receiver, state, input_canceler);
                            return;
                        } else {
                            fire_key_input(
                                KeyInput::press(waiting_key_kind),
                                time,
                                single_hotkeys_map,
                                key_receiver,
                                state,
                                input_canceler,
                            );
                            *waiting_key = Some((key_name, time));
                            fire_waiting_key_delay((key_name, time), tx.clone());
                        }
                    }
                    _ => {
                        *waiting_key = Some((key_name, time));
                        fire_waiting_key_delay((key_name, time), tx.clone());
                    }
                }
            }
            KeyInputWithRepeat(key_name, KeyInputKindWithRepeat::Repeat)
                if pressing_pair.pair.contains(&key_name) =>
            {
                perform_action(
                    &pressing_pair.action.unwrap(),
                    time,
                    key_receiver,
                    state,
                    input_canceler,
                );
            }
            _ => {
                if !pressing_pair.pair.remove(&key.0) {
                    fire_waiting_key(
                        waiting_key,
                        single_hotkeys_map,
                        key_receiver,
                        state,
                        input_canceler,
                    );
                    fire_key_input(
                        key.into(),
                        time,
                        single_hotkeys_map,
                        key_receiver,
                        state,
                        input_canceler,
                    );
                }
            }
        };
    } else {
        log::debug!("input {:?} canceled", key.0);
    }
}

impl KeyRecorderUnit {
    fn new(
        key_config: KeyConfigUnit,
        mut key_receiver: impl KeyReceiver + 'static,
    ) -> KeyRecorderUnit {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();
        let mut state = 0;
        log::debug!("{:?}", key_config);
        thread::spawn(move || {
            let pair_input_keys: HashSet<(Key, State)> = key_config
                .pair_hotkeys
                .iter()
                .flat_map(|PairHotkeyEntry { cond, input, .. }| {
                    input.map(move |i| (i, cond.clone()))
                })
                .collect();
            let pair_hotkeys_map: HashMap<(BTreeSet<Key>, State), Action> = key_config
                .pair_hotkeys
                .into_iter()
                .map(
                    |PairHotkeyEntry {
                         cond,
                         input,
                         output_keys,
                         transition,
                     }| {
                        (
                            (input.iter().copied().collect(), cond),
                            Action {
                                output_keys,
                                transition,
                                input_canceler: Vec::new(),
                            },
                        )
                    },
                )
                .collect();
            let single_hotkeys_map: HashMap<(KeyInput, State), Action> = key_config
                .single_hotkeys
                .into_iter()
                .map(
                    |SingleHotkeyEntry {
                         cond,
                         input,
                         output,
                         transition,
                         input_canceler,
                     }| {
                        (
                            (input, cond),
                            Action {
                                output_keys: output,
                                transition,
                                input_canceler,
                            },
                        )
                    },
                )
                .collect();
            log::debug!("{:?}", single_hotkeys_map);
            let mut input_canceler: HashSet<KeyInput> = HashSet::new();
            let mut waiting_key: Option<KeyEv> = None;
            let mut pressing_pair: PressingPair = Default::default();
            for received in rx {
                match received {
                    KeyRecorderBehavior::FireSpecificWaitingKey((key, time)) => {
                        fire_specific_waiting_key_handler(
                            &mut waiting_key,
                            key,
                            time,
                            &single_hotkeys_map,
                            &mut key_receiver,
                            &mut state,
                            &mut input_canceler,
                        )
                    }
                    KeyRecorderBehavior::SendKey((key, time)) => send_key_handler(
                        &mut waiting_key,
                        key,
                        time,
                        &pair_hotkeys_map,
                        &mut key_receiver,
                        &pair_input_keys,
                        &single_hotkeys_map,
                        &mut state,
                        &mut input_canceler,
                        &mut pressing_pair,
                        &tx_clone,
                    ),
                }
            }
        });
        KeyRecorderUnit { tx }
    }
}

impl KeyReceiver for KeyRecorderUnit {
    fn send_key(&mut self, key: KeyInputWithRepeat, time: SystemTime) {
        self.tx
            .send(KeyRecorderBehavior::SendKey((key, time)))
            .unwrap();
    }
}

impl KeyReceiver for KeyWriter {
    fn send_key(&mut self, key: KeyInputWithRepeat, _: SystemTime) {
        self.fire_key_input(key.into());
    }
}

pub struct KeyRecorder {
    first_recorder: KeyRecorderUnit,
}

pub type KeyConfig = Vec<KeyConfigUnit>;

impl KeyRecorder {
    pub fn new(key_config: KeyConfig) -> Self {
        let mut key_config = key_config.into_iter().rev();
        let key_receiver = write_keys::KeyWriter::new();
        let mut key_receiver = KeyRecorderUnit::new(key_config.next().unwrap(), key_receiver);
        for conf_unit in key_config {
            key_receiver = KeyRecorderUnit::new(conf_unit, key_receiver);
        }
        KeyRecorder {
            first_recorder: key_receiver,
        }
    }

    pub fn send_key(&mut self, key: KeyInputWithRepeat, time: SystemTime) {
        self.first_recorder.send_key(key, time);
    }
}
