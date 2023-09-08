use std::path;

use crate::context::AppContext;
use crate::error::AppResult;
use crate::history::DirectoryHistory;

pub fn new_directory(context: &mut AppContext, p: &path::Path) -> AppResult {
    std::fs::create_dir_all(p)?;
    let options = context.config_ref().display_options_ref().clone();
    let curr_path = context.tab_context_ref().curr_tab_ref().cwd().to_path_buf();
    for (_, tab) in context.tab_context_mut().iter_mut() {
        let tab_options = tab.option_ref().clone();
        tab.history_mut()
            .reload(&curr_path, &options, &tab_options)?;
    }
    Ok(())
}
