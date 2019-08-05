use serde_derive::Deserialize;
use std::collections::HashMap;

use crate::THEME_FILE;

use super::{parse_config_file, ConfigStructure};

const fn default_zero() -> i16 {
    0
}
const fn default_false() -> bool {
    false
}
const fn default_prefix() -> Option<JoshutoPrefix> {
    None
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoshutoColorPair {
    pub id: i16,
    #[serde(default = "default_zero")]
    pub fg: i16,
    #[serde(default = "default_zero")]
    pub bg: i16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoshutoPrefix {
    prefix: String,
    size: usize,
}

impl JoshutoPrefix {
    pub fn prefix(&self) -> &str {
        self.prefix.as_str()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoshutoColorTheme {
    pub colorpair: i16,
    #[serde(default = "default_false")]
    pub bold: bool,
    #[serde(default = "default_false")]
    pub underline: bool,
    #[serde(default = "default_prefix")]
    pub prefix: Option<JoshutoPrefix>,
}

impl std::default::Default for JoshutoColorTheme {
    fn default() -> Self {
        JoshutoColorTheme {
            colorpair: default_zero(),
            bold: default_false(),
            underline: default_false(),
            prefix: default_prefix(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoshutoTheme {
    #[serde(default)]
    pub colorpair: Vec<JoshutoColorPair>,
    #[serde(default)]
    pub regular: JoshutoColorTheme,
    #[serde(default)]
    pub selection: JoshutoColorTheme,
    #[serde(default)]
    pub directory: JoshutoColorTheme,
    #[serde(default)]
    pub executable: JoshutoColorTheme,
    #[serde(default)]
    pub link: JoshutoColorTheme,
    #[serde(default)]
    pub socket: JoshutoColorTheme,
    #[serde(default)]
    pub ext: HashMap<String, JoshutoColorTheme>,
}

impl ConfigStructure for JoshutoTheme {
    fn get_config() -> Self {
        parse_config_file::<JoshutoTheme>(THEME_FILE).unwrap_or_else(Self::default)
    }
}

impl std::default::Default for JoshutoTheme {
    fn default() -> Self {
        let colorpair: Vec<JoshutoColorPair> = Vec::new();
        let selection = JoshutoColorTheme::default();
        let executable = JoshutoColorTheme::default();
        let regular = JoshutoColorTheme::default();
        let directory = JoshutoColorTheme::default();
        let link = JoshutoColorTheme::default();
        let socket = JoshutoColorTheme::default();
        let ext = HashMap::new();

        JoshutoTheme {
            colorpair,
            selection,
            executable,
            regular,
            directory,
            link,
            socket,
            ext,
        }
    }
}
