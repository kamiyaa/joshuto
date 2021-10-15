use std::io;
use std::path;
use std::thread;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::fs::JoshutoDirList;
use crate::history::DirectoryHistory;

#[allow(dead_code)]
pub struct Foreground {}

impl Foreground {
    pub fn load_preview(context: &mut AppContext, p: path::PathBuf) -> io::Result<()> {
        let options = context.config_ref().display_options_ref().clone();
        let history = context.tab_context_mut().curr_tab_mut().history_mut();
        if history
            .create_or_soft_update(p.as_path(), &options)
            .is_err()
        {
            history.remove(p.as_path());
        }
        Ok(())
    }
}

pub struct Background {}

impl Background {
    pub fn load_preview(context: &mut AppContext, p: path::PathBuf) -> thread::JoinHandle<()> {
        let event_tx = context.events.event_tx.clone();
        let options = context.config_ref().display_options_ref().clone();
        let handle = thread::spawn(move || match JoshutoDirList::from_path(p, &options) {
            Ok(dirlist) => {
                let _ = event_tx.send(AppEvent::PreviewDir(Ok(dirlist)));
            }
            Err(_) => {}
        });
        handle
    }
}
