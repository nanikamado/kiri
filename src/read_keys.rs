use evdev_rs::enums::EV_KEY;
use evdev_rs::*;
use std::collections::{HashMap, HashSet};
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

fn release_key_handler(
    previous_key: &mut Option<(EV_KEY, TimeVal)>,
    key: (EV_KEY, TimeVal),
    single_hotkeys: &HashMap<EV_KEY, &[EV_KEY]>,
    key_writer: &write_keys::KeyWriter,
) {
    if Some(key) == *previous_key {
        *previous_key = None;
        if let Some(os) = single_hotkeys.get(&key.0) {
            for output_key in *os {
                key_writer.put_with_time(*output_key, &key.1);
            }
        } else {
            key_writer.put_with_time(key.0, &key.1);
        }
    }
}

fn send_key_handler(
    previous_key: &mut Option<(EV_KEY, TimeVal)>,
    key: (EV_KEY, TimeVal),
    pair_hotkeys: &[(&[EV_KEY], &[EV_KEY])],
    key_writer: &write_keys::KeyWriter,
    pair_input_keys: &HashSet<EV_KEY>,
    single_hotkeys_map: &HashMap<EV_KEY, &[EV_KEY]>,
    tx: &Sender<KeyRecorderBehavior>,
) {
    if pair_input_keys.contains(&key.0) {
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
                dbg!("reserved");
            }
        }
    } else {
        let os = single_hotkeys_map[&key.0];
        for o in os {
            key_writer.put_with_time(*o, &key.1);
        }
    }
}

impl KeyRecorder {
    pub fn new(d: &Device, key_config: &'static [(&[EV_KEY], &[EV_KEY])]) -> KeyRecorder {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();
        let key_writer = write_keys::KeyWriter::new(d);
        let pair_hotkeys: Vec<(&[EV_KEY], &[EV_KEY])> = key_config
            .iter()
            .filter(|(input, _)| input.len() == 2)
            .copied()
            .collect();
        let single_hotkeys_map: HashMap<EV_KEY, &[EV_KEY]> = key_config
            .iter()
            .filter(|(input, _)| input.len() == 1)
            .map(|(i, o)| (i[0], *o))
            .collect();
        let pair_input_keys: HashSet<EV_KEY> = pair_hotkeys
            .iter()
            .flat_map(|(i, _)| i.iter())
            .copied()
            .collect();
        dbg!(&pair_input_keys);
        thread::spawn(move || {
            let mut previous_key: Option<KeyEv> = None;
            for received in rx {
                match received {
                    KeyRecorderBehavior::ReleaseKey(key) => release_key_handler(
                        &mut previous_key,
                        key,
                        &single_hotkeys_map,
                        &key_writer,
                    ),
                    KeyRecorderBehavior::SendKey(key) => send_key_handler(
                        &mut previous_key,
                        key,
                        &pair_hotkeys,
                        &key_writer,
                        &pair_input_keys,
                        &single_hotkeys_map,
                        &tx_clone,
                    ),
                    KeyRecorderBehavior::EventWrite(e) => {
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
