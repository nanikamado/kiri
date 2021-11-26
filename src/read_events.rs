use std::collections::HashSet;

use evdev_rs::enums::EventCode;
use evdev_rs::{Device, InputEvent, ReadFlag, ReadStatus};

use crate::read_keys::KeyRecorder;

fn print_event(ev: &InputEvent) {
    match ev.event_code {
        EventCode::EV_SYN(_) => println!(
            "Event: time {}.{}, ++++++++++++++++++++ {} +++++++++++++++",
            ev.time.tv_sec,
            ev.time.tv_usec,
            ev.event_type().unwrap()
        ),
        _ => println!(
            "Event: time {}.{}, type {} , code {} , value {}",
            ev.time.tv_sec,
            ev.time.tv_usec,
            ev.event_type()
                .map(|ev_type| format!("{}", ev_type))
                .unwrap_or_else(|| "None".to_owned()),
            ev.event_code,
            ev.value
        ),
    }
}

fn print_sync_dropped_event(ev: &InputEvent) {
    print!("SYNC DROPPED: ");
    print_event(ev);
}

use evdev_rs::enums::EV_KEY::{self, *};

static KEY_SETTINGS: &[(&[EV_KEY], &[EV_KEY])] = &[
    (&[KEY_G, KEY_I], &[KEY_C, KEY_H, KEY_O]),
    (&[KEY_G], &[KEY_L, KEY_T, KEY_U]),
    (&[KEY_S], &[KEY_T, KEY_O]),
    (&[KEY_S, KEY_L], &[KEY_S, KEY_A]),
    (&[KEY_9], &[KEY_MUHENKAN]),
    (&[KEY_0], &[KEY_HENKAN]),
    (&[KEY_HENKAN], &[KEY_ENTER]),
    (&[KEY_MUHENKAN], &[KEY_BACKSPACE]),
];

pub fn run(d: Device) {
    let key_recorder = KeyRecorder::new(&d, KEY_SETTINGS);
    let watching_keys: HashSet<_> = KEY_SETTINGS.iter().flat_map(|(i, _)| i.iter()).collect();
    loop {
        match d.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING) {
            Ok((ReadStatus::Success, e)) => {
                println!("{}, {}", e.event_code, e.value);
                if let InputEvent {
                    event_code: EventCode::EV_KEY(key),
                    time,
                    value: input_event_velue,
                    ..
                } = e
                {
                    if input_event_velue == 1 && evdev_rs::enums::EV_KEY::KEY_ESC == key {
                        break;
                    }
                    if watching_keys.contains(&key) {
                        if input_event_velue == 1 {
                            key_recorder.send_key(key, time);
                        }
                    } else {
                        key_recorder.event_write(e)
                    }
                }
            }
            Ok((ReadStatus::Sync, e)) => {
                println!("::::::::::::::::::::: dropped ::::::::::::::::::::::");
                print_sync_dropped_event(&e);
                while let Ok((ReadStatus::Sync, e)) = d.next_event(ReadFlag::SYNC) {
                    print_sync_dropped_event(&e)
                }
                println!("::::::::::::::::::::: re-synced ::::::::::::::::::::");
            }
            Err(err) => {
                if !matches!(err.raw_os_error(), Some(libc::EAGAIN)) {
                    println!("{}", err);
                    break;
                }
            }
        }
    }
}
