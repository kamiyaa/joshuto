use allmytoes::ThumbSize;
use ratatui_image::picker::ProtocolType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PreviewProtocol {
    #[default]
    Auto,
    Disabled,
    #[serde(untagged)]
    ProtocolType(ProtocolType),
}

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
    pub fn to_amt_size(&self) -> ThumbSize {
        match &self {
            XDGThumbSizes::Normal => ThumbSize::Normal,
            XDGThumbSizes::Large => ThumbSize::Large,
            XDGThumbSizes::XLarge => ThumbSize::XLarge,
            XDGThumbSizes::XXLarge => ThumbSize::XXLarge,
        }
    }
}
