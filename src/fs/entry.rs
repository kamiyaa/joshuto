use std::{fs, path};

use tui::style::{Modifier, Style};

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

        Ok(Self {
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

    pub fn file_path(&self) -> &path::Path {
        self.path.as_path()
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

    pub fn get_modifier(&self) -> Modifier {
        let metadata = &self.metadata;
        let filetype = &metadata.file_type;

        if filetype.is_dir() {
            THEME_T.directory.modifier
        } else if filetype.is_symlink() {
            THEME_T.link.modifier
        } else {
            match self.file_path().extension() {
                None => Modifier::empty(),
                Some(os_str) => match os_str.to_str() {
                    None => Modifier::empty(),
                    Some(s) => match THEME_T.ext.get(s) {
                        None => Modifier::empty(),
                        Some(t) => t.modifier,
                    },
                },
            }
        }
    }

    pub fn get_style(&self) -> Style {
        let metadata = &self.metadata;
        let filetype = &metadata.file_type;

        if self.is_selected() {
            Style::default()
                .fg(THEME_T.selection.fg)
                .bg(THEME_T.selection.bg)
                .add_modifier(THEME_T.selection.modifier)
        } else if filetype.is_dir() {
            Style::default()
                .fg(THEME_T.directory.fg)
                .bg(THEME_T.directory.bg)
                .add_modifier(THEME_T.directory.modifier)
        } else if filetype.is_symlink() {
            Style::default()
                .fg(THEME_T.link.fg)
                .bg(THEME_T.link.bg)
                .add_modifier(THEME_T.link.modifier)
        } else if unix::is_executable(metadata.mode) {
            Style::default()
                .fg(THEME_T.executable.fg)
                .bg(THEME_T.executable.bg)
                .add_modifier(THEME_T.executable.modifier)
        } else {
            match self.file_path().extension() {
                None => Style::default(),
                Some(os_str) => match os_str.to_str() {
                    None => Style::default(),
                    Some(s) => match THEME_T.ext.get(s) {
                        None => Style::default(),
                        Some(t) => Style::default().fg(t.fg).bg(t.bg).add_modifier(t.modifier),
                    },
                },
            }
        }
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
