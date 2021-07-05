use std::io;
use std::path;

use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoResult};
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
    let new_cwd = if path.is_absolute() {
        let new_cwd = path.canonicalize()?;
        if !new_cwd.exists() {
            let err = io::Error::new(
                io::ErrorKind::NotFound,
                "No such file or directory".to_string(),
            );
            let err = JoshutoError::from(err);
            return Err(err);
        }
        new_cwd
    } else {
        let mut new_cwd = std::env::current_dir()?;
        new_cwd.push(path.canonicalize()?);
        if !new_cwd.exists() {
            let err = io::Error::new(
                io::ErrorKind::NotFound,
                "No such file or directory".to_string(),
            );
            let err = JoshutoError::from(err);
            return Err(err);
        }
        new_cwd
    };

    _change_directory(new_cwd.as_path(), context)?;
    Ok(())
}
