use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    Device, EventType, InputEvent, Key,
};

use crate::read_keys::KeyInput;
// use evdev_rs::{enums, Device, InputEvent, TimeVal, UInputDevice};

pub struct KeyWriter {
    device: VirtualDevice,
}

impl KeyWriter {
    pub fn new(d: &Device) -> KeyWriter {
        KeyWriter {
            device: VirtualDeviceBuilder::new()
                .unwrap()
                .name(b"kiri virtual keyboard")
                .with_keys(d.supported_keys().unwrap())
                .unwrap()
                .input_id(d.input_id())
                .build()
                .unwrap(),
        }
    }

    pub fn put_with_time(&mut self, key: Key) {
        let msg = [
            InputEvent::new(EventType::KEY, key.code(), 1),
            InputEvent::new(EventType::KEY, key.code(), 0),
        ];
        self.device.emit(&msg).unwrap();
    }

    pub fn fire_key_input(&mut self, key: KeyInput) {
        log::debug!("{:?}", key);
        let msg = [InputEvent::new(EventType::KEY, key.0.code(), key.1.into())];
        self.device.emit(&msg).unwrap();
    }

    pub fn write_event(&mut self, event: &InputEvent) -> Result<(), std::io::Error> {
        log::debug!("{:?}", event);
        self.device.emit(&[*event])
    }
}
