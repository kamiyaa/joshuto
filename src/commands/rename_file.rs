use std::path;

use crate::config::clean::keymap::AppKeyMapping;
use crate::context::AppContext;
use crate::error::AppResult;
use crate::history::create_dirlist_with_history;
use crate::ui::AppBackend;

use super::command_line;

pub fn _rename_file(
    context: &mut AppContext,
    src: &path::Path,
    dest: &path::Path,
) -> std::io::Result<()> {
    let new_path = dest;
    if new_path.exists() {
        let err = std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Filename already exists");
        return Err(err);
    }
    std::fs::rename(src, dest)?;

    let curr_tab = context.tab_context_ref().curr_tab_ref();

    let path = curr_tab
        .curr_list_ref()
        .map(|lst| lst.file_path().to_path_buf());

    if let Some(path) = path {
        let options = context.config_ref().display_options_ref().clone();
        let tab_options = context
            .tab_context_ref()
            .curr_tab_ref()
            .option_ref()
            .clone();
        let history = context.tab_context_mut().curr_tab_mut().history_mut();
        let new_dirlist =
            create_dirlist_with_history(history, path.as_path(), &options, &tab_options)?;
        history.insert(path, new_dirlist);
    }
    Ok(())
}

pub fn rename_file(context: &mut AppContext, dest: &path::Path) -> AppResult {
    let path: Option<path::PathBuf> = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|s| s.curr_entry_ref())
        .map(|s| s.file_path().to_path_buf());

    if let Some(path) = path {
        _rename_file(context, path.as_path(), dest)?;
    }
    Ok(())
}

fn _get_current_file_name(context: &mut AppContext) -> Option<String> {
    context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|list| list.curr_entry_ref().map(|s| s.file_name().to_string()))
}

pub fn rename_file_append(
    context: &mut AppContext,
    backend: &mut AppBackend,
    keymap_t: &AppKeyMapping,
) -> AppResult {
    if let Some(file_name) = _get_current_file_name(context) {
        let (prefix, suffix) = (format!("rename {}", file_name), "".to_string());
        command_line::read_and_execute(context, backend, keymap_t, &prefix, &suffix)?;
    }
    Ok(())
}

pub fn rename_file_append_base(
    context: &mut AppContext,
    backend: &mut AppBackend,
    keymap_t: &AppKeyMapping,
) -> AppResult {
    if let Some(file_name) = _get_current_file_name(context) {
        let (prefix, suffix): (String, String) = match file_name.rfind('.') {
            Some(ext) => (
                format!("rename {}", &file_name[0..ext]),
                file_name[ext..].to_string(),
            ),
            None => (format!("rename {}", file_name), "".to_string()),
        };
        command_line::read_and_execute(context, backend, keymap_t, &prefix, &suffix)?;
    }
    Ok(())
}

pub fn rename_file_prepend(
    context: &mut AppContext,
    backend: &mut AppBackend,
    keymap_t: &AppKeyMapping,
) -> AppResult {
    if let Some(file_name) = _get_current_file_name(context) {
        let (prefix, suffix) = ("rename ".to_string(), file_name);
        command_line::read_and_execute(context, backend, keymap_t, &prefix, &suffix)?;
    }
    Ok(())
}

pub fn rename_file_keep_ext(
    context: &mut AppContext,
    backend: &mut AppBackend,
    keymap_t: &AppKeyMapping,
) -> AppResult {
    if let Some(file_name) = _get_current_file_name(context) {
        let (prefix, suffix): (String, String) = match file_name.rfind('.') {
            Some(ext) => ("rename ".to_string(), file_name[ext..].to_string()),
            None => ("rename ".to_string(), "".to_string()),
        };
        command_line::read_and_execute(context, backend, keymap_t, &prefix, &suffix)?;
    }
    Ok(())
}
