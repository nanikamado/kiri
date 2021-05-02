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

pub struct KeyRecorder {
    tx: Sender<KeyRecorderBehavior>,
}

static KEY_SETTINGS: &[([EV_KEY; 2], [EV_KEY; 3])] = &[([KEY_G, KEY_I], [KEY_C, KEY_H, KEY_O])];

fn set_previous_key(
    previous_key: &mut Option<(EV_KEY, TimeVal)>,
    key: (EV_KEY, TimeVal),
    tx: Sender<KeyRecorderBehavior>,
) {
    *previous_key = Some(key);
    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(50));
        tx.send(KeyRecorderBehavior::ReleaseKey(key)).unwrap();
    });
}

impl KeyRecorder {
    pub fn new(d: &Device) -> KeyRecorder {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();
        let key_writer = write_keys::KeyWriter::new(d);
        thread::spawn(move || {
            let mut previous_key: Option<KeyEv> = None;
            'read_event: for received in rx {
                match received {
                    KeyRecorderBehavior::ReleaseKey(key) => {
                        if Some(key) == previous_key {
                            previous_key = None;
                            println!("{:?}", key.0);
                        }
                    }
                    KeyRecorderBehavior::SendKey(key) => match previous_key {
                        Some((previous_ev_key, _)) => {
                            let key_set = [previous_ev_key, key.0];
                            let key_set = key_set.iter().collect::<HashSet<&enums::EV_KEY>>();
                            for (input, output) in KEY_SETTINGS {
                                let candidate = input.iter().collect::<HashSet<&enums::EV_KEY>>();
                                if key_set == candidate {
                                    previous_key = None;
                                    println!("as !!!!!!!!!!!!!!");
                                    for output_key in output {
                                        key_writer.put_with_time(*output_key, &key.1);
                                    }
                                    continue 'read_event;
                                }
                            }
                            set_previous_key(&mut previous_key, key, tx_clone.clone());
                        }
                        _ => {
                            set_previous_key(&mut previous_key, key, tx_clone.clone());
                        }
                    },
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
