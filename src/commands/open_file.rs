use std::path::{Path, PathBuf};

use crate::commands::{ChangeDirectory, JoshutoCommand, JoshutoRunnable};
use crate::config::mimetype::JoshutoMimetypeEntry;
use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::ui::widgets::{TuiMenu, TuiTextField};
use crate::ui::TuiBackend;
use crate::util::load_child::LoadChild;

use crate::MIMETYPE_T;

#[derive(Clone, Debug)]
pub struct OpenFile;

impl OpenFile {
    pub fn new() -> Self {
        OpenFile
    }
    pub const fn command() -> &'static str {
        "open_file"
    }

    pub fn get_options<'a>(path: &Path) -> Vec<&'a JoshutoMimetypeEntry> {
        let mut mimetype_options: Vec<&JoshutoMimetypeEntry> = Vec::new();

        /* extensions have priority */
        if let Some(file_ext) = path.extension() {
            if let Some(file_ext) = file_ext.to_str() {
                let ext_entries = MIMETYPE_T.get_entries_for_ext(file_ext);
                mimetype_options.extend(ext_entries);
            }
        }
        mimetype_options
    }

    fn open(context: &mut JoshutoContext, backend: &mut TuiBackend) -> std::io::Result<()> {
        let mut dirpath = None;
        let mut filepaths = None;

        if let Some(curr_list) = context.tabs[context.curr_tab_index].curr_list_ref() {
            if let Some(index) = curr_list.index {
                let child_path = curr_list.contents[index].file_path();
                if child_path.is_dir() {
                    dirpath = Some(child_path.clone());
                } else {
                    filepaths = Some(curr_list.get_selected_paths());
                }
            }
        }
        if let Some(path) = dirpath {
            ChangeDirectory::cd(path.as_path(), context)?;
            LoadChild::load_child(context)?;
        } else if let Some(paths) = filepaths {
            let options = Self::get_options(paths[0]);
            if options.len() > 0 {
                backend.terminal_drop();
                let res = options[0].execute_with(&paths);
                backend.terminal_restore()?;
                return res;
            } else {
                OpenFileWith::open_with(context, backend, &paths)?;
            }
        }
        Ok(())
    }
}

impl JoshutoCommand for OpenFile {}

impl std::fmt::Display for OpenFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for OpenFile {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        Self::open(context, backend)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct OpenFileWith;

impl OpenFileWith {
    pub fn new() -> Self {
        OpenFileWith
    }
    pub const fn command() -> &'static str {
        "open_file_with"
    }

    pub fn open_with(
        context: &JoshutoContext,
        backend: &mut TuiBackend,
        paths: &[&PathBuf],
    ) -> std::io::Result<()> {
        const PROMPT: &'static str = "open_with ";

        let mimetype_options: Vec<&JoshutoMimetypeEntry> = OpenFile::get_options(&paths[0]);

        let user_input: Option<String> = {
            let menu_options: Vec<String> = mimetype_options
                .iter()
                .enumerate()
                .map(|(i, e)| format!("  {} | {}", i, e))
                .collect();
            let menu_options_str: Vec<&str> = menu_options.iter().map(|e| e.as_str()).collect();
            let mut menu_widget = TuiMenu::new(&menu_options_str);

            let mut textfield = TuiTextField::default()
                .prompt(":")
                .prefix(PROMPT)
                .menu(&mut menu_widget);
            textfield.get_input(backend, &context)
        };

        match user_input.as_ref() {
            Some(user_input) if user_input.starts_with(PROMPT) => {
                let user_input = &user_input[PROMPT.len()..];

                match user_input.parse::<usize>() {
                    Ok(n) if n >= mimetype_options.len() => Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "option does not exist".to_string(),
                    )),
                    Ok(n) => {
                        let mimetype_entry = &mimetype_options[n];
                        if mimetype_entry.get_fork() {
                            mimetype_entry.execute_with(paths)
                        } else {
                            backend.terminal_drop();
                            let res = mimetype_entry.execute_with(paths);
                            backend.terminal_restore()?;
                            res
                        }
                    }
                    Err(_) => {
                        let mut args_iter = user_input.split_whitespace();
                        match args_iter.next() {
                            Some(cmd) => {
                                backend.terminal_drop();
                                let res = JoshutoMimetypeEntry::new(String::from(cmd))
                                    .args(args_iter)
                                    .execute_with(paths);
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
}

impl JoshutoCommand for OpenFileWith {}

impl std::fmt::Display for OpenFileWith {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for OpenFileWith {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        let paths = match &context.tabs[context.curr_tab_index].curr_list_ref() {
            Some(curr_list) => curr_list.get_selected_paths(),
            None => vec![],
        };

        if paths.is_empty() {
            return Err(JoshutoError::new(
                JoshutoErrorKind::IONotFound,
                String::from("No files selected"),
            ));
        }
        Self::open_with(context, backend, &paths)?;
        Ok(())
    }
}
