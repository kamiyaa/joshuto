use std::path::{Path, PathBuf};

use crate::commands::{ChangeDirectory, JoshutoCommand, JoshutoRunnable};
use crate::config::mimetype::JoshutoMimetypeEntry;
use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::fs::{JoshutoDirEntry, JoshutoMetadata};
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

    pub fn get_options<'a>(entry: &JoshutoDirEntry) -> Vec<&'a JoshutoMimetypeEntry> {
        let mut mimetype_options: Vec<&JoshutoMimetypeEntry> = Vec::new();

        /* extensions have priority */
        if let Some(file_ext) = entry.file_path().extension() {
            if let Some(file_ext) = file_ext.to_str() {
                let ext_entries = MIMETYPE_T.get_entries_for_ext(file_ext);
                mimetype_options.extend(ext_entries);
            }
        }
        #[cfg(feature = "file_mimetype")]
        {
            if let Some(mimetype) = entry.metadata.mimetype.as_ref() {
                let mime_entries = MIMETYPE_T.get_entries_for_mimetype(mimetype.as_str());
                mimetype_options.extend(mime_entries);
            }
        }
        mimetype_options
    }

    fn open(context: &mut JoshutoContext, backend: &mut TuiBackend) -> std::io::Result<()> {
        let mut dirpath = None;
        let mut selected_entries = None;

        {
            let curr_tab = context.curr_tab_ref();
            match curr_tab.curr_list_ref() {
                None => return Ok(()),
                Some(curr_list) => match curr_list.get_curr_ref() {
                    Some(entry) if entry.file_path().is_dir() => {
                        let path = entry.file_path().clone();
                        dirpath = Some(path);
                    }
                    Some(entry) => {
                        let vec: Vec<&JoshutoDirEntry> = curr_list.selected_entries().collect();
                        if vec.is_empty() {
                            selected_entries = Some(vec![entry]);
                        } else {
                            selected_entries = Some(vec);
                        }
                    }
                    None => return Ok(()),
                },
            }
        }

        if let Some(path) = dirpath {
            ChangeDirectory::cd(path.as_path(), context)?;
            LoadChild::load_child(context)?;
        } else if let Some(entries) = selected_entries {
            let options = Self::get_options(entries[0]);
            let entry_paths: Vec<&Path> = entries.iter().map(|e| e.file_path().as_path()).collect();
            if !options.is_empty() {
                let res = if options[0].get_fork() {
                    options[0].execute_with(entry_paths.as_slice())
                } else {
                    backend.terminal_drop();
                    let res = options[0].execute_with(entry_paths.as_slice());
                    backend.terminal_restore()?;
                    res
                };
                return res;
            } else {
                OpenFileWith::open_with(context, backend, &entries)?;
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
        entries: &[&JoshutoDirEntry],
    ) -> std::io::Result<()> {
        const PROMPT: &str = "open_with ";

        let mimetype_options: Vec<&JoshutoMimetypeEntry> = OpenFile::get_options(&entries[0]);

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
        let entry_paths: Vec<&Path> = entries.iter().map(|e| e.file_path().as_path()).collect();

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
                            mimetype_entry.execute_with(entry_paths.as_slice())
                        } else {
                            backend.terminal_drop();
                            let res = mimetype_entry.execute_with(entry_paths.as_slice());
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
                                    .execute_with(entry_paths.as_slice());
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
        let selected_entries = {
            let curr_tab = context.curr_tab_ref();
            match curr_tab.curr_list_ref() {
                None => vec![],
                Some(curr_list) => match curr_list.get_curr_ref() {
                    Some(entry) => {
                        let vec: Vec<&JoshutoDirEntry> = curr_list.selected_entries().collect();
                        if vec.is_empty() {
                            vec![entry]
                        } else {
                            vec
                        }
                    }
                    None => vec![],
                },
            }
        };

        if selected_entries.is_empty() {
            return Err(JoshutoError::new(
                JoshutoErrorKind::IONotFound,
                String::from("No files selected"),
            ));
        }
        Self::open_with(context, backend, &selected_entries)?;
        Ok(())
    }
}
