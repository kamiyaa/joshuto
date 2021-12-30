use std::path;

use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;

pub fn cd(path: &path::Path, context: &mut AppContext) -> std::io::Result<()> {
    std::env::set_current_dir(path)?;
    context.tab_context_mut().curr_tab_mut().set_cwd(path);
    Ok(())
}

fn change_directory_helper(path: &path::Path, context: &mut AppContext) -> std::io::Result<()> {
    cd(path, context)?;
    let options = context.config_ref().display_options_ref().clone();
    let ui_context = context.ui_context_ref().clone();
    context
        .tab_context_mut()
        .curr_tab_mut()
        .history_mut()
        .populate_to_root(path, &ui_context, &options)?;
    Ok(())
}

pub fn change_directory(context: &mut AppContext, path: &path::Path) -> JoshutoResult<()> {
    let new_cwd = if path.is_absolute() {
        path.canonicalize()?
    } else {
        let mut new_cwd = std::env::current_dir()?;
        new_cwd.push(path.canonicalize()?);
        new_cwd
    };

    change_directory_helper(new_cwd.as_path(), context)?;
    Ok(())
}
