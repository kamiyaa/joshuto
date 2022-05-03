use std::path::Path;
use std::process::{Command, Stdio};

use crate::commands::change_directory;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;

pub fn zoxide(context: &mut AppContext, args: &str) -> JoshutoResult {
    let cwd = std::env::current_dir()?;

    let zoxide_output = Command::new("zoxide")
        .arg("query")
        .arg("--exclude")
        .arg(&cwd)
        .arg("--")
        .arg(args)
        .output()?;

    if zoxide_output.status.success() {
        if let Ok(zoxide_str) = std::str::from_utf8(&zoxide_output.stdout) {
            let zoxide_path = zoxide_str.trim();
            let path = Path::new(zoxide_path);
            context
                .message_queue_mut()
                .push_info(format!("z {}", zoxide_path));
            change_directory::change_directory(context, &path)?;
        }
    }
    Ok(())
}

pub fn zoxide_interactive(context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult {
    backend.terminal_drop();

    let zoxide_process = Command::new("zoxide")
        .arg("query")
        .arg("-i")
        .arg("--")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let zoxide_output = zoxide_process.wait_with_output()?;

    backend.terminal_restore()?;

    if zoxide_output.status.success() {
        if let Ok(zoxide_str) = std::str::from_utf8(&zoxide_output.stdout) {
            let zoxide_path = zoxide_str.trim();
            let path = Path::new(zoxide_path);
            context
                .message_queue_mut()
                .push_info(format!("z {}", zoxide_path));
            change_directory::change_directory(context, &path)?;
        }
    }
    Ok(())
}
