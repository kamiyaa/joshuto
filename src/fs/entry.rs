use std::{fs, path};

use crate::fs::{FileType, JoshutoMetadata};

#[cfg(feature = "devicons")]
use crate::util::devicons::*;

#[derive(Clone, Debug)]
pub struct JoshutoDirEntry {
    name: String,
    label: String,
    path: path::PathBuf,
    pub metadata: JoshutoMetadata,
    selected: bool,
    marked: bool,
}

impl JoshutoDirEntry {
    pub fn from(direntry: &fs::DirEntry, show_icons: bool) -> std::io::Result<Self> {
        let path = direntry.path();
        let metadata = JoshutoMetadata::from(&path)?;
        let name = direntry
            .file_name()
            .as_os_str()
            .to_string_lossy()
            .to_string();

        #[cfg(feature = "devicons")]
        let label = if show_icons {
            let icon = match metadata.file_type() {
                FileType::Directory => DIR_NODE_EXACT_MATCHES
                    .get(name.as_str())
                    .cloned()
                    .unwrap_or(DEFAULT_DIR),
                _ => FILE_NODE_EXACT_MATCHES
                    .get(name.as_str())
                    .cloned()
                    .unwrap_or(match path.extension() {
                        Some(s) => FILE_NODE_EXTENSIONS
                            .get(match s.to_str() {
                                Some(s) => s,
                                None => {
                                    return Err(std::io::Error::new(
                                        std::io::ErrorKind::Other,
                                        "Failed converting OsStr to str",
                                    ))
                                }
                            })
                            .unwrap_or(&DEFAULT_FILE),
                        None => DEFAULT_FILE,
                    }),
            };
            format!("{} {}", icon, name)
        } else {
            name.clone()
        };
        #[cfg(not(feature = "devicons"))]
        let label = name.clone();

        Ok(Self {
            name,
            label,
            path,
            metadata,
            selected: false,
            marked: false,
        })
    }

    pub fn file_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    pub fn file_path(&self) -> &path::Path {
        self.path.as_path()
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
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
