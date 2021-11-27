use evdev_rs::enums::EV_KEY;
use evdev_rs::*;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::{
    sync::mpsc::{channel, Sender},
    thread, time,
};

use crate::write_keys;

type State = u64;

#[derive(Debug, Clone, Copy)]
pub struct KeyConfigEntry<'a> {
    pub cond: State,
    pub input: &'a [EV_KEY],
    pub output: &'a [EV_KEY],
    pub transition: Option<State>,
}

type KeyEv = (enums::EV_KEY, TimeVal);
pub type KeyConfig<'a> = Vec<KeyConfigEntry<'a>>;

#[derive(Debug)]
enum KeyRecorderBehavior {
    ReleaseKey(KeyEv),
    SendKey(KeyEv),
    EventWrite(InputEvent),
}

fn reserve_release_key(key: (EV_KEY, TimeVal), tx: Sender<KeyRecorderBehavior>) {
    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(1000));
        tx.send(KeyRecorderBehavior::ReleaseKey(key)).unwrap();
    });
}

pub struct KeyRecorder {
    tx: Sender<KeyRecorderBehavior>,
}

fn put_single_hotkey(
    key: (EV_KEY, TimeVal),
    single_hotkeys: &HashMap<(EV_KEY, State), (&[EV_KEY], Option<State>)>,
    key_writer: &write_keys::KeyWriter,
    state: &mut State,
) {
    if let Some((os, tra)) = single_hotkeys.get(&(key.0, *state)) {
        dbg!(os);
        for output_key in *os {
            key_writer.put_with_time(*output_key, &key.1);
        }
        if let Some(s) = tra {
            println!("state {:?} -> {:?}", *state, s);
            *state = *s;
        }
    } else {
        key_writer.put_with_time(key.0, &key.1);
    }
}

fn release_key_handler(
    previous_key: &mut Option<(EV_KEY, TimeVal)>,
    key: (EV_KEY, TimeVal),
    single_hotkeys: &HashMap<(EV_KEY, State), (&[EV_KEY], Option<State>)>,
    key_writer: &write_keys::KeyWriter,
    state: &mut State,
) {
    if Some(key) == *previous_key {
        *previous_key = None;
        put_single_hotkey(key, single_hotkeys, key_writer, state);
    }
}

fn release_waiting_key(
    previous_key: &mut Option<(EV_KEY, TimeVal)>,
    single_hotkeys: &HashMap<(EV_KEY, State), (&[EV_KEY], Option<State>)>,
    key_writer: &write_keys::KeyWriter,
    state: &mut State,
) {
    if let Some(key) = *previous_key {
        *previous_key = None;
        put_single_hotkey(key, single_hotkeys, key_writer, state);
    }
}

fn send_key_handler(
    previous_key: &mut Option<(EV_KEY, TimeVal)>,
    key: (EV_KEY, TimeVal),
    pair_hotkeys_map: &HashMap<(BTreeSet<EV_KEY>, State), (&[EV_KEY], Option<State>)>,
    key_writer: &write_keys::KeyWriter,
    pair_input_keys: &HashSet<(EV_KEY, State)>,
    single_hotkeys_map: &HashMap<(EV_KEY, State), (&[EV_KEY], Option<State>)>,
    state: &mut State,
    tx: &Sender<KeyRecorderBehavior>,
) {
    if pair_input_keys.contains(&(key.0, *state)) {
        match *previous_key {
            Some((previous_ev_key, previous_key_time)) => {
                let key_set = [previous_ev_key, key.0];
                let key_set = key_set.iter().copied().collect::<BTreeSet<enums::EV_KEY>>();
                if let Some(&(output, transition)) = pair_hotkeys_map.get(&(key_set, *state)) {
                    *previous_key = None;
                    for output_key in output {
                        key_writer.put_with_time(*output_key, &key.1);
                    }
                    dbg!("ok");
                    if let Some(s) = transition {
                        println!("state {:?} -> {:?}", *state, s);
                        *state = s;
                    }
                    return;
                } else {
                    put_single_hotkey(
                        (previous_ev_key, previous_key_time),
                        single_hotkeys_map,
                        key_writer,
                        state,
                    );
                    *previous_key = Some(key);
                    reserve_release_key(key, tx.clone());
                }
            }
            _ => {
                *previous_key = Some(key);
                reserve_release_key(key, tx.clone());
                dbg!("reserved");
            }
        }
    } else {
        dbg!(&previous_key);
        release_waiting_key(previous_key, single_hotkeys_map, key_writer, state);
        dbg!(&key);
        put_single_hotkey(key, single_hotkeys_map, key_writer, state);
    }
}

impl KeyRecorder {
    pub fn new(d: &Device, key_config: &KeyConfig<'static>) -> KeyRecorder {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();
        let key_writer = write_keys::KeyWriter::new(d);
        let pair_hotkeys: KeyConfig = key_config
            .iter()
            .filter(|s| s.input.len() == 2)
            .copied()
            .collect();
        let pair_hotkeys_map: HashMap<(BTreeSet<EV_KEY>, State), (&[EV_KEY], Option<State>)> =
            pair_hotkeys
                .iter()
                .map(
                    |&KeyConfigEntry {
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
        let single_hotkeys_map: HashMap<(EV_KEY, State), (&[EV_KEY], Option<State>)> =
            key_config
                .iter()
                .filter(|s| s.input.len() == 1)
                .map(|s| ((s.input[0], s.cond), (s.output, s.transition)))
                .collect();
        let pair_input_keys: HashSet<(EV_KEY, State)> = pair_hotkeys
            .iter()
            .flat_map(
                |&KeyConfigEntry {
                     cond,
                     input,
                     output: _,
                     transition: _,
                 }| input.iter().map(move |&i| (i, cond)),
            )
            .collect();
        let mut state = 0;
        dbg!(&key_config);
        dbg!(&single_hotkeys_map);
        thread::spawn(move || {
            let mut previous_key: Option<KeyEv> = None;
            for received in rx {
                match received {
                    KeyRecorderBehavior::ReleaseKey(key) => release_key_handler(
                        &mut previous_key,
                        key,
                        &single_hotkeys_map,
                        &key_writer,
                        &mut state,
                    ),
                    KeyRecorderBehavior::SendKey(key) => send_key_handler(
                        &mut previous_key,
                        key,
                        &pair_hotkeys_map,
                        &key_writer,
                        &pair_input_keys,
                        &single_hotkeys_map,
                        &mut state,
                        &tx_clone,
                    ),
                    KeyRecorderBehavior::EventWrite(e) => {
                        release_waiting_key(
                            &mut previous_key,
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

    pub fn send_key(&self, key: enums::EV_KEY, time: TimeVal) {
        self.tx
            .send(KeyRecorderBehavior::SendKey((key, time)))
            .unwrap();
    }

    pub fn event_write(&self, e: InputEvent) {
        self.tx.send(KeyRecorderBehavior::EventWrite(e)).unwrap();
    }
}
