use evdev_rs::enums::EV_KEY;
use evdev_rs::*;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::{
    sync::mpsc::{channel, Sender},
    thread, time,
};

use crate::write_keys;

type State = u64;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct PairHotkeyEntry<'a> {
    pub cond: State,
    pub input: [EV_KEY; 2],
    pub output: &'a [EV_KEY],
    pub transition: Option<State>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum KeyInput {
    Press(EV_KEY),
    Release(EV_KEY),
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct SingleHotkeyEntry {
    pub cond: State,
    pub input: KeyInput,
    pub output: Vec<KeyInput>,
    pub transition: Option<State>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct KeyConfig<'a> {
    pub pair_hotkeys: Vec<PairHotkeyEntry<'a>>,
    pub single_hotkeys: Vec<SingleHotkeyEntry>,
}

type KeyEv = (enums::EV_KEY, TimeVal);

#[derive(Debug)]
enum KeyRecorderBehavior {
    FireSpecificWaitingKey(KeyEv),
    SendKey((KeyInput, TimeVal)),
    EventWrite(InputEvent),
}

fn fire_waiting_key_delay(key: (EV_KEY, TimeVal), tx: Sender<KeyRecorderBehavior>) {
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
    key: (EV_KEY, TimeVal),
    single_hotkeys: &HashMap<(KeyInput, State), (&[KeyInput], Option<State>)>,
    key_writer: &write_keys::KeyWriter,
    state: &mut State,
) {
    if let Some((os, tra)) = single_hotkeys.get(&(KeyInput::Press(key.0), *state)) {
        for output_key in *os {
            key_writer.fire_key_input(*output_key, &key.1);
        }
        if let Some(s) = tra {
            println!("state {:?} -> {:?}", *state, s);
            *state = *s;
        }
    } else {
        key_writer.put_with_time(key.0, &key.1);
    }
}

fn fire_key_input(
    key: (KeyInput, TimeVal),
    single_hotkeys: &HashMap<(KeyInput, State), (&[KeyInput], Option<State>)>,
    key_writer: &write_keys::KeyWriter,
    state: &mut State,
) {
    if let Some((outputs, tra)) = single_hotkeys.get(&(key.0, *state)) {
        for output_key in *outputs {
            key_writer.fire_key_input(*output_key, &key.1);
        }
        if let Some(s) = tra {
            println!("state {:?} -> {:?}", *state, s);
            *state = *s;
        }
    } else {
        key_writer.fire_key_input(key.0, &key.1);
    }
}

fn fire_specific_waiting_key_handler(
    waiting_key: &mut Option<(EV_KEY, TimeVal)>,
    key: (EV_KEY, TimeVal),
    single_hotkeys: &HashMap<(KeyInput, State), (&[KeyInput], Option<State>)>,
    key_writer: &write_keys::KeyWriter,
    state: &mut State,
) {
    if Some(key) == *waiting_key {
        *waiting_key = None;
        put_single_hotkey(key, single_hotkeys, key_writer, state);
    }
}

fn fire_waiting_key(
    waiting_key: &mut Option<(EV_KEY, TimeVal)>,
    single_hotkeys: &HashMap<(KeyInput, State), (&[KeyInput], Option<State>)>,
    key_writer: &write_keys::KeyWriter,
    state: &mut State,
) {
    if let Some(key) = *waiting_key {
        *waiting_key = None;
        put_single_hotkey(key, single_hotkeys, key_writer, state);
    }
}

fn send_key_handler(
    waiting_key: &mut Option<(EV_KEY, TimeVal)>,
    key: (KeyInput, TimeVal),
    pair_hotkeys_map: &HashMap<(BTreeSet<EV_KEY>, State), (&[EV_KEY], Option<State>)>,
    key_writer: &write_keys::KeyWriter,
    pair_input_keys: &HashSet<(EV_KEY, State)>,
    single_hotkeys_map: &HashMap<(KeyInput, State), (&[KeyInput], Option<State>)>,
    state: &mut State,
    tx: &Sender<KeyRecorderBehavior>,
) {
    match key {
        (KeyInput::Press(key_name), time) if pair_input_keys.contains(&(key_name, *state)) => {
            match *waiting_key {
                Some((waiting_key_kind, waiting_key_time)) => {
                    let key_set = [waiting_key_kind, key_name];
                    let key_set = key_set.iter().copied().collect::<BTreeSet<enums::EV_KEY>>();
                    if let Some(&(output, transition)) = pair_hotkeys_map.get(&(key_set, *state)) {
                        *waiting_key = None;
                        for output_key in output {
                            key_writer.put_with_time(*output_key, &key.1);
                        }
                        if let Some(s) = transition {
                            println!("state {:?} -> {:?}", *state, s);
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
            fire_key_input(key, single_hotkeys_map, key_writer, state);
        }
    };
}

impl KeyRecorder {
    pub fn new(d: &Device, key_config: KeyConfig<'static>) -> KeyRecorder {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();
        let key_writer = write_keys::KeyWriter::new(d);
        let mut state = 0;
        dbg!(&key_config);
        thread::spawn(move || {
            let pair_hotkeys_map: HashMap<(BTreeSet<EV_KEY>, State), (&[EV_KEY], Option<State>)> =
                key_config
                    .pair_hotkeys
                    .iter()
                    .map(
                        |&PairHotkeyEntry {
                             cond,
                             input,
                             output,
                             transition,
                         }| {
                            (
                                (input.iter().copied().collect(), cond),
                                (output, transition),
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
            dbg!(&single_hotkeys_map);
            let pair_input_keys: HashSet<(EV_KEY, State)> = key_config
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
                            &key_writer,
                            &mut state,
                        )
                    }
                    KeyRecorderBehavior::SendKey(key) => send_key_handler(
                        &mut waiting_key,
                        key,
                        &pair_hotkeys_map,
                        &key_writer,
                        &pair_input_keys,
                        &single_hotkeys_map,
                        &mut state,
                        &tx_clone,
                    ),
                    KeyRecorderBehavior::EventWrite(e) => {
                        fire_waiting_key(
                            &mut waiting_key,
                            &single_hotkeys_map,
                            &key_writer,
                            &mut state,
                        );
                        key_writer.write_event(&e).unwrap();
                    }
                }
            }
        });
        KeyRecorder { tx }
    }

    pub fn send_key(&self, key: KeyInput, time: TimeVal) {
        self.tx
            .send(KeyRecorderBehavior::SendKey((key, time)))
            .unwrap();
    }

    pub fn event_write(&self, e: InputEvent) {
        self.tx.send(KeyRecorderBehavior::EventWrite(e)).unwrap();
    }
}
