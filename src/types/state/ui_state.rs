use ratatui::layout::Rect;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiState {
    pub layout: Vec<Rect>,
}
