use crate::write_keys::{self, KeyWriter};
use evdev::Key;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;
use std::collections::BTreeSet;
use std::fmt::{self, Debug};
use std::hash::Hash;
use std::time::SystemTime;
use std::{
    sync::mpsc::{channel, Sender},
    thread, time,
};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct PairHotkeyEntry<State> {
    pub cond: State,
    pub input: [Key; 2],
    pub output_keys: Vec<KeyInput>,
    pub transition: State,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum KeyInputKind {
    Press,
    Release,
}

impl From<KeyInputKind> for i32 {
    fn from(k: KeyInputKind) -> Self {
        match k {
            KeyInputKind::Press => 1,
            KeyInputKind::Release => 0,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct KeyInput(pub Key, pub KeyInputKind);

impl KeyInput {
    pub fn press(key: Key) -> KeyInput {
        Self(key, KeyInputKind::Press)
    }

    pub fn release(key: Key) -> KeyInput {
        Self(key, KeyInputKind::Release)
    }
}

impl Debug for KeyInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use KeyInputKind::*;
        let KeyInput(key, input_kind) = self;
        write!(
            f,
            "{:?} {}",
            key,
            match input_kind {
                Press => "↓",
                Release => "↑",
            }
        )
    }
}

impl From<i32> for KeyInputKind {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::Release,
            1 => Self::Press,
            2 => Self::Press,
            _ => panic!("unknown input_event value"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct SingleHotkeyEntry<State> {
    pub cond: State,
    pub input: KeyInput,
    pub output: Vec<KeyInput>,
    pub transition: State,
}

#[derive(PartialEq, Eq, Clone)]
pub struct KeyConfigUnit<State> {
    pub pair_hotkeys: Vec<PairHotkeyEntry<State>>,
    pub single_hotkeys: Vec<SingleHotkeyEntry<State>>,
    pub layer_name: &'static str,
    pub initial_state: State,
}

impl<State: Debug> fmt::Debug for KeyConfigUnit<State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "pair_hotkeys: ")?;
        for e in &self.pair_hotkeys {
            writeln!(f, "    {:?}", e)?;
        }
        write!(f, "single_hotkeys: ")?;
        for e in &self.single_hotkeys {
            write!(f, "\n    {:?}", e)?;
        }
        Ok(())
    }
}

impl<State: Debug> fmt::Debug for PairHotkeyEntry<State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}, {:?}, {:?}, {:?}",
            self.cond, self.input, self.output_keys, self.transition
        )
    }
}

impl<State: Debug> fmt::Debug for SingleHotkeyEntry<State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}, {:?}, {:?}, {:?}",
            self.cond, self.input, self.output, self.transition,
        )
    }
}

type KeyEv = (Key, SystemTime);

#[derive(Debug)]
enum KeyRecorderBehavior {
    FireSpecificWaitingKey(KeyEv),
    SendKey((KeyInput, SystemTime)),
}

fn fire_waiting_key_with_delay(key: (Key, SystemTime), tx: Sender<KeyRecorderBehavior>) {
    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(50));
        tx.send(KeyRecorderBehavior::FireSpecificWaitingKey(key))
            .unwrap();
    });
}

