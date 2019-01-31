extern crate toml;
extern crate xdg;

use std::collections::HashMap;

use joshuto::config::{Flattenable, parse_config};

#[derive(Debug, Deserialize, Clone)]
pub struct JoshutoColorPair {
    pub id: i16,
    pub fg: i16,
    pub bg: i16,
}

impl JoshutoColorPair {
    pub fn new(id: i16, fg: i16, bg: i16) -> Self
    {
        JoshutoColorPair {
            id,
            fg,
            bg,
        }
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

impl JoshutoRawColorTheme {
    pub fn flatten(self) -> JoshutoColorTheme
    {
        JoshutoColorTheme {
            colorpair: self.colorpair,
            bold: self.bold.unwrap_or(false),
            underline: self.underline.unwrap_or(false),
            prefix: self.prefix,
            prefixsize: self.prefixsize,
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
    fn flatten(self) -> JoshutoTheme
    {
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

        let selection = self.selection.unwrap_or(JoshutoRawColorTheme {
            colorpair: 3,
            bold: Some(true),
            underline: None,
            prefix: None,
            prefixsize: None,
            });

        let executable = self.executable.unwrap_or(JoshutoRawColorTheme {
            colorpair: 2,
            bold: Some(true),
            underline: None,
            prefix: None,
            prefixsize: None,
            });

        let regular = self.regular.unwrap_or(JoshutoRawColorTheme {
            colorpair: 0,
            bold: None,
            underline: None,
            prefix: None,
            prefixsize: None,
            });

        let directory = self.directory.unwrap_or(JoshutoRawColorTheme {
            colorpair: 4,
            bold: Some(true),
            underline: None,
            prefix: None,
            prefixsize: None,
            });

        let link = self.link.unwrap_or(JoshutoRawColorTheme {
            colorpair: 6,
            bold: Some(true),
            underline: None,
            prefix: None,
            prefixsize: None,
            });

        let socket = self.socket.unwrap_or(JoshutoRawColorTheme {
            colorpair: 6,
            bold: Some(true),
            underline: None,
            prefix: None,
            prefixsize: None,
            });

        let mut extraw = self.ext.unwrap_or(HashMap::new());
        let mut ext: HashMap<String, JoshutoColorTheme> = HashMap::new();
        for (k, v) in extraw.drain() {
            ext.insert(k, v.flatten());
        }

        JoshutoTheme {
            colorpair,
            regular: regular.flatten(),
            directory: directory.flatten(),
            selection: selection.flatten(),
            executable: executable.flatten(),
            link: link.flatten(),
            socket: socket.flatten(),
            ext,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JoshutoColorTheme {
    pub colorpair: i16,
    pub bold: bool,
    pub underline: bool,
    pub prefix: Option<String>,
    pub prefixsize: Option<usize>,
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
    pub ext: HashMap<String, JoshutoColorTheme>
}

impl JoshutoTheme {
    pub fn new() -> Self
    {
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
            prefix: None,
            prefixsize: None,
            };

        let executable = JoshutoColorTheme {
            colorpair: 2,
            bold: true,
            underline: false,
            prefix: None,
            prefixsize: None,
            };

        let regular = JoshutoColorTheme {
            colorpair: 0,
            bold: false,
            underline: false,
            prefix: None,
            prefixsize: None,
            };

        let directory = JoshutoColorTheme {
            colorpair: 4,
            bold: true,
            underline: false,
            prefix: None,
            prefixsize: None,
            };

        let link = JoshutoColorTheme {
            colorpair: 6,
            bold: true,
            underline: false,
            prefix: None,
            prefixsize: None,
            };

        let socket = JoshutoColorTheme {
            colorpair: 6,
            bold: true,
            underline: false,
            prefix: None,
            prefixsize: None,
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

    pub fn get_config() -> JoshutoTheme {
        parse_config::<JoshutoRawTheme, JoshutoTheme>(::THEME_FILE)
            .unwrap_or_else(|| JoshutoTheme::new())
    }
}
