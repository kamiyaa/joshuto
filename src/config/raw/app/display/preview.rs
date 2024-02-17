use ratatui_image::picker::ProtocolType;
use serde::Deserialize;

pub const fn default_max_preview_size() -> u64 {
    2 * 1024 * 1024 // 2 MB
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PreviewProtocol {
    #[default]
    Auto,
    Disabled,
    #[serde(untagged)]
    ProtocolType(ProtocolType),
}

#[derive(Clone, Debug, Deserialize)]
pub struct PreviewOptionRaw {
    #[serde(default = "default_max_preview_size")]
    pub max_preview_size: u64,
    #[serde(default)]
    pub preview_protocol: PreviewProtocol,
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
            preview_protocol: PreviewProtocol::Auto,
            preview_script: None,
            preview_shown_hook_script: None,
            preview_removed_hook_script: None,
        }
    }
}
