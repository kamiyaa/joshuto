use std::path;

use crate::commands::{CommandLine, JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::window::JoshutoView;

use rustyline::completion::{escape, Quote};

#[cfg(unix)]
static DEFAULT_BREAK_CHARS: [u8; 18] = [
    b' ', b'\t', b'\n', b'"', b'\\', b'\'', b'`', b'@', b'$', b'>', b'<', b'=', b';', b'|', b'&',
    b'{', b'(', b'\0',
];
#[cfg(unix)]
static ESCAPE_CHAR: Option<char> = Some('\\');

#[derive(Clone, Debug)]
pub struct RenameFile {
    path: path::PathBuf,
}

impl RenameFile {
    pub fn new(path: path::PathBuf) -> Self {
        RenameFile { path }
    }
    pub const fn command() -> &'static str {
        "rename"
    }

    pub fn rename_file(
        &self,
        path: &path::PathBuf,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), std::io::Error> {
        let new_path = &self.path;
        if new_path.exists() {
            let err =
                std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Filename already exists");
            return Err(err);
        }
        std::fs::rename(&path, &new_path)?;
        let curr_tab = &mut context.tabs[context.curr_tab_index];
        curr_tab
            .curr_list
            .update_contents(&context.config_t.sort_option)?;
        curr_tab.refresh_curr(&view.mid_win, context.config_t.scroll_offset);
        curr_tab.refresh_preview(&view.right_win, &context.config_t);
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

        let curr_list = &context.tabs[context.curr_tab_index].curr_list;
        if let Some(s) = curr_list.get_curr_ref() {
            path = Some(s.path.clone());
        }

        if let Some(path) = path {
            match self.rename_file(&path, context, view) {
                Ok(_) => {}
                Err(e) => return Err(JoshutoError::IO(e)),
            }
            ncurses::doupdate();
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct RenameFileAppend;

impl RenameFileAppend {
    pub fn new() -> Self {
        RenameFileAppend {}
    }
    pub const fn command() -> &'static str {
        "rename_append"
    }

    pub fn rename_file(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
        file_name: String,
    ) -> Result<(), JoshutoError> {
        let prefix;
        let suffix;
        if let Some(ext) = file_name.rfind('.') {
            prefix = format!("rename {}", &file_name[0..ext]);
            suffix = String::from(&file_name[ext..]);
        } else {
            prefix = format!("rename {}", file_name);
            suffix = String::new();
        }

        let command = CommandLine::new(prefix, suffix);
        command.readline(context, view)
    }
}

impl JoshutoCommand for RenameFileAppend {}

impl std::fmt::Display for RenameFileAppend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for RenameFileAppend {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        let curr_list = &context.tabs[context.curr_tab_index].curr_list;
        let file_name = match curr_list.get_curr_ref() {
            Some(s) => {
                let escaped = escape(
                    s.file_name_as_string.clone(),
                    ESCAPE_CHAR,
                    &DEFAULT_BREAK_CHARS,
                    Quote::None,
                );
                Some(escaped)
            }
            None => None,
        };

        if let Some(file_name) = file_name {
            self.rename_file(context, view, file_name)?;
            ncurses::doupdate();
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct RenameFilePrepend;

impl RenameFilePrepend {
    pub fn new() -> Self {
        RenameFilePrepend {}
    }
    pub const fn command() -> &'static str {
        "rename_prepend"
    }

    pub fn rename_file(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
        file_name: String,
    ) -> Result<(), JoshutoError> {
        let prefix = String::from("rename ");
        let suffix = file_name;

        let command = CommandLine::new(prefix, suffix);
        command.readline(context, view)
    }
}

impl JoshutoCommand for RenameFilePrepend {}

impl std::fmt::Display for RenameFilePrepend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for RenameFilePrepend {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        let curr_list = &context.tabs[context.curr_tab_index].curr_list;
        let file_name = match curr_list.get_curr_ref() {
            Some(s) => {
                let escaped = escape(
                    s.file_name_as_string.clone(),
                    ESCAPE_CHAR,
                    &DEFAULT_BREAK_CHARS,
                    Quote::None,
                );
                Some(escaped)
            }
            None => None,
        };

        if let Some(file_name) = file_name {
            self.rename_file(context, view, file_name)?;
            ncurses::doupdate();
        }
        Ok(())
    }
}
