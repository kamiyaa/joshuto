use std::collections::HashSet;
use std::path;

use notify::{RecursiveMode, Watcher};

use crate::state::{
    CommandLineState, LocalStateState, MatchState, Message, MessageQueue, PreviewState, TabState,
    UiState, WorkerState,
};

use super::AppHandlePool;

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
    /// Status of the IO worker: live progress while a task runs, then its success/error result
    /// until dismissed by user input. Shown only when `message_queue` is empty.
    pub io_worker_message: Option<Message>,
    /// app_state related to io workers
    pub worker_state: WorkerState,
    /// thread pool of child processes
    pub thread_pool: AppHandlePool,
    /// app_state related to previews
    pub preview_state: PreviewState,
    /// app_state related to command line
    pub commandline_state: CommandLineState,
    /// user interface app_state; data which is input to both, the UI rendering and the app state
    pub ui_state: UiState,
    /// filesystem watcher to inform about changes in shown directories;
    /// `None` if the platform's watcher couldn't be created
    pub watcher: Option<notify::RecommendedWatcher>,
    /// list of watched paths; seems not to be possible to get them from a notify::Watcher
    pub watched_paths: HashSet<path::PathBuf>,
    /// the stdout of the last `shell` command
    pub last_stdout: Option<String>,
}

impl FileManagerState {
    /// Updates the file system supervision with the currently shown directories.
    pub fn update_watcher(&mut self) {
        let Some(watcher) = self.watcher.as_mut() else {
            return;
        };

        // collect the paths that shall be watched...
        let mut new_paths_to_watch: HashSet<path::PathBuf> = HashSet::with_capacity(3);

        let curr_tab_ref = self.tab_state.curr_tab_ref();

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
                let _ = watcher.unwatch(old_watched_path.as_path());
            }
        }
        // add paths to watcher which need to be watched...
        for new_watched_path in &new_paths_to_watch {
            if !self.watched_paths.contains(new_watched_path.as_path()) {
                let _ = watcher.watch(new_watched_path.as_path(), RecursiveMode::NonRecursive);
            }
        }
        // update own list of watched paths
        self.watched_paths = new_paths_to_watch;
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
}
