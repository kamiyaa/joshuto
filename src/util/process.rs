use std::io::{self, Write};
use std::process;
use std::sync::mpsc;
use std::thread;

use crate::config::clean::mimetype::ProgramEntry;
use crate::event::AppEvent;

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

    let _ = command.status()?;

    if entry.get_confirm_exit() {
        wait_for_enter()?;
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
