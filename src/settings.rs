use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use serde::Deserialize;
use serde_yaml::Mapping;
use thiserror::Error;

use crate::{bspc::Node, colors::Color, formatter::Formatter};

pub struct DrawSettings {
    pub prefix: Option<String>,
    pub postfix: Option<String>,
    pub separator: String,
    pub node_draw_mode: Formatter,
    pub focused_node_draw_mode: Formatter,
    pub urgent_node_draw_mode: Formatter,
    pub workspace_draw_mode: Formatter,
    pub focused_workspace_draw_mode: Formatter,
}

#[derive(Debug)]
pub enum Predicate {
    Matches(regex::Regex),
    Contains(String),
    StartsWith(String),
}

impl Predicate {
    fn is_match(&self, v: &str) -> bool {
        let v = v.to_lowercase();
        match self {
            Predicate::Matches(r) => r.is_match(v.as_str()),
            Predicate::Contains(r) => v.contains(r.as_str()),
            Predicate::StartsWith(r) => v.starts_with(r.as_str()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Icon(String);
impl Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug)]
pub struct PropertyListItem {
    predicate: Predicate,
    icon: Icon,
    sub_list: Option<Box<Property>>,
}

#[derive(Debug)]
pub enum Property {
    ClassName(Vec<PropertyListItem>),
    WindowTitle(Vec<PropertyListItem>),
}

#[derive(Default, Debug)]
pub struct Icons {
    pub icons: Vec<Property>,
    pub tick_rate: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct IconYaml {
    pub icons: Vec<Mapping>,
    pub tick_rate: Option<u64>,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed parsing config: {0}")]
    ParseFailed(String),
    #[error("Unknown key: {0}")]
    UnknownKey(String),
    #[error("Missing predicate")]
    MissingPredicate,
}

fn get_properties(key: &str, seq: &[serde_yaml::Value]) -> Property {
    let known_keys = ["class_name", "window_title"];
    type V = serde_yaml::Value;
    let mappings = seq
        .iter()
        .filter_map(|s| match s {
            V::Mapping(m) => Some(m),
            _ => None,
        })
        .collect::<Vec<_>>();

    let mut list_items = vec![];
    for mapping in mappings {
        fn get_predicate(key: &str, value: &str) -> Option<Predicate> {
            match key {
                "matches" => Some(Predicate::Matches(
                    regex::Regex::from_str(value).expect("Failed to parse pattern."),
                )),
                "contains" => Some(Predicate::Contains(value.to_string())),
                "starts_with" => Some(Predicate::StartsWith(value.to_string())),
                _ => None,
            }
        }
        let icon = mapping.iter().find_map(|m| match m {
            (V::String(key), V::String(value)) if key == "icon" => Some(Icon(value.to_string())),
            _ => None,
        });
        let predicate = mapping.iter().find_map(|m| match m {
            (V::String(ref key), V::String(ref value)) => get_predicate(key, value),
            _ => None,
        });
        let sublist = mapping.iter().find_map(|m| match m {
            (V::String(key), V::Sequence(seq)) if known_keys.contains(&key.as_str()) => {
                Some(get_properties(key, seq))
            }
            _ => None,
        });
        let Some(predicate) = predicate else {
            eprintln!("Missing predicate in mapping {:?}", mapping);
            continue;
        };
        let Some(icon) = icon else {
            eprintln!("Missing predicate in mapping {:?}", mapping);
            continue;
        };
        let list_item = PropertyListItem {
            predicate,
            icon,
            sub_list: sublist.map(Box::new),
        };
        list_items.push(list_item);
    }
    match key {
        "class_name" => Property::ClassName(list_items),
        "window_title" => Property::WindowTitle(list_items),
        _ => panic!("Unknown property key: {}", key),
    }
}

impl PropertyListItem {
    fn get_icon(&self, s: &str, node: &Node) -> Option<Icon> {
        if self.predicate.is_match(s) {
            if let Some(ref prop) = self.sub_list {
                return prop.get_icon(node).or(Some(self.icon.clone()));
            }
            Some(self.icon.clone())
        } else {
            None
        }
    }
}

impl Property {
    fn get_icon(&self, node: &Node) -> Option<Icon> {
        match self {
            Property::ClassName(conditions) => {
                let s = node.client.as_ref().unwrap().class_name.to_string();
                return conditions.iter().find_map(|c| c.get_icon(s.as_str(), node));
            }
            Property::WindowTitle(conditions) => {
                let s = node.get_wm_name();
                if let Some(ref s) = s {
                    return conditions.iter().find_map(|c| c.get_icon(s.as_str(), node));
                }
                None
            }
        }
    }
}

impl Icons {
    fn new(yaml: &IconYaml) -> Result<Self> {
        let mut result = Icons {
            tick_rate: yaml.tick_rate,
            ..Default::default()
        };

        let known_keys = ["class_name", "window_title"];
        type V = serde_yaml::Value;
        for mapping in yaml.icons.iter() {
            for m in mapping.iter() {
                match m {
                    (V::String(key), V::Sequence(seq)) if known_keys.contains(&key.as_str()) => {
                        result.icons.push(get_properties(key, seq));
                    }
                    _ => Err(ParseError::ParseFailed(format!(
                        "Expected 'class_name' or 'window_title' -> sequence, found {:?}",
                        m.0
                    )))?,
                }
            }
        }
        Ok(result)
    }
    pub fn get_icon(&self, node: &Node) -> String {
        for icon in self.icons.iter() {
            if let Some(icon) = icon.get_icon(node) {
                return icon.to_string();
            }
        }
        node.get_wm_name()
            .unwrap_or(node.client.as_ref().unwrap().class_name.clone())
    }
}

pub struct Settings {
    pub monitor: Option<String>,
    pub icons: Icons,
    pub draw_settings: DrawSettings,
}

pub fn get_settings() -> Settings {
    let cfg = guess_config_path().expect("No config found. Please specify a config file or create one at ~/.config/iconography/config.yml");
    let r = std::fs::read(cfg).expect("Failed to read config file.");
    let icon_settings: IconYaml = serde_yaml::from_slice(&r).expect("Failed to parse config");
    let mut settings = get_default_settings();
    settings.icons = Icons::new(&icon_settings).expect("Failed to parse map");
    settings
}

fn guess_config_path() -> Option<String> {
    let config_home = if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
        std::path::PathBuf::from(config_home)
    } else if let Ok(home) = std::env::var("HOME") {
        std::path::Path::new(&home).join(".config")
    } else {
        return None;
    };
    let path = config_home.join("iconography").join("config.yml");
    if path.is_file() {
        return Some(path.to_string_lossy().to_string());
    }
    None
}

fn get_default_settings() -> Settings {
    use std::env::var;
    fn col_env(e: &str) -> Option<Color> {
        var(e).ok().and_then(|c| {
            if c.is_empty() {
                None
            } else {
                let col = Color::try_from(c.as_str()).unwrap_or_else(|_| {
                    panic!("Error parsing color from environment '{}'. Was: '{}'", e, c)
                });
                Some(col)
            }
        })
    }

    Settings {
        monitor: var("MONITOR").ok().or(None),
        icons: Default::default(),
        draw_settings: DrawSettings {
            prefix: var("WS_START").ok(),
            postfix: var("WS_END").ok(),
            separator: var("WS_SEPARATOR").unwrap_or("┊".to_string()),
            node_draw_mode: Formatter {
                foreground: None,
                background: None,
                underline: None,
                overline: None,
                highlight: false,
            },
            focused_node_draw_mode: Formatter {
                foreground: None,
                background: None,
                underline: None,
                overline: col_env("ACCENT").or(Color::try_from("fff").ok()),
                highlight: false,
            },
            urgent_node_draw_mode: Formatter {
                foreground: col_env("URGENT_FOREGROUND").or(None),
                background: col_env("URGENT_BACKGROUND").or(Color::try_from("a22").ok()),
                overline: col_env("URGENT_ACCENT").or(None),
                underline: None,
                highlight: false,
            },
            workspace_draw_mode: Formatter {
                foreground: col_env("FOREGROUND"),
                background: col_env("BACKGROUND").or(None),
                overline: None,
                underline: None,
                highlight: false,
            },
            focused_workspace_draw_mode: Formatter {
                foreground: col_env("FOCUSED_FOREGROUND").or(Color::try_from("fff").ok()),
                background: col_env("FOCUSED_BACKGROUND"),
                overline: None,
                underline: col_env("FOCUSED_ACCENT").or(Color::try_from("ac21c4").ok()),
                highlight: false,
            },
        },
    }
}
