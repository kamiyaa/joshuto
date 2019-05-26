use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
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
        initial: String,
    ) -> Result<(), std::io::Error> {
        const PROMPT: &str = ":rename_file ";
        let (term_rows, term_cols) = ui::getmaxyx();
        let user_input: Option<String> = {
            let prefix: String;
            let suffix: String;
            match self.method {
                RenameFileMethod::Append => {
                    if let Some(ext) = initial.rfind('.') {
                        prefix = String::from(&initial[0..ext]);
                        suffix = String::from(&initial[ext..]);
                    } else {
                        prefix = initial;
                        suffix = String::new();
                    }
                }
                RenameFileMethod::Prepend => {
                    prefix = String::new();
                    suffix = initial;
                }
                RenameFileMethod::Overwrite => {
                    prefix = String::new();
                    suffix = String::new();
                }
            }
            let textfield = JoshutoTextField::new(
                1,
                term_cols,
                (term_rows as usize - 1, 0),
                PROMPT.to_string(),
                prefix,
                suffix,
            );
            textfield.readline()
        };

        if let Some(s) = user_input {
            let mut new_path = path.parent().unwrap().to_path_buf();

            new_path.push(s);
            if new_path.exists() {
                let err = std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "Filename already exists",
                );
                return Err(err);
            }
            std::fs::rename(&path, &new_path)?;
            let curr_tab = &mut context.tabs[context.curr_tab_index];
            curr_tab
                .curr_list
                .update_contents(&context.config_t.sort_option)?;
            curr_tab.refresh_curr(&view.mid_win, context.config_t.scroll_offset);
            curr_tab.refresh_preview(&view.right_win, &context.config_t);
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

        let curr_list = &context.tabs[context.curr_tab_index].curr_list;
        if let Some(s) = curr_list.get_curr_ref() {
            path = Some(s.path.clone());
            file_name = Some(s.file_name_as_string.clone());
        }

        if let Some(file_name) = file_name {
            if let Some(path) = path {
                match self.rename_file(&path, context, view, file_name.clone()) {
                    Ok(_) => {}
                    Err(e) => return Err(JoshutoError::IO(e)),
                }
                ncurses::doupdate();
            }
        }
        Ok(())
    }
}
