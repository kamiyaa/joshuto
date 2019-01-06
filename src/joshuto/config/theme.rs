extern crate toml;
extern crate xdg;

use std::collections::HashMap;
use std::fs;
use std::process;

#[derive(Debug, Deserialize, Clone)]
pub struct JoshutoColorTheme {
    fg: i16,
    bg: i16,
    bold: bool,
    underline: bool,
}

#[derive(Debug, Deserialize)]
pub struct JoshutoRawTheme {
    selection: Option<JoshutoColorTheme>,
    directory: Option<JoshutoColorTheme>,
    executable: Option<JoshutoColorTheme>,
    link: Option<JoshutoColorTheme>,
    ext: Option<HashMap<String, JoshutoColorTheme>>,
}

impl JoshutoRawTheme {
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        JoshutoRawTheme {
            selection: None,
            directory: None,
            executable: None,
            link: None,
            ext: None,
        }
    }

    pub fn flatten(self) -> JoshutoTheme
    {
        let selection = self.selection.unwrap_or(
            JoshutoColorTheme {
                fg: ncurses::COLOR_YELLOW,
                bg: -1,
                bold: true,
                underline: false,
                }
            );

        let directory = self.directory.unwrap_or(
            JoshutoColorTheme {
                fg: ncurses::COLOR_BLUE,
                bg: -1,
                bold: true,
                underline: false,
                }
            );

        let executable = self.executable.unwrap_or(
            JoshutoColorTheme {
                fg: ncurses::COLOR_GREEN,
                bg: -1,
                bold: true,
                underline: false,
                }
            );

        let link = self.link.unwrap_or(
            JoshutoColorTheme {
                fg: ncurses::COLOR_CYAN,
                bg: -1,
                bold: true,
                underline: false,
                }
            );

        let ext = self.ext.unwrap_or(HashMap::new());

        JoshutoTheme {
            directory,
            selection,
            executable,
            link,
            ext,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JoshutoTheme {
    selection: JoshutoColorTheme,
    directory: JoshutoColorTheme,
    executable: JoshutoColorTheme,
    link: JoshutoColorTheme,
    ext: HashMap<String, JoshutoColorTheme>
}

impl JoshutoTheme {
    pub fn new() -> Self
    {
        let selection = JoshutoColorTheme {
            fg: ncurses::COLOR_YELLOW,
            bg: -1,
            bold: true,
            underline: false,
            };

        let directory = JoshutoColorTheme {
            fg: ncurses::COLOR_BLUE,
            bg: -1,
            bold: true,
            underline: false,
            };

        let executable = JoshutoColorTheme {
            fg: ncurses::COLOR_GREEN,
            bg: -1,
            bold: true,
            underline: false,
            };

        let link = JoshutoColorTheme {
            fg: ncurses::COLOR_CYAN,
            bg: -1,
            bold: true,
            underline: false,
            };

        JoshutoTheme {
            directory,
            selection,
            executable,
            link,
            ext: HashMap::new(),
        }

    }

    fn read_config() -> Option<JoshutoRawTheme>
    {
        match xdg::BaseDirectories::with_profile(::PROGRAM_NAME, "") {
            Ok(dirs) => {
                let config_path = dirs.find_config_file(::THEME_FILE)?;
                match fs::read_to_string(&config_path) {
                    Ok(config_contents) => {
                        match toml::from_str(&config_contents) {
                            Ok(config) => {
                                Some(config)
                            },
                            Err(e) => {
                                eprintln!("{}", e);
                                process::exit(1);
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                        None
                    },
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                None
            },
        }
    }

    pub fn get_config() -> Self
    {
        match Self::read_config() {
            Some(config) => {
                config.flatten()
            }
            None => {
                Self::new()
            }
        }
    }
}
