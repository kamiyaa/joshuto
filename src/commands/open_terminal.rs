use std::io;
use std::path;

use crate::commands::{quit, reload};
use crate::config::ProgramEntry;
use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::ui::views::TuiTextField;
use crate::ui::AppBackend;
use crate::util::mimetype::get_mimetype;
use crate::util::process::{execute_and_wait, fork_execute, fork_execute_empty};

use super::change_directory;

use crate::MIMETYPE_T;

fn _get_options<'a>(path: &path::Path) -> Vec<&'a ProgramEntry> {
    let mut options: Vec<&ProgramEntry> = Vec::new();

    if let Some(file_ext) = path.extension().and_then(|ext| ext.to_str()) {
        if let Some(entries) = MIMETYPE_T.app_list_for_ext(file_ext) {
            options.extend(entries);
            return options;
        }
    }
    if let Ok(file_mimetype) = get_mimetype(path) {
        if let Some(entry) = MIMETYPE_T.app_list_for_mimetype(file_mimetype.get_type()) {
            match entry.subtypes().get(file_mimetype.get_subtype()) {
                Some(entries) => {
                    options.extend(entries);
                    return options;
                }
                None => {
                    let entries = entry.app_list();
                    options.extend(entries);
                    return options;
                }
            }
        }
    }
    options
}

fn _open_with_xdg(
    context: &mut AppContext,
    backend: &mut AppBackend,
    path: &path::Path,
) -> std::io::Result<()> {
    let config = context.config_ref();

    if config.xdg_open_fork {
        open::that_in_background(path);
    } else {
        backend.terminal_drop();
        let result = open::that(path);
        backend.terminal_restore()?;
        result?;
    }
    Ok(())
}

pub fn open_terminal(context: &mut AppContext, backend: &mut AppBackend) -> JoshutoResult {
    let curr_list = context.tab_context_ref().curr_tab_ref().curr_list_ref();
    let entry = curr_list.and_then(|s| s.curr_entry_ref().cloned());
    let (child_id, handle) = fork_execute_empty().unwrap();
    Ok(())
}
