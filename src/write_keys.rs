use evdev_rs::{enums, Device, InputEvent, TimeVal, UInputDevice};

pub struct KeyWriter {
    device: UInputDevice,
}

impl KeyWriter {
    pub fn new(d: &Device) -> KeyWriter {
        KeyWriter {
            device: UInputDevice::create_from_device(d).unwrap(),
        }
    }

    pub fn put_with_time(&self, key: evdev_rs::enums::EV_KEY, time: &TimeVal) {
        self.device
            .write_event(&InputEvent::new(time, &enums::EventCode::EV_KEY(key), 1))
            .unwrap();
        self.device
            .write_event(&InputEvent::new(
                time,
                &enums::EventCode::EV_SYN(enums::EV_SYN::SYN_REPORT),
                0,
            ))
            .unwrap();
        self.device
            .write_event(&InputEvent::new(time, &enums::EventCode::EV_KEY(key), 0))
            .unwrap();
        self.device
            .write_event(&InputEvent::new(
                time,
                &enums::EventCode::EV_SYN(enums::EV_SYN::SYN_REPORT),
                0,
            ))
            .unwrap();
    }

    pub fn write_event(&self, event: &InputEvent) -> Result<(), std::io::Error> {
        self.device.write_event(event)?;
        self.device.write_event(&InputEvent::new(
                &event.time,
                &enums::EventCode::EV_SYN(enums::EV_SYN::SYN_REPORT),
                0,
            ))
    }
}
