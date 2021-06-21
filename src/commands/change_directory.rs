use std::path;

use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;

pub fn cd(path: &path::Path, context: &mut AppContext) -> std::io::Result<()> {
    std::env::set_current_dir(path)?;
    context.tab_context_mut().curr_tab_mut().set_cwd(path);
    Ok(())
}

fn _change_directory(path: &path::Path, context: &mut AppContext) -> std::io::Result<()> {
    cd(path, context)?;
    let options = context.config_ref().display_options_ref().clone();
    context
        .tab_context_mut()
        .curr_tab_mut()
        .history_mut()
        .populate_to_root(&path, &options)?;

    Ok(())
}

pub fn change_directory(context: &mut AppContext, path: &path::Path) -> JoshutoResult<()> {
    _change_directory(path, context)?;
    Ok(())
}
