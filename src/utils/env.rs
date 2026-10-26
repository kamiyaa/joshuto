use std::path;

use crate::{
    error::{AppError, AppErrorCode, AppResult},
    HOSTNAME,
};

/// Sets the process's current directory to `path` and notifies the terminal via an OSC 7
/// escape sequence, so external tools (e.g. new terminal tabs) can pick up the cwd.
pub fn set_current_dir(path: &path::Path) -> AppResult {
    std::env::set_current_dir(path).map_err(|err| {
        let error_msg = format!("Failed to set current directory: {:?}", err);
        tracing::error!(?err, ?path, "{error_msg}");
        AppError::new(AppErrorCode::SetCurrentDirectory, error_msg)
    })?;

    // OSC 7:
    // Escape sequences to advise the terminal of the working directory
    print!(
        "\x1b]7;file://{}{}\x1b\\",
        HOSTNAME.as_str(),
        path.display()
    );
    Ok(())
}
