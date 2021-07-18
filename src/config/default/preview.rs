use std::path;

use serde_derive::Deserialize;

use crate::config::{search_directories, Flattenable};
use crate::CONFIG_HIERARCHY;

const fn default_max_preview_size() -> u64 {
    2 * 1024 * 1024 // 2 MB
}

#[derive(Clone, Debug, Deserialize)]
pub struct PreviewRawOption {
    #[serde(default = "default_max_preview_size")]
    max_preview_size: u64,
    #[serde(default)]
    preview_images: bool,
    #[serde(default)]
    preview_script: Option<String>,
}

impl std::default::Default for PreviewRawOption {
    fn default() -> Self {
        Self {
            max_preview_size: default_max_preview_size(),
            preview_images: false,
            preview_script: None,
        }
    }
}

impl Flattenable<PreviewOption> for PreviewRawOption {
    fn flatten(self) -> PreviewOption {
        let preview_script = match self.preview_script {
            Some(s) => {
                let tilde_cow = shellexpand::tilde_with_context(s.as_str(), dirs_next::home_dir);
                let tilde_path = path::PathBuf::from(tilde_cow.as_ref());
                Some(tilde_path)
            }
            None => search_directories("preview.sh", &CONFIG_HIERARCHY),
        };

        PreviewOption {
            max_preview_size: self.max_preview_size,
            preview_images: self.preview_images,
            preview_script,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PreviewOption {
    pub max_preview_size: u64,
    pub preview_images: bool,
    pub preview_script: Option<path::PathBuf>,
}

impl std::default::Default for PreviewOption {
    fn default() -> Self {
        Self {
            max_preview_size: default_max_preview_size(),
            preview_images: false,
            preview_script: None,
        }
    }
}
