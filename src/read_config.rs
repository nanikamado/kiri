// use evdev_rs::enums::EV_KEY;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

#[allow(clippy::single_match)]
#[derive(Debug, Clone)]
pub enum Flag {
    Is(String),
    Not(String),
}

pub type Flags = Vec<Flag>;

#[derive(Debug, Clone)]
pub enum KeyOutput {
    Tap(String),
    // Down(String),
    // Up(String),
    Toggle(String),
}

#[derive(Debug, Clone)]
pub struct HotKey {
    // input: HashSet<EV_KEY>,
    pub input: HashSet<String>,
    // output: Vec<EV_KEY>,
    pub output: Vec<KeyOutput>,
    pub condition: Flags,
}

static IF_PATTERN: Lazy<regex::Regex> = Lazy::new(|| Regex::new(r"^if\((.+)\)$").unwrap());
static IF_NOT_PATTERN: Lazy<regex::Regex> = Lazy::new(|| Regex::new(r"^if-not\((.+)\)$").unwrap());
static TOGGLE_PATTERN: Lazy<regex::Regex> = Lazy::new(|| Regex::new(r"^toggle\((.+)\)$").unwrap());

fn read_config_(
    mut hotkeys: Vec<HotKey>,
    config: &toml::value::Map<String, toml::Value>,
    context: HotKey,
    packed: bool,
) -> Vec<HotKey> {
    for c in config {
        if let Some(flag_name) = IF_PATTERN
            .captures(c.0)
            .map(|caps| caps.get(1).unwrap().as_str())
        {
            let mut context = context.clone();
            context.condition.push(Flag::Is(flag_name.to_string()));
            hotkeys = read_config_(hotkeys, c.1.as_table().unwrap(), context, packed);
        } else if let Some(flag_name) = IF_NOT_PATTERN
            .captures(c.0)
            .map(|caps| caps.get(1).unwrap().as_str())
        {
            let mut context = context.clone();
            context.condition.push(Flag::Not(flag_name.to_string()));
            hotkeys = read_config_(hotkeys, c.1.as_table().unwrap(), context, packed);
        } else if c.0 == "packed()" {
            hotkeys = read_config_(hotkeys, c.1.as_table().unwrap(), context.clone(), true);
        } else if packed {
            for (input, output) in c.0.chars().zip(c.1.as_array().unwrap().iter()) {
                let output: Vec<_> = output
                    .as_str()
                    .unwrap()
                    .chars()
                    .map(|c| KeyOutput::Tap(c.to_string()))
                    .collect();
                if !output.is_empty() {
                    let mut context = context.clone();
                    context.input.insert(input.to_string());
                    context.output = output;
                    hotkeys.push(context);
                }
            }
        } else {
            match c.1 {
                toml::Value::String(output) => {
                    if !output.is_empty() {
                        let mut context = context.clone();
                        context.input.insert(c.0.to_string());
                        if let Some(flag_name) = TOGGLE_PATTERN
                            .captures(output)
                            .map(|caps| caps.get(1).unwrap().as_str())
                        {
                            context.output = vec![KeyOutput::Toggle(flag_name.to_string())];
                        } else {
                            context.output = output
                                .chars()
                                .map(|c| KeyOutput::Tap(c.to_string()))
                                .collect();
                        }
                        hotkeys.push(context);
                    }
                }
                toml::Value::Table(config) => {
                    let mut context = context.clone();
                    context.input.insert(c.0.to_string());
                    hotkeys = read_config_(hotkeys, config, context, false);
                }
                _ => panic!("got: {:?}", c.1),
            }
        }
    }
    hotkeys
}

fn read_config(config: &toml::value::Map<String, toml::Value>) -> Vec<HotKey> {
    read_config_(
        Vec::new(),
        config,
        HotKey {
            input: HashSet::new(),
            output: Vec::new(),
            condition: Vec::new(),
        },
        false,
    )
}

pub fn run() -> Result<Vec<HotKey>, Box<dyn std::error::Error>> {
    let toml_string = std::fs::read_to_string("src/singeta.toml").expect("");
    let config: toml::Value = toml::from_str(&toml_string)?;
    let config = config.as_table().ok_or("config file is not a table")?;
    let hotkeys = read_config(config);
    // dbg!(hotkeys);
    // println!("{:?}", hotkeys);
    Ok(hotkeys)
}
