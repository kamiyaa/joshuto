use std::{fs, io, path};

use crate::{
    config::clean::app::{display::DisplayOption, AppConfig},
    fs::{FileType, JoshutoMetadata},
};

#[cfg(feature = "devicons")]
use crate::ICONS_T;

#[derive(Clone, Debug)]
pub struct JoshutoDirEntry {
    pub name: String,
    pub ext: Option<String>,
    pub path: path::PathBuf,
    pub metadata: JoshutoMetadata,
    /// Directly selected by the user, _not_ by a current visual mode selection
    permanent_selected: bool,
    /// Temporarily selected by the visual mode range
    visual_mode_selected: bool,
    _marked: bool,
}

impl JoshutoDirEntry {
    pub fn from(
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

        let ext = direntry
            .path()
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());

        let mut metadata = JoshutoMetadata::from(&path)?;

        if options.automatically_count_files() && metadata.file_type().is_dir() {
            if let Ok(size) = get_directory_size(path.as_path()) {
                metadata.update_directory_size(size);
            }
        }

        Ok(Self {
            name,
            ext,
            path,
            metadata,
            permanent_selected: false,
            visual_mode_selected: false,
            _marked: false,
        })
    }

    pub fn file_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn ext(&self) -> Option<&str> {
        self.ext.as_deref()
    }

    pub fn file_path(&self) -> &path::Path {
        self.path.as_path()
    }

    pub fn file_path_buf(&self) -> path::PathBuf {
        self.path.clone()
    }

    pub fn is_selected(&self) -> bool {
        self.permanent_selected || self.visual_mode_selected
    }

    pub fn is_permanent_selected(&self) -> bool {
        self.permanent_selected
    }

    pub fn is_visual_mode_selected(&self) -> bool {
        self.visual_mode_selected
    }

    pub fn set_permanent_selected(&mut self, selected: bool) {
        self.permanent_selected = selected;
    }

    pub fn set_visual_mode_selected(&mut self, visual_mode_selected: bool) {
        self.visual_mode_selected = visual_mode_selected;
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

fn get_directory_size(path: &path::Path) -> io::Result<usize> {
    fs::read_dir(path).map(|s| s.count())
}
