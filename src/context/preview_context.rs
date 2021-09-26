use std::collections::HashMap;
use std::path;

use crate::preview::preview_file::FilePreview;

pub struct PreviewContext {
    previews: HashMap<path::PathBuf, Option<FilePreview>>,
}

impl PreviewContext {
    pub fn new() -> Self {
        Self {
            previews: HashMap::new(),
        }
    }

    pub fn preview_exists(&self, p: &path::Path) -> bool {
        self.previews.get(p).is_some()
    }

    pub fn get_preview(&self, p: &path::Path) -> Option<&Option<FilePreview>> {
        self.previews.get(p)
    }

    pub fn insert_preview(&mut self, p: path::PathBuf, preview: Option<FilePreview>) {
        self.previews.insert(p, preview);
    }
}
