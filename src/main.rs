use evdev::Key;
use evdev_keys::*;
use read_events::KeyConfigRun;
use read_keys::{
    AddLayer, EmptyCinfig, KeyConfigUnit, KeyInput, PairHotkeyEntry, SingleHotkeyEntry,
};

mod read_events;
mod read_keys;
mod write_keys;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum StateGeta {
    Normal,
    JpInput,
    JpInputWithModifires,
}

#[allow(clippy::type_complexity)]
fn mk_config() -> KeyConfigUnit<StateGeta> {
    use StateGeta::*;
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
    let mut singeta_config: Vec<(&[StateGeta], &[Key], &[Key], Option<StateGeta>)> = singeta_config
        .iter()
        .map(|(i, o)| -> (&[StateGeta], _, _, _) { (&[JpInput], *i, *o, None) })
        .collect();
    let key_config_r: &[(&[StateGeta], &[Key], &[Key], Option<StateGeta>)] = &[
        (&[JpInput], &[KEY_R, KEY_G], &[KEY_SLASH], None),
        (
            &[JpInput],
            &[KEY_H, KEY_J],
            &[KEY_RIGHTBRACE, KEY_BACKSLASH, KEY_RIGHT],
            None,
        ),
        (&[Normal], &[KEY_J, KEY_K], &[KEY_RIGHTBRACE], None),
        (&[Normal], &[KEY_D, KEY_SEMICOLON], &[KEY_END], None),
        (&[Normal], &[KEY_A, KEY_K], &[KEY_HOME], None),
        (&[Normal], &[KEY_F, KEY_SEMICOLON], &[KEY_END], None),
        (&[Normal], &[KEY_A, KEY_J], &[KEY_HOME], None),
    ];
    let key_config_r = {
        let mut k = key_config_r.to_vec();
        k.append(&mut singeta_config);
        k
    };
    let pair_keys_with_modifiers_config: &[(&[StateGeta], [Key; 2], Vec<_>, Option<StateGeta>)] = &[
        (
            &[Normal, JpInput],
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
            &[Normal, JpInput],
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
            &[Normal, JpInput],
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
            &[JpInput],
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
            &[Normal],
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
            &[Normal],
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
            &[Normal],
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
            &[Normal],
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
            &[Normal],
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
            &[Normal],
            [KEY_F, KEY_J],
            vec![
                KeyInput::press(KEY_LEFTMETA),
                KeyInput::press(KEY_SPACE),
                KeyInput::release(KEY_SPACE),
                KeyInput::release(KEY_LEFTMETA),
            ],
            Some(JpInput),
        ),
        (
            &[JpInput],
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
            Some(Normal),
        ),
        (
            &[Normal],
            [KEY_D, KEY_S],
            vec![
                KeyInput::press(KEY_LEFTMETA),
                KeyInput::press(KEY_SPACE),
                KeyInput::release(KEY_SPACE),
                KeyInput::release(KEY_LEFTMETA),
            ],
            Some(JpInput),
        ),
        (
            &[JpInput],
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
            Some(Normal),
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
                (JpInput, KeyInput::press(*key), Some(JpInputWithModifires)),
                (JpInputWithModifires, KeyInput::release(*key), Some(JpInput)),
            ]
            .map(|(c, i, t)| SingleHotkeyEntry {
                cond: c,
                input: i,
                output: vec![i],
                transition: t,
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
                cs.iter()
                    .map(move |c| SingleHotkeyEntry {
                        cond: *c,
                        input: KeyInput::press(i[0]),
                        output: (*o)
                            .iter()
                            .flat_map(|key| [KeyInput::press(*key), KeyInput::release(*key)])
                            .collect::<Vec<_>>(),
                        transition: *t,
                    })
                    .chain(cs.iter().map(move |c| SingleHotkeyEntry {
                        cond: *c,
                        input: KeyInput::release(i[0]),
                        output: Vec::new(),
                        transition: *t,
                    }))
            })
            .chain(modifiers_trans)
            .collect(),
        layer_name: "big config",
        initial_state: Normal,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum StateCapsLock {
    Normal,
    CL,
    Clw,
    Cle,
    Clr,
    Clf,
}

fn config_caps_lock_arrow() -> KeyConfigUnit<StateCapsLock> {
    use StateCapsLock::*;
    let capslock_side = [
        (CL, KEY_I, None, KEY_UP),
        (CL, KEY_J, None, KEY_LEFT),
        (CL, KEY_K, None, KEY_DOWN),
        (CL, KEY_L, None, KEY_RIGHT),
        (CL, KEY_ENTER, Some(KEY_LEFTCTRL), KEY_S),
        (CL, KEY_N, Some(KEY_LEFTCTRL), KEY_C),
        (CL, KEY_M, Some(KEY_LEFTCTRL), KEY_V),
        (CL, KEY_U, Some(KEY_LEFTCTRL), KEY_Z),
        (CL, KEY_O, Some(KEY_LEFTCTRL), KEY_Y),
        (CL, KEY_DOT, Some(KEY_LEFTCTRL), KEY_DOT),
        (Cle, KEY_I, Some(KEY_LEFTCTRL), KEY_UP),
        (Cle, KEY_J, Some(KEY_LEFTCTRL), KEY_LEFT),
        (Cle, KEY_K, Some(KEY_LEFTCTRL), KEY_DOWN),
        (Cle, KEY_L, Some(KEY_LEFTCTRL), KEY_RIGHT),
        (Clr, KEY_I, Some(KEY_LEFTMETA), KEY_I),
        (Clr, KEY_J, Some(KEY_LEFTMETA), KEY_J),
        (Clr, KEY_K, Some(KEY_LEFTMETA), KEY_K),
        (Clr, KEY_L, Some(KEY_LEFTMETA), KEY_L),
        (Clf, KEY_I, None, KEY_ESC),
        (Clf, KEY_J, None, KEY_HOME),
        (Clf, KEY_K, None, KEY_ESC),
        (Clf, KEY_L, None, KEY_END),
        (Clw, KEY_J, Some(KEY_LEFTMETA), KEY_PAGEUP),
        (Clw, KEY_L, Some(KEY_LEFTMETA), KEY_PAGEDOWN),
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
        });
    let single_hotkeys: &[(&[StateCapsLock], KeyInput, &[KeyInput], StateCapsLock)] = &[
        (&[Normal, CL], KeyInput::press(KEY_CAPSLOCK), &[], CL),
        (&[CL, Cle, Clr, Clf, Clw], KeyInput::press(KEY_E), &[], Cle),
        (&[CL, Cle, Clr, Clf, Clw], KeyInput::press(KEY_R), &[], Clr),
        (&[CL, Cle, Clr, Clf, Clw], KeyInput::press(KEY_F), &[], Clf),
        (&[CL, Cle, Clr, Clf, Clw], KeyInput::press(KEY_W), &[], Clw),
        (&[Cle], KeyInput::press(KEY_CAPSLOCK), &[], Cle),
        (&[Clr], KeyInput::press(KEY_CAPSLOCK), &[], Clr),
        (&[Clf], KeyInput::press(KEY_CAPSLOCK), &[], Clf),
        (&[Clw], KeyInput::press(KEY_CAPSLOCK), &[], Clw),
        (
            &[Normal, CL, Cle, Clr, Clf, Clw],
            KeyInput::release(KEY_CAPSLOCK),
            &[],
            Normal,
        ),
        (&[Cle], KeyInput::release(KEY_E), &[], CL),
        (&[Clr], KeyInput::release(KEY_R), &[], CL),
        (&[Clf], KeyInput::release(KEY_F), &[], CL),
        (&[Clw], KeyInput::release(KEY_W), &[], CL),
    ];
    let single_hotkyes = single_hotkeys.iter().flat_map(|(c, i, o, t)| {
        c.iter().map(move |c| SingleHotkeyEntry {
            cond: *c,
            input: *i,
            output: o.to_vec(),
            transition: Some(*t),
        })
    });
    KeyConfigUnit {
        pair_hotkeys: Vec::new(),
        single_hotkeys: single_hotkyes.chain(capslock_side).collect(),
        layer_name: "caps lock arrows",
        initial_state: Normal,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum StateSands {
    Normal,
    Space,
    Shift,
}

fn config_sands() -> KeyConfigUnit<StateSands> {
    use StateSands::*;
    #[allow(clippy::type_complexity)]
    let config: &[(&[StateSands], KeyInput, &[KeyInput], Option<StateSands>)] = &[
        (
            &[Normal],
            KeyInput::press(KEY_SPACE),
            &[KeyInput::press(KEY_LEFTSHIFT)],
            Some(Space),
        ),
        (&[Space, Shift], KeyInput::press(KEY_SPACE), &[], None),
        (
            &[Space],
            KeyInput::release(KEY_SPACE),
            &[
                KeyInput::release(KEY_LEFTSHIFT),
                KeyInput::press(KEY_SPACE),
                KeyInput::release(KEY_SPACE),
            ],
            Some(Normal),
        ),
        (
            &[Shift],
            KeyInput::release(KEY_SPACE),
            &[KeyInput::release(KEY_LEFTSHIFT)],
            Some(Normal),
        ),
    ];
    let config = config.iter().flat_map(|(cs, i, o, t)| {
        cs.iter().map(move |c| SingleHotkeyEntry {
            cond: *c,
            input: *i,
            output: o.to_vec(),
            transition: *t,
        })
    });
    let config2 = all_keys()
        .filter(|k| *k != KEY_SPACE)
        .map(|k| SingleHotkeyEntry {
            cond: Space,
            input: KeyInput::press(k),
            output: vec![KeyInput::press(k)],
            transition: Some(Shift),
        });
    KeyConfigUnit {
        pair_hotkeys: Vec::new(),
        single_hotkeys: config.chain(config2).collect(),
        layer_name: "SandS",
        initial_state: Normal,
    }
}

fn config_simple_remap() -> KeyConfigUnit<()> {
    let key_config_r: &[(Key, Key)] = &[(KEY_HENKAN, KEY_ENTER), (KEY_MUHENKAN, KEY_BACKSPACE)];
    KeyConfigUnit {
        pair_hotkeys: Vec::new(),
        single_hotkeys: key_config_r
            .iter()
            .map(|(i, o)| SingleHotkeyEntry {
                cond: (),
                input: KeyInput::press(*i),
                output: vec![KeyInput::press(*o)],
                transition: None,
            })
            .chain(key_config_r.iter().map(|(i, o)| SingleHotkeyEntry {
                cond: (),
                input: KeyInput::release(*i),
                output: vec![KeyInput::release(*o)],
                transition: None,
            }))
            .collect(),
        layer_name: "simple remap",
        initial_state: (),
    }
}

fn main() {
    EmptyCinfig
        .add_layer(config_simple_remap())
        .add_layer(config_caps_lock_arrow())
        .add_layer(config_sands())
        .add_layer(mk_config())
        .run();
}
