use crate::read_keys::KeyInput;
use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};
use evdev::{EventType, InputEvent};
use std::io;

pub struct KeyWriter {
    device: VirtualDevice,
}

impl KeyWriter {
    pub fn new() -> Result<KeyWriter, io::Error> {
        let mut key_set = evdev::AttributeSet::<evdev::Key>::new();
        evdev_keys::all_keys().for_each(|key| {
            key_set.insert(key);
        });
        Ok(KeyWriter {
            device: VirtualDeviceBuilder::new()?
                .name(b"kiri virtual keyboard")
                .with_keys(&key_set)
                .unwrap()
                .build()
                .unwrap(),
        })
    }

    pub fn fire_key_input(&mut self, key: KeyInput) {
        log::debug!("-----> {:?}", key);
        let msg = [InputEvent::new(EventType::KEY, key.0.code(), key.1.into())];
        self.device.emit(&msg).unwrap();
        std::thread::sleep(core::time::Duration::from_millis(5));
    }
}
