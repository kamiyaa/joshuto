use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use crate::context::JoshutoContext;
use crate::io::{IOWorkerThread, Options};
use crate::util::event::Event;

use super::local_state::LocalState;
use super::name_resolution::rename_filename_conflict;

pub fn recursive_copy(dest: &Path, src: &Path, options: &Options) -> std::io::Result<u64> {
    let mut dest_buf = dest.to_path_buf();
    if let Some(s) = src.file_name() {
        dest_buf.push(s);
    }
    rename_filename_conflict(&mut dest_buf);
    if !src.is_dir() {
        std::fs::copy(src, dest_buf)
    } else {
        fs::create_dir(dest_buf.as_path())?;
        let mut total = 0;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            total += recursive_copy(dest_buf.as_path(), entry_path.as_path(), options)?;
        }
        Ok(total)
    }
}

pub fn paste_copy(
    context: &mut JoshutoContext,
    options: Options,
) -> std::io::Result<IOWorkerThread> {
    let paths = LocalState::take_selected_files()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "no files selected"))?;
    if paths.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "no files selected",
        ));
    }

    let tab_dest = context.curr_tab_index;
    let dest = context.tabs[tab_dest].curr_path.clone();

    let (tx_start, rx_start) = mpsc::channel();
    let (tx, rx) = mpsc::channel();
    let handle: thread::JoinHandle<std::io::Result<u64>> = thread::spawn(move || {
        let mut total = 0;
        rx_start.recv();
        for path in paths {
            total += recursive_copy(dest.as_path(), path.as_path(), &options)?;
            tx.send(Event::IOWorkerProgress(total));
        }
        Ok(total)
    });

    let thread = IOWorkerThread {
        handle,
        tx_start,
        rx,
    };

    Ok(thread)
}
