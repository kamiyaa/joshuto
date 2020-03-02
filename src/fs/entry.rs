use std::{fs, path};

use tui::style::{Color, Modifier, Style};

use crate::fs::JoshutoMetadata;

use crate::util::unix;
use crate::THEME_T;

#[derive(Clone, Debug)]
pub struct JoshutoDirEntry {
    name: String,
    path: path::PathBuf,
    pub metadata: JoshutoMetadata,
    selected: bool,
    marked: bool,
}

impl JoshutoDirEntry {
    pub fn from(direntry: &fs::DirEntry) -> std::io::Result<Self> {
        let name = match direntry.file_name().into_string() {
            Ok(s) => s,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed converting OsString to String",
                ));
            }
        };

        let path = direntry.path();
        let metadata = JoshutoMetadata::from(&path)?;

        Ok(JoshutoDirEntry {
            name,
            path,
            metadata,
            selected: false,
            marked: false,
        })
    }

    pub fn file_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn file_path(&self) -> &path::PathBuf {
        &self.path
    }

    /*
        pub fn is_marked(&self) -> bool {
            self.marked
        }

        pub fn set_marked(&mut self, marked: bool) {
            self.marked = marked;
        }
    */

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn get_fg_color(&self) -> Color {
        let metadata = &self.metadata;
        let filetype = metadata.file_type;

        if self.is_selected() {
            THEME_T.selection.fg
        } else if filetype.is_dir() {
            THEME_T.directory.fg
        } else if filetype.is_symlink() {
            THEME_T.link.fg
        } else {
            match self.file_path().extension() {
                None => Color::White,
                Some(os_str) => match os_str.to_str() {
                    None => Color::White,
                    Some(s) => match THEME_T.ext.get(s) {
                        None => Color::White,
                        Some(t) => t.fg,
                    },
                },
            }
        }
    }

    pub fn get_bg_color(&self) -> Color {
        let metadata = &self.metadata;
        let filetype = metadata.file_type;

        if self.is_selected() {
            THEME_T.selection.bg
        } else if filetype.is_dir() {
            THEME_T.directory.bg
        } else if filetype.is_symlink() {
            THEME_T.link.bg
        } else {
            match self.file_path().extension() {
                None => Color::Reset,
                Some(os_str) => match os_str.to_str() {
                    None => Color::Reset,
                    Some(s) => match THEME_T.ext.get(s) {
                        None => Color::Reset,
                        Some(t) => t.bg,
                    },
                },
            }
        }
    }

    pub fn get_modifier(&self) -> Modifier {
        let metadata = &self.metadata;
        let filetype = metadata.file_type;

        let mut modifier = Modifier::empty();

        if filetype.is_dir() {
            if THEME_T.directory.bold {
                modifier.insert(Modifier::BOLD);
            }
            if THEME_T.directory.underline {
                modifier.insert(Modifier::UNDERLINED);
            }
        } else if filetype.is_symlink() {
            if THEME_T.link.bold {
                modifier.insert(Modifier::BOLD);
            }
            if THEME_T.link.underline {
                modifier.insert(Modifier::UNDERLINED);
            }
        } else {
            match self.file_path().extension() {
                None => {}
                Some(os_str) => match os_str.to_str() {
                    None => {}
                    Some(s) => match THEME_T.ext.get(s) {
                        None => {}
                        Some(t) => {
                            if t.bold {
                                modifier.insert(Modifier::BOLD);
                            }
                            if t.underline {
                                modifier.insert(Modifier::UNDERLINED);
                            }
                        }
                    },
                },
            };
        }
        modifier
    }

    pub fn get_style(&self) -> Style {
        let metadata = &self.metadata;
        let filetype = metadata.file_type;

        let mut style = Style::default();

        if self.is_selected() {
            let mut modifier = Modifier::empty();
            if THEME_T.selection.bold {
                modifier.insert(Modifier::BOLD);
            }
            if THEME_T.selection.underline {
                modifier.insert(Modifier::UNDERLINED);
            }

            style = style.fg(THEME_T.selection.fg).bg(THEME_T.selection.bg);
            style = style.modifier(modifier);
        } else if filetype.is_dir() {
            let mut modifier = Modifier::empty();
            if THEME_T.directory.bold {
                modifier.insert(Modifier::BOLD);
            }
            if THEME_T.directory.underline {
                modifier.insert(Modifier::UNDERLINED);
            }

            style = style.fg(THEME_T.directory.fg).bg(THEME_T.directory.bg);
            style = style.modifier(modifier);
        } else if filetype.is_symlink() {
            let mut modifier = Modifier::empty();
            if THEME_T.link.bold {
                modifier.insert(Modifier::BOLD);
            }
            if THEME_T.link.underline {
                modifier.insert(Modifier::UNDERLINED);
            }

            style = style.fg(THEME_T.link.fg).bg(THEME_T.link.bg);
            style = style.modifier(modifier);
        } else if unix::is_executable(metadata.mode) {
            let mut modifier = Modifier::empty();
            if THEME_T.link.bold {
                modifier.insert(Modifier::BOLD);
            }
            if THEME_T.link.underline {
                modifier.insert(Modifier::UNDERLINED);
            }

            style = style.fg(THEME_T.executable.fg).bg(THEME_T.executable.bg);
            style = style.modifier(modifier);
        } else {
            match self.file_path().extension() {
                None => {}
                Some(os_str) => match os_str.to_str() {
                    None => {}
                    Some(s) => match THEME_T.ext.get(s) {
                        None => {}
                        Some(t) => {
                            let mut modifier = Modifier::empty();
                            if t.bold {
                                modifier.insert(Modifier::BOLD);
                            }
                            if t.underline {
                                modifier.insert(Modifier::UNDERLINED);
                            }
                            style = style.fg(t.fg).bg(t.bg);
                            style = style.modifier(modifier);
                        }
                    },
                },
            }
        }
        style
    }
}

impl std::fmt::Display for JoshutoDirEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.file_name())
    }
}

impl std::convert::AsRef<str> for JoshutoDirEntry {
    fn as_ref(&self) -> &str {
        self.file_name()
    }
}

impl std::cmp::PartialEq for JoshutoDirEntry {
    fn eq(&self, other: &Self) -> bool {
        self.file_path() == other.file_path()
    }
}
impl std::cmp::Eq for JoshutoDirEntry {}

impl std::cmp::PartialOrd for JoshutoDirEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for JoshutoDirEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.file_path().cmp(other.file_path())
    }
}
