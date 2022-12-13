use crate::read_keys::{KeyConfig, KeyInput, KeyRecorder};
use env_logger::Env;
use evdev::{Device, InputEvent, InputEventKind, Key};
use std::{
    hash::Hash,
    process::exit,
    sync::mpsc::{channel, Receiver},
    thread,
};

pub fn get_keyboard_devices() -> impl Iterator<Item = Device> {
    evdev::enumerate().filter(|device| {
        device.supported_keys().map_or(false, |supported_keys| {
            supported_keys.contains(Key::KEY_A)
                && supported_keys.contains(Key::KEY_Z)
                && supported_keys.contains(Key::KEY_SPACE)
        })
    })
}

pub fn make_read_channel(devices: impl Iterator<Item = Device>) -> Receiver<InputEvent> {
    let (tx, rx) = channel();
    for mut d in devices {
        let tx = tx.clone();
        d.grab().unwrap();
        thread::spawn(move || loop {
            for input_event in d.fetch_events().expect("cannot read device") {
                tx.send(input_event).unwrap();
            }
        });
    }
    rx
}

pub fn run<State: Eq + Copy + std::fmt::Debug + Hash + Send>(config: KeyConfig<State>) {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let keyboards = get_keyboard_devices().collect::<Vec<_>>();
    if keyboards.is_empty() {
        eprintln!("keyboard not found");
        exit(1);
    }
    let mut key_recorder = KeyRecorder::new(config);
    log::info!("config loaded");
    for input_event in make_read_channel(keyboards.into_iter()) {
        if let InputEventKind::Key(key) = input_event.kind() {
            if input_event.value() == 1 && Key::KEY_CALC == key {
                break;
            }
            let key = KeyInput(key, input_event.value().into());
            key_recorder.send_key(key, input_event.timestamp());
        }
    }
}