pub struct KeyRecorder {
    tx: Sender<KeyRecorderBehavior>,
    layer_name: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Action<State> {
    pub output_keys: Vec<KeyInput>,
    pub transition: State,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct PressingPair<'a, State> {
    pub pair: BTreeSet<Key>,
    pub action: Option<&'a Action<State>>,
}

pub trait KeyReceiver: Send {
    fn send_key(&mut self, key: KeyInput, time: SystemTime);
}

fn perform_action<T, State: Eq>(
    action: &Action<State>,
    time: SystemTime,
    layer_name: &'static str,
    recorder_state: &mut KeyRecorderUnitState<T, State>,
) where
    T: KeyReceiver,
    State: Debug + Copy,
{
    for output_key in &action.output_keys {
        recorder_state.key_receiver.send_key(*output_key, time);
    }
    if action.transition != recorder_state.state {
        log::debug!(
            "[{}] state : {:?} =====> {:?}",
            layer_name,
            recorder_state.state,
            action.transition
        );
        recorder_state.state = action.transition;
    }
}

fn fire_key_input<T, State: Eq + Copy + Debug + Hash>(
    key: KeyInput,
    time: SystemTime,
    recorder_info: &KeyRecorderUnitInfo<State>,
    recorder_state: &mut KeyRecorderUnitState<T, State>,
) where
    T: KeyReceiver,
{
    if let Some(action) = recorder_info
        .single_hotkeys_map
        .get(&(key, recorder_state.state))
    {
        perform_action(action, time, recorder_info.layer_name, recorder_state);
    } else {
        recorder_state.key_receiver.send_key(key, time);
    }
}

fn fire_specific_waiting_key_handler<T, State: Eq + Copy + Debug + Hash>(
    key: Key,
    time: SystemTime,
    recorder_info: &KeyRecorderUnitInfo<State>,
    recorder_state: &mut KeyRecorderUnitState<T, State>,
) where
    T: KeyReceiver,
{
    if Some((key, time)) == recorder_state.waiting_key {
        recorder_state.waiting_key = None;
        fire_key_input(KeyInput::press(key), time, recorder_info, recorder_state);
    }
}

fn fire_waiting_key<T, State: Eq + Copy + Debug + Hash>(
    recorder_info: &KeyRecorderUnitInfo<State>,
    recorder_state: &mut KeyRecorderUnitState<T, State>,
) where
    T: KeyReceiver,
{
    if let Some((key, time)) = recorder_state.waiting_key {
        recorder_state.waiting_key = None;
        fire_key_input(KeyInput::press(key), time, recorder_info, recorder_state);
    }
}

fn send_key_handler<'a, T, State: Eq + Copy + Debug + Hash>(
    key: KeyInput,
    time: SystemTime,
    recorder_info: &'a KeyRecorderUnitInfo<State>,
    pressing_pair: &mut PressingPair<'a, State>,
    recorder_state: &mut KeyRecorderUnitState<T, State>,
    tx: &Sender<KeyRecorderBehavior>,
) where
    T: KeyReceiver,
{
    match key {
        KeyInput(key_name, KeyInputKind::Press)
            if recorder_info
                .pair_input_keys
                .contains(&(key_name, recorder_state.state)) =>
        {
            match recorder_state.waiting_key {
                Some((waiting_key_kind, _)) => {
                    let key_set = [waiting_key_kind, key_name];
                    let key_set = key_set.iter().copied().collect::<BTreeSet<Key>>();
                    let key_set_state = (key_set, recorder_state.state);
                    if let Some(action) = recorder_info.pair_hotkeys_map.get(&key_set_state) {
                        recorder_state.waiting_key = None;
                        *pressing_pair = PressingPair {
                            pair: key_set_state.0,
                            action: Some(action),
                        };
                        perform_action(action, time, recorder_info.layer_name, recorder_state);
                    } else {
                        fire_key_input(
                            KeyInput::press(waiting_key_kind),
                            time,
                            recorder_info,
                            recorder_state,
                        );
                        recorder_state.waiting_key = Some((key_name, time));
                        fire_waiting_key_with_delay((key_name, time), tx.clone());
                    }
                }
                _ => {
                    recorder_state.waiting_key = Some((key_name, time));
                    fire_waiting_key_with_delay((key_name, time), tx.clone());
                }
            }
        }
        _ => {
            pressing_pair.pair.remove(&key.0);
            fire_waiting_key(recorder_info, recorder_state);
            fire_key_input(key, time, recorder_info, recorder_state);
        }
    };
}

struct KeyRecorderUnitState<T, State>
where
    T: KeyReceiver,
{
    key_receiver: T,
    state: State,
    waiting_key: Option<(Key, SystemTime)>,
}

struct KeyRecorderUnitInfo<State: Eq + Copy + Debug + Hash> {
    pair_hotkeys_map: HashMap<(BTreeSet<Key>, State), Action<State>>,
    pair_input_keys: HashSet<(Key, State)>,
    single_hotkeys_map: HashMap<(KeyInput, State), Action<State>>,
    layer_name: &'static str,
}

