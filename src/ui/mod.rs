use std::path;

mod tui_backend;
pub mod views;
pub mod widgets;

pub use tui_backend::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone)]
pub struct RenderResult {
    pub file_preview_path: path::PathBuf,
    pub preview_area: Rect,
}

impl RenderResult {
    pub fn new(file_preview_path: path::PathBuf, preview_area: Rect) -> RenderResult {
        RenderResult {
            file_preview_path,
            preview_area,
        }
    }
}
