use evdev_rs::enums::EV_KEY::{self, *};
use evdev_rs::*;
use std::collections::HashSet;
use std::{
    sync::mpsc::{channel, Sender},
    thread, time,
};

use crate::write_keys;

type KeyEv = (enums::EV_KEY, TimeVal);

#[derive(Debug)]
enum KeyRecorderBehavior {
    ReleaseKey(KeyEv),
    SendKey(KeyEv),
}

static KEY_SETTINGS: &[(&[EV_KEY], &[EV_KEY])] = &[
    (&[KEY_G, KEY_I], &[KEY_C, KEY_H, KEY_O]),
    (&[KEY_G], &[KEY_L, KEY_T, KEY_U]),
    (&[KEY_S], &[KEY_T, KEY_O]),
    (&[KEY_S, KEY_L], &[KEY_S, KEY_A]),
    (&[KEY_9], &[KEY_MUHENKAN]),
    (&[KEY_0], &[KEY_HENKAN]),
];

fn reserve_release_key(key: (EV_KEY, TimeVal), tx: Sender<KeyRecorderBehavior>) {
    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(50));
        tx.send(KeyRecorderBehavior::ReleaseKey(key)).unwrap();
    });
}
pub struct KeyRecorder {
    tx: Sender<KeyRecorderBehavior>,
}

fn release_key_handler(
    previous_key: &mut Option<(EV_KEY, TimeVal)>,
    key: (EV_KEY, TimeVal),
    single_hotkeys: &[(&[EV_KEY], &[EV_KEY])],
    key_writer: &write_keys::KeyWriter,
) {
    if Some(key) == *previous_key {
        *previous_key = None;
        for (input, output) in single_hotkeys {
            if key.0 == input[0] {
                for output_key in *output {
                    key_writer.put_with_time(*output_key, &key.1);
                }
                return;
            }
        }
    }
}

fn send_key_handler(
    previous_key: &mut Option<(EV_KEY, TimeVal)>,
    key: (EV_KEY, TimeVal),
    pair_hotkeys: &[(&[EV_KEY], &[EV_KEY])],
    key_writer: &write_keys::KeyWriter,
    all_input_keys: &HashSet<EV_KEY>,
    tx: &Sender<KeyRecorderBehavior>,
) {
    if all_input_keys.contains(&key.0) {
        match previous_key {
            Some((previous_ev_key, _)) => {
                let key_set = [*previous_ev_key, key.0];
                let key_set = key_set.iter().collect::<HashSet<&enums::EV_KEY>>();
                for (input, output) in pair_hotkeys {
                    let candidate = input.iter().collect::<HashSet<&enums::EV_KEY>>();
                    if key_set == candidate {
                        *previous_key = None;
                        for output_key in *output {
                            key_writer.put_with_time(*output_key, &key.1);
                        }
                        return;
                    }
                }
                *previous_key = Some(key);
                reserve_release_key(key, tx.clone());
            }
            _ => {
                *previous_key = Some(key);
                reserve_release_key(key, tx.clone());
            }
        }
    } else {
        key_writer.put_with_time(key.0, &key.1);
    }
}

impl KeyRecorder {
    pub fn new(d: &Device) -> KeyRecorder {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();
        let key_writer = write_keys::KeyWriter::new(d);
        let pair_hotkeys: Vec<(&[EV_KEY], &[EV_KEY])> = KEY_SETTINGS
            .iter()
            .filter(|(input, _)| input.len() == 2)
            .copied()
            .collect();
        let single_hotkeys: Vec<(&[EV_KEY], &[EV_KEY])> = KEY_SETTINGS
            .iter()
            .filter(|(input, _)| input.len() == 1)
            .copied()
            .collect();
        let all_input_keys: HashSet<EV_KEY> = KEY_SETTINGS
            .iter()
            .flat_map(|(i, _)| i.iter())
            .copied()
            .collect();
        thread::spawn(move || {
            let mut previous_key: Option<KeyEv> = None;
            for received in rx {
                match received {
                    KeyRecorderBehavior::ReleaseKey(key) => {
                        release_key_handler(&mut previous_key, key, &single_hotkeys, &key_writer)
                    }
                    KeyRecorderBehavior::SendKey(key) => send_key_handler(
                        &mut previous_key,
                        key,
                        &pair_hotkeys,
                        &key_writer,
                        &all_input_keys,
                        &tx_clone,
                    ),
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
}
