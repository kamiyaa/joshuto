use std::path::{Path, PathBuf};

use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::config::mimetype::JoshutoMimetypeEntry;
use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::history::DirectoryHistory;
use crate::textfield::JoshutoTextField;
use crate::ui;
use crate::window;
use crate::window::JoshutoView;

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

    fn open(context: &mut JoshutoContext, view: &JoshutoView) -> std::io::Result<()> {
        let mut path: Option<PathBuf> = None;
        {
            let curr_list = &context.tabs[context.curr_tab_index].curr_list;
            if let Some(entry) = curr_list.get_curr_ref() {
                if entry.file_path().is_dir() {
                    path = Some(entry.file_path().clone());
                }
            }
        }
        if let Some(path) = path {
            Self::open_directory(&path, context)?;
            let curr_tab = &mut context.tabs[context.curr_tab_index];
            if curr_tab.curr_list.need_update() {
                curr_tab
                    .curr_list
                    .reload_contents(&context.config_t.sort_option)?;
                curr_tab
                    .curr_list
                    .sort(context.config_t.sort_option.compare_func());
            }
            curr_tab.refresh(view, &context.config_t);
        } else {
            let curr_tab = &context.tabs[context.curr_tab_index];
            let paths = curr_tab.curr_list.get_selected_paths();

            if paths.is_empty() {
                let err = std::io::Error::new(std::io::ErrorKind::NotFound, "No files selected");
                return Err(err);
            }
            let mimetype_options = Self::get_options(&paths[0]);

            /* try executing with user defined entries */
            if !mimetype_options.is_empty() {
                mimetype_options[0].execute_with(&paths)?;
            } else if context.config_t.xdg_open {   // try system defined entries
                ncurses::savetty();
                ncurses::endwin();
                open::that(paths[0]).unwrap();
                ncurses::resetty();
                ncurses::refresh();
            } else {    // ask user for command
                OpenFileWith::open_with(&paths)?;
            }
            let curr_tab = &mut context.tabs[context.curr_tab_index];
            if curr_tab.curr_list.need_update() {
                curr_tab
                    .curr_list
                    .reload_contents(&context.config_t.sort_option)?;
                curr_tab
                    .curr_list
                    .sort(context.config_t.sort_option.compare_func());
            }
            curr_tab.refresh(view, &context.config_t);
        }
        ncurses::doupdate();
        Ok(())
    }

    fn open_directory(path: &Path, context: &mut JoshutoContext) -> std::io::Result<()> {
        std::env::set_current_dir(path)?;

        let curr_tab = &mut context.tabs[context.curr_tab_index];
        let mut new_curr_list = curr_tab
            .history
            .pop_or_create(path, &context.config_t.sort_option)?;

        std::mem::swap(&mut curr_tab.curr_list, &mut new_curr_list);
        curr_tab
            .history
            .insert(new_curr_list.file_path().clone(), new_curr_list);

        curr_tab.curr_path = path.to_path_buf().clone();
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
    fn execute(&self, context: &mut JoshutoContext, view: &JoshutoView) -> JoshutoResult<()> {
        Self::open(context, view)?;
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
    fn execute(&self, context: &mut JoshutoContext, _: &JoshutoView) -> JoshutoResult<()> {
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
