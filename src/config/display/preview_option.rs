use std::path;

use allmytoes::ThumbSize;

use crate::{
    config::{
        utils::{default_max_preview_size, search_directories},
        PreviewOptionRaw,
    },
    global,
    utils::unix_helper,
};

use super::PreviewProtocol;

/// Resolved settings controlling how file previews are generated and displayed.
#[derive(Clone, Debug)]
pub struct PreviewOption {
    pub max_preview_size: u64,
    pub preview_protocol: PreviewProtocol,
    pub preview_script: Option<path::PathBuf>,
    pub use_xdg_thumbs: bool,
    pub xdg_thumb_size: ThumbSize,
    pub preview_shown_hook_script: Option<path::PathBuf>,
    pub preview_removed_hook_script: Option<path::PathBuf>,
}

impl std::default::Default for PreviewOption {
    fn default() -> Self {
        Self {
            max_preview_size: default_max_preview_size(),
            preview_protocol: PreviewProtocol::Auto,
            use_xdg_thumbs: true,
            xdg_thumb_size: ThumbSize::XLarge,
            preview_script: None,
            preview_shown_hook_script: None,
            preview_removed_hook_script: None,
        }
    }
}

impl From<PreviewOptionRaw> for PreviewOption {
    fn from(value: PreviewOptionRaw) -> Self {
        let preview_script = value
            .preview_script
            .map(|s| unix_helper::expand_shell_string(&s))
            .or_else(|| search_directories(&global::CONFIG_HIERARCHY, "preview.sh"));

        let preview_shown_hook_script = value
            .preview_shown_hook_script
            .map(|s| unix_helper::expand_shell_string(&s));

        let preview_removed_hook_script = value
            .preview_removed_hook_script
            .map(|s| unix_helper::expand_shell_string(&s));

        Self {
            max_preview_size: value.max_preview_size,
            preview_protocol: value.preview_protocol,
            preview_script,
            use_xdg_thumbs: value.use_xdg_thumbs,
            xdg_thumb_size: value.xdg_thumb_size.to_amt_size(),
            preview_shown_hook_script,
            preview_removed_hook_script,
        }
    }
}
