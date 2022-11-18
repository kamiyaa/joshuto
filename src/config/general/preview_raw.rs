use std::convert::From;

use serde_derive::Deserialize;

use crate::config::option::PreviewOption;
use crate::config::search_directories;
use crate::util::unix;
use crate::CONFIG_HIERARCHY;

pub const fn default_max_preview_size() -> u64 {
    2 * 1024 * 1024 // 2 MB
}

#[derive(Clone, Debug, Deserialize)]
pub struct PreviewOptionRaw {
    #[serde(default = "default_max_preview_size")]
    pub max_preview_size: u64,
    #[serde(default)]
    pub preview_script: Option<String>,
    #[serde(default)]
    pub preview_shown_hook_script: Option<String>,
    #[serde(default)]
    pub preview_removed_hook_script: Option<String>,
}

impl std::default::Default for PreviewOptionRaw {
    fn default() -> Self {
        Self {
            max_preview_size: default_max_preview_size(),
            preview_script: None,
            preview_shown_hook_script: None,
            preview_removed_hook_script: None,
        }
    }
}

impl From<PreviewOptionRaw> for PreviewOption {
    fn from(raw: PreviewOptionRaw) -> Self {
        let preview_script = raw
            .preview_script
            .map(|s| unix::expand_shell_string(&s))
            .or_else(|| search_directories("preview.sh", &CONFIG_HIERARCHY));

        let preview_shown_hook_script = raw
            .preview_shown_hook_script
            .map(|s| unix::expand_shell_string(&s));

        let preview_removed_hook_script = raw
            .preview_removed_hook_script
            .map(|s| unix::expand_shell_string(&s));

        Self {
            max_preview_size: raw.max_preview_size,
            preview_script,
            preview_shown_hook_script,
            preview_removed_hook_script,
        }
    }
}
