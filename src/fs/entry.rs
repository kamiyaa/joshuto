use std::{fs, io, path};

use crate::config::option::DisplayOption;
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
    _marked: bool,
}

impl JoshutoDirEntry {
    pub fn from(direntry: &fs::DirEntry, options: &DisplayOption) -> io::Result<Self> {
        let path = direntry.path();

        let name = direntry
            .file_name()
            .as_os_str()
            .to_string_lossy()
            .to_string();

        Self::gen_entry(path, name, options)
    }

    pub fn from_walk(
        direntry: &walkdir::DirEntry,
        base: &path::Path,
        options: &DisplayOption,
    ) -> io::Result<Self> {
        let path = direntry.path().to_path_buf();

        let name = direntry
            .path()
            .strip_prefix(base)
            .unwrap()
            .to_string_lossy()
            .to_string();

        Self::gen_entry(path, name, options)
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

    pub fn file_path_buf(&self) -> path::PathBuf {
        self.path.clone()
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn get_ext(&self) -> &str {
        let fname = self.file_name();
        match fname.rfind('.') {
            Some(pos) => &fname[pos..],
            None => "",
        }
    }

    fn gen_entry(path: path::PathBuf, name: String, options: &DisplayOption) -> io::Result<Self> {
        let mut metadata = JoshutoMetadata::from(&path)?;

        if options.automatically_count_files() && metadata.file_type().is_dir() {
            if let Ok(size) = get_directory_size(path.as_path()) {
                metadata.update_directory_size(size);
            }
        }

        #[cfg(feature = "devicons")]
        let label = if options.show_icons() {
            create_icon_label(name.as_str(), &metadata)
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
            _marked: false,
        })
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

fn create_icon_label(name: &str, metadata: &JoshutoMetadata) -> String {
    let label = {
        let icon =
            match metadata.file_type() {
                FileType::Directory => DIR_NODE_EXACT_MATCHES
                    .get(name)
                    .cloned()
                    .unwrap_or(DEFAULT_DIR),
                _ => FILE_NODE_EXACT_MATCHES.get(name).cloned().unwrap_or(
                    match name.rsplit_once('.') {
                        Some((_, ext)) => FILE_NODE_EXTENSIONS.get(ext).unwrap_or(&DEFAULT_FILE),
                        None => DEFAULT_FILE,
                    },
                ),
            };
        format!("{} {}", icon, name)
    };
    label
}

fn get_directory_size(path: &path::Path) -> io::Result<usize> {
    fs::read_dir(path).map(|s| s.count())
}
