use std::process;
use std::sync::mpsc;
use std::thread;

use crate::config::AppMimetypeEntry;
use crate::event::AppEvent;

pub fn fork_execute<I, S>(
    entry: &AppMimetypeEntry,
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

pub fn execute_and_wait<I, S>(entry: &AppMimetypeEntry, paths: I) -> std::io::Result<()>
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

    command.args(entry.get_args());
    command.args(paths);

    let _ = command.status()?;
    Ok(())
}
