use std::io;
use std::path;

use crate::commands::reload;
use crate::config::AppMimetypeEntry;
use crate::context::{AppContext, QuitType};
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::ui::views::TuiTextField;
use crate::ui::TuiBackend;

use super::change_directory;

use crate::MIMETYPE_T;

pub fn get_options<'a>(path: &path::Path) -> Vec<&'a AppMimetypeEntry> {
    let mut options: Vec<&AppMimetypeEntry> = Vec::new();
    if let Some(file_ext) = path.extension() {
        if let Some(file_ext) = file_ext.to_str() {
            let ext_entries = MIMETYPE_T.app_list_for_ext(file_ext);
            options.extend(ext_entries);
        }
    }
    options
}

pub fn open(context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult {
    let config = context.config_ref();

    let curr_list = context.tab_context_ref().curr_tab_ref().curr_list_ref();
    let entry = curr_list.and_then(|s| s.curr_entry_ref());

    match entry {
        None => (),
        Some(entry) if entry.file_path().is_dir() => {
            let path = entry.file_path().to_path_buf();
            change_directory::cd(path.as_path(), context)?;
            reload::soft_reload(context.tab_context_ref().index, context)?;
        }
        Some(_) if context.args.choosefiles.is_some() => {
            context.quit = QuitType::ChooseFiles;
        }
        Some(_) => {
            let paths = curr_list.map_or_else(Vec::new, |s| s.get_selected_paths());
            let path = paths.get(0).ok_or_else(|| {
                JoshutoError::new(
                    JoshutoErrorKind::Io(io::ErrorKind::NotFound),
                    String::from("No files selected"),
                )
            })?;

            let files: Vec<&std::ffi::OsStr> = paths.iter().filter_map(|e| e.file_name()).collect();
            let option = get_options(path)
                .into_iter()
                .find(|option| option.program_exists());

            if let Some(option) = option {
                if option.get_fork() {
                    option.execute_with(files.as_slice())?;
                } else {
                    backend.terminal_drop();
                    let result = option.execute_with(files.as_slice());
                    backend.terminal_restore()?;
                    result?;
                }
            } else if config.xdg_open {
                if config.xdg_open_fork {
                    open::that_in_background(path);
                } else {
                    backend.terminal_drop();
                    let result = open::that(path);
                    backend.terminal_restore()?;
                    result?;
                }
            } else {
                open_with_helper(context, backend, get_options(path), files)?;
            }
        }
    }
    Ok(())
}

pub fn open_with_helper<S>(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    options: Vec<&AppMimetypeEntry>,
    files: Vec<S>,
) -> std::io::Result<()>
where
    S: AsRef<std::ffi::OsStr>,
{
    const PROMPT: &str = "open_with ";

    let user_input: Option<String> = {
        context.flush_event();

        let menu_options: Vec<String> = options
            .iter()
            .enumerate()
            .map(|(i, e)| format!("  {} | {}", i, e))
            .collect();

        TuiTextField::default()
            .prompt(":")
            .prefix(PROMPT)
            .menu_items(menu_options.iter().map(|s| s.as_str()))
            .get_input(backend, context)
    };
    match user_input.as_ref() {
        Some(user_input) if user_input.starts_with(PROMPT) => {
            let user_input = &user_input[PROMPT.len()..];

            match user_input.parse::<usize>() {
                Ok(n) if n >= options.len() => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "option does not exist".to_string(),
                )),
                Ok(n) => {
                    let mimetype_entry = &options[n];
                    if mimetype_entry.get_fork() {
                        mimetype_entry.execute_with(files.as_slice())
                    } else {
                        backend.terminal_drop();
                        let res = mimetype_entry.execute_with(files.as_slice());
                        backend.terminal_restore()?;
                        res
                    }
                }
                Err(_) => {
                    let mut args_iter = user_input.split_whitespace();
                    match args_iter.next() {
                        Some(cmd) => {
                            backend.terminal_drop();
                            let res = AppMimetypeEntry::new(String::from(cmd))
                                .args(args_iter)
                                .execute_with(files.as_slice());
                            backend.terminal_restore()?;
                            res
                        }
                        None => Ok(()),
                    }
                }
            }
        }
        _ => Ok(()),
    }
}

pub fn open_with_interactive(context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult {
    let paths = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map_or(vec![], |s| s.get_selected_paths());

    if paths.is_empty() {
        return Err(JoshutoError::new(
            JoshutoErrorKind::Io(io::ErrorKind::NotFound),
            String::from("No files selected"),
        ));
    }
    let files: Vec<&std::ffi::OsStr> = paths.iter().filter_map(|e| e.file_name()).collect();
    let options = get_options(paths[0].as_path());

    open_with_helper(context, backend, options, files)?;
    Ok(())
}

pub fn open_with_index(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    index: usize,
) -> JoshutoResult {
    let paths = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map_or(vec![], |s| s.get_selected_paths());

    if paths.is_empty() {
        return Err(JoshutoError::new(
            JoshutoErrorKind::Io(io::ErrorKind::NotFound),
            String::from("No files selected"),
        ));
    }
    let files: Vec<&std::ffi::OsStr> = paths.iter().filter_map(|e| e.file_name()).collect();
    let options = get_options(paths[0].as_path());

    if index >= options.len() {
        return Err(JoshutoError::new(
            JoshutoErrorKind::Io(std::io::ErrorKind::InvalidData),
            "option does not exist".to_string(),
        ));
    }

    let mimetype_entry = &options[index];
    if mimetype_entry.get_fork() {
        mimetype_entry.execute_with(files.as_slice())?;
        Ok(())
    } else {
        backend.terminal_drop();
        let res = mimetype_entry.execute_with(files.as_slice());
        backend.terminal_restore()?;
        res?;
        Ok(())
    }
}
