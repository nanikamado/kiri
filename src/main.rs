use evdev_rs::{GrabMode, UninitDevice};
use read_keys::{KeyConfig, KeyConfigEntry};
use std::{fs::File, thread};
use evdev_rs::enums::EV_KEY::{self, *};

mod print_info;
mod read_events;
mod read_keys;
mod write_keys;

fn usage() {
    println!("Usage: evtest /path/to/device");
}

fn mk_config() -> KeyConfig<'static>{
    let key_config_r: &[(&[u64], &[EV_KEY], _, _)] = &[
        (&[0, 1], &[KEY_HENKAN], &[KEY_ENTER], None),
        (&[0, 1], &[KEY_MUHENKAN], &[KEY_BACKSPACE], None),
        (&[0], &[KEY_KPPLUS], &[KEY_KPPLUS], Some(1)),
        (&[1], &[KEY_KPPLUS], &[KEY_KPPLUS], Some(0)),
        (&[1], &[KEY_KPSLASH], &[KEY_2], None),
        (&[1], &[KEY_KPSLASH, KEY_KPASTERISK], &[KEY_4], None),
        // (&[1], &[KEY_KPSLASH], &[KEY_2], None),
    ];
    key_config_r
        .iter()
        .flat_map(|(cs, i, o, t)| {
            cs.iter().map(move |c| KeyConfigEntry {
                cond: *c,
                input: *i,
                output: *o,
                transition: *t,
            })
        })
        .collect()
}

fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        usage();
        std::process::exit(1);
    }
    let path = &args.nth(1).unwrap();
    let f = File::open(path).unwrap();
    let u_d = UninitDevice::new().unwrap();
    let mut d = u_d.set_file(f).unwrap();
    thread::sleep(std::time::Duration::from_secs(1));
    print_info::print_info(&d);
    d.grab(GrabMode::Grab).unwrap();
    read_events::run(d, mk_config());
}
