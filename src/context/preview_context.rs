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

    pub fn get_preview_ref(&self, p: &path::Path) -> Option<&Option<FilePreview>> {
        self.previews.get(p)
    }

    pub fn get_preview_mut(&mut self, p: &path::Path) -> Option<&mut Option<FilePreview>> {
        self.previews.get_mut(p)
    }

    pub fn insert_preview(&mut self, p: path::PathBuf, preview: Option<FilePreview>) {
        self.previews.insert(p, preview);
    }
}
