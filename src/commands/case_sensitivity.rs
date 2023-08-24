use crate::config::option::CaseSensitivity;
use crate::context::AppContext;
use crate::error::JoshutoResult;

#[derive(Clone, Copy, Debug)]
pub enum SetType {
    String,
    Glob,
    Regex,
    Fzf,
}

pub fn set_case_sensitivity(
    context: &mut AppContext,
    case_sensitivity: CaseSensitivity,
    set_type: SetType,
) -> JoshutoResult {
    let options = context.config_mut().search_options_mut();

    match set_type {
        SetType::String => options.string_case_sensitivity = case_sensitivity,
        SetType::Glob => options.glob_case_sensitivity = case_sensitivity,
        SetType::Regex => options.regex_case_sensitivity = case_sensitivity,
        SetType::Fzf => options.fzf_case_sensitivity = case_sensitivity,
    }

    Ok(())
}
