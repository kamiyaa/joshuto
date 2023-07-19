use std::collections::HashSet;
use std::process;
use std::sync::mpsc;
use std::thread;

use crate::commands::quit::QuitAction;
use crate::config;
use crate::context::{
    CommandLineContext, LocalStateContext, MessageQueue, PreviewContext, TabContext, UiContext,
    WorkerContext,
};
use crate::event::{AppEvent, Events};
use crate::ui::views;
use crate::ui::PreviewArea;
use crate::util::search::SearchPattern;
use crate::Args;
use notify::{RecursiveMode, Watcher};
use std::path;

pub struct AppContext {
    pub quit: QuitAction,
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
    // user interface context; data which is input to both, the UI rendering and the app state
    ui_context: UiContext,
    // filesystem watcher to inform about changes in shown directories
    watcher: notify::RecommendedWatcher,
    // list of watched paths; seems not to be possible to get them from a notify::Watcher
    watched_paths: HashSet<path::PathBuf>,
    // the last preview area (or None if now preview shown) to check if a preview hook script needs
    // to be called
    preview_area: Option<PreviewArea>,
}

impl AppContext {
    pub fn new(config: config::AppConfig, args: Args) -> Self {
        let events = Events::new();
        let event_tx = events.event_tx.clone();

        let mut commandline_context = CommandLineContext::new();
        let _ = commandline_context.history_mut().set_max_len(20);

        let event_tx_for_fs_notification = event_tx.clone();
        let watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                let _ = event_tx_for_fs_notification.send(AppEvent::Filesystem(event));
            }
        })
        .unwrap();
        let watched_paths = HashSet::with_capacity(3);

        Self {
            quit: QuitAction::DoNot,
            events,
            args,
            tab_context: TabContext::new(),
            local_state: None,
            search_context: None,
            message_queue: MessageQueue::new(),
            worker_context: WorkerContext::new(event_tx),
            preview_context: PreviewContext::new(),
            ui_context: UiContext { layout: vec![] },
            commandline_context,
            config,
            watcher,
            watched_paths,
            preview_area: None,
        }
    }

    /// Calls the "preview shown hook script" if it's configured.
    ///
    /// This method takes the current preview area as argument to check for both, the path of the
    /// currently previewed file and the geometry of the preview area.
    fn call_preview_shown_hook(&self, preview_area: PreviewArea) {
        let preview_options = self.config_ref().preview_options_ref();
        let preview_shown_hook_script = preview_options.preview_shown_hook_script.as_ref();
        if let Some(hook_script) = preview_shown_hook_script {
            let hook_script = hook_script.to_path_buf();
            let _ = thread::spawn(move || {
                let _ = process::Command::new(hook_script.as_path())
                    .arg(preview_area.file_preview_path.as_path())
                    .arg(preview_area.preview_area.x.to_string())
                    .arg(preview_area.preview_area.y.to_string())
                    .arg(preview_area.preview_area.width.to_string())
                    .arg(preview_area.preview_area.height.to_string())
                    .status();
            });
        }
    }

    /// Calls the "preview removed hook script" if it's configured.
    fn call_preview_removed_hook(&self) {
        let preview_options = self.config_ref().preview_options_ref();
        let preview_removed_hook_script = preview_options.preview_removed_hook_script.as_ref();
        if let Some(hook_script) = preview_removed_hook_script {
            let hook_script = hook_script.to_path_buf();
            let _ = thread::spawn(|| {
                let _ = process::Command::new(hook_script).status();
            });
        }
    }

    /// Updates the external preview to the current preview in Joshuto.
    ///
    /// The function checks if the current preview content is the same as the preview content which
    /// has been last communicated to an external preview logic with the preview hook scripts.
    /// If the preview content has changed, one of the hook scripts is called. Either the "preview
    /// shown hook", if a preview is shown in Joshuto, or the "preview removed hook", if Joshuto has
    /// changed from an entry with preview to an entry without a preview.
    ///
    /// This function shall be called each time a change of Joshuto's preview can be expected.
    /// (As of now, it's called in each cycle of the main loop.)
    pub fn update_external_preview(&mut self) {
        let layout = &self.ui_context_ref().layout;
        let new_preview_area = views::calculate_preview(self, layout[2]);
        match new_preview_area.as_ref() {
            Some(new) => {
                let should_preview = if let Some(old) = &self.preview_area {
                    new.file_preview_path != old.file_preview_path
                        || new.preview_area != old.preview_area
                } else {
                    true
                };
                if should_preview {
                    self.call_preview_shown_hook(new.clone())
                }
            }
            None => {
                if self.preview_area.is_some() {
                    self.call_preview_removed_hook()
                }
            }
        }
        self.preview_area = new_preview_area
    }

    /// Remove the external preview, if any is present.
    ///
    /// If the last preview hook script called was the "preview shown hook", this function will
    /// call the "preview removed hook" to remove any external preview.
    /// Otherwise it won't do anything.
    ///
    /// To restore the external preview, `update_external_preview` is called which will detect the
    /// difference and call the "preview shown hook" again for the current preview (if any).
    ///
    /// This function can be called if an external preview shall be temporarily removed, for example
    /// when entering the help screen.
    pub fn remove_external_preview(&mut self) {
        if self.preview_area.is_some() {
            self.call_preview_removed_hook();
            self.preview_area = None;
        }
    }

    /// Updates the file system supervision with the currently shown directories.
    pub fn update_watcher(&mut self) {
        // collect the paths that shall be watched...
        let mut new_paths_to_watch: HashSet<path::PathBuf> = HashSet::with_capacity(3);

        let curr_tab_ref = self.tab_context_ref().curr_tab_ref();

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

    pub fn ui_context_ref(&self) -> &UiContext {
        &self.ui_context
    }
    pub fn ui_context_mut(&mut self) -> &mut UiContext {
        &mut self.ui_context
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
