mod read_keys;
mod write_keys;

pub use crate::read_keys::{
    AddLayer, KeyConfig, KeyInput, PairRemapEntry, RemapLayer, SingleRemapEntry,
};
use crate::read_keys::{KeyReceiver, ToKeyRecorder};
pub use evdev::Key;
use evdev::{Device, InputEvent, InputEventKind};
pub use evdev_keys;
use std::process::exit;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

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
        if let Err(e) = d.grab() {
            match e.raw_os_error() {
                Some(16) => {
                    log::error!(
                        "Could not grab \"{}\". {e}. \
                        Maybe there is another key remapper running.",
                        d.name().unwrap_or("unknown"),
                    )
                }
                _ => {
                    log::error!("Could not grab \"{}\". {e}.", d.name().unwrap_or("unknown"),);
                }
            }
        } else {
            log::info!("Successfully grabed \"{}\".", d.name().unwrap_or("unknown"))
        }

        thread::spawn(move || loop {
            for input_event in d.fetch_events().expect("Cannot read device") {
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
            eprintln!("Keyboard not found");
            exit(1);
        }
        match self.layers.to_key_recorder() {
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::PermissionDenied => {
                        eprintln!("{e}");
                        eprintln!(
                            "Kiri has to be run with superuser privileges. \
                            Retry with sudo."
                        );
                    }
                    _ => {
                        eprintln!("{e}");
                    }
                };
                exit(1)
            }
            Ok(mut key_recorder) => {
                log::info!("Config loaded");
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
    }
}
