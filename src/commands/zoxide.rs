use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::commands::change_directory;
use crate::context::AppContext;
use crate::error::AppResult;
use crate::ui::AppBackend;

pub fn zoxide_query(context: &mut AppContext, args: &str) -> AppResult {
    let cwd = std::env::current_dir()?;

    let path = Path::new(args);
    if change_directory::change_directory(context, path).is_ok() {
        if !context.config_ref().zoxide_update {
            let cwd = context
                .tab_context_ref()
                .curr_tab_ref()
                .cwd()
                .to_str()
                .expect("path cannot be converted to string");
            zoxide_add(cwd)?;
        }
        return Ok(());
    }

    let zoxide_output = Command::new("zoxide")
        .arg("query")
        .arg("--exclude")
        .arg(&cwd)
        .arg("--")
        .args(args.split(' '))
        .output()?;

    if zoxide_output.status.success() {
        if let Ok(zoxide_str) = std::str::from_utf8(&zoxide_output.stdout) {
            let zoxide_path = &zoxide_str[..zoxide_str.len() - 1];
            if !context.config_ref().zoxide_update {
                zoxide_add(zoxide_path)?;
            }

            let path = Path::new(zoxide_path);
            context
                .message_queue_mut()
                .push_info(format!("z {:?}", zoxide_path));
            change_directory::change_directory(context, path)?;
        }
    } else if let Ok(zoxide_str) = std::str::from_utf8(&zoxide_output.stderr) {
        context
            .message_queue_mut()
            .push_error(zoxide_str.to_string());
    }
    Ok(())
}

pub fn zoxide_query_interactive(
    context: &mut AppContext,
    backend: &mut AppBackend,
    args: &str,
) -> AppResult {
    backend.terminal_drop();

    let zoxide_process = Command::new("zoxide")
        .arg("query")
        .arg("-i")
        .arg("--")
        .args(args.split(' '))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let zoxide_output = zoxide_process.wait_with_output()?;

    backend.terminal_restore()?;

    if zoxide_output.status.success() {
        if let Ok(zoxide_str) = std::str::from_utf8(&zoxide_output.stdout) {
            let zoxide_path = &zoxide_str[..zoxide_str.len() - 1];
            if !context.config_ref().zoxide_update {
                zoxide_add(zoxide_path)?;
            }

            let path = Path::new(zoxide_path);
            context
                .message_queue_mut()
                .push_info(format!("zi {:?}", zoxide_path));
            change_directory::change_directory(context, path)?;
        }
    } else if let Ok(zoxide_str) = std::str::from_utf8(&zoxide_output.stderr) {
        context
            .message_queue_mut()
            .push_error(zoxide_str.to_string());
    }
    Ok(())
}

pub fn zoxide_add(s: &str) -> io::Result<()> {
    Command::new("zoxide").arg("add").arg(s).output()?;
    Ok(())
}
