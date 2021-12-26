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

pub struct RenderResult {
    pub file_preview_path: Option<path::PathBuf>,
    pub preview_area: Option<Rect>,
}

impl RenderResult {
    pub fn new() -> RenderResult {
        RenderResult {
            file_preview_path: None,
            preview_area: None,
        }
    }
}
