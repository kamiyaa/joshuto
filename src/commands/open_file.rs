use std::path::{Path, PathBuf};

use crate::commands::{ChangeDirectory, JoshutoCommand, JoshutoRunnable, LoadChild};
use crate::config::mimetype::JoshutoMimetypeEntry;
use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::history::DirectoryHistory;
use crate::textfield::TextField;
use crate::ui::TuiBackend;
use crate::window;

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
            LoadChild::load_child(context, backend);
        } else if let Some(paths) = filepaths {
            let options = Self::get_options(paths[0]);
            if options.len() > 0 {
                options[0].execute_with(&paths)?;
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
/*
#[derive(Clone, Debug)]
pub struct OpenFileWith;

impl OpenFileWith {
    pub fn new() -> Self {
        OpenFileWith
    }
    pub const fn command() -> &'static str {
        "open_file_with"
    }

    pub fn open_with(paths: &[&PathBuf]) -> std::io::Result<()> {
        const PROMPT: &str = ":open_with ";

        let mimetype_options: Vec<&JoshutoMimetypeEntry> = OpenFile::get_options(&paths[0]);
        let user_input: Option<String> = {
            let (term_rows, term_cols) = ui::getmaxyx();

            let option_size = mimetype_options.len();
            let display_win = window::JoshutoPanel::new(
                option_size as i32 + 2,
                term_cols,
                (term_rows as usize - option_size - 2, 0),
            );

            let mut display_vec: Vec<String> = Vec::with_capacity(option_size);
            for (i, val) in mimetype_options.iter().enumerate() {
                display_vec.push(format!("  {}\t{}", i, val));
            }
            display_vec.sort();

            display_win.move_to_top();
            ui::display_menu(&display_win, &display_vec);
            ncurses::doupdate();

            let textfield =
                JoshutoTextField::new(1, term_cols, (term_rows as usize - 1, 0), PROMPT, "", "");
            textfield.readline()
        };
        ncurses::doupdate();

        match user_input.as_ref() {
            None => Ok(()),
            Some(user_input) if user_input.is_empty() => Ok(()),
            Some(user_input) => match user_input.parse::<usize>() {
                Ok(n) if n >= mimetype_options.len() => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "option does not exist".to_owned(),
                )),
                Ok(n) => mimetype_options[n].execute_with(paths),
                Err(_) => {
                    let mut args_iter = user_input.split_whitespace();
                    match args_iter.next() {
                        Some(cmd) => JoshutoMimetypeEntry::new(String::from(cmd))
                            .add_args(args_iter)
                            .execute_with(paths),
                        None => Ok(()),
                    }
                }
            },
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
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        let curr_list = &context.tabs[context.curr_tab_index].curr_list;
        match curr_list.index {
            None => {
                return Err(JoshutoError::new(
                    JoshutoErrorKind::IONotFound,
                    String::from("No files selected"),
                ))
            }
            Some(_) => {}
        }
        let paths = curr_list.get_selected_paths();
        Self::open_with(&paths)?;
        Ok(())
    }
}
*/
