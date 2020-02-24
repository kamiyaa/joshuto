use std::path::{Path, PathBuf};

use crate::commands::{ChangeDirectory, JoshutoCommand, JoshutoRunnable};
use crate::config::mimetype::JoshutoMimetypeEntry;
use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::history::DirectoryHistory;
use crate::ui::widgets::TuiTextField;
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
                options[0].execute_with(&paths)?;
                backend.terminal_restore();
            } else {
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

    pub fn open_with(context: &JoshutoContext, backend: &mut TuiBackend, paths: &[&PathBuf]) -> std::io::Result<()> {
        let mimetype_options: Vec<&JoshutoMimetypeEntry> = OpenFile::get_options(&paths[0]);

        let mut textfield = TuiTextField::default()
            .prompt(":")
            .prefix("open_with ");
        let user_input: Option<String> = textfield.get_input(backend, &context);

        match user_input.as_ref() {
            None => Ok(()),
            Some(user_input) => match user_input.parse::<usize>() {
                Ok(n) if n >= mimetype_options.len() => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "option does not exist".to_owned(),
                )),
                Ok(n) => {
                    backend.terminal_drop();
                    let res = mimetype_options[n].execute_with(paths);
                    backend.terminal_restore()?;
                    res
                }
                Err(_) => {
                    let mut args_iter = user_input.split_whitespace();
                    args_iter.next();
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
            Some(curr_list) => {
                curr_list.get_selected_paths()
            }
            None => vec![],
        };

        if paths.is_empty() {
            return Err(JoshutoError::new(
                JoshutoErrorKind::IONotFound,
                String::from("No files selected"),
            ))
        }
        Self::open_with(context, backend, &paths)?;
        Ok(())
    }
}
