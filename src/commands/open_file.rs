use std::io;
use std::path;

use crate::commands::{quit, reload};
use crate::config::ProgramEntry;
use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::ui::views::DummyListener;
use crate::ui::views::TuiTextField;
use crate::ui::AppBackend;
use crate::util::mimetype::get_mimetype;
use crate::util::process::{execute_and_wait, fork_execute};

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

fn _open_with_entry<S>(
    context: &mut AppContext,
    backend: &mut AppBackend,
    option: &ProgramEntry,
    files: &[S],
) -> std::io::Result<()>
where
    S: AsRef<std::ffi::OsStr>,
{
    if option.get_fork() {
        let (child_id, handle) = fork_execute(option, files, context.clone_event_tx())?;
        context.worker_context_mut().push_child(child_id, handle);
    } else {
        backend.terminal_drop();
        execute_and_wait(option, files)?;
        backend.terminal_restore()?;
    }
    Ok(())
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

fn _open_with_helper<S>(
    context: &mut AppContext,
    backend: &mut AppBackend,
    options: Vec<&ProgramEntry>,
    files: &[S],
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

        let mut listener = DummyListener {};
        TuiTextField::default()
            .prompt(":")
            .prefix(PROMPT)
            .menu_items(menu_options.iter().map(|s| s.as_str()))
            .get_input(backend, context, &mut listener)
    };
    match user_input.as_ref() {
        Some(user_input) if user_input.starts_with(PROMPT) => {
            let user_input = &user_input[PROMPT.len()..];

            match user_input.parse::<usize>() {
                Ok(n) if n >= options.len() => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "option does not exist".to_string(),
                    ))
                }
                Ok(n) => {
                    let option = &options[n];
                    _open_with_entry(context, backend, option, files)?;
                }
                Err(_) => {
                    let mut args_iter = user_input.split_whitespace();
                    if let Some(cmd) = args_iter.next() {
                        backend.terminal_drop();
                        let mut option = ProgramEntry::new(String::from(cmd));
                        option.args(args_iter);
                        let res = execute_and_wait(&option, files);
                        backend.terminal_restore()?;
                        res?
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn open(context: &mut AppContext, backend: &mut AppBackend) -> JoshutoResult {
    let curr_list = context.tab_context_ref().curr_tab_ref().curr_list_ref();
    let entry = curr_list.and_then(|s| s.curr_entry_ref().cloned());

    match entry {
        None => (),
        Some(entry) if entry.file_path().is_dir() => {
            let path = entry.file_path().to_path_buf();
            change_directory::cd(path.as_path(), context)?;
            reload::soft_reload_curr_tab(context)?;
        }
        Some(entry) => {
            if context.args.file_chooser {
                return quit::quit_with_action(context, quit::QuitAction::OutputSelectedFiles);
            }

            let paths = curr_list.map_or_else(Vec::new, |s| s.iter_selected().cloned().collect());
            let (path, files) = if paths.is_empty() {
                (entry.file_path(), vec![entry.file_name()])
            } else {
                (
                    paths.get(0).unwrap().file_path(),
                    paths.iter().map(|e| e.file_name()).collect(),
                )
            };
            let options = _get_options(path);
            let option = options.iter().find(|option| option.program_exists());

            let config = context.config_ref();

            if let Some(option) = option {
                _open_with_entry(context, backend, option, &files)?;
            } else if config.xdg_open {
                _open_with_xdg(context, backend, path)?;
            } else {
                _open_with_helper(context, backend, options, &files)?;
            }
        }
    }
    Ok(())
}

pub fn open_with_index(
    context: &mut AppContext,
    backend: &mut AppBackend,
    index: usize,
) -> JoshutoResult {
    let paths = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map_or(vec![], |s| s.iter_selected().cloned().collect());

    if paths.is_empty() {
        return Err(JoshutoError::new(
            JoshutoErrorKind::Io(io::ErrorKind::NotFound),
            String::from("No files selected"),
        ));
    }
    let files: Vec<&str> = paths.iter().map(|e| e.file_name()).collect();
    let options = _get_options(paths[0].file_path());

    if index >= options.len() {
        return Err(JoshutoError::new(
            JoshutoErrorKind::Io(std::io::ErrorKind::InvalidData),
            "option does not exist".to_string(),
        ));
    }

    let option = &options[index];
    _open_with_entry(context, backend, option, &files)?;
    Ok(())
}

pub fn open_with_interactive(context: &mut AppContext, backend: &mut AppBackend) -> JoshutoResult {
    let mut paths = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map_or(vec![], |s| s.iter_selected().cloned().collect());

    if paths.is_empty() {
        paths.push(
            context
                .tab_context_ref()
                .curr_tab_ref()
                .curr_list_ref()
                .and_then(|s| s.curr_entry_ref())
                .unwrap()
                .clone(),
        );
    }
    let files: Vec<&str> = paths.iter().map(|e| e.file_name()).collect();
    let options = _get_options(paths[0].file_path());

    _open_with_helper(context, backend, options, &files)?;
    Ok(())
}
