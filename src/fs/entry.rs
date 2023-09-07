use std::{fs, io, path};

use crate::{
    config::clean::app::display::DisplayOption,
    fs::{FileType, JoshutoMetadata},
};

#[cfg(feature = "devicons")]
use crate::ICONS_T;

#[derive(Clone, Debug)]
pub struct JoshutoDirEntry {
    name: String,
    label: String,
    path: path::PathBuf,
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
            permanent_selected: false,
            visual_mode_selected: false,
            _marked: false,
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

    pub fn get_ext(&self) -> &str {
        let fname = self.file_name();
        match fname.rfind('.') {
            Some(pos) => &fname[pos..],
            None => "",
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

#[cfg(feature = "devicons")]
fn create_icon_label(name: &str, metadata: &JoshutoMetadata) -> String {
    let label = {
        let icon = match metadata.file_type() {
            FileType::Directory => ICONS_T
                .directory_exact
                .get(name)
                .cloned()
                .unwrap_or(ICONS_T.default_dir.clone()),
            _ => ICONS_T
                .file_exact
                .get(name)
                .cloned()
                .unwrap_or(match name.rsplit_once('.') {
                    Some((_, ext)) => ICONS_T
                        .ext
                        .get(ext)
                        .unwrap_or(&ICONS_T.default_file)
                        .to_string(),
                    None => ICONS_T.default_file.clone(),
                }),
        };
        format!("{} {}", icon, name)
    };
    label
}

fn get_directory_size(path: &path::Path) -> io::Result<usize> {
    fs::read_dir(path).map(|s| s.count())
}
