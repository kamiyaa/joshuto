use std::path;

use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use crate::util::load_child::LoadChild;

pub fn cd(path: &path::Path, context: &mut JoshutoContext) -> std::io::Result<()> {
    std::env::set_current_dir(path)?;
    context.tab_context_mut().curr_tab_mut().set_pwd(path);
    Ok(())
}

fn _change_directory(path: &path::Path, context: &mut JoshutoContext) -> std::io::Result<()> {
    cd(path, context)?;
    let options = context.display_options_ref().clone();
    context
        .tab_context_mut()
        .curr_tab_mut()
        .history_mut()
        .populate_to_root(&path, &options)?;

    Ok(())
}

pub fn change_directory(context: &mut JoshutoContext, path: &path::Path) -> JoshutoResult<()> {
    _change_directory(path, context)?;
    LoadChild::load_child(context)?;
    Ok(())
}
