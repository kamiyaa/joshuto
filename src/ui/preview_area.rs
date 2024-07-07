use std::path;

use super::rect::Rect;

#[derive(Debug, Clone)]
pub struct PreviewArea {
    pub file_preview_path: path::PathBuf,
    pub preview_area: Rect,
}

impl PreviewArea {
    pub fn new(file_preview_path: path::PathBuf, preview_area: Rect) -> Self {
        Self {
            file_preview_path,
            preview_area,
        }
    }
}
