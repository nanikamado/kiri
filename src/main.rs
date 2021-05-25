use evdev_rs::{GrabMode, UninitDevice};
use std::{fs::File, thread};

mod print_info;
mod read_events;
mod read_keys;
mod write_keys;
mod read_config;

fn usage() {
    println!("Usage: evtest /path/to/device");
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
    read_events::run(d);
}
