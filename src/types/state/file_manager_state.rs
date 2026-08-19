use std::collections::HashSet;
use std::path;

use notify::{RecursiveMode, Watcher};

use crate::types::state::{
    CommandLineState, LocalStateState, MatchState, MessageQueue, PreviewState, TabState, UiState,
    WorkerState,
};

use super::ThreadPool;

/// State related to the file-manager's core function: tabs, previews, background workers, and
/// filesystem watching. Distinct from [`UiState`], which holds only rendering-layout input.
pub struct FileManagerState {
    /// app_state related to tabs
    pub tab_state: TabState,
    /// app_state related to local file state
    pub local_state: Option<LocalStateState>,
    /// app_state related to searching
    pub search_state: Option<MatchState>,
    /// message queue for displaying messages
    pub message_queue: MessageQueue,
    /// app_state related to io workers
    pub worker_state: WorkerState,
    /// thread pool of child processes
    pub thread_pool: ThreadPool,
    /// app_state related to previews
    pub preview_state: PreviewState,
    /// app_state related to command line
    pub commandline_state: CommandLineState,
    /// user interface app_state; data which is input to both, the UI rendering and the app state
    pub ui_state: UiState,
    /// filesystem watcher to inform about changes in shown directories
    pub watcher: notify::RecommendedWatcher,
    /// list of watched paths; seems not to be possible to get them from a notify::Watcher
    pub watched_paths: HashSet<path::PathBuf>,
    /// the stdout of the last `shell` command
    pub last_stdout: Option<String>,
}

impl FileManagerState {
    /// Updates the file system supervision with the currently shown directories.
    pub fn update_watcher(&mut self) {
        // collect the paths that shall be watched...
        let mut new_paths_to_watch: HashSet<path::PathBuf> = HashSet::with_capacity(3);

        let curr_tab_ref = self.tab_state_ref().curr_tab_ref();

        let watched_lists = [
            curr_tab_ref.parent_list_ref(),
            curr_tab_ref.curr_list_ref(),
            curr_tab_ref.child_list_ref(),
        ];

        for list in watched_lists.iter().flatten() {
            new_paths_to_watch.insert(list.file_path().to_path_buf());
        }

        // remove paths from watcher which don't need to be watched anymore...
        for old_watched_path in &self.watched_paths {
            if !new_paths_to_watch.contains(old_watched_path.as_path()) {
                let _ = self.watcher.unwatch(old_watched_path.as_path());
            }
        }
        // add paths to watcher which need to be watched...
        for new_watched_path in &new_paths_to_watch {
            if !self.watched_paths.contains(new_watched_path.as_path()) {
                let _ = self
                    .watcher
                    .watch(new_watched_path.as_path(), RecursiveMode::NonRecursive);
            }
        }
        // update own list of watched paths
        self.watched_paths = new_paths_to_watch;
    }

    /// Returns the state of all tabs.
    pub fn tab_state_ref(&self) -> &TabState {
        &self.tab_state
    }
    /// Returns a mutable reference to the state of all tabs.
    pub fn tab_state_mut(&mut self) -> &mut TabState {
        &mut self.tab_state
    }

    /// Returns the queue of messages pending display in the UI.
    pub fn message_queue_ref(&self) -> &MessageQueue {
        &self.message_queue
    }
    /// Returns a mutable reference to the message queue.
    pub fn message_queue_mut(&mut self) -> &mut MessageQueue {
        &mut self.message_queue
    }

    // local state related
    /// Stashes `state` (e.g. a pending cut/copy selection) for later use.
    pub fn set_local_state(&mut self, state: LocalStateState) {
        self.local_state = Some(state);
    }
    /// Takes and clears the stashed local state, if any.
    pub fn take_local_state(&mut self) -> Option<LocalStateState> {
        self.local_state.take()
    }

    /// Returns the active search/filter match state, if any.
    pub fn get_search_state(&self) -> Option<&MatchState> {
        self.search_state.as_ref()
    }
    /// Sets the active search/filter match state.
    pub fn set_search_state(&mut self, app_state: MatchState) {
        self.search_state = Some(app_state);
    }

    /// Returns the current preview state.
    pub fn preview_state_ref(&self) -> &PreviewState {
        &self.preview_state
    }
    /// Returns a mutable reference to the preview state.
    pub fn preview_state_mut(&mut self) -> &mut PreviewState {
        &mut self.preview_state
    }

    /// Returns the current UI rendering-layout state.
    pub fn ui_state_ref(&self) -> &UiState {
        &self.ui_state
    }
    /// Returns a mutable reference to the UI rendering-layout state.
    pub fn ui_state_mut(&mut self) -> &mut UiState {
        &mut self.ui_state
    }

    /// Returns the state of background IO workers.
    pub fn worker_state_ref(&self) -> &WorkerState {
        &self.worker_state
    }
    /// Returns a mutable reference to the background IO worker state.
    pub fn worker_state_mut(&mut self) -> &mut WorkerState {
        &mut self.worker_state
    }

    /// Returns the command-line state.
    pub fn commandline_state_ref(&self) -> &CommandLineState {
        &self.commandline_state
    }
    /// Returns a mutable reference to the command-line state.
    pub fn commandline_state_mut(&mut self) -> &mut CommandLineState {
        &mut self.commandline_state
    }
}
