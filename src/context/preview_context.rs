use std::collections::HashMap;
use std::path;

use crate::preview::preview_file::FilePreview;

pub struct PreviewContext {
    pub previews: HashMap<path::PathBuf, FilePreview>,
}

impl PreviewContext {
    pub fn new() -> Self {
        Self {
            previews: HashMap::new(),
        }
    }

    pub fn get_preview(&self, p: &path::Path) -> Option<&FilePreview> {
        self.previews.get(p)
    }

    pub fn insert_preview(&mut self, p: path::PathBuf, file_preview: FilePreview) {
        self.previews.insert(p, file_preview);
    }
}
