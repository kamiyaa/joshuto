use std::path;

use crate::commands::{CommandLine, JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;
use crate::util::load_child::LoadChild;

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
    ) -> std::io::Result<()> {
        let new_path = &self.path;
        if new_path.exists() {
            let err =
                std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Filename already exists");
            return Err(err);
        }
        std::fs::rename(&path, &new_path)?;
        let options = context.config_t.sort_option.clone();
        if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
            curr_list.reload_contents(&options)?;
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
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        let mut path: Option<path::PathBuf> = None;

        if let Some(s) = context
            .tab_context_ref()
            .curr_tab_ref()
            .curr_list_ref()
            .and_then(|s| s.get_curr_ref())
        {
            path = Some(s.file_path().to_path_buf());
        }

        if let Some(path) = path {
            self.rename_file(&path, context)?;
        }
        LoadChild::load_child(context)?;
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
        backend: &mut TuiBackend,
        file_name: &str,
    ) -> JoshutoResult<()> {
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
        command.readline(context, backend)
    }
}

impl JoshutoCommand for RenameFileAppend {}

impl std::fmt::Display for RenameFileAppend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for RenameFileAppend {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        let mut file_name: Option<String> = None;

        if let Some(curr_list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
            file_name = curr_list.get_curr_ref().map(|s| s.file_name().to_string());
        }

        if let Some(file_name) = file_name {
            self.rename_file(context, backend, file_name.as_str())?;
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
        backend: &mut TuiBackend,
        file_name: String,
    ) -> JoshutoResult<()> {
        let prefix = String::from("rename ");
        let suffix = file_name;

        let command = CommandLine::new(prefix, suffix);
        command.readline(context, backend)
    }
}

impl JoshutoCommand for RenameFilePrepend {}

impl std::fmt::Display for RenameFilePrepend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for RenameFilePrepend {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        let mut file_name: Option<String> = None;

        if let Some(curr_list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
            file_name = curr_list.get_curr_ref().map(|s| s.file_name().to_string());
        }

        if let Some(file_name) = file_name {
            self.rename_file(context, backend, file_name)?;
        }
        Ok(())
    }
}
