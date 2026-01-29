use std::io::{self, Write};
use std::process;
use std::sync::mpsc;
use std::thread;

use crate::types::event::AppEvent;
use crate::types::mimetype::ProgramEntry;
use crate::utils::format::clear_screen;

pub fn fork_execute<I, S>(
    entry: &ProgramEntry,
    paths: I,
    event_tx: mpsc::Sender<AppEvent>,
) -> std::io::Result<(u32, thread::JoinHandle<()>)>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let program = String::from(entry.get_command());

    let mut command = process::Command::new(program);
    if entry.get_silent() {
        command.stdout(process::Stdio::null());
        command.stderr(process::Stdio::null());
    }

    let pwd = std::env::current_dir()?;
    command.env("PWD", pwd);
    command.args(entry.get_args());
    command.args(paths);

    let mut child = command.spawn()?;
    let child_id = child.id();

    let handle = thread::spawn(move || {
        let child_id = child.id();
        let _ = child.wait();
        let _ = event_tx.send(AppEvent::ChildProcessComplete(child_id));
    });

    Ok((child_id, handle))
}

pub fn execute_and_wait<I, S>(entry: &ProgramEntry, paths: I) -> std::io::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let program = String::from(entry.get_command());

    let mut command = process::Command::new(program);
    if entry.get_silent() {
        command.stdout(process::Stdio::null());
        command.stderr(process::Stdio::null());
    }

    let pwd = std::env::current_dir()?;
    command.env("PWD", pwd);
    command.args(entry.get_args());
    command.args(paths);

    if entry.get_pager() {
        clear_screen();
        let pager_env = std::env::var("PAGER").unwrap_or_else(|_| String::from("tail"));
        let pager_args: Vec<&str> = pager_env.split_whitespace().collect();

        if let Some(child_stdout) = command
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::piped())
            .spawn()?
            .stdout
        {
            process::Command::new(pager_args[0])
                .args(&pager_args[1..])
                .stdin(child_stdout)
                .status()?;
        }
        command.status()?;
    } else {
        let _ = command.status()?;
        if entry.get_confirm_exit() {
            wait_for_enter()?;
        }
    }
    Ok(())
}

pub fn wait_for_enter() -> io::Result<()> {
    print!("===============\nPress ENTER to continue... ");
    std::io::stdout().flush()?;

    let mut user_input = String::with_capacity(4);
    std::io::stdin().read_line(&mut user_input)?;
    Ok(())
}
