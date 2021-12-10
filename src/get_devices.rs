// use evdev::{Device, Key};

// pub fn get_keyboard_devices() -> Vec<Device> {
//     evdev::enumerate().filter(|device| {
//         device.supported_keys().map_or(false, |supported_keys| {
//             supported_keys.contains(Key::KEY_A)
//                 && supported_keys.contains(Key::KEY_Z)
//                 && supported_keys.contains(Key::KEY_SPACE)
//         })
//     });
// }
