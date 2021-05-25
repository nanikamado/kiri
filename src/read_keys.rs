use evdev_rs::enums::EV_KEY;
use evdev_rs::*;
use std::collections::HashSet;
use std::sync::mpsc::{channel, Sender};
use std::{thread, time};

use crate::read_config;
use crate::write_keys;

type KeyEv = (enums::EV_KEY, TimeVal);

#[derive(Debug)]
enum KeyRecorderBehavior {
    ReleaseKey(KeyEv),
    SendKey(KeyEv),
}

fn reserve_release_key(key: (EV_KEY, TimeVal), tx: Sender<KeyRecorderBehavior>) {
    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(50));
        tx.send(KeyRecorderBehavior::ReleaseKey(key)).unwrap();
    });
}

pub struct KeyRecorder {
    tx: Sender<KeyRecorderBehavior>,
}

fn handle_ev_key_output(
    outputs: &Vec<EvKeyOutput>,
    time: &TimeVal,
    key_writer: &write_keys::KeyWriter,
    flags: &mut HashSet<String>,
    condition: &Vec<read_config::Flag>,
) {
    dbg!(&flags, condition);
    for output in outputs {
        match output {
            EvKeyOutput::Tap(output_key) => {
                if condition.iter().all(|f| match f {
                    read_config::Flag::Is(f) => flags.contains(f),
                    read_config::Flag::Not(f) => !flags.contains(f),
                }) {
                    key_writer.put_with_time(*output_key, time)
                }
            }
            EvKeyOutput::Toggle(f) => {
                if flags.contains(f) {
                    flags.remove(f);
                } else {
                    flags.insert(f.to_string());
                }
            }
        }
    }
}

fn release_key_handler(
    previous_key: &mut Option<(EV_KEY, TimeVal)>,
    key: (EV_KEY, TimeVal),
    single_hotkeys: &Vec<EvHotKey>,
    key_writer: &write_keys::KeyWriter,
    flags: &mut HashSet<String>,
) {
    if Some(key) == *previous_key {
        *previous_key = None;
        for EvHotKey {
            input,
            output,
            condition,
        } in single_hotkeys
        {
            if key.0 == *input.iter().next().unwrap() {
                handle_ev_key_output(output, &key.1, key_writer, flags, condition);
            }
        }
    }
}

fn send_key_handler(
    previous_key: &mut Option<(EV_KEY, TimeVal)>,
    key: (EV_KEY, TimeVal),
    pair_hotkeys: &Vec<EvHotKey>,
    key_writer: &write_keys::KeyWriter,
    all_input_keys: &HashSet<EV_KEY>,
    tx: &Sender<KeyRecorderBehavior>,
    flags: &mut HashSet<String>,
) {
    if all_input_keys.contains(&key.0) {
        dbg!(&key, &previous_key);
        match previous_key {
            Some((previous_ev_key, _)) => {
                let key_set = [*previous_ev_key, key.0];
                let key_set = key_set.iter().copied().collect::<HashSet<enums::EV_KEY>>();
                for EvHotKey {
                    input,
                    output,
                    condition,
                } in pair_hotkeys
                {
                    if key_set == *input {
                        *previous_key = None;
                        handle_ev_key_output(output, &key.1, key_writer, flags, condition);
                        return;
                    }
                }
                dbg!(&key);
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

#[derive(Debug, Clone)]
enum EvKeyOutput {
    Tap(EV_KEY),
    // Down(String),
    // Up(String),
    Toggle(String),
}

#[derive(Debug, Clone)]
pub struct EvHotKey {
    input: HashSet<EV_KEY>,
    output: Vec<EvKeyOutput>,
    condition: read_config::Flags,
}

fn key_name_convert(name: &str) -> EV_KEY {
    let name = match name {
        ";" => "SEMICOLON",
        "," => "COMMA",
        "." => "DOT",
        "/" => "SLASH",
        "-" => "MINUS",
        "@" => "LEFTBRACE",
        "zenkakuhankaku" => "GRAVE",
        n => n,
    };
    format!("KEY_{}", name.to_uppercase())
        .parse()
        .unwrap_or_else(|_| {
            println!("cannot parse: {}", name);
            EV_KEY::KEY_1
        })
}

fn setup_config() -> Result<Vec<EvHotKey>, Box<dyn std::error::Error>> {
    read_config::run()
        .unwrap()
        .into_iter()
        .map(|hk| {
            Ok(EvHotKey {
                input: hk.input.into_iter().map(|f| key_name_convert(&f)).collect(),
                output: hk
                    .output
                    .into_iter()
                    .map(|f| match f {
                        read_config::KeyOutput::Tap(s) => EvKeyOutput::Tap(key_name_convert(&s)),
                        read_config::KeyOutput::Toggle(f) => EvKeyOutput::Toggle(f.to_string()),
                    })
                    .collect(),
                condition: hk.condition,
            })
        })
        .collect()
}

impl KeyRecorder {
    pub fn new(d: &Device) -> KeyRecorder {
        let config = setup_config().unwrap();
        let single_hotkeys: Vec<EvHotKey> = config
            .iter()
            .filter(|ev_hotkey| ev_hotkey.input.len() == 1)
            .cloned()
            .collect();
        let pair_hotkeys: Vec<EvHotKey> = config
            .iter()
            .filter(|ev_hotkey| ev_hotkey.input.len() == 2)
            .cloned()
            .collect();
        let (tx, rx) = channel();
        let tx_clone = tx.clone();
        let key_writer = write_keys::KeyWriter::new(d);
        let all_input_keys: HashSet<EV_KEY> = config
            .into_iter()
            .flat_map(|k| k.input.into_iter())
            .collect();
        thread::spawn(move || {
            let mut previous_key: Option<KeyEv> = None;
            let mut flags: HashSet<String> = HashSet::new();
            for received in rx {
                match received {
                    KeyRecorderBehavior::ReleaseKey(key) => release_key_handler(
                        &mut previous_key,
                        key,
                        &single_hotkeys,
                        &key_writer,
                        &mut flags,
                    ),
                    KeyRecorderBehavior::SendKey(key) => send_key_handler(
                        &mut previous_key,
                        key,
                        &pair_hotkeys,
                        &key_writer,
                        &all_input_keys,
                        &tx_clone,
                        &mut flags,
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
