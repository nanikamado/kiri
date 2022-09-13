use evdev::Key;
use evdev_keys::*;
use read_keys::{KeyConfigUnit, KeyInput, PairHotkeyEntry, SingleHotkeyEntry, State, Transition};

mod read_events;
mod read_keys;
mod write_keys;

fn mk_config() -> KeyConfigUnit {
    // 0  : normal
    // 7   : jp input
    // 1   : jp input with modifiers
    let singeta_config: &[(&[Key], &[Key])] = &[
        (&[KEY_A], &[KEY_N, KEY_O]),
        (&[KEY_S], &[KEY_T, KEY_O]),
        (&[KEY_D], &[KEY_K, KEY_A]),
        (&[KEY_F], &[KEY_N, KEY_N]),
        (&[KEY_G], &[KEY_L, KEY_T, KEY_U]),
        (&[KEY_D, KEY_H], &[KEY_H, KEY_E]),
        (&[KEY_D, KEY_J], &[KEY_A]),
        (&[KEY_D, KEY_SEMICOLON], &[KEY_E]),
        (&[KEY_D, KEY_N], &[KEY_S, KEY_E]),
        (&[KEY_D, KEY_M], &[KEY_N, KEY_E]),
        (&[KEY_D, KEY_COMMA], &[KEY_B, KEY_E]),
        (&[KEY_D, KEY_DOT], &[KEY_P, KEY_U]),
        (&[KEY_D, KEY_SLASH], &[KEY_V, KEY_U]),
        (&[KEY_D, KEY_Y], &[KEY_W, KEY_I]),
        (&[KEY_D, KEY_U], &[KEY_P, KEY_A]),
        (&[KEY_D, KEY_I], &[KEY_Y, KEY_O]),
        (&[KEY_D, KEY_O], &[KEY_M, KEY_I]),
        (&[KEY_D, KEY_P], &[KEY_W, KEY_E]),
        (&[KEY_D, KEY_LEFTBRACE], &[KEY_U, KEY_L, KEY_O]),
        (&[KEY_H], &[KEY_K, KEY_U]),
        (&[KEY_J], &[KEY_U]),
        (&[KEY_K], &[KEY_I]),
        (&[KEY_L], &[KEY_S, KEY_H, KEY_I]),
        (&[KEY_SEMICOLON], &[KEY_N, KEY_A]),
        (&[KEY_I, KEY_1], &[KEY_L, KEY_Y, KEY_U]),
        (&[KEY_I, KEY_2], &[KEY_B, KEY_Y, KEY_A]),
        (&[KEY_I, KEY_3], &[KEY_B, KEY_Y, KEY_U]),
        (&[KEY_I, KEY_4], &[KEY_B, KEY_Y, KEY_O]),
        (&[KEY_I, KEY_A], &[KEY_H, KEY_Y, KEY_O]),
        (&[KEY_I, KEY_F], &[KEY_K, KEY_Y, KEY_O]),
        (&[KEY_I, KEY_G], &[KEY_C, KEY_H, KEY_O]),
        (&[KEY_I, KEY_Q], &[KEY_H, KEY_Y, KEY_U]),
        (&[KEY_I, KEY_W], &[KEY_S, KEY_Y, KEY_U]),
        (&[KEY_I, KEY_E], &[KEY_S, KEY_Y, KEY_O]),
        (&[KEY_I, KEY_R], &[KEY_K, KEY_Y, KEY_U]),
        (&[KEY_I, KEY_T], &[KEY_C, KEY_H, KEY_U]),
        (&[KEY_I, KEY_Z], &[KEY_H, KEY_Y, KEY_A]),
        (&[KEY_I, KEY_C], &[KEY_S, KEY_H, KEY_A]),
        (&[KEY_I, KEY_V], &[KEY_K, KEY_Y, KEY_A]),
        (&[KEY_I, KEY_B], &[KEY_C, KEY_H, KEY_A]),
        (&[KEY_K, KEY_1], &[KEY_L, KEY_A]),
        (&[KEY_K, KEY_2], &[KEY_L, KEY_I]),
        (&[KEY_K, KEY_3], &[KEY_L, KEY_U]),
        (&[KEY_K, KEY_4], &[KEY_L, KEY_E]),
        (&[KEY_K, KEY_5], &[KEY_L, KEY_O]),
        (&[KEY_K, KEY_A], &[KEY_H, KEY_O]),
        (&[KEY_K, KEY_S], &[KEY_J, KEY_I]),
        (&[KEY_K, KEY_D], &[KEY_R, KEY_E]),
        (&[KEY_K, KEY_F], &[KEY_M, KEY_O]),
        (&[KEY_K, KEY_G], &[KEY_Y, KEY_U]),
        (&[KEY_K, KEY_Q], &[KEY_F, KEY_A]),
        (&[KEY_K, KEY_W], &[KEY_G, KEY_O]),
        (&[KEY_K, KEY_E], &[KEY_F, KEY_U]),
        (&[KEY_K, KEY_R], &[KEY_F, KEY_I]),
        (&[KEY_K, KEY_T], &[KEY_F, KEY_E]),
        (&[KEY_K, KEY_Z], &[KEY_D, KEY_U]),
        (&[KEY_K, KEY_X], &[KEY_Z, KEY_O]),
        (&[KEY_K, KEY_C], &[KEY_B, KEY_O]),
        (&[KEY_K, KEY_V], &[KEY_M, KEY_U]),
        (&[KEY_K, KEY_B], &[KEY_F, KEY_O]),
        (&[KEY_L, KEY_1], &[KEY_L, KEY_Y, KEY_A]),
        (&[KEY_L, KEY_2], &[KEY_M, KEY_Y, KEY_A]),
        (&[KEY_L, KEY_3], &[KEY_M, KEY_Y, KEY_U]),
        (&[KEY_L, KEY_4], &[KEY_M, KEY_Y, KEY_O]),
        (&[KEY_L, KEY_5], &[KEY_W, KEY_A]),
        (&[KEY_L, KEY_A], &[KEY_W, KEY_O]),
        (&[KEY_L, KEY_S], &[KEY_S, KEY_A]),
        (&[KEY_L, KEY_D], &[KEY_O]),
        (&[KEY_L, KEY_F], &[KEY_R, KEY_I]),
        (&[KEY_L, KEY_G], &[KEY_Z, KEY_U]),
        (&[KEY_L, KEY_Q], &[KEY_D, KEY_I]),
        (&[KEY_L, KEY_W], &[KEY_M, KEY_E]),
        (&[KEY_L, KEY_E], &[KEY_K, KEY_E]),
        (&[KEY_L, KEY_R], &[KEY_T, KEY_E, KEY_L, KEY_I]),
        (&[KEY_L, KEY_T], &[KEY_D, KEY_E, KEY_L, KEY_I]),
        (&[KEY_L, KEY_Z], &[KEY_Z, KEY_E]),
        (&[KEY_L, KEY_X], &[KEY_Z, KEY_A]),
        (&[KEY_L, KEY_C], &[KEY_G, KEY_I]),
        (&[KEY_L, KEY_V], &[KEY_R, KEY_O]),
        (&[KEY_L, KEY_B], &[KEY_N, KEY_U]),
        (&[KEY_N], &[KEY_T, KEY_E]),
        (&[KEY_M], &[KEY_T, KEY_A]),
        (&[KEY_COMMA], &[KEY_D, KEY_E]),
        (&[KEY_DOT], &[KEY_DOT]),
        (&[KEY_SLASH], &[KEY_B, KEY_U]),
        (&[KEY_O, KEY_1], &[KEY_L, KEY_Y, KEY_O]),
        (&[KEY_O, KEY_2], &[KEY_P, KEY_Y, KEY_A]),
        (&[KEY_O, KEY_3], &[KEY_P, KEY_Y, KEY_U]),
        (&[KEY_O, KEY_4], &[KEY_P, KEY_Y, KEY_O]),
        (&[KEY_O, KEY_A], &[KEY_R, KEY_Y, KEY_O]),
        (&[KEY_O, KEY_F], &[KEY_G, KEY_Y, KEY_O]),
        (&[KEY_O, KEY_G], &[KEY_N, KEY_Y, KEY_O]),
        (&[KEY_O, KEY_Q], &[KEY_R, KEY_Y, KEY_U]),
        (&[KEY_O, KEY_W], &[KEY_J, KEY_U]),
        (&[KEY_O, KEY_E], &[KEY_J, KEY_O]),
        (&[KEY_O, KEY_R], &[KEY_G, KEY_Y, KEY_U]),
        (&[KEY_O, KEY_T], &[KEY_N, KEY_Y, KEY_U]),
        (&[KEY_O, KEY_Z], &[KEY_R, KEY_Y, KEY_A]),
        (&[KEY_O, KEY_C], &[KEY_J, KEY_A]),
        (&[KEY_O, KEY_V], &[KEY_G, KEY_Y, KEY_A]),
        (&[KEY_O, KEY_B], &[KEY_N, KEY_Y, KEY_A]),
        (&[KEY_Q], &[KEY_MINUS]),
        (&[KEY_W], &[KEY_N, KEY_I]),
        (&[KEY_E], &[KEY_H, KEY_A]),
        (&[KEY_R], &[KEY_COMMA]),
        (&[KEY_T], &[KEY_C, KEY_H, KEY_I]),
        (&[KEY_S, KEY_H], &[KEY_B, KEY_I]),
        (&[KEY_S, KEY_J], &[KEY_R, KEY_A]),
        (&[KEY_S, KEY_SEMICOLON], &[KEY_S, KEY_O]),
        (&[KEY_S, KEY_N], &[KEY_W, KEY_A]),
        (&[KEY_S, KEY_M], &[KEY_D, KEY_A]),
        (&[KEY_S, KEY_COMMA], &[KEY_P, KEY_I]),
        (&[KEY_S, KEY_DOT], &[KEY_P, KEY_O]),
        (&[KEY_S, KEY_SLASH], &[KEY_T, KEY_Y, KEY_E]),
        (&[KEY_S, KEY_Y], &[KEY_S, KEY_Y, KEY_E]),
        (&[KEY_S, KEY_U], &[KEY_P, KEY_E]),
        (&[KEY_S, KEY_I], &[KEY_D, KEY_O]),
        (&[KEY_S, KEY_O], &[KEY_Y, KEY_A]),
        (&[KEY_S, KEY_P], &[KEY_J, KEY_E]),
        (&[KEY_Y], &[KEY_G, KEY_U]),
        (&[KEY_U], &[KEY_B, KEY_A]),
        (&[KEY_I], &[KEY_K, KEY_O]),
        (&[KEY_O], &[KEY_G, KEY_A]),
        (&[KEY_P], &[KEY_H, KEY_I]),
        (&[KEY_LEFTBRACE], &[KEY_G, KEY_E]),
        (&[KEY_Z], &[KEY_S, KEY_U]),
        (&[KEY_X], &[KEY_M, KEY_A]),
        (&[KEY_C], &[KEY_K, KEY_I]),
        (&[KEY_V], &[KEY_R, KEY_U]),
        (&[KEY_B], &[KEY_T, KEY_U]),
    ];
    let mut singeta_config: Vec<(&[State], &[Key], &[Key], Option<Transition>)> = singeta_config
        .iter()
        .map(|(i, o)| -> (&[State], _, _, _) { (&[7], *i, *o, None) })
        .collect();
    let key_config_r: &[(&[State], &[Key], &[Key], Option<Transition>)] = &[
        (&[7], &[KEY_R, KEY_G], &[KEY_SLASH], None),
        (
            &[7],
            &[KEY_H, KEY_J],
            &[KEY_RIGHTBRACE, KEY_BACKSLASH, KEY_RIGHT],
            None,
        ),
        (&[0], &[KEY_J, KEY_K], &[KEY_RIGHTBRACE], None),
        (&[0], &[KEY_D, KEY_SEMICOLON], &[KEY_END], None),
        (&[0], &[KEY_A, KEY_K], &[KEY_HOME], None),
        (&[0], &[KEY_F, KEY_SEMICOLON], &[KEY_END], None),
        (&[0], &[KEY_A, KEY_J], &[KEY_HOME], None),
    ];
    let key_config_r = {
        let mut k = key_config_r.to_vec();
        k.append(&mut singeta_config);
        k
    };
    let pair_keys_with_modifiers_config: &[(&[State], [Key; 2], Vec<_>, Option<Transition>)] = &[
        (
            &[0, 7],
            [KEY_J, KEY_N],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_SLASH),
                KeyInput::release(KEY_SLASH),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[0, 7],
            [KEY_F, KEY_V],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_1),
                KeyInput::release(KEY_1),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[0, 7],
            [KEY_F, KEY_B],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_1),
                KeyInput::release(KEY_1),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[7],
            [KEY_F, KEY_G],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_8),
                KeyInput::release(KEY_8),
                KeyInput::press(KEY_9),
                KeyInput::release(KEY_9),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[0],
            [KEY_D, KEY_F],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_8),
                KeyInput::release(KEY_8),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[0],
            [KEY_F, KEY_G],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_9),
                KeyInput::release(KEY_9),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[0],
            [KEY_K, KEY_L],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_RO),
                KeyInput::release(KEY_RO),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[0],
            [KEY_E, KEY_O],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_7),
                KeyInput::release(KEY_7),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[0],
            [KEY_W, KEY_I],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_2),
                KeyInput::release(KEY_2),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[0],
            [KEY_F, KEY_J],
            vec![
                KeyInput::press(KEY_LEFTMETA),
                KeyInput::press(KEY_SPACE),
                KeyInput::release(KEY_SPACE),
                KeyInput::release(KEY_LEFTMETA),
            ],
            Some(7),
        ),
        (
            &[7],
            [KEY_F, KEY_J],
            vec![
                KeyInput::press(KEY_LEFTCTRL),
                KeyInput::press(KEY_RIGHTBRACE),
                KeyInput::release(KEY_RIGHTBRACE),
                KeyInput::release(KEY_LEFTCTRL),
                KeyInput::press(KEY_LEFTMETA),
                KeyInput::press(KEY_SPACE),
                KeyInput::release(KEY_SPACE),
                KeyInput::release(KEY_LEFTMETA),
            ],
            Some(0),
        ),
        (
            &[0],
            [KEY_D, KEY_S],
            vec![
                KeyInput::press(KEY_LEFTMETA),
                KeyInput::press(KEY_SPACE),
                KeyInput::release(KEY_SPACE),
                KeyInput::release(KEY_LEFTMETA),
            ],
            Some(7),
        ),
        (
            &[7],
            [KEY_D, KEY_S],
            vec![
                KeyInput::press(KEY_LEFTCTRL),
                KeyInput::press(KEY_RIGHTBRACE),
                KeyInput::release(KEY_RIGHTBRACE),
                KeyInput::release(KEY_LEFTCTRL),
                KeyInput::press(KEY_LEFTMETA),
                KeyInput::press(KEY_SPACE),
                KeyInput::release(KEY_SPACE),
                KeyInput::release(KEY_LEFTMETA),
            ],
            Some(0),
        ),
    ];
    let modifires = [
        KEY_LEFTCTRL,
        KEY_LEFTMETA,
        KEY_LEFTALT,
        KEY_LEFTSHIFT,
        KEY_RIGHTCTRL,
        KEY_RIGHTMETA,
        KEY_RIGHTALT,
        KEY_RIGHTSHIFT,
    ];
    let modifiers_trans = modifires
        .iter()
        .flat_map(|key| {
            [
                (7, KeyInput::press(*key), Some(1)),
                (1, KeyInput::release(*key), Some(7)),
            ]
            .map(|(c, i, t)| SingleHotkeyEntry {
                cond: c,
                input: i,
                output: vec![i],
                transition: t,
                input_canceler: Vec::new(),
            })
        })
        .collect::<Vec<_>>();
    KeyConfigUnit {
        pair_hotkeys: key_config_r
            .iter()
            .filter(|(_, i, _, _)| i.len() == 2)
            .flat_map(|(cs, i, o, t)| {
                cs.iter().map(move |c| PairHotkeyEntry {
                    cond: *c,
                    input: [i[0], i[1]],
                    output_keys: o
                        .iter()
                        .flat_map(|key| [KeyInput::press(*key), KeyInput::release(*key)])
                        .collect(),
                    transition: *t,
                })
            })
            .chain(
                pair_keys_with_modifiers_config
                    .iter()
                    .flat_map(|(cs, i, o, t)| {
                        cs.iter().map(move |c| PairHotkeyEntry {
                            cond: *c,
                            input: *i,
                            output_keys: o.clone(),
                            transition: *t,
                        })
                    }),
            )
            .collect(),
        single_hotkeys: key_config_r
            .iter()
            .filter(|(_, i, _, _)| i.len() == 1)
            .flat_map(|(cs, i, o, t)| {
                cs.iter().map(move |c| SingleHotkeyEntry {
                    cond: *c,
                    input: KeyInput::press(i[0]),
                    output: (*o)
                        .iter()
                        .flat_map(|key| [KeyInput::press(*key), KeyInput::release(*key)])
                        .collect::<Vec<_>>(),
                    transition: *t,
                    input_canceler: vec![KeyInput::release(i[0])],
                })
            })
            .chain(modifiers_trans)
            .collect(),
        layer_name: "big config",
    }
}

