use evdev_rs::enums::EV_KEY;

pub struct KeyConfig {
    pub singles: (EV_KEY, Vec<EV_KEY>),
    pub doubles: (Vec<EV_KEY>, Vec<EV_KEY>),
}

fn zip_keys(keys: std::string::String, out_strings: Vec<toml::Value>) -> Vec<(char, String)> {
    keys.chars()
        .zip(out_strings)
        .fold([].to_vec(), |mut acc, i| match i {
            (c, toml::Value::String(s)) => {
                acc.push((c, s));
                acc
            }
            _ => acc,
        })
}

fn read_combinated_hotkeys(
    head_key: char,
    key_map: &toml::map::Map<String, toml::Value>,
) -> std::vec::Vec<(char, char, String)> {
    key_map
        .iter()
        .fold([].to_vec(), |mut acc, (keys, v)| match v {
            toml::Value::Array(out_strings) => {
                let mut combination: Vec<(char, char, String)> =
                    zip_keys(keys.to_string(), out_strings.to_vec())
                        .iter()
                        .zip(std::iter::repeat(head_key))
                        .map(|((key1, out), key2)| (*key1, key2, out.to_string()))
                        .collect();
                combination.append(&mut acc);
                combination
            }
            _ => acc,
        })
}

fn load_toml() -> Option<(Vec<(char, String)>, Vec<(char, char, String)>)> {
    let toml_string = r#"
        # 新下駄配列
        # http://kouy.exblog.jp/13627994/
        
        qwert = ["-",  "ni", "ha", ",",  "chi"]
        asdfg = ["no", "to", "ka", "nn", "ltu"]
        zxcvb = ["su", "ma", "ki", "ru", "tu"]
        
        "yuiop@" = ["gu", "ba", "ko", "ga",  "hi", "ge"]
        "hjkl;"  = ["ku", "u",  "i",  "shi", "na"]
        "nm,./"  = ["te", "ta", "de", ".",   "bu"]
        
        [k]
        12345 = ["la", "li", "lu", "le", "lo"]
        qwert = ["fa", "go", "fu", "fi", "fe"]
        asdfg = ["ho", "ji", "re", "mo", "yu"]
        zxcvb = ["du", "zo", "bo", "mu", "fo"]
        "#;
    println!("{}", toml_string);
    let config: toml::Value = toml::from_str(toml_string).unwrap();
    println!("{:?}", config);

    type SingleHotkey = (char, String);
    type CombinatedHotkey = (char, char, String);

    let result = config.as_table()?.iter().fold(
        ([].to_vec(), [].to_vec()),
        |(mut single_acc, double_acc): (Vec<SingleHotkey>, Vec<CombinatedHotkey>),
         (s, v): (&String, &toml::Value)| match v {
            toml::Value::Array(a) => (
                {
                    single_acc.append(&mut zip_keys(s.to_string(), a.to_vec()));
                    single_acc
                },
                double_acc,
            ),
            toml::Value::Table(a) => (
                single_acc,
                read_combinated_hotkeys(s.chars().next().unwrap(), a),
            ),
            _ => (single_acc, double_acc),
        },
    );
    Some(result)
}

pub fn run() -> &[(&[EV_KEY], &[EV_KEY])] {
    let (singles, pairs) = load_toml()?;
}
