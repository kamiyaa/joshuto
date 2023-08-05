use crate::config::option::CaseSensitivity;
use crate::context::AppContext;
use crate::error::JoshutoResult;

pub fn set_case_sensitivity(
    context: &mut AppContext,
    case_sensitivity: CaseSensitivity,
) -> JoshutoResult {
    let options = context.config_mut().search_options_mut();
    options.case_sensitivity = case_sensitivity;
    Ok(())
}
