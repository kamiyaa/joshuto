use std::convert::From;
use std::path;

use serde_derive::Deserialize;

use crate::config::option::PreviewOption;
use crate::config::search_directories;
use crate::CONFIG_HIERARCHY;

pub const fn default_max_preview_size() -> u64 {
    2 * 1024 * 1024 // 2 MB
}

#[derive(Clone, Debug, Deserialize)]
pub struct PreviewOptionCrude {
    #[serde(default = "default_max_preview_size")]
    pub max_preview_size: u64,
    #[serde(default)]
    pub preview_script: Option<String>,
    #[serde(default)]
    pub preview_shown_hook_script: Option<String>,
    #[serde(default)]
    pub preview_removed_hook_script: Option<String>,
}

impl std::default::Default for PreviewOptionCrude {
    fn default() -> Self {
        Self {
            max_preview_size: default_max_preview_size(),
            preview_script: None,
            preview_shown_hook_script: None,
            preview_removed_hook_script: None,
        }
    }
}

impl From<PreviewOptionCrude> for PreviewOption {
    fn from(crude: PreviewOptionCrude) -> Self {
        let preview_script = match crude.preview_script {
            Some(s) => {
                let tilde_cow = shellexpand::tilde_with_context(s.as_str(), dirs_next::home_dir);
                let tilde_path = path::PathBuf::from(tilde_cow.as_ref());
                Some(tilde_path)
            }
            None => search_directories("preview.sh", &CONFIG_HIERARCHY),
        };
        let preview_shown_hook_script = match crude.preview_shown_hook_script {
            Some(s) => {
                let tilde_cow = shellexpand::tilde_with_context(s.as_str(), dirs_next::home_dir);
                let tilde_path = path::PathBuf::from(tilde_cow.as_ref());
                Some(tilde_path)
            }
            None => None,
        };
        let preview_removed_hook_script = match crude.preview_removed_hook_script {
            Some(s) => {
                let tilde_cow = shellexpand::tilde_with_context(s.as_str(), dirs_next::home_dir);
                let tilde_path = path::PathBuf::from(tilde_cow.as_ref());
                Some(tilde_path)
            }
            None => None,
        };

        Self {
            max_preview_size: crude.max_preview_size,
            preview_script,
            preview_shown_hook_script,
            preview_removed_hook_script,
        }
    }
}
