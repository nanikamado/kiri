use crate::write_keys;
use evdev::{Device, Key};
use smallbitset::Set64;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt;
use std::time::SystemTime;
use std::{
    sync::mpsc::{channel, Sender},
    thread, time,
};

pub type State = Set64;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum TransitionOp {
    Insert(u8),
    Remove(u8),
}

pub type Transition = Vec<TransitionOp>;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct PairHotkeyEntry {
    pub cond: State,
    pub input: [Key; 2],
    pub output_keys: Vec<KeyInput>,
    pub transition: Transition,
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

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct KeyInputWithRepeat(pub Key, pub KeyInputKindWithRepeat);

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct SingleHotkeyEntry {
    pub cond: State,
    pub shift_key: Vec<Key>,
    pub input: KeyInput,
    pub output: Vec<KeyInput>,
    pub transition: Transition,
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

#[derive(PartialEq, Eq, Clone)]
pub struct KeyConfig {
    pub pair_hotkeys: Vec<PairHotkeyEntry>,
    pub single_hotkeys: Vec<SingleHotkeyEntry>,
}

impl fmt::Debug for KeyConfig {
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

pub struct KeyRecorder {
    tx: Sender<KeyRecorderBehavior>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Action {
    pub output_keys: Vec<KeyInput>,
    pub transition: Transition,
    pub input_canceler: Vec<KeyInput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct PressingPair<'a> {
    pub pair: BTreeSet<Key>,
    pub action: Option<&'a Action>,
}

fn perform_action(
    action: &Action,
    key_writer: &mut write_keys::KeyWriter,
    state: &mut State,
    input_canceler: &mut HashSet<KeyInput>,
) {
    for output_key in &action.output_keys {
        key_writer.fire_key_input(*output_key);
    }
    for op in &action.transition {
        match op {
            TransitionOp::Insert(e) => {
                *state = state.insert(*e);
            }
            TransitionOp::Remove(e) => {
                *state = state.remove(*e);
            }
        }
    }
    if !action.transition.is_empty() {
        log::debug!("state : {:?}", *state);
    }
    for c in &action.input_canceler {
        input_canceler.insert(*c);
    }
}

fn fire_key_input(
    key: KeyInput,
    single_hotkeys: &HashMap<(KeyInput, State), Action>,
    key_writer: &mut write_keys::KeyWriter,
    state: &mut State,
    input_canceler: &mut HashSet<KeyInput>,
) {
    if let Some(action) = single_hotkeys.get(&(key, state.clone())) {
        perform_action(action, key_writer, state, input_canceler);
    } else {
        key_writer.fire_key_input(key);
    }
}

fn fire_specific_waiting_key_handler(
    waiting_key: &mut Option<(Key, SystemTime)>,
    key: (Key, SystemTime),
    single_hotkeys: &HashMap<(KeyInput, State), Action>,
    key_writer: &mut write_keys::KeyWriter,
    state: &mut State,
    input_canceler: &mut HashSet<KeyInput>,
) {
    if Some(key) == *waiting_key {
        *waiting_key = None;
        fire_key_input(
            KeyInput::press(key.0),
            single_hotkeys,
            key_writer,
            state,
            input_canceler,
        );
    }
}

fn fire_waiting_key(
    waiting_key: &mut Option<(Key, SystemTime)>,
    single_hotkeys: &HashMap<(KeyInput, State), Action>,
    key_writer: &mut write_keys::KeyWriter,
    state: &mut State,
    input_canceler: &mut HashSet<KeyInput>,
) {
    if let Some((key, _)) = *waiting_key {
        *waiting_key = None;
        fire_key_input(
            KeyInput::press(key),
            single_hotkeys,
            key_writer,
            state,
            input_canceler,
        );
    }
}

fn send_key_handler<'a>(
    waiting_key: &mut Option<(Key, SystemTime)>,
    key: (KeyInputWithRepeat, SystemTime),
    pair_hotkeys_map: &'a HashMap<(BTreeSet<Key>, State), Action>,
    key_writer: &mut write_keys::KeyWriter,
    pair_input_keys: &HashSet<(Key, State)>,
    single_hotkeys_map: &HashMap<(KeyInput, State), Action>,
    state: &mut State,
    input_canceler: &mut HashSet<KeyInput>,
    pressing_pair: &mut PressingPair<'a>,
    pressing_keys: &mut HashSet<Key>,
    tx: &Sender<KeyRecorderBehavior>,
) {
    match key.0 {
        KeyInputWithRepeat(key_name, KeyInputKindWithRepeat::Press) => {
            pressing_keys.insert(key_name);
        }
        KeyInputWithRepeat(key_name, KeyInputKindWithRepeat::Release) => {
            pressing_keys.remove(&key_name);
        }
        _ => (),
    }
    if !input_canceler.remove(&key.0.into()) {
        match key {
            (KeyInputWithRepeat(key_name, KeyInputKindWithRepeat::Press), time)
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
                            perform_action(action, key_writer, state, input_canceler);
                            return;
                        } else {
                            fire_key_input(
                                KeyInput::press(waiting_key_kind),
                                single_hotkeys_map,
                                key_writer,
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
            (KeyInputWithRepeat(key_name, KeyInputKindWithRepeat::Repeat), _)
                if pressing_pair.pair.contains(&key_name) =>
            {
                perform_action(
                    &pressing_pair.action.unwrap(),
                    key_writer,
                    state,
                    input_canceler,
                );
            }
            _ => {
                if !pressing_pair.pair.remove(&key.0 .0) {
                    fire_waiting_key(
                        waiting_key,
                        single_hotkeys_map,
                        key_writer,
                        state,
                        input_canceler,
                    );
                    fire_key_input(
                        key.0.into(),
                        single_hotkeys_map,
                        key_writer,
                        state,
                        input_canceler,
                    );
                }
            }
        };
    }
}

impl KeyRecorder {
    pub fn new(d: &Device, key_config: KeyConfig) -> KeyRecorder {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();
        let mut key_writer = write_keys::KeyWriter::new(d);
        let mut state = Set64::empty();
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
                         shift_key: _,
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
            let mut pressing_keys: HashSet<Key> = HashSet::new();
            for received in rx {
                match received {
                    KeyRecorderBehavior::FireSpecificWaitingKey(key) => {
                        fire_specific_waiting_key_handler(
                            &mut waiting_key,
                            key,
                            &single_hotkeys_map,
                            &mut key_writer,
                            &mut state,
                            &mut input_canceler,
                        )
                    }
                    KeyRecorderBehavior::SendKey(key) => send_key_handler(
                        &mut waiting_key,
                        key,
                        &pair_hotkeys_map,
                        &mut key_writer,
                        &pair_input_keys,
                        &single_hotkeys_map,
                        &mut state,
                        &mut input_canceler,
                        &mut pressing_pair,
                        &mut pressing_keys,
                        &tx_clone,
                    ),
                }
            }
        });
        KeyRecorder { tx }
    }

    pub fn send_key(&self, key: KeyInputWithRepeat, time: SystemTime) {
        self.tx
            .send(KeyRecorderBehavior::SendKey((key, time)))
            .unwrap();
    }
}
