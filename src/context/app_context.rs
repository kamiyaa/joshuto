use std::collections::HashSet;
use std::sync::mpsc;

use crate::config;
use crate::context::{
    CommandLineContext, LocalStateContext, MessageQueue, PreviewContext, TabContext, WorkerContext,
};
use crate::event::{AppEvent, Events};
use crate::util::search::SearchPattern;
use crate::Args;
use notify::{RecursiveMode, Watcher};
use std::path;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QuitType {
    DoNot,
    Normal,
    Force,
    ToCurrentDirectory,
    ChooseFiles,
}

pub struct AppContext {
    pub quit: QuitType,
    // event loop querying
    pub events: Events,
    // args from the command line
    pub args: Args,
    // app config
    config: config::AppConfig,
    // context related to tabs
    tab_context: TabContext,
    // context related to local file state
    local_state: Option<LocalStateContext>,
    // context related to searching
    search_context: Option<SearchPattern>,
    // message queue for displaying messages
    message_queue: MessageQueue,
    // context related to io workers
    worker_context: WorkerContext,
    // context related to previews
    preview_context: PreviewContext,
    // context related to command line
    commandline_context: CommandLineContext,
    // filesystem watcher to inform about changes in shown directories
    #[cfg(target_os = "linux")]
    watcher: notify::INotifyWatcher,
    #[cfg(target_os = "macos")]
    watcher: notify::FsEventWatcher,
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    watcher: notify::NullWatcher,
    // list of watched paths; seems not to be possible to get them from a notify::Watcher
    watched_paths: HashSet<path::PathBuf>,
}

impl AppContext {
    pub fn new(config: config::AppConfig, args: Args) -> Self {
        let events = Events::new();
        let event_tx = events.event_tx.clone();

        let mut commandline_context = CommandLineContext::new();
        commandline_context.history_mut().set_max_len(20);

        let event_tx_for_fs_notification = event_tx.clone();
        let watcher = notify::recommended_watcher(move |res| match res {
            Ok(event) => {
                let _ = event_tx_for_fs_notification.send(AppEvent::Filesystem(event));
            }
            Err(_) => {}
        })
        .unwrap();
        let watched_paths = HashSet::with_capacity(3);

        Self {
            quit: QuitType::DoNot,
            events,
            args,
            tab_context: TabContext::new(),
            local_state: None,
            search_context: None,
            message_queue: MessageQueue::new(),
            worker_context: WorkerContext::new(event_tx),
            preview_context: PreviewContext::new(),
            commandline_context,
            config,
            watcher,
            watched_paths,
        }
    }

    pub fn update_watcher(&mut self) {
        // collect the paths that shall be watched...
        let mut new_paths_to_watch: HashSet<path::PathBuf> = HashSet::with_capacity(3);
        if let Some(dir_list) = self.tab_context_ref().curr_tab_ref().curr_list_ref() {
            new_paths_to_watch.insert(dir_list.file_path().to_path_buf());
        }
        if let Some(dir_list) = self.tab_context_ref().curr_tab_ref().parent_list_ref() {
            new_paths_to_watch.insert(dir_list.file_path().to_path_buf());
        }
        if let Some(dir_list) = self.tab_context_ref().curr_tab_ref().child_list_ref() {
            new_paths_to_watch.insert(dir_list.file_path().to_path_buf());
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

    // event related
    pub fn poll_event(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.events.next()
    }
    pub fn flush_event(&self) {
        self.events.flush();
    }
    pub fn clone_event_tx(&self) -> mpsc::Sender<AppEvent> {
        self.events.event_tx.clone()
    }

    pub fn config_ref(&self) -> &config::AppConfig {
        &self.config
    }
    pub fn config_mut(&mut self) -> &mut config::AppConfig {
        &mut self.config
    }

    pub fn tab_context_ref(&self) -> &TabContext {
        &self.tab_context
    }
    pub fn tab_context_mut(&mut self) -> &mut TabContext {
        &mut self.tab_context
    }

    pub fn message_queue_ref(&self) -> &MessageQueue {
        &self.message_queue
    }
    pub fn message_queue_mut(&mut self) -> &mut MessageQueue {
        &mut self.message_queue
    }

    // local state related
    pub fn set_local_state(&mut self, state: LocalStateContext) {
        self.local_state = Some(state);
    }
    pub fn take_local_state(&mut self) -> Option<LocalStateContext> {
        self.local_state.take()
    }

    pub fn get_search_context(&self) -> Option<&SearchPattern> {
        self.search_context.as_ref()
    }
    pub fn set_search_context(&mut self, pattern: SearchPattern) {
        self.search_context = Some(pattern);
    }

    pub fn preview_context_ref(&self) -> &PreviewContext {
        &self.preview_context
    }
    pub fn preview_context_mut(&mut self) -> &mut PreviewContext {
        &mut self.preview_context
    }

    pub fn worker_context_ref(&self) -> &WorkerContext {
        &self.worker_context
    }
    pub fn worker_context_mut(&mut self) -> &mut WorkerContext {
        &mut self.worker_context
    }

    pub fn commandline_context_ref(&self) -> &CommandLineContext {
        &self.commandline_context
    }

    pub fn commandline_context_mut(&mut self) -> &mut CommandLineContext {
        &mut self.commandline_context
    }
}
