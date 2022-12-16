mod read_keys;
mod write_keys;

pub use crate::read_keys::{
    AddLayer, KeyConfig, KeyConfigUnit, KeyInput, PairHotkeyEntry, SingleHotkeyEntry,
};
use crate::read_keys::{KeyReceiver, ToKeyRecorder};
pub use evdev::Key;
use evdev::{Device, InputEvent, InputEventKind};
pub use evdev_keys;
use std::{
    process::exit,
    sync::mpsc::{channel, Receiver},
    thread,
};

fn get_keyboard_devices() -> impl Iterator<Item = Device> {
    evdev::enumerate().filter_map(|(_, device)| {
        if device.supported_keys().map_or(false, |supported_keys| {
            supported_keys.contains(Key::KEY_A)
                && supported_keys.contains(Key::KEY_Z)
                && supported_keys.contains(Key::KEY_SPACE)
        }) {
            Some(device)
        } else {
            None
        }
    })
}

fn make_read_channel(devices: impl Iterator<Item = Device>) -> Receiver<InputEvent> {
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

pub trait KeyConfigRun {
    fn run(self);
}

impl<T: ToKeyRecorder> KeyConfigRun for KeyConfig<T> {
    fn run(self) {
        let keyboards = get_keyboard_devices().collect::<Vec<_>>();
        if keyboards.is_empty() {
            eprintln!("keyboard not found");
            exit(1);
        }
        let mut key_recorder = self.layers.to_key_recorder();
        log::info!("config loaded");
        for input_event in make_read_channel(keyboards.into_iter()) {
            if let InputEventKind::Key(key) = input_event.kind() {
                if input_event.value() == 1 && Some(key) == self.emergency_stop_key {
                    break;
                }
                let key = KeyInput(key, input_event.value().into());
                key_recorder.send_key(key, input_event.timestamp());
            }
        }
    }
}
