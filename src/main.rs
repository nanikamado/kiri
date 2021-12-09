use evdev_rs::enums::EV_KEY::{self, *};
use evdev_rs::{GrabMode, UninitDevice};
use read_keys::{KeyConfig, KeyConfigEntry};
use std::{fs::File, thread};

mod print_info;
mod read_events;
mod read_keys;
mod write_keys;

fn usage() {
    println!("Usage: evtest /path/to/device");
}

fn mk_config() -> KeyConfig<'static> {
    let key_config_r: &[(&[u64], &[EV_KEY], &[EV_KEY], _)] = &[
        (&[0, 1], &[KEY_HENKAN], &[KEY_ENTER], None),
        (&[0, 1], &[KEY_MUHENKAN], &[KEY_BACKSPACE], None),
        (&[0], &[KEY_GRAVE], &[KEY_HENKAN], Some(1)),
        (&[1], &[KEY_GRAVE], &[KEY_MUHENKAN], Some(0)),
        //
        (&[1], &[KEY_A], &[KEY_N, KEY_O], None),
        (&[1], &[KEY_S], &[KEY_T, KEY_O], None),
        (&[1], &[KEY_D], &[KEY_K, KEY_A], None),
        (&[1], &[KEY_F], &[KEY_N, KEY_N], None),
        (&[1], &[KEY_G], &[KEY_L, KEY_T, KEY_U], None),
        (&[1], &[KEY_D, KEY_H], &[KEY_H, KEY_E], None),
        (&[1], &[KEY_D, KEY_J], &[KEY_A], None),
        (&[1], &[KEY_D, KEY_SEMICOLON], &[KEY_E], None),
        (&[1], &[KEY_D, KEY_N], &[KEY_S, KEY_E], None),
        (&[1], &[KEY_D, KEY_M], &[KEY_N, KEY_E], None),
        (&[1], &[KEY_D, KEY_COMMA], &[KEY_B, KEY_E], None),
        (&[1], &[KEY_D, KEY_DOT], &[KEY_P, KEY_U], None),
        (&[1], &[KEY_D, KEY_SLASH], &[KEY_V, KEY_U], None),
        (&[1], &[KEY_D, KEY_Y], &[KEY_W, KEY_I], None),
        (&[1], &[KEY_D, KEY_U], &[KEY_P, KEY_A], None),
        (&[1], &[KEY_D, KEY_I], &[KEY_Y, KEY_O], None),
        (&[1], &[KEY_D, KEY_O], &[KEY_M, KEY_I], None),
        (&[1], &[KEY_D, KEY_P], &[KEY_W, KEY_E], None),
        (&[1], &[KEY_D, KEY_LEFTBRACE], &[KEY_U, KEY_L, KEY_O], None),
        (&[1], &[KEY_H], &[KEY_K, KEY_U], None),
        (&[1], &[KEY_J], &[KEY_U], None),
        (&[1], &[KEY_K], &[KEY_I], None),
        (&[1], &[KEY_L], &[KEY_S, KEY_H, KEY_I], None),
        (&[1], &[KEY_SEMICOLON], &[KEY_N, KEY_A], None),
        (&[1], &[KEY_I, KEY_1], &[KEY_L, KEY_Y, KEY_U], None),
        (&[1], &[KEY_I, KEY_2], &[KEY_B, KEY_Y, KEY_A], None),
        (&[1], &[KEY_I, KEY_3], &[KEY_B, KEY_Y, KEY_U], None),
        (&[1], &[KEY_I, KEY_4], &[KEY_B, KEY_Y, KEY_O], None),
        (&[1], &[KEY_I, KEY_A], &[KEY_H, KEY_Y, KEY_O], None),
        (&[1], &[KEY_I, KEY_F], &[KEY_K, KEY_Y, KEY_O], None),
        (&[1], &[KEY_I, KEY_G], &[KEY_C, KEY_H, KEY_O], None),
        (&[1], &[KEY_I, KEY_Q], &[KEY_H, KEY_Y, KEY_U], None),
        (&[1], &[KEY_I, KEY_W], &[KEY_S, KEY_Y, KEY_U], None),
        (&[1], &[KEY_I, KEY_E], &[KEY_S, KEY_Y, KEY_O], None),
        (&[1], &[KEY_I, KEY_R], &[KEY_K, KEY_Y, KEY_U], None),
        (&[1], &[KEY_I, KEY_T], &[KEY_C, KEY_H, KEY_U], None),
        (&[1], &[KEY_I, KEY_Z], &[KEY_H, KEY_Y, KEY_A], None),
        (&[1], &[KEY_I, KEY_C], &[KEY_S, KEY_H, KEY_A], None),
        (&[1], &[KEY_I, KEY_V], &[KEY_K, KEY_Y, KEY_A], None),
        (&[1], &[KEY_I, KEY_B], &[KEY_C, KEY_H, KEY_A], None),
        (&[1], &[KEY_K, KEY_1], &[KEY_L, KEY_A], None),
        (&[1], &[KEY_K, KEY_2], &[KEY_L, KEY_I], None),
        (&[1], &[KEY_K, KEY_3], &[KEY_L, KEY_U], None),
        (&[1], &[KEY_K, KEY_4], &[KEY_L, KEY_E], None),
        (&[1], &[KEY_K, KEY_5], &[KEY_L, KEY_O], None),
        (&[1], &[KEY_K, KEY_A], &[KEY_H, KEY_O], None),
        (&[1], &[KEY_K, KEY_S], &[KEY_J, KEY_I], None),
        (&[1], &[KEY_K, KEY_D], &[KEY_R, KEY_E], None),
        (&[1], &[KEY_K, KEY_F], &[KEY_M, KEY_O], None),
        (&[1], &[KEY_K, KEY_G], &[KEY_Y, KEY_U], None),
        (&[1], &[KEY_K, KEY_Q], &[KEY_F, KEY_A], None),
        (&[1], &[KEY_K, KEY_W], &[KEY_G, KEY_O], None),
        (&[1], &[KEY_K, KEY_E], &[KEY_F, KEY_U], None),
        (&[1], &[KEY_K, KEY_R], &[KEY_F, KEY_I], None),
        (&[1], &[KEY_K, KEY_T], &[KEY_F, KEY_E], None),
        (&[1], &[KEY_K, KEY_Z], &[KEY_D, KEY_U], None),
        (&[1], &[KEY_K, KEY_X], &[KEY_Z, KEY_O], None),
        (&[1], &[KEY_K, KEY_C], &[KEY_B, KEY_O], None),
        (&[1], &[KEY_K, KEY_V], &[KEY_M, KEY_U], None),
        (&[1], &[KEY_K, KEY_B], &[KEY_F, KEY_O], None),
        (&[1], &[KEY_L, KEY_1], &[KEY_L, KEY_Y, KEY_A], None),
        (&[1], &[KEY_L, KEY_2], &[KEY_M, KEY_Y, KEY_A], None),
        (&[1], &[KEY_L, KEY_3], &[KEY_M, KEY_Y, KEY_U], None),
        (&[1], &[KEY_L, KEY_4], &[KEY_M, KEY_Y, KEY_O], None),
        (&[1], &[KEY_L, KEY_5], &[KEY_W, KEY_A], None),
        (&[1], &[KEY_L, KEY_A], &[KEY_W, KEY_O], None),
        (&[1], &[KEY_L, KEY_S], &[KEY_S, KEY_A], None),
        (&[1], &[KEY_L, KEY_D], &[KEY_O], None),
        (&[1], &[KEY_L, KEY_F], &[KEY_R, KEY_I], None),
        (&[1], &[KEY_L, KEY_G], &[KEY_Z, KEY_U], None),
        (&[1], &[KEY_L, KEY_Q], &[KEY_D, KEY_I], None),
        (&[1], &[KEY_L, KEY_W], &[KEY_M, KEY_E], None),
        (&[1], &[KEY_L, KEY_E], &[KEY_K, KEY_E], None),
        (&[1], &[KEY_L, KEY_R], &[KEY_T, KEY_E, KEY_L, KEY_I], None),
        (&[1], &[KEY_L, KEY_T], &[KEY_D, KEY_E, KEY_L, KEY_I], None),
        (&[1], &[KEY_L, KEY_Z], &[KEY_Z, KEY_E], None),
        (&[1], &[KEY_L, KEY_X], &[KEY_Z, KEY_A], None),
        (&[1], &[KEY_L, KEY_C], &[KEY_G, KEY_I], None),
        (&[1], &[KEY_L, KEY_V], &[KEY_R, KEY_O], None),
        (&[1], &[KEY_L, KEY_B], &[KEY_N, KEY_U], None),
        (&[1], &[KEY_N], &[KEY_T, KEY_E], None),
        (&[1], &[KEY_M], &[KEY_T, KEY_A], None),
        (&[1], &[KEY_COMMA], &[KEY_D, KEY_E], None),
        (&[1], &[KEY_DOT], &[KEY_DOT], None),
        (&[1], &[KEY_SLASH], &[KEY_B, KEY_U], None),
        (&[1], &[KEY_O, KEY_1], &[KEY_L, KEY_Y, KEY_O], None),
        (&[1], &[KEY_O, KEY_2], &[KEY_P, KEY_Y, KEY_A], None),
        (&[1], &[KEY_O, KEY_3], &[KEY_P, KEY_Y, KEY_U], None),
        (&[1], &[KEY_O, KEY_4], &[KEY_P, KEY_Y, KEY_O], None),
        (&[1], &[KEY_O, KEY_A], &[KEY_R, KEY_Y, KEY_O], None),
        (&[1], &[KEY_O, KEY_F], &[KEY_G, KEY_Y, KEY_O], None),
        (&[1], &[KEY_O, KEY_G], &[KEY_N, KEY_Y, KEY_O], None),
        (&[1], &[KEY_O, KEY_Q], &[KEY_R, KEY_Y, KEY_U], None),
        (&[1], &[KEY_O, KEY_W], &[KEY_J, KEY_U], None),
        (&[1], &[KEY_O, KEY_E], &[KEY_J, KEY_O], None),
        (&[1], &[KEY_O, KEY_R], &[KEY_G, KEY_Y, KEY_U], None),
        (&[1], &[KEY_O, KEY_T], &[KEY_N, KEY_Y, KEY_U], None),
        (&[1], &[KEY_O, KEY_Z], &[KEY_R, KEY_Y, KEY_A], None),
        (&[1], &[KEY_O, KEY_C], &[KEY_J, KEY_A], None),
        (&[1], &[KEY_O, KEY_V], &[KEY_G, KEY_Y, KEY_A], None),
        (&[1], &[KEY_O, KEY_B], &[KEY_N, KEY_Y, KEY_A], None),
        (&[1], &[KEY_Q], &[KEY_MINUS], None),
        (&[1], &[KEY_W], &[KEY_N, KEY_I], None),
        (&[1], &[KEY_E], &[KEY_H, KEY_A], None),
        (&[1], &[KEY_R], &[KEY_COMMA], None),
        (&[1], &[KEY_T], &[KEY_C, KEY_H, KEY_I], None),
        (&[1], &[KEY_S, KEY_H], &[KEY_B, KEY_I], None),
        (&[1], &[KEY_S, KEY_J], &[KEY_R, KEY_A], None),
        (&[1], &[KEY_S, KEY_SEMICOLON], &[KEY_S, KEY_O], None),
        (&[1], &[KEY_S, KEY_N], &[KEY_W, KEY_A], None),
        (&[1], &[KEY_S, KEY_M], &[KEY_D, KEY_A], None),
        (&[1], &[KEY_S, KEY_COMMA], &[KEY_P, KEY_E], None),
        (&[1], &[KEY_S, KEY_DOT], &[KEY_P, KEY_O], None),
        (&[1], &[KEY_S, KEY_SLASH], &[KEY_T, KEY_Y, KEY_E], None),
        (&[1], &[KEY_S, KEY_Y], &[KEY_S, KEY_Y, KEY_E], None),
        (&[1], &[KEY_S, KEY_U], &[KEY_P, KEY_E], None),
        (&[1], &[KEY_S, KEY_I], &[KEY_D, KEY_O], None),
        (&[1], &[KEY_S, KEY_O], &[KEY_Y, KEY_A], None),
        (&[1], &[KEY_S, KEY_P], &[KEY_S, KEY_Y, KEY_E], None),
        (&[1], &[KEY_Y], &[KEY_G, KEY_U], None),
        (&[1], &[KEY_U], &[KEY_B, KEY_A], None),
        (&[1], &[KEY_I], &[KEY_K, KEY_O], None),
        (&[1], &[KEY_O], &[KEY_G, KEY_A], None),
        (&[1], &[KEY_P], &[KEY_H, KEY_I], None),
        (&[1], &[KEY_LEFTBRACE], &[KEY_G, KEY_E], None),
        (&[1], &[KEY_Z], &[KEY_S, KEY_U], None),
        (&[1], &[KEY_X], &[KEY_M, KEY_A], None),
        (&[1], &[KEY_C], &[KEY_K, KEY_I], None),
        (&[1], &[KEY_V], &[KEY_R, KEY_U], None),
        (&[1], &[KEY_B], &[KEY_T, KEY_U], None),
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
