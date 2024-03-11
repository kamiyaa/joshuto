use std::path;

use crate::commands::cursor_move;
use crate::context::AppContext;
use crate::error::AppResult;

use super::tab_ops;

pub fn new_directory(context: &mut AppContext, p: &path::Path) -> AppResult {
    std::fs::create_dir_all(p)?;

    let curr_path = context.tab_context_ref().curr_tab_ref().cwd().to_path_buf();
    tab_ops::reload_all_tabs(context, curr_path.as_path())?;

    if context.config_ref().focus_on_create {
        cursor_move::to_path(context, p)?;
    }

    Ok(())
}