fn config_caps_lock_arrow() -> KeyConfigUnit {
    let capslock_side = [
        (1, KEY_I, None, KEY_UP),
        (1, KEY_J, None, KEY_LEFT),
        (1, KEY_K, None, KEY_DOWN),
        (1, KEY_L, None, KEY_RIGHT),
        (2, KEY_I, Some(KEY_LEFTCTRL), KEY_UP),
        (2, KEY_J, Some(KEY_LEFTCTRL), KEY_LEFT),
        (2, KEY_K, Some(KEY_LEFTCTRL), KEY_DOWN),
        (2, KEY_L, Some(KEY_LEFTCTRL), KEY_RIGHT),
        (3, KEY_I, Some(KEY_LEFTMETA), KEY_I),
        (3, KEY_J, Some(KEY_LEFTMETA), KEY_J),
        (3, KEY_K, Some(KEY_LEFTMETA), KEY_K),
        (3, KEY_L, Some(KEY_LEFTMETA), KEY_L),
        (4, KEY_I, None, KEY_ESC),
        (4, KEY_J, None, KEY_HOME),
        (4, KEY_K, None, KEY_ESC),
        (4, KEY_L, None, KEY_END),
    ];
    let capslock_side = capslock_side
        .iter()
        .map(|&(c, i, o1, o2)| SingleHotkeyEntry {
            cond: c,
            input: KeyInput::press(i),
            output: if let Some(o1) = o1 {
                vec![
                    KeyInput::press(o1),
                    KeyInput::press(o2),
                    KeyInput::release(o2),
                    KeyInput::release(o1),
                ]
            } else {
                vec![KeyInput::press(o2), KeyInput::release(o2)]
            },
            transition: None,
            input_canceler: Vec::new(),
        });
    let single_hotkeys: &[(&[State], KeyInput, &[KeyInput], State)] = &[
        (&[0, 1], KeyInput::press(KEY_CAPSLOCK), &[], 1),
        (&[1, 2, 3, 4], KeyInput::press(KEY_E), &[], 2),
        (&[1, 2, 3, 4], KeyInput::press(KEY_R), &[], 3),
        (&[1, 2, 3, 4], KeyInput::press(KEY_F), &[], 4),
        (&[2], KeyInput::press(KEY_CAPSLOCK), &[], 2),
        (&[3], KeyInput::press(KEY_CAPSLOCK), &[], 3),
        (&[4], KeyInput::press(KEY_CAPSLOCK), &[], 4),
        (&[0, 1, 2, 3, 4], KeyInput::release(KEY_CAPSLOCK), &[], 0),
        (&[2], KeyInput::release(KEY_E), &[], 1),
        (&[3], KeyInput::release(KEY_R), &[], 1),
        (&[4], KeyInput::release(KEY_F), &[], 1),
    ];
    let single_hotkyes = single_hotkeys.iter().flat_map(|(c, i, o, t)| {
        c.iter().map(move |c| SingleHotkeyEntry {
            cond: *c,
            input: *i,
            output: o.to_vec(),
            transition: Some(*t),
            input_canceler: Vec::new(),
        })
    });
    KeyConfigUnit {
        pair_hotkeys: Vec::new(),
        single_hotkeys: single_hotkyes.chain(capslock_side).collect(),
        layer_name: "caps lock arrows",
    }
}

