use evdev_rs::enums::EV_KEY::*;
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

impl KeyRecorder {
    pub fn new(d: &Device) -> KeyRecorder {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();
        let key_writer = write_keys::KeyWriter::new(d);
        thread::spawn(move || {
            let mut previous_key: Option<KeyEv> = None;
            for received in rx {
                match received {
                    KeyRecorderBehavior::ReleaseKey(key) => {
                        if Some(key) == previous_key {
                            previous_key = None;
                            println!("{:?}", key.0);
                        }
                    }
                    KeyRecorderBehavior::SendKey(key) => match previous_key {
                        Some((previous_ev_key, _))
                            if {
                                let key_set = [previous_ev_key, key.0];
                                let key_set = key_set.iter().collect::<HashSet<&enums::EV_KEY>>();
                                let key_set_as =
                                    [KEY_G, KEY_I].iter().collect::<HashSet<&enums::EV_KEY>>();
                                key_set == key_set_as
                            } =>
                        {
                            previous_key = None;
                            println!("as !!!!!!!!!!!!!!");
                            key_writer.put_with_time(KEY_C, &key.1);
                            key_writer.put_with_time(KEY_H, &key.1);
                            key_writer.put_with_time(KEY_O, &key.1);
                        }
                        _ => {
                            previous_key = Some(key);
                            let tx_clone = tx_clone.clone();
                            thread::spawn(move || {
                                thread::sleep(time::Duration::from_millis(50));
                                tx_clone.send(KeyRecorderBehavior::ReleaseKey(key)).unwrap();
                            });
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
