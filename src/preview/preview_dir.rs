use std::path;
use std::thread;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::fs::JoshutoDirList;

#[derive(Debug, Clone)]
pub enum PreviewDirState {
    Loading,
    Error { message: String },
}

impl PreviewDirState {
    pub fn is_loading(&self) -> bool {
        matches!(*self, Self::Loading)
    }
}

pub struct Background {}

impl Background {
    pub fn load_preview(context: &mut AppContext, p: path::PathBuf) -> thread::JoinHandle<()> {
        let event_tx = context.events.event_tx.clone();
        let config = context.config_ref().clone();
        let options = context.config_ref().display_options_ref().clone();
        let tab_options = context
            .tab_context_ref()
            .curr_tab_ref()
            .option_ref()
            .clone();
        let tab_id = context.tab_context_ref().curr_tab_id();

        // add to loading state
        context
            .tab_context_mut()
            .curr_tab_mut()
            .history_metadata_mut()
            .insert(p.clone(), PreviewDirState::Loading);

        thread::spawn(move || {
            let path_clone = p.clone();
            let dir_res = JoshutoDirList::from_path(p, &config, &options, &tab_options);
            let res = AppEvent::PreviewDir {
                id: tab_id,
                path: path_clone,
                res: Box::new(dir_res),
            };
            let _ = event_tx.send(res);
        })
    }
}
