use evdev::{Device, InputEventKind, Key};
use std::collections::HashSet;
use crate::read_keys::{KeyConfig, KeyInputWithRepeat, KeyRecorder};

pub fn run(mut d: Device, config: KeyConfig) {
    env_logger::init();
    let key_recorder = KeyRecorder::new(&d, config.clone());
    log::info!("config loaded");
    let shadowed_keys: HashSet<_> = config.shadowed_keys;
    loop {
        for input_event in d.fetch_events().expect("cannot read device") {
            log::debug!("{:?}", input_event.kind());
            log::debug!("{:?}", input_event.value());
            if let InputEventKind::Key(key) = input_event.kind()
            {
                if input_event.value() == 1 && Key::KEY_CALC == key {
                    break;
                }
                let key = KeyInputWithRepeat(key, input_event.value().into());
                if shadowed_keys.contains(&key.into()) {
                    key_recorder.send_key(key, input_event.timestamp());
                } else {
                    key_recorder.event_write(input_event);
                }
            }
        }
    }
}
