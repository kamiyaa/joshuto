use allmytoes::ThumbSize;
use ratatui_image::picker::ProtocolType;
use serde::{Deserialize, Serialize};

/// How image previews are rendered in the terminal: auto-detected, disabled, or a specific
/// graphics protocol.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PreviewProtocol {
    #[default]
    Auto,
    Disabled,
    #[serde(untagged)]
    ProtocolType(ProtocolType),
}

/// Requested size for XDG thumbnail previews.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum XDGThumbSizes {
    Normal,
    Large,
    #[default]
    XLarge,
    XXLarge,
}

impl XDGThumbSizes {
    /// Converts to the `allmytoes` crate's equivalent thumbnail size.
    pub fn to_amt_size(&self) -> ThumbSize {
        match &self {
            XDGThumbSizes::Normal => ThumbSize::Normal,
            XDGThumbSizes::Large => ThumbSize::Large,
            XDGThumbSizes::XLarge => ThumbSize::XLarge,
            XDGThumbSizes::XXLarge => ThumbSize::XXLarge,
        }
    }
}
