use serde_derive::Deserialize;
use std::collections::HashMap;

use crate::config::{parse_config_file, Flattenable};

#[derive(Debug, Deserialize, Clone)]
pub struct JoshutoColorPair {
    pub id: i16,
    pub fg: i16,
    pub bg: i16,
}

impl JoshutoColorPair {
    pub fn new(id: i16, fg: i16, bg: i16) -> Self {
        JoshutoColorPair { id, fg, bg }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct JoshutoRawColorTheme {
    pub colorpair: i16,
    pub bold: Option<bool>,
    pub underline: Option<bool>,
    pub prefix: Option<String>,
    pub prefixsize: Option<usize>,
}

impl Flattenable<JoshutoColorTheme> for JoshutoRawColorTheme {
    fn flatten(self) -> JoshutoColorTheme {
        JoshutoColorTheme {
            colorpair: self.colorpair,
            bold: self.bold.unwrap_or(false),
            underline: self.underline.unwrap_or(false),
            prefix: self
                .prefixsize
                .and_then(|size| self.prefix.and_then(|p| Some((size, p)))),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct JoshutoRawTheme {
    colorpair: Option<Vec<JoshutoColorPair>>,
    selection: Option<JoshutoRawColorTheme>,
    executable: Option<JoshutoRawColorTheme>,
    regular: Option<JoshutoRawColorTheme>,
    directory: Option<JoshutoRawColorTheme>,
    link: Option<JoshutoRawColorTheme>,
    socket: Option<JoshutoRawColorTheme>,
    ext: Option<HashMap<String, JoshutoRawColorTheme>>,
}

impl Flattenable<JoshutoTheme> for JoshutoRawTheme {
    fn flatten(self) -> JoshutoTheme {
        let colorpair = match self.colorpair {
            Some(s) => s,
            None => {
                let mut colorpair: Vec<JoshutoColorPair> = Vec::with_capacity(10);
                colorpair.push(JoshutoColorPair::new(2, 2, -1));
                colorpair.push(JoshutoColorPair::new(3, 3, -1));
                colorpair.push(JoshutoColorPair::new(4, 4, -1));
                colorpair.push(JoshutoColorPair::new(5, 5, -1));
                colorpair.push(JoshutoColorPair::new(6, 6, -1));
                colorpair
            }
        };

        let selection = self
            .selection
            .and_then(|x| Some(x.flatten()))
            .unwrap_or_else(|| JoshutoColorTheme {
                colorpair: 3,
                bold: true,
                underline: false,
                prefix: Some((2, String::from("  "))),
            });

        let executable = self
            .executable
            .and_then(|x| Some(x.flatten()))
            .unwrap_or_else(|| JoshutoColorTheme {
                colorpair: 2,
                bold: true,
                underline: false,
                prefix: None,
            });

        let regular = self
            .regular
            .and_then(|x| Some(x.flatten()))
            .unwrap_or_else(|| JoshutoColorTheme {
                colorpair: 0,
                bold: false,
                underline: false,
                prefix: None,
            });

        let directory = self
            .directory
            .and_then(|x| Some(x.flatten()))
            .unwrap_or_else(|| JoshutoColorTheme {
                colorpair: 4,
                bold: true,
                underline: false,
                prefix: None,
            });

        let link = self
            .link
            .and_then(|x| Some(x.flatten()))
            .unwrap_or_else(|| JoshutoColorTheme {
                colorpair: 6,
                bold: true,
                underline: false,
                prefix: None,
            });

        let socket = self
            .socket
            .and_then(|x| Some(x.flatten()))
            .unwrap_or_else(|| JoshutoColorTheme {
                colorpair: 6,
                bold: true,
                underline: false,
                prefix: None,
            });

        let mut extraw = self.ext.unwrap_or_default();
        let mut ext: HashMap<String, JoshutoColorTheme> = HashMap::with_capacity(extraw.capacity());
        for (k, v) in extraw.drain() {
            ext.insert(k, v.flatten());
        }

        JoshutoTheme {
            colorpair,
            regular,
            directory,
            selection,
            executable,
            link,
            socket,
            ext,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JoshutoColorTheme {
    pub colorpair: i16,
    pub bold: bool,
    pub underline: bool,
    pub prefix: Option<(usize, String)>,
}

#[derive(Debug, Clone)]
pub struct JoshutoTheme {
    pub colorpair: Vec<JoshutoColorPair>,
    pub regular: JoshutoColorTheme,
    pub selection: JoshutoColorTheme,
    pub directory: JoshutoColorTheme,
    pub executable: JoshutoColorTheme,
    pub link: JoshutoColorTheme,
    pub socket: JoshutoColorTheme,
    pub ext: HashMap<String, JoshutoColorTheme>,
}

impl JoshutoTheme {
    pub fn get_config() -> JoshutoTheme {
        parse_config_file::<JoshutoRawTheme, JoshutoTheme>(crate::THEME_FILE)
            .unwrap_or_else(JoshutoTheme::default)
    }
}

impl std::default::Default for JoshutoTheme {
    fn default() -> Self {
        let mut colorpair: Vec<JoshutoColorPair> = Vec::with_capacity(10);
        colorpair.push(JoshutoColorPair::new(2, 2, -1));
        colorpair.push(JoshutoColorPair::new(3, 3, -1));
        colorpair.push(JoshutoColorPair::new(4, 4, -1));
        colorpair.push(JoshutoColorPair::new(5, 5, -1));
        colorpair.push(JoshutoColorPair::new(6, 6, -1));

        let selection = JoshutoColorTheme {
            colorpair: 3,
            bold: true,
            underline: false,
            prefix: Some((2, String::from("  "))),
        };

        let executable = JoshutoColorTheme {
            colorpair: 2,
            bold: true,
            underline: false,
            prefix: None,
        };

        let regular = JoshutoColorTheme {
            colorpair: 0,
            bold: false,
            underline: false,
            prefix: None,
        };

        let directory = JoshutoColorTheme {
            colorpair: 4,
            bold: true,
            underline: false,
            prefix: None,
        };

        let link = JoshutoColorTheme {
            colorpair: 6,
            bold: true,
            underline: false,
            prefix: None,
        };

        let socket = JoshutoColorTheme {
            colorpair: 6,
            bold: true,
            underline: false,
            prefix: None,
        };

        JoshutoTheme {
            colorpair,
            selection,
            executable,
            regular,
            directory,
            link,
            socket,
            ext: HashMap::new(),
        }
    }
}
