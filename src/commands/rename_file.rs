use std::fs;
use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::preview;
use crate::textfield::JoshutoTextField;
use crate::ui;
use crate::window::JoshutoView;

#[derive(Clone, Debug)]
pub enum RenameFileMethod {
    Append,
    Prepend,
    Overwrite,
}

#[derive(Clone, Debug)]
pub struct RenameFile {
    method: RenameFileMethod,
}

impl RenameFile {
    pub fn new(method: RenameFileMethod) -> Self {
        RenameFile { method }
    }
    pub const fn command() -> &'static str {
        "rename_file"
    }

    pub fn rename_file(
        &self,
        path: &path::PathBuf,
        context: &mut JoshutoContext,
        view: &JoshutoView,
        start_str: String,
    ) -> Result<(), JoshutoError> {
        const PROMPT: &str = ":rename_file ";
        let (term_rows, term_cols) = ui::getmaxyx();
        let user_input: Option<String> = {
            let textfield = JoshutoTextField::new(
                1,
                term_cols,
                (term_rows as usize - 1, 0),
                PROMPT.to_string(),
            );

            match self.method {
                RenameFileMethod::Append => {
                    if let Some(ext) = start_str.rfind('.') {
                        textfield.readline_with_initial(&start_str[0..ext], &start_str[ext..])
                    } else {
                        textfield.readline_with_initial(&start_str, "")
                    }
                }
                RenameFileMethod::Prepend => textfield.readline_with_initial("", &start_str),
                RenameFileMethod::Overwrite => textfield.readline_with_initial("", ""),
            }
        };

        if let Some(s) = user_input {
            let mut new_path = path.parent().unwrap().to_path_buf();

            new_path.push(s);
            if new_path.exists() {
                let err = std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "Filename already exists",
                );
                return Err(JoshutoError::IO(err));
            }
            match fs::rename(&path, &new_path) {
                Ok(_) => {
                    let curr_tab = &mut context.tabs[context.curr_tab_index];
                    if let Some(ref mut s) = curr_tab.curr_list {
                        s.update_contents(&context.config_t.sort_option).unwrap();
                    }
                    curr_tab.refresh_curr(&view.mid_win, context.config_t.scroll_offset);
                }
                Err(e) => {
                    return Err(JoshutoError::IO(e));
                }
            }
        } else {
            let curr_tab = &context.tabs[context.curr_tab_index];
            curr_tab.refresh_file_status(&view.bot_win);
        }
        Ok(())
    }
}

impl JoshutoCommand for RenameFile {}

impl std::fmt::Display for RenameFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for RenameFile {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        let mut path: Option<path::PathBuf> = None;
        let mut file_name: Option<String> = None;

        if let Some(s) = context.tabs[context.curr_tab_index].curr_list.as_ref() {
            if let Some(s) = s.get_curr_ref() {
                path = Some(s.path.clone());
                file_name = Some(s.file_name_as_string.clone());
            }
        }

        if let Some(file_name) = file_name {
            if let Some(path) = path {
                let res = self.rename_file(&path, context, view, file_name);
                preview::preview_file(
                    &mut context.tabs[context.curr_tab_index],
                    view,
                    &context.config_t,
                );
                ncurses::doupdate();
                return res;
            }
        }
        return Ok(());
    }
}
