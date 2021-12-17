use evdev::Key;
use evdev_keys::*;
use read_keys::{KeyConfig, KeyInput, PairHotkeyEntry, SingleHotkeyEntry, TransitionOp};

mod read_events;
mod read_keys;
mod write_keys;

fn mk_config() -> KeyConfig {
    use TransitionOp::*;
    let all_states: &[&[u8]] = &[&[], &[0], &[1], &[2], &[3], &[4], &[5]];
    // []  : normal
    // 0   : jp input
    // 1   : jp input with modifiers
    // 2   : capslock plessing, move to jp after release
    // 3   : other key pressed while capslock is on,
    //       or capslock plessing after jp, so move to []
    //       after release
    // 4   : space is pressing
    // 5   : other key pressed while space is pressed
    let key_config_r: &[(&[&[u8]], &[Key], &[Key], &[TransitionOp])] = &[
        (all_states, &[KEY_HENKAN], &[KEY_ENTER], &[]),
        (all_states, &[KEY_MUHENKAN], &[KEY_BACKSPACE], &[]),
        // (&[0], &[KEY_GRAVE], &[KEY_HENKAN], Some(1)),
        // (&[1], &[KEY_GRAVE], &[KEY_MUHENKAN], Some(0)),
        // (&[0], &[KEY_F, KEY_J], &[KEY_HENKAN], Some(1)),
        // (&[1], &[KEY_F, KEY_J], &[KEY_MUHENKAN], Some(0)),
        //
        (&[&[0]], &[KEY_A], &[KEY_N, KEY_O], &[]),
        (&[&[0]], &[KEY_S], &[KEY_T, KEY_O], &[]),
        (&[&[0]], &[KEY_D], &[KEY_K, KEY_A], &[]),
        (&[&[0]], &[KEY_F], &[KEY_N, KEY_N], &[]),
        (&[&[0]], &[KEY_G], &[KEY_L, KEY_T, KEY_U], &[]),
        (&[&[0]], &[KEY_D, KEY_H], &[KEY_H, KEY_E], &[]),
        (&[&[0]], &[KEY_D, KEY_J], &[KEY_A], &[]),
        (&[&[0]], &[KEY_D, KEY_SEMICOLON], &[KEY_E], &[]),
        (&[&[0]], &[KEY_D, KEY_N], &[KEY_S, KEY_E], &[]),
        (&[&[0]], &[KEY_D, KEY_M], &[KEY_N, KEY_E], &[]),
        (&[&[0]], &[KEY_D, KEY_COMMA], &[KEY_B, KEY_E], &[]),
        (&[&[0]], &[KEY_D, KEY_DOT], &[KEY_P, KEY_U], &[]),
        (&[&[0]], &[KEY_D, KEY_SLASH], &[KEY_V, KEY_U], &[]),
        (&[&[0]], &[KEY_D, KEY_Y], &[KEY_W, KEY_I], &[]),
        (&[&[0]], &[KEY_D, KEY_U], &[KEY_P, KEY_A], &[]),
        (&[&[0]], &[KEY_D, KEY_I], &[KEY_Y, KEY_O], &[]),
        (&[&[0]], &[KEY_D, KEY_O], &[KEY_M, KEY_I], &[]),
        (&[&[0]], &[KEY_D, KEY_P], &[KEY_W, KEY_E], &[]),
        (
            &[&[0]],
            &[KEY_D, KEY_LEFTBRACE],
            &[KEY_U, KEY_L, KEY_O],
            &[],
        ),
        (&[&[0]], &[KEY_H], &[KEY_K, KEY_U], &[]),
        (&[&[0]], &[KEY_J], &[KEY_U], &[]),
        (&[&[0]], &[KEY_K], &[KEY_I], &[]),
        (&[&[0]], &[KEY_L], &[KEY_S, KEY_H, KEY_I], &[]),
        (&[&[0]], &[KEY_SEMICOLON], &[KEY_N, KEY_A], &[]),
        (&[&[0]], &[KEY_I, KEY_1], &[KEY_L, KEY_Y, KEY_U], &[]),
        (&[&[0]], &[KEY_I, KEY_2], &[KEY_B, KEY_Y, KEY_A], &[]),
        (&[&[0]], &[KEY_I, KEY_3], &[KEY_B, KEY_Y, KEY_U], &[]),
        (&[&[0]], &[KEY_I, KEY_4], &[KEY_B, KEY_Y, KEY_O], &[]),
        (&[&[0]], &[KEY_I, KEY_A], &[KEY_H, KEY_Y, KEY_O], &[]),
        (&[&[0]], &[KEY_I, KEY_F], &[KEY_K, KEY_Y, KEY_O], &[]),
        (&[&[0]], &[KEY_I, KEY_G], &[KEY_C, KEY_H, KEY_O], &[]),
        (&[&[0]], &[KEY_I, KEY_Q], &[KEY_H, KEY_Y, KEY_U], &[]),
        (&[&[0]], &[KEY_I, KEY_W], &[KEY_S, KEY_Y, KEY_U], &[]),
        (&[&[0]], &[KEY_I, KEY_E], &[KEY_S, KEY_Y, KEY_O], &[]),
        (&[&[0]], &[KEY_I, KEY_R], &[KEY_K, KEY_Y, KEY_U], &[]),
        (&[&[0]], &[KEY_I, KEY_T], &[KEY_C, KEY_H, KEY_U], &[]),
        (&[&[0]], &[KEY_I, KEY_Z], &[KEY_H, KEY_Y, KEY_A], &[]),
        (&[&[0]], &[KEY_I, KEY_C], &[KEY_S, KEY_H, KEY_A], &[]),
        (&[&[0]], &[KEY_I, KEY_V], &[KEY_K, KEY_Y, KEY_A], &[]),
        (&[&[0]], &[KEY_I, KEY_B], &[KEY_C, KEY_H, KEY_A], &[]),
        (&[&[0]], &[KEY_K, KEY_1], &[KEY_L, KEY_A], &[]),
        (&[&[0]], &[KEY_K, KEY_2], &[KEY_L, KEY_I], &[]),
        (&[&[0]], &[KEY_K, KEY_3], &[KEY_L, KEY_U], &[]),
        (&[&[0]], &[KEY_K, KEY_4], &[KEY_L, KEY_E], &[]),
        (&[&[0]], &[KEY_K, KEY_5], &[KEY_L, KEY_O], &[]),
        (&[&[0]], &[KEY_K, KEY_A], &[KEY_H, KEY_O], &[]),
        (&[&[0]], &[KEY_K, KEY_S], &[KEY_J, KEY_I], &[]),
        (&[&[0]], &[KEY_K, KEY_D], &[KEY_R, KEY_E], &[]),
        (&[&[0]], &[KEY_K, KEY_F], &[KEY_M, KEY_O], &[]),
        (&[&[0]], &[KEY_K, KEY_G], &[KEY_Y, KEY_U], &[]),
        (&[&[0]], &[KEY_K, KEY_Q], &[KEY_F, KEY_A], &[]),
        (&[&[0]], &[KEY_K, KEY_W], &[KEY_G, KEY_O], &[]),
        (&[&[0]], &[KEY_K, KEY_E], &[KEY_F, KEY_U], &[]),
        (&[&[0]], &[KEY_K, KEY_R], &[KEY_F, KEY_I], &[]),
        (&[&[0]], &[KEY_K, KEY_T], &[KEY_F, KEY_E], &[]),
        (&[&[0]], &[KEY_K, KEY_Z], &[KEY_D, KEY_U], &[]),
        (&[&[0]], &[KEY_K, KEY_X], &[KEY_Z, KEY_O], &[]),
        (&[&[0]], &[KEY_K, KEY_C], &[KEY_B, KEY_O], &[]),
        (&[&[0]], &[KEY_K, KEY_V], &[KEY_M, KEY_U], &[]),
        (&[&[0]], &[KEY_K, KEY_B], &[KEY_F, KEY_O], &[]),
        (&[&[0]], &[KEY_L, KEY_1], &[KEY_L, KEY_Y, KEY_A], &[]),
        (&[&[0]], &[KEY_L, KEY_2], &[KEY_M, KEY_Y, KEY_A], &[]),
        (&[&[0]], &[KEY_L, KEY_3], &[KEY_M, KEY_Y, KEY_U], &[]),
        (&[&[0]], &[KEY_L, KEY_4], &[KEY_M, KEY_Y, KEY_O], &[]),
        (&[&[0]], &[KEY_L, KEY_5], &[KEY_W, KEY_A], &[]),
        (&[&[0]], &[KEY_L, KEY_A], &[KEY_W, KEY_O], &[]),
        (&[&[0]], &[KEY_L, KEY_S], &[KEY_S, KEY_A], &[]),
        (&[&[0]], &[KEY_L, KEY_D], &[KEY_O], &[]),
        (&[&[0]], &[KEY_L, KEY_F], &[KEY_R, KEY_I], &[]),
        (&[&[0]], &[KEY_L, KEY_G], &[KEY_Z, KEY_U], &[]),
        (&[&[0]], &[KEY_L, KEY_Q], &[KEY_D, KEY_I], &[]),
        (&[&[0]], &[KEY_L, KEY_W], &[KEY_M, KEY_E], &[]),
        (&[&[0]], &[KEY_L, KEY_E], &[KEY_K, KEY_E], &[]),
        (&[&[0]], &[KEY_L, KEY_R], &[KEY_T, KEY_E, KEY_L, KEY_I], &[]),
        (&[&[0]], &[KEY_L, KEY_T], &[KEY_D, KEY_E, KEY_L, KEY_I], &[]),
        (&[&[0]], &[KEY_L, KEY_Z], &[KEY_Z, KEY_E], &[]),
        (&[&[0]], &[KEY_L, KEY_X], &[KEY_Z, KEY_A], &[]),
        (&[&[0]], &[KEY_L, KEY_C], &[KEY_G, KEY_I], &[]),
        (&[&[0]], &[KEY_L, KEY_V], &[KEY_R, KEY_O], &[]),
        (&[&[0]], &[KEY_L, KEY_B], &[KEY_N, KEY_U], &[]),
        (&[&[0]], &[KEY_N], &[KEY_T, KEY_E], &[]),
        (&[&[0]], &[KEY_M], &[KEY_T, KEY_A], &[]),
        (&[&[0]], &[KEY_COMMA], &[KEY_D, KEY_E], &[]),
        (&[&[0]], &[KEY_DOT], &[KEY_DOT], &[]),
        (&[&[0]], &[KEY_SLASH], &[KEY_B, KEY_U], &[]),
        (&[&[0]], &[KEY_O, KEY_1], &[KEY_L, KEY_Y, KEY_O], &[]),
        (&[&[0]], &[KEY_O, KEY_2], &[KEY_P, KEY_Y, KEY_A], &[]),
        (&[&[0]], &[KEY_O, KEY_3], &[KEY_P, KEY_Y, KEY_U], &[]),
        (&[&[0]], &[KEY_O, KEY_4], &[KEY_P, KEY_Y, KEY_O], &[]),
        (&[&[0]], &[KEY_O, KEY_A], &[KEY_R, KEY_Y, KEY_O], &[]),
        (&[&[0]], &[KEY_O, KEY_F], &[KEY_G, KEY_Y, KEY_O], &[]),
        (&[&[0]], &[KEY_O, KEY_G], &[KEY_N, KEY_Y, KEY_O], &[]),
        (&[&[0]], &[KEY_O, KEY_Q], &[KEY_R, KEY_Y, KEY_U], &[]),
        (&[&[0]], &[KEY_O, KEY_W], &[KEY_J, KEY_U], &[]),
        (&[&[0]], &[KEY_O, KEY_E], &[KEY_J, KEY_O], &[]),
        (&[&[0]], &[KEY_O, KEY_R], &[KEY_G, KEY_Y, KEY_U], &[]),
        (&[&[0]], &[KEY_O, KEY_T], &[KEY_N, KEY_Y, KEY_U], &[]),
        (&[&[0]], &[KEY_O, KEY_Z], &[KEY_R, KEY_Y, KEY_A], &[]),
        (&[&[0]], &[KEY_O, KEY_C], &[KEY_J, KEY_A], &[]),
        (&[&[0]], &[KEY_O, KEY_V], &[KEY_G, KEY_Y, KEY_A], &[]),
        (&[&[0]], &[KEY_O, KEY_B], &[KEY_N, KEY_Y, KEY_A], &[]),
        (&[&[0]], &[KEY_Q], &[KEY_MINUS], &[]),
        (&[&[0]], &[KEY_W], &[KEY_N, KEY_I], &[]),
        (&[&[0]], &[KEY_E], &[KEY_H, KEY_A], &[]),
        (&[&[0]], &[KEY_R], &[KEY_COMMA], &[]),
        (&[&[0]], &[KEY_T], &[KEY_C, KEY_H, KEY_I], &[]),
        (&[&[0]], &[KEY_S, KEY_H], &[KEY_B, KEY_I], &[]),
        (&[&[0]], &[KEY_S, KEY_J], &[KEY_R, KEY_A], &[]),
        (&[&[0]], &[KEY_S, KEY_SEMICOLON], &[KEY_S, KEY_O], &[]),
        (&[&[0]], &[KEY_S, KEY_N], &[KEY_W, KEY_A], &[]),
        (&[&[0]], &[KEY_S, KEY_M], &[KEY_D, KEY_A], &[]),
        (&[&[0]], &[KEY_S, KEY_COMMA], &[KEY_P, KEY_I], &[]),
        (&[&[0]], &[KEY_S, KEY_DOT], &[KEY_P, KEY_O], &[]),
        (&[&[0]], &[KEY_S, KEY_SLASH], &[KEY_T, KEY_Y, KEY_E], &[]),
        (&[&[0]], &[KEY_S, KEY_Y], &[KEY_S, KEY_Y, KEY_E], &[]),
        (&[&[0]], &[KEY_S, KEY_U], &[KEY_P, KEY_E], &[]),
        (&[&[0]], &[KEY_S, KEY_I], &[KEY_D, KEY_O], &[]),
        (&[&[0]], &[KEY_S, KEY_O], &[KEY_Y, KEY_A], &[]),
        (&[&[0]], &[KEY_S, KEY_P], &[KEY_S, KEY_Y, KEY_E], &[]),
        (&[&[0]], &[KEY_Y], &[KEY_G, KEY_U], &[]),
        (&[&[0]], &[KEY_U], &[KEY_B, KEY_A], &[]),
        (&[&[0]], &[KEY_I], &[KEY_K, KEY_O], &[]),
        (&[&[0]], &[KEY_O], &[KEY_G, KEY_A], &[]),
        (&[&[0]], &[KEY_P], &[KEY_H, KEY_I], &[]),
        (&[&[0]], &[KEY_LEFTBRACE], &[KEY_G, KEY_E], &[]),
        (&[&[0]], &[KEY_Z], &[KEY_S, KEY_U], &[]),
        (&[&[0]], &[KEY_X], &[KEY_M, KEY_A], &[]),
        (&[&[0]], &[KEY_C], &[KEY_K, KEY_I], &[]),
        (&[&[0]], &[KEY_V], &[KEY_R, KEY_U], &[]),
        (&[&[0]], &[KEY_B], &[KEY_T, KEY_U], &[]),
        //
        (&[&[0]], &[KEY_R, KEY_G], &[KEY_SLASH], &[]),
        (
            &[&[0]],
            &[KEY_H, KEY_J],
            &[KEY_RIGHTBRACE, KEY_BACKSLASH, KEY_RIGHT],
            &[],
        ),
        (&[&[]], &[KEY_J, KEY_K], &[KEY_RIGHTBRACE], &[]),
        (&[&[]], &[KEY_D, KEY_SEMICOLON], &[KEY_END], &[]),
        (&[&[]], &[KEY_A, KEY_K], &[KEY_HOME], &[]),
        (&[&[]], &[KEY_F, KEY_SEMICOLON], &[KEY_END], &[]),
        (&[&[]], &[KEY_A, KEY_J], &[KEY_HOME], &[]),
    ];
    let single_keys_with_modifires: &[(
        &[&[u8]],
        KeyInput,
        Vec<_>,
        &[TransitionOp],
        &[KeyInput],
    )] = &[
        (
            &[&[]],
            KeyInput::press(KEY_CAPSLOCK),
            Vec::new(),
            &[Insert(2)],
            &[],
        ),
        (
            &[&[0]],
            KeyInput::press(KEY_CAPSLOCK),
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
            &[Remove(0), Insert(3)],
            &[],
        ),
        (
            &[&[2]],
            KeyInput::release(KEY_CAPSLOCK),
            vec![
                KeyInput::press(KEY_LEFTMETA),
                KeyInput::press(KEY_SPACE),
                KeyInput::release(KEY_SPACE),
                KeyInput::release(KEY_LEFTMETA),
            ],
            &[Insert(0), Remove(2)],
            &[],
        ),
        (&[&[2]], KeyInput::press(KEY_CAPSLOCK), Vec::new(), &[], &[]),
        (
            &[&[3]],
            KeyInput::release(KEY_CAPSLOCK),
            Vec::new(),
            &[Remove(3)],
            &[],
        ),
        (
            &[&[]],
            KeyInput::press(KEY_SPACE),
            vec![KeyInput::press(KEY_LEFTSHIFT)],
            &[Insert(4)],
            &[],
        ),
        (
            &[&[4], &[5]],
            KeyInput::press(KEY_SPACE),
            Vec::new(),
            &[],
            &[],
        ),
        (
            &[&[4]],
            KeyInput::release(KEY_SPACE),
            vec![
                KeyInput::release(KEY_LEFTSHIFT),
                KeyInput::press(KEY_SPACE),
                KeyInput::release(KEY_SPACE),
            ],
            &[Remove(4)],
            &[],
        ),
        (
            &[&[5]],
            KeyInput::release(KEY_SPACE),
            vec![KeyInput::release(KEY_LEFTSHIFT)],
            &[Remove(5)],
            &[],
        ),
    ];
    let capslock_side: &[(Key, Vec<_>)] = &[
        (KEY_I, vec![KEY_UP]),
        (KEY_J, vec![KEY_LEFT]),
        (KEY_K, vec![KEY_DOWN]),
        (KEY_L, vec![KEY_RIGHT]),
    ];
    let pair_keys_with_modifiers_config: &[(&[&[u8]], [Key; 2], Vec<_>)] = &[
        (
            &[&[], &[0]],
            [KEY_J, KEY_N],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_SLASH),
                KeyInput::release(KEY_SLASH),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
        ),
        (
            &[&[], &[0]],
            [KEY_F, KEY_V],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_1),
                KeyInput::release(KEY_1),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
        ),
        (
            &[&[], &[0]],
            [KEY_F, KEY_B],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_1),
                KeyInput::release(KEY_1),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
        ),
        (
            &[&[], &[0]],
            [KEY_F, KEY_G],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_8),
                KeyInput::release(KEY_8),
                KeyInput::press(KEY_9),
                KeyInput::release(KEY_LEFTSHIFT),
                KeyInput::release(KEY_9),
                // KeyInput::press(KEY_LEFTCTRL),
                // KeyInput::press(KEY_LEFTBRACE),
                // KeyInput::release(KEY_LEFTBRACE),
                // KeyInput::release(KEY_LEFTCTRL),
                // KeyInput::press(KEY_LEFT),
                // KeyInput::release(KEY_LEFT),
            ],
        ),
        (
            &[&[]],
            [KEY_D, KEY_F],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_8),
                KeyInput::release(KEY_8),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
        ),
        (
            &[&[]],
            [KEY_F, KEY_G],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_9),
                KeyInput::release(KEY_9),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
        ),
        (
            &[&[]],
            [KEY_K, KEY_L],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_RO),
                KeyInput::release(KEY_RO),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
        ),
        // (
        //     &[0],
        //     [KEY_F, KEY_J],
        //     vec![
        //         KeyInput::press(KEY_LEFTMETA),
        //         KeyInput::press(KEY_SPACE),
        //         KeyInput::release(KEY_SPACE),
        //         KeyInput::release(KEY_LEFTMETA),
        //     ],
        //     Some(1),
        // ),
        // (
        //     &[1],
        //     [KEY_F, KEY_J],
        //     vec![
        //         KeyInput::press(KEY_LEFTCTRL),
        //         KeyInput::press(KEY_RIGHTBRACE),
        //         KeyInput::release(KEY_RIGHTBRACE),
        //         KeyInput::release(KEY_LEFTCTRL),
        //         KeyInput::press(KEY_LEFTMETA),
        //         KeyInput::press(KEY_SPACE),
        //         KeyInput::release(KEY_SPACE),
        //         KeyInput::release(KEY_LEFTMETA),
        //     ],
        //     Some(0),
        // ),
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
                (vec![0], KeyInput::press(*key), &[Remove(0), Insert(1)]),
                (vec![1], KeyInput::release(*key), &[Insert(0), Remove(1)]),
            ]
            .map(|(c, i, t)| SingleHotkeyEntry {
                cond: c.into_iter().collect(),
                input: i,
                output: vec![i],
                transition: t.to_vec(),
                input_canceler: Vec::new(),
            })
        })
        .collect::<Vec<_>>();
    let all_alphabet_keys = &[
        KEY_A, KEY_B, KEY_C, KEY_D, KEY_E, KEY_F, KEY_G, KEY_H, KEY_I, KEY_J, KEY_K, KEY_L, KEY_M,
        KEY_N, KEY_O, KEY_P, KEY_Q, KEY_R, KEY_S, KEY_T, KEY_U, KEY_V, KEY_W, KEY_X, KEY_Y, KEY_Z,
    ];
    let sands_config = all_alphabet_keys.iter().map(|k| SingleHotkeyEntry {
        cond: smallbitset::Set64::singleton(4),
        input: KeyInput::press(*k),
        output: vec![KeyInput::press(*k)],
        transition: vec![Remove(4), Insert(5)],
        input_canceler: Vec::new(),
    });
    KeyConfig {
        pair_hotkeys: key_config_r
            .iter()
            .filter(|(_, i, _, _)| i.len() == 2)
            .flat_map(|(cs, i, o, t)| {
                cs.iter().map(move |c| PairHotkeyEntry {
                    cond: c.iter().copied().collect(),
                    input: [i[0], i[1]],
                    output_keys: o
                        .iter()
                        .flat_map(|key| [KeyInput::press(*key), KeyInput::release(*key)])
                        .collect(),
                    transition: t.to_vec(),
                })
            })
            .chain(
                pair_keys_with_modifiers_config
                    .iter()
                    .flat_map(|(cs, i, o)| {
                        cs.iter().map(move |c| PairHotkeyEntry {
                            cond: c.iter().copied().collect(),
                            input: *i,
                            output_keys: o.clone(),
                            transition: Vec::new(),
                        })
                    }),
            )
            .collect(),
        single_hotkeys: key_config_r
            .iter()
            .filter(|(_, i, _, _)| i.len() == 1)
            .flat_map(|(cs, i, o, t)| {
                cs.iter().map(move |c| SingleHotkeyEntry {
                    cond: c.iter().copied().collect(),
                    input: KeyInput::press(i[0]),
                    output: (*o)
                        .iter()
                        .flat_map(|key| [KeyInput::press(*key), KeyInput::release(*key)])
                        .collect::<Vec<_>>(),
                    transition: t.to_vec(),
                    input_canceler: vec![KeyInput::release(i[0])],
                })
            })
            .chain(
                single_keys_with_modifires
                    .iter()
                    .flat_map(|(cs, i, o, t, canceler)| {
                        cs.iter().map(move |c| SingleHotkeyEntry {
                            cond: c.iter().copied().collect(),
                            input: *i,
                            output: o.clone(),
                            transition: t.to_vec(),
                            input_canceler: canceler.to_vec(),
                        })
                    }),
            )
            .chain(capslock_side.iter().flat_map(|(input, output)| {
                [vec![2], vec![3]].map(|c| SingleHotkeyEntry {
                    cond: c.iter().copied().collect(),
                    input: KeyInput::press(*input),
                    output: (*output)
                        .iter()
                        .flat_map(|key| [KeyInput::press(*key), KeyInput::release(*key)])
                        .collect::<Vec<_>>(),
                    transition: vec![Remove(2), Insert(3)],
                    input_canceler: vec![KeyInput::release(*input)],
                })
            }))
            .chain(modifiers_trans)
            .chain(sands_config)
            .collect(),
    }
}

fn main() {
    read_events::run(mk_config());
}
