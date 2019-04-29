use std::path::{Path, PathBuf};

use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::config::mimetype;
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::history::DirectoryHistory;
use crate::textfield::JoshutoTextField;
use crate::ui;
use crate::unix;
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

    pub fn get_options<'a>(path: &Path) -> Vec<&'a mimetype::JoshutoMimetypeEntry> {
        let mut mimetype_options: Vec<&mimetype::JoshutoMimetypeEntry> = Vec::new();

        /* extensions have priority */
        if let Some(file_ext) = path.extension() {
            if let Some(file_ext) = file_ext.to_str() {
                if let Some(s) = MIMETYPE_T.extension.get(file_ext) {
                    mimetype_options.extend(s.iter());
                }
            }
        }
/*
        match tree_magic::from_filepath(&path) {
            None => {},
            Some(mimetype) => {
                /* mime subtype have second priority */
                if let Some(s) = MIMETYPE_T.mimetype.get(&mimetype) {
                    mimetype_options.extend(s.iter());
                }

                /* generic mime type have last priority */
                if let Some(s) = mimetype.find('/') {
                    let mimetype_type = &mimetype[..s];
                    if let Some(s) = MIMETYPE_T.mimetype.get(mimetype_type) {
                        mimetype_options.extend(s.iter());
                    }
                }
            }
        }
*/
        mimetype_options
    }

    fn open(context: &mut JoshutoContext, view: &JoshutoView) -> Result<(), std::io::Error> {
        let mut path: Option<PathBuf> = None;
        {
            let curr_list = &context.tabs[context.curr_tab_index].curr_list;
            if let Some(entry) = curr_list.get_curr_ref() {
                if entry.path.is_dir() {
                    path = Some(entry.path.clone());
                }
            }
        }
        if let Some(path) = path {
            Self::open_directory(&path, context)?;
            let curr_tab = &mut context.tabs[context.curr_tab_index];
            if curr_tab.curr_list.need_update() {
                curr_tab
                    .curr_list
                    .update_contents(&context.config_t.sort_option)?;
            }
            curr_tab.refresh(
                view,
                &context.config_t,
                &context.username,
                &context.hostname,
            );
        } else {
            let paths: Option<Vec<PathBuf>> = context.tabs[context.curr_tab_index]
                .curr_list
                .get_selected_paths();

            if let Some(paths) = paths {
                Self::open_file(&paths);
            } else {
                let err = std::io::Error::new(std::io::ErrorKind::NotFound, "No files selected");
                return Err(err);
            }
            let curr_tab = &mut context.tabs[context.curr_tab_index];
            if curr_tab.curr_list.need_update() {
                curr_tab
                    .curr_list
                    .update_contents(&context.config_t.sort_option)?;
            }
            curr_tab.refresh(
                view,
                &context.config_t,
                &context.username,
                &context.hostname,
            );
        }
        ncurses::doupdate();
        Ok(())
    }

    fn open_directory(path: &Path, context: &mut JoshutoContext) -> Result<(), std::io::Error> {
        let curr_tab = &mut context.tabs[context.curr_tab_index];

        std::env::set_current_dir(path)?;

        let mut new_curr_list = curr_tab
            .history
            .pop_or_create(path, &context.config_t.sort_option)?;

        std::mem::swap(&mut curr_tab.curr_list, &mut new_curr_list);
        curr_tab
            .history
            .insert(new_curr_list.path.clone(), new_curr_list);

        curr_tab.curr_path = path.to_path_buf().clone();
        Ok(())
    }

    fn open_file(paths: &[PathBuf]) {
        let mimetype_options = Self::get_options(&paths[0]);

        ncurses::savetty();
        ncurses::endwin();
        if mimetype_options.is_empty() {
            open::that(&paths[0]).unwrap();
        } else {
            unix::open_with_entry(paths, &mimetype_options[0]);
        }
        ncurses::resetty();
        ncurses::refresh();
        ncurses::doupdate();
    }
}

impl JoshutoCommand for OpenFile {}

impl std::fmt::Display for OpenFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for OpenFile {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        match Self::open(context, view) {
            Ok(_) => Ok(()),
            Err(e) => Err(JoshutoError::IO(e)),
        }
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

    pub fn open_with(paths: &[PathBuf]) {
        const PROMPT: &str = ":open_with ";

        let mimetype_options: Vec<&mimetype::JoshutoMimetypeEntry> =
            OpenFile::get_options(&paths[0]);
        let user_input: Option<String>;
        {
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
            ui::display_options(&display_win, &display_vec);
            ncurses::doupdate();

            let textfield = JoshutoTextField::new(
                1,
                term_cols,
                (term_rows as usize - 1, 0),
                PROMPT.to_string(),
            );
            user_input = textfield.readline_with_initial("", "");
        }
        ncurses::doupdate();

        if let Some(user_input) = user_input {
            if user_input.is_empty() {
                return;
            }
            match user_input.parse::<usize>() {
                Ok(s) => {
                    if s < mimetype_options.len() {
                        ncurses::savetty();
                        ncurses::endwin();
                        unix::open_with_entry(&paths, &mimetype_options[s]);
                        ncurses::resetty();
                        ncurses::refresh();
                    }
                }
                Err(_) => {
                    let args: Vec<String> =
                        user_input.split_whitespace().map(String::from).collect();
                    ncurses::savetty();
                    ncurses::endwin();
                    unix::open_with_args(&paths, &args);
                    ncurses::resetty();
                    ncurses::refresh();
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
    fn execute(&self, context: &mut JoshutoContext, _: &JoshutoView) -> Result<(), JoshutoError> {
        let curr_list = &context.tabs[context.curr_tab_index].curr_list;
        if let Some(paths) = curr_list.get_selected_paths() {
            Self::open_with(&paths);
        }
        Ok(())
    }
}