impl KeyRecorder {
    fn new<State: Eq + Copy + Debug + Hash + Send + 'static>(
        key_config: KeyConfigUnit<State>,
        key_receiver: impl KeyReceiver + 'static,
    ) -> KeyRecorder {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();
        let layer_name = key_config.layer_name;
        let initial_state = key_config.initial_state;
        thread::spawn(move || {
            let pair_input_keys: HashSet<(Key, State)> = key_config
                .pair_hotkeys
                .iter()
                .flat_map(|PairHotkeyEntry { cond, input, .. }| input.map(move |i| (i, *cond)))
                .collect();
            let pair_hotkeys_map: HashMap<(BTreeSet<Key>, State), Action<State>> = key_config
                .pair_hotkeys
                .into_iter()
                .map(
                    |PairHotkeyEntry {
                         cond,
                         input,
                         output_keys,
                         transition,
                     }| {
                        (
                            (input.iter().copied().collect(), cond),
                            Action {
                                output_keys,
                                transition,
                            },
                        )
                    },
                )
                .collect();
            let single_hotkeys_map: HashMap<(KeyInput, State), Action<State>> = key_config
                .single_hotkeys
                .into_iter()
                .map(
                    |SingleHotkeyEntry {
                         cond,
                         input,
                         output,
                         transition,
                     }| {
                        (
                            (input, cond),
                            Action {
                                output_keys: output,
                                transition,
                            },
                        )
                    },
                )
                .collect();
            let mut pressing_pair: PressingPair<State> = PressingPair {
                pair: Default::default(),
                action: Default::default(),
            };
            let mut recorder_state = KeyRecorderUnitState {
                key_receiver,
                state: initial_state,
                waiting_key: None,
            };
            let recorder_info = KeyRecorderUnitInfo {
                pair_hotkeys_map,
                pair_input_keys,
                single_hotkeys_map,
                layer_name,
            };
            for received in rx {
                match received {
                    KeyRecorderBehavior::FireSpecificWaitingKey((key, time)) => {
                        fire_specific_waiting_key_handler(
                            key,
                            time,
                            &recorder_info,
                            &mut recorder_state,
                        )
                    }
                    KeyRecorderBehavior::SendKey((key, time)) => send_key_handler(
                        key,
                        time,
                        &recorder_info,
                        &mut pressing_pair,
                        &mut recorder_state,
                        &tx_clone,
                    ),
                }
            }
        });
        KeyRecorder { tx, layer_name }
    }
}

impl KeyReceiver for KeyRecorder {
    fn send_key(&mut self, key: KeyInput, time: SystemTime) {
        log::debug!("[{}] {:?}", self.layer_name, key);
        self.tx
            .send(KeyRecorderBehavior::SendKey((key, time)))
            .unwrap();
    }
}

impl KeyReceiver for KeyWriter {
    fn send_key(&mut self, key: KeyInput, _: SystemTime) {
        self.fire_key_input(key);
    }
}

pub struct KeyConfigLayer<State: Eq + Copy + Debug + Hash + 'static, Tail>(
    KeyConfigUnit<State>,
    Tail,
);

pub trait KeyConfig {
    fn to_key_recorder_unit(&self) -> KeyRecorder;
}

impl<State: Eq + Copy + Debug + Hash + Send + 'static, T: KeyConfig> KeyConfig
    for KeyConfigLayer<State, T>
{
    fn to_key_recorder_unit(&self) -> KeyRecorder {
        KeyRecorder::new(self.0.clone(), self.1.to_key_recorder_unit())
    }
}

impl<State: Eq + Copy + Debug + Hash + Send + 'static> KeyConfig for KeyConfigUnit<State> {
    fn to_key_recorder_unit(&self) -> KeyRecorder {
        KeyRecorder::new(self.clone(), write_keys::KeyWriter::new())
    }
}

pub trait AddLayer {
    type LayerAdded<A>;

    fn add_layer<T: AddLayer>(self, tail: T) -> Self::LayerAdded<T>;
}

impl<State: Eq + Copy + Debug + Hash + 'static, Tail: AddLayer> AddLayer
    for KeyConfigLayer<State, Tail>
{
    type LayerAdded<A> = KeyConfigLayer<State, Tail::LayerAdded<A>>;

    fn add_layer<T: AddLayer>(self, tail: T) -> Self::LayerAdded<T> {
        KeyConfigLayer(self.0, self.1.add_layer(tail))
    }
}

impl<State: Eq + Copy + Debug + Hash + 'static> AddLayer for KeyConfigUnit<State> {
    type LayerAdded<A> = KeyConfigLayer<State, A>;

    fn add_layer<T: AddLayer>(self, tail: T) -> Self::LayerAdded<T> {
        KeyConfigLayer(self, tail)
    }
}

pub struct EmptyCinfig;

impl AddLayer for EmptyCinfig {
    type LayerAdded<A> = A;

    fn add_layer<T: AddLayer>(self, tail: T) -> Self::LayerAdded<T> {
        tail
    }
}
