use std::path;

use crate::commands::{ChangeDirectory, JoshutoCommand, JoshutoRunnable};
use crate::config::mimetype::JoshutoMimetypeEntry;
use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
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

    pub fn get_options<'a>(path: &path::Path) -> Vec<&'a JoshutoMimetypeEntry> {
        let mut options: Vec<&JoshutoMimetypeEntry> = Vec::new();
        if let Some(file_ext) = path.extension() {
            if let Some(file_ext) = file_ext.to_str() {
                let ext_entries = MIMETYPE_T.get_entries_for_ext(file_ext);
                options.extend(ext_entries);
            }
        }
        options
    }
    /*
        pub fn get_options<'a>(entry: &JoshutoDirEntry) -> Vec<&'a JoshutoMimetypeEntry> {
            let mut mimetype_options: Vec<&JoshutoMimetypeEntry> = Vec::new();

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
    */
}

impl JoshutoCommand for OpenFile {}

impl std::fmt::Display for OpenFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for OpenFile {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        if let Some(entry) = context
            .tab_context_ref()
            .curr_tab_ref()
            .curr_list_ref()
            .and_then(|s| s.get_curr_ref())
        {
            if entry.file_path().is_dir() {
                let path = entry.file_path().to_path_buf();
                ChangeDirectory::cd(path.as_path(), context)?;
                LoadChild::load_child(context)?;
            } else {
                let paths: Vec<path::PathBuf> =
                    match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
                        Some(a) => a.get_selected_paths(),
                        None => vec![],
                    };
                if paths.is_empty() {
                    return Err(JoshutoError::new(
                        JoshutoErrorKind::IONotFound,
                        String::from("No files selected"),
                    ));
                }
                let files: Vec<&std::ffi::OsStr> =
                    paths.iter().filter_map(|e| e.file_name()).collect();
                let options = Self::get_options(paths[0].as_path());

                if !options.is_empty() {
                    if options[0].get_fork() {
                        options[0].execute_with(files.as_slice())?;
                    } else {
                        backend.terminal_drop();
                        let res = options[0].execute_with(files.as_slice());
                        backend.terminal_restore()?;
                        res?;
                    }
                } else {
                    OpenFileWith::open_with(context, backend, options, files)?;
                }
            }
        }
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

    pub fn open_with<S>(
        context: &mut JoshutoContext,
        backend: &mut TuiBackend,
        options: Vec<&JoshutoMimetypeEntry>,
        files: Vec<S>,
    ) -> std::io::Result<()>
    where
        S: AsRef<std::ffi::OsStr>,
    {
        const PROMPT: &str = "open_with ";

        let user_input: Option<String> = {
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
                                let res = JoshutoMimetypeEntry::new(String::from(cmd))
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
}

impl JoshutoCommand for OpenFileWith {}

impl std::fmt::Display for OpenFileWith {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for OpenFileWith {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        let paths: Vec<path::PathBuf> =
            match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
                Some(a) => a.get_selected_paths(),
                None => vec![],
            };
        if paths.is_empty() {
            return Err(JoshutoError::new(
                JoshutoErrorKind::IONotFound,
                String::from("No files selected"),
            ));
        }
        let files: Vec<&std::ffi::OsStr> = paths.iter().filter_map(|e| e.file_name()).collect();
        let options = OpenFile::get_options(paths[0].as_path());

        Self::open_with(context, backend, options, files)?;
        Ok(())
    }
}
