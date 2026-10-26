use ratatui::layout::Rect;

/// The current pane layout rectangles, computed each frame and used as input to both rendering
/// and viewport calculations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiState {
    pub layout: Vec<Rect>,
}
