use std::path;

use super::rect::Rect;

/// The file currently shown in the preview pane and the screen area it occupies, used to detect
/// changes for the external preview hook scripts.
#[derive(Debug, Clone)]
pub struct PreviewArea {
    pub file_preview_path: path::PathBuf,
    pub preview_area: Rect,
}

impl PreviewArea {
    /// Builds a `PreviewArea` from the previewed file's path and its on-screen rectangle.
    pub fn new(file_preview_path: path::PathBuf, preview_area: Rect) -> Self {
        Self {
            file_preview_path,
            preview_area,
        }
    }
}
