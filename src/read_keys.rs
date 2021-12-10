// use evdev_rs::enums::Key;
// use evdev_rs::*;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::time::SystemTime;
use std::{
    sync::mpsc::{channel, Sender},
    thread, time,
};

use evdev::{Key, InputEvent, Device};

use crate::write_keys;

type State = u64;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct PairHotkeyEntry {
    pub cond: State,
    pub input: [Key; 2],
    pub output: Vec<KeyInput>,
    pub transition: Option<State>,
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
}

impl From<KeyInputWithRepeat> for KeyInput {
    fn from(KeyInputWithRepeat(name, kind): KeyInputWithRepeat) -> Self {
        match kind {
            KeyInputKindWithRepeat::Press | KeyInputKindWithRepeat::Repeat => Self(name, KeyInputKind::Press),
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
    EventWrite(InputEvent),
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

fn put_single_hotkey(
    key: (Key, SystemTime),
    single_hotkeys: &HashMap<(KeyInput, State), (&[KeyInput], Option<State>)>,
    key_writer: &mut write_keys::KeyWriter,
    state: &mut State,
) {
    if let Some((os, tra)) = single_hotkeys.get(&(KeyInput::press(key.0), *state)) {
        for output_key in *os {
            key_writer.fire_key_input(*output_key);
        }
        if let Some(s) = tra {
            log::debug!("state {:?} -> {:?}", *state, s);
            *state = *s;
        }
    } else {
        key_writer.put_with_time(key.0);
    }
}

fn fire_key_input(
    key: (KeyInput, SystemTime),
    single_hotkeys: &HashMap<(KeyInput, State), (&[KeyInput], Option<State>)>,
    key_writer: &mut write_keys::KeyWriter,
    state: &mut State,
) {
    if let Some((outputs, tra)) = single_hotkeys.get(&(key.0, *state)) {
        for output_key in *outputs {
            key_writer.fire_key_input(*output_key);
        }
        if let Some(s) = tra {
            log::debug!("state {:?} -> {:?}", *state, s);
            *state = *s;
        }
    } else {
        key_writer.fire_key_input(key.0);
    }
}

fn fire_specific_waiting_key_handler(
    waiting_key: &mut Option<(Key, SystemTime)>,
    key: (Key, SystemTime),
    single_hotkeys: &HashMap<(KeyInput, State), (&[KeyInput], Option<State>)>,
    key_writer: &mut write_keys::KeyWriter,
    state: &mut State,
) {
    if Some(key) == *waiting_key {
        *waiting_key = None;
        put_single_hotkey(key, single_hotkeys, key_writer, state);
    }
}

fn fire_waiting_key(
    waiting_key: &mut Option<(Key, SystemTime)>,
    single_hotkeys: &HashMap<(KeyInput, State), (&[KeyInput], Option<State>)>,
    key_writer: &mut write_keys::KeyWriter,
    state: &mut State,
) {
    if let Some(key) = *waiting_key {
        *waiting_key = None;
        put_single_hotkey(key, single_hotkeys, key_writer, state);
    }
}

fn send_key_handler(
    waiting_key: &mut Option<(Key, SystemTime)>,
    key: (KeyInputWithRepeat, SystemTime),
    pair_hotkeys_map: &HashMap<(BTreeSet<Key>, State), (&[KeyInput], Option<State>)>,
    key_writer: &mut write_keys::KeyWriter,
    pair_input_keys: &HashSet<(Key, State)>,
    single_hotkeys_map: &HashMap<(KeyInput, State), (&[KeyInput], Option<State>)>,
    state: &mut State,
    tx: &Sender<KeyRecorderBehavior>,
) {
    match key {
        (KeyInputWithRepeat(key_name, KeyInputKindWithRepeat::Press), time)
            if pair_input_keys.contains(&(key_name, *state)) =>
        {
            match *waiting_key {
                Some((waiting_key_kind, waiting_key_time)) => {
                    let key_set = [waiting_key_kind, key_name];
                    let key_set = key_set.iter().copied().collect::<BTreeSet<Key>>();
                    if let Some(&(output, transition)) = pair_hotkeys_map.get(&(key_set, *state)) {
                        *waiting_key = None;
                        for output_key in output {
                            key_writer.fire_key_input(*output_key);
                        }
                        if let Some(s) = transition {
                            log::debug!("state {:?} -> {:?}", *state, s);
                            *state = s;
                        }
                        return;
                    } else {
                        put_single_hotkey(
                            (waiting_key_kind, waiting_key_time),
                            single_hotkeys_map,
                            key_writer,
                            state,
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
            fire_waiting_key(waiting_key, single_hotkeys_map, key_writer, state);
            fire_key_input((key.0.into(), key.1), single_hotkeys_map, key_writer, state);
        }
    };
}

impl KeyRecorder {
    pub fn new(d: &Device, key_config: KeyConfig) -> KeyRecorder {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();
        let mut key_writer = write_keys::KeyWriter::new(d);
        let mut state = 0;
        log::debug!("{:?}", key_config);
        thread::spawn(move || {
            let pair_hotkeys_map: HashMap<(BTreeSet<Key>, State), (&[KeyInput], Option<State>)> =
                key_config
                    .pair_hotkeys
                    .iter()
                    .map(
                        |PairHotkeyEntry {
                             cond,
                             input,
                             output,
                             transition,
                         }|
                         -> (_, (&[KeyInput], _)) {
                            (
                                (input.iter().copied().collect(), *cond),
                                (output, *transition),
                            )
                        },
                    )
                    .collect();
            let single_hotkeys_map: HashMap<(KeyInput, State), (&[KeyInput], Option<State>)> =
                key_config
                    .single_hotkeys
                    .iter()
                    .map(
                        |SingleHotkeyEntry {
                             cond,
                             input,
                             output,
                             transition,
                         }|
                         -> (_, (&[KeyInput], _)) {
                            ((*input, *cond), (output, *transition))
                        },
                    )
                    .collect();
            log::debug!("{:?}", single_hotkeys_map);
            let pair_input_keys: HashSet<(Key, State)> = key_config
                .pair_hotkeys
                .iter()
                .flat_map(
                    |&PairHotkeyEntry {
                         cond,
                         input,
                         output: _,
                         transition: _,
                     }| input.map(move |i| (i, cond)),
                )
                .collect();
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
                        &tx_clone,
                    ),
                    KeyRecorderBehavior::EventWrite(e) => {
                        fire_waiting_key(
                            &mut waiting_key,
                            &single_hotkeys_map,
                            &mut key_writer,
                            &mut state,
                        );
                        key_writer.write_event(&e).unwrap();
                    }
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

    pub fn event_write(&self, e: InputEvent) {
        self.tx.send(KeyRecorderBehavior::EventWrite(e)).unwrap();
    }
}
