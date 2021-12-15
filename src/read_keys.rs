// use evdev_rs::enums::Key;
// use evdev_rs::*;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::time::SystemTime;
use std::{
    sync::mpsc::{channel, Sender},
    thread, time,
};

use evdev::{Device, Key};

use crate::write_keys;

type State = u64;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct PairHotkeyEntry {
    pub cond: State,
    pub input: [Key; 2],
    pub action: Action,
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

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct SingleHotkeyEntry {
    pub cond: State,
    pub input: KeyInput,
    pub output: Vec<KeyInput>,
    pub transition: Option<State>,
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct KeyConfig {
    pub pair_hotkeys: Vec<PairHotkeyEntry>,
    pub single_hotkeys: Vec<SingleHotkeyEntry>,
    pub shadowed_keys: HashSet<KeyInput>,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Action {
    pub output_keys: Vec<KeyInput>,
    pub transition: Option<State>,
    pub input_canceler: Vec<KeyInput>,
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
    if let Some(s) = action.transition {
        log::debug!("state {:?} -> {:?}", *state, s);
        *state = s;
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
    if let Some(action) = single_hotkeys.get(&(key, *state)) {
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

fn send_key_handler(
    waiting_key: &mut Option<(Key, SystemTime)>,
    key: (KeyInputWithRepeat, SystemTime),
    pair_hotkeys_map: &HashMap<(BTreeSet<Key>, State), Action>,
    key_writer: &mut write_keys::KeyWriter,
    pair_input_keys: &HashSet<(Key, State)>,
    single_hotkeys_map: &HashMap<(KeyInput, State), Action>,
    state: &mut State,
    input_canceler: &mut HashSet<KeyInput>,
    tx: &Sender<KeyRecorderBehavior>,
) {
    if !input_canceler.remove(&key.0.into()) {
        match key {
            (KeyInputWithRepeat(key_name, KeyInputKindWithRepeat::Press), time)
                if pair_input_keys.contains(&(key_name, *state)) =>
            {
                match *waiting_key {
                    Some((waiting_key_kind, _)) => {
                        let key_set = [waiting_key_kind, key_name];
                        let key_set = key_set.iter().copied().collect::<BTreeSet<Key>>();
                        if let Some(action) = pair_hotkeys_map.get(&(key_set, *state)) {
                            *waiting_key = None;
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
            _ => {
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
        };
    }
}

impl KeyRecorder {
    pub fn new(d: &Device, key_config: KeyConfig) -> KeyRecorder {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();
        let mut key_writer = write_keys::KeyWriter::new(d);
        let mut state = 0;
        log::debug!("{:?}", key_config);
        thread::spawn(move || {
            let pair_input_keys: HashSet<(Key, State)> = key_config
                .pair_hotkeys
                .iter()
                .flat_map(
                    |&PairHotkeyEntry {
                         cond,
                         input,
                         action: _,
                     }| input.map(move |i| (i, cond)),
                )
                .collect();
            let pair_hotkeys_map: HashMap<(BTreeSet<Key>, State), Action> = key_config
                .pair_hotkeys
                .into_iter()
                .map(
                    |PairHotkeyEntry {
                         cond,
                         input,
                         action,
                     }| { ((input.iter().copied().collect(), cond), action) },
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
