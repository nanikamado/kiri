use std::collections::HashSet;

use evdev_rs::enums::EventCode;
use evdev_rs::{Device, InputEvent, ReadFlag, ReadStatus};

use crate::read_keys::{KeyConfig, KeyRecorder};

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

pub fn run(d: Device, config: KeyConfig<'static>) {
    let key_recorder = KeyRecorder::new(&d, &config);
    let watching_keys: HashSet<_> = config
        .pair_hotkeys
        .iter()
        .flat_map(|s| s.input.iter())
        .chain(config.single_hotkeys.iter().map(|s| &s.input))
        .collect();
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
                        if input_event_velue == 1 || input_event_velue == 2 {
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
