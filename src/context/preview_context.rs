use std::collections::HashMap;
use std::path;

use crate::preview::preview_file::PreviewFileState;

type FilePreviewMetadata = HashMap<path::PathBuf, PreviewFileState>;

pub struct PreviewContext {
    previews: FilePreviewMetadata,
}

impl PreviewContext {
    pub fn new() -> Self {
        Self {
            previews: HashMap::new(),
        }
    }

    pub fn previews_ref(&self) -> &FilePreviewMetadata {
        &self.previews
    }
    pub fn previews_mut(&mut self) -> &mut FilePreviewMetadata {
        &mut self.previews
    }
}
