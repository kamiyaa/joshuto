use crate::error::AppResult;
use crate::types::option::search::CaseSensitivity;
use crate::types::state::AppState;

#[derive(Clone, Copy, Debug)]
pub enum SetType {
    String,
    Glob,
    Regex,
    Fzf,
}

pub fn set_case_sensitivity(
    app_state: &mut AppState,
    case_sensitivity: CaseSensitivity,
    set_type: SetType,
) -> AppResult {
    let options = &mut app_state.config.search_options;

    match set_type {
        SetType::String => options.string_case_sensitivity = case_sensitivity,
        SetType::Glob => options.glob_case_sensitivity = case_sensitivity,
        SetType::Regex => options.regex_case_sensitivity = case_sensitivity,
        SetType::Fzf => options.fzf_case_sensitivity = case_sensitivity,
    }

    Ok(())
}
