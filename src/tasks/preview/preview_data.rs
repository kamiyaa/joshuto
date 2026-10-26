use std::fmt::Debug;

use ratatui_image::protocol::Protocol;

use crate::tasks::preview::preview_file::FilePreview;

/// A successfully-generated file preview: either script/text output or a rendered image.
pub enum PreviewData {
    Script(Box<FilePreview>),
    Image(Box<Protocol>),
}

impl Debug for PreviewData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Script(_) => f.debug_tuple("Script").field(&"_").finish(),
            Self::Image(_) => f.debug_tuple("Image").field(&"_").finish(),
        }
    }
}
