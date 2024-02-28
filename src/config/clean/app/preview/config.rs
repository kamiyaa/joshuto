use std::path;

use crate::{
    config::{
        raw::app::display::preview::{default_max_preview_size, PreviewOptionRaw, PreviewProtocol},
        search_directories,
    },
    util::unix,
    CONFIG_HIERARCHY,
};

#[derive(Clone, Debug)]
pub struct PreviewOption {
    pub max_preview_size: u64,
    pub preview_protocol: PreviewProtocol,
    pub preview_script: Option<path::PathBuf>,
    pub preview_shown_hook_script: Option<path::PathBuf>,
    pub preview_removed_hook_script: Option<path::PathBuf>,
}

impl std::default::Default for PreviewOption {
    fn default() -> Self {
        Self {
            max_preview_size: default_max_preview_size(),
            preview_protocol: PreviewProtocol::Auto,
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
            preview_protocol: raw.preview_protocol,
            preview_script,
            preview_shown_hook_script,
            preview_removed_hook_script,
        }
    }
}
