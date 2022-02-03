use std::path;
use std::thread;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::fs::JoshutoDirList;

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
