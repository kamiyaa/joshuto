use std::path;

use crate::commands::{reload, zoxide};
use crate::context::AppContext;
use crate::error::AppResult;
use crate::history::{generate_entries_to_root, DirectoryHistory};
use crate::util::cwd;

// ChangeDirectory command
pub fn cd(path: &path::Path, context: &mut AppContext) -> std::io::Result<()> {
    cwd::set_current_dir(path)?;
    context.tab_context_mut().curr_tab_mut().set_cwd(path);
    if context.config_ref().zoxide_update {
        debug_assert!(path.is_absolute());
        zoxide::zoxide_add(path.to_str().expect("cannot convert path to string"))?;
    }
    Ok(())
}

pub fn change_directory(context: &mut AppContext, mut path: &path::Path) -> AppResult {
    let new_cwd = if path.is_absolute() {
        path.to_path_buf()
    } else {
        while let Ok(p) = path.strip_prefix("../") {
            parent_directory(context)?;
            path = p;
        }

        let mut new_cwd = std::env::current_dir()?;
        new_cwd.push(path);
        new_cwd
    };

    cd(new_cwd.as_path(), context)?;
    let dirlists = generate_entries_to_root(
        new_cwd.as_path(),
        context.tab_context_ref().curr_tab_ref().history_ref(),
        context.ui_context_ref(),
        context.config_ref().display_options_ref(),
        context.tab_context_ref().curr_tab_ref().option_ref(),
    )?;
    context
        .tab_context_mut()
        .curr_tab_mut()
        .history_mut()
        .insert_entries(dirlists);
    Ok(())
}

// ParentDirectory command
pub fn parent_directory(context: &mut AppContext) -> AppResult {
    if let Some(parent) = context
        .tab_context_ref()
        .curr_tab_ref()
        .cwd()
        .parent()
        .map(|p| p.to_path_buf())
    {
        cwd::set_current_dir(&parent)?;
        context
            .tab_context_mut()
            .curr_tab_mut()
            .set_cwd(parent.as_path());
        reload::soft_reload_curr_tab(context)?;
    }
    Ok(())
}

// PreviousDirectory command
pub fn previous_directory(context: &mut AppContext) -> AppResult {
    if let Some(path) = context.tab_context_ref().curr_tab_ref().previous_dir() {
        let path = path.to_path_buf();
        cwd::set_current_dir(&path)?;
        context
            .tab_context_mut()
            .curr_tab_mut()
            .set_cwd(path.as_path());
        reload::soft_reload_curr_tab(context)?;
    }
    Ok(())
}
