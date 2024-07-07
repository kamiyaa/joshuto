use std::path;
use std::thread;

use crate::fs::JoshutoDirList;
use crate::types::event::AppEvent;
use crate::types::state::AppState;

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
    pub fn load_preview(
        app_state: &mut AppState,
        dir_path: path::PathBuf,
    ) -> thread::JoinHandle<()> {
        let event_tx = app_state.events.event_tx.clone();
        let options = app_state.config.display_options.clone();
        let tab_options = app_state
            .state
            .tab_state_ref()
            .curr_tab_ref()
            .option_ref()
            .clone();
        let tab_id = app_state.state.tab_state_ref().curr_tab_id();

        // add to loading state
        app_state
            .state
            .tab_state_mut()
            .curr_tab_mut()
            .history_metadata_mut()
            .insert(dir_path.clone(), PreviewDirState::Loading);

        thread::spawn(move || {
            let path_clone = dir_path.clone();
            let dir_res = JoshutoDirList::from_path(dir_path, &options, &tab_options);
            let res = AppEvent::PreviewDir {
                id: tab_id,
                path: path_clone,
                res: Box::new(dir_res),
            };
            let _ = event_tx.send(res);
        })
    }
}
