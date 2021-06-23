use std::io;
use std::path;
use std::thread;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::fs::JoshutoDirList;
use crate::history::DirectoryHistory;

pub fn load_preview(context: &mut AppContext, p: path::PathBuf) -> io::Result<()> {
    let options = context.config_ref().display_options_ref().clone();
    context
        .tab_context_mut()
        .curr_tab_mut()
        .history_mut()
        .create_or_soft_update(p.as_path(), &options)?;
    Ok(())
}

pub fn background_load_preview(
    context: &mut AppContext,
    p: path::PathBuf,
) -> thread::JoinHandle<()> {
    let need_to_load = match context
        .tab_context_mut()
        .curr_tab_mut()
        .history_mut()
        .get(p.as_path())
    {
        Some(entry) => entry.need_update(),
        None => true,
    };
    if need_to_load {
        let event_tx = context.events.event_tx.clone();
        let options = context.config_ref().display_options_ref().clone();
        let handle = thread::spawn(move || {
            match JoshutoDirList::new(p, &options) {
                Ok(dirlist) => {
                    event_tx.send(AppEvent::PreviewDir(Ok(dirlist)));
                }
                Err(_) => {}
            }
            ()
        });
        handle
    } else {
        let handle = thread::spawn(|| ());
        handle
    }
}
