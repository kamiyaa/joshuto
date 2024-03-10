use crate::commands::change_directory::change_directory;
use crate::context::AppContext;
use crate::error::{AppError, AppErrorKind, AppResult};
use crate::util::unix::expand_shell_string;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum PostProcessor {
    ChangeDirectory,
}

impl PostProcessor {
    pub fn from_str(args: &str) -> Option<Self> {
        match args {
            "cd" => Some(PostProcessor::ChangeDirectory),
            _ => None,
        }
    }
}

fn assert_one_line(stdout: &str) -> AppResult {
    match stdout.lines().count() {
        1 => Ok(()),
        _ => Err(AppError::new(AppErrorKind::StateError, "The last `capture` stdout does not have exactly one line as expected for this stdout processor".to_string()))
    }
}

fn as_one_existing_directory(stdout: &str) -> AppResult<PathBuf> {
    assert_one_line(stdout)?;
    let path = expand_shell_string(stdout);
    if path.exists() {
        if path.is_file() {
            if let Some(parent) = path.parent() {
                Ok(parent.to_path_buf())
            } else {
                Err(AppError::new(AppErrorKind::StateError, "The last `capture` output is a file but without a valid directory as parent in the file system".to_string()))
            }
        } else {
            Ok(path.to_path_buf())
        }
    } else {
        Err(AppError::new(
            AppErrorKind::StateError,
            "The last `capture` output line is not an existing path".to_string(),
        ))
    }
}

pub fn post_process_std_out(processor: &PostProcessor, context: &mut AppContext) -> AppResult {
    let last_stdout = &context.last_stdout;
    if let Some(stdout) = last_stdout {
        let stdout = stdout.trim();
        match processor {
            PostProcessor::ChangeDirectory => {
                change_directory(context, as_one_existing_directory(stdout)?.as_path())
            }
        }
    } else {
        Err(AppError::new(
            AppErrorKind::StateError,
            "No result from a former `shell` available".to_string(),
        ))
    }
}
