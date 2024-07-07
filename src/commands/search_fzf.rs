use crate::commands::{cursor_move, fzf};
use crate::error::{AppError, AppErrorKind, AppResult};
use crate::types::state::AppState;
use crate::ui::AppBackend;

pub fn search_fzf(app_state: &mut AppState, backend: &mut AppBackend) -> AppResult {
    let items = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|list| {
            let v: Vec<String> = list
                .iter()
                .enumerate()
                .map(|(i, entry)| format!("{} {}\n", i, entry.file_name()))
                .collect();
            v
        })
        .unwrap_or_default();

    if items.is_empty() {
        return Err(AppError::new(
            AppErrorKind::Io,
            "no files to select".to_string(),
        ));
    }

    let fzf_output = fzf::fzf(app_state, backend, items)?;
    let selected_idx_str = fzf_output.split_once(' ');

    if let Some((selected_idx_str, _)) = selected_idx_str {
        if let Ok(index) = selected_idx_str.parse::<usize>() {
            cursor_move::cursor_move(app_state, index);
        }
    }

    Ok(())
}
