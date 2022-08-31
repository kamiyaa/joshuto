use std::path;
use std::thread;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::fs::JoshutoDirList;
use crate::preview::preview_default::PreviewState;

pub struct Background {}

impl Background {
    pub fn load_preview(context: &mut AppContext, p: path::PathBuf) -> thread::JoinHandle<()> {
        let event_tx = context.events.event_tx.clone();
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
            .insert(p.clone(), PreviewState::Loading);

        thread::spawn(move || {
            let path_clone = p.clone();
            let dir_res = JoshutoDirList::from_path(p, &options, &tab_options);
            let res = AppEvent::PreviewDir {
                id: tab_id,
                path: path_clone,
                res: Box::new(dir_res),
            };
            let _ = event_tx.send(res);
        })
    }
}