fn config_sands() -> KeyConfigUnit {
    let config: &[(&[u64], KeyInput, &[KeyInput], Option<u64>)] = &[
        (
            &[0],
            KeyInput::press(KEY_SPACE),
            &[KeyInput::press(KEY_LEFTSHIFT)],
            Some(1),
        ),
        (&[1, 2], KeyInput::press(KEY_SPACE), &[], None),
        (
            &[1],
            KeyInput::release(KEY_SPACE),
            &[
                KeyInput::release(KEY_LEFTSHIFT),
                KeyInput::press(KEY_SPACE),
                KeyInput::release(KEY_SPACE),
            ],
            Some(0),
        ),
        (
            &[2],
            KeyInput::release(KEY_SPACE),
            &[KeyInput::release(KEY_LEFTSHIFT)],
            Some(0),
        ),
    ];
    let config = config.iter().flat_map(|(cs, i, o, t)| {
        cs.iter().map(move |c| SingleHotkeyEntry {
            cond: *c,
            input: *i,
            output: o.to_vec(),
            transition: *t,
            input_canceler: Vec::new(),
        })
    });
    let config2 = all_keys()
        .filter(|k| *k != KEY_SPACE)
        .map(|k| SingleHotkeyEntry {
            cond: 1,
            input: KeyInput::press(k),
            output: vec![KeyInput::press(k)],
            transition: Some(2),
            input_canceler: Vec::new(),
        });
    KeyConfigUnit {
        pair_hotkeys: Vec::new(),
        single_hotkeys: config.chain(config2).collect(),
        layer_name: "SandS",
    }
}

fn config_simple_remap() -> KeyConfigUnit {
    let key_config_r: &[(Key, Key)] = &[(KEY_HENKAN, KEY_ENTER), (KEY_MUHENKAN, KEY_BACKSPACE)];
    KeyConfigUnit {
        pair_hotkeys: Vec::new(),
        single_hotkeys: key_config_r
            .iter()
            .map(|(i, o)| SingleHotkeyEntry {
                cond: 0,
                input: KeyInput::press(*i),
                output: vec![KeyInput::press(*o)],
                transition: None,
                input_canceler: Vec::new(),
            })
            .chain(key_config_r.iter().map(|(i, o)| SingleHotkeyEntry {
                cond: 0,
                input: KeyInput::release(*i),
                output: vec![KeyInput::release(*o)],
                transition: None,
                input_canceler: Vec::new(),
            }))
            .collect(),
        layer_name: "simple remap",
    }
}

fn main() {
    read_events::run(vec![
        config_simple_remap(),
        config_caps_lock_arrow(),
        config_sands(),
        mk_config(),
    ]);
}
