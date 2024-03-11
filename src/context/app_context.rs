use std::collections::HashSet;
use std::sync::mpsc;

use crate::commands::quit::QuitAction;
use crate::config::clean::app::AppConfig;
use crate::config::raw::app::display::preview::PreviewProtocol;
use crate::context::{
    CommandLineContext, LocalStateContext, MatchContext, MessageQueue, PreviewContext, TabContext,
    UiContext, WorkerContext,
};
use crate::event::{AppEvent, Events};
use crate::preview::preview_file::PreviewFileState;
use crate::ui::AppBackend;
use crate::Args;
use notify::{RecursiveMode, Watcher};
use ratatui_image::picker::Picker;
use std::path;

pub struct AppContext {
    pub quit: QuitAction,
    // event loop querying
    pub events: Events,
    // args from the command line
    pub args: Args,
    // app config
    config: AppConfig,
    // context related to tabs
    tab_context: TabContext,
    // context related to local file state
    local_state: Option<LocalStateContext>,
    // context related to searching
    search_context: Option<MatchContext>,
    // message queue for displaying messages
    message_queue: MessageQueue,
    // context related to io workers
    worker_context: WorkerContext,
    // context related to previews
    pub preview_context: PreviewContext,
    // context related to command line
    commandline_context: CommandLineContext,
    // user interface context; data which is input to both, the UI rendering and the app state
    ui_context: UiContext,
    // filesystem watcher to inform about changes in shown directories
    watcher: notify::RecommendedWatcher,
    // list of watched paths; seems not to be possible to get them from a notify::Watcher
    watched_paths: HashSet<path::PathBuf>,
    // the stdout of the last `shell` command
    pub last_stdout: Option<String>,
}

impl AppContext {
    pub fn new(config: AppConfig, args: Args) -> Self {
        let picker = if config
            .preview_options_ref()
            .preview_shown_hook_script
            .is_none()
        {
            Picker::from_termios().ok().and_then(|mut picker| {
                match config.preview_options_ref().preview_protocol {
                    PreviewProtocol::Auto => {
                        picker.guess_protocol(); // Must run before Events::new() because it makes ioctl calls.
                        Some(picker)
                    }
                    PreviewProtocol::Disabled => None,
                    PreviewProtocol::ProtocolType(protocol_type) => {
                        picker.protocol_type = protocol_type;
                        Some(picker)
                    }
                }
            })
        } else {
            None
        };

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

        let preview_script = config.preview_options_ref().preview_script.clone();

        Self {
            quit: QuitAction::DoNot,
            events,
            config,
            args,
            tab_context: TabContext::new(),
            local_state: None,
            search_context: None,
            message_queue: MessageQueue::new(),
            worker_context: WorkerContext::new(event_tx.clone()),
            preview_context: PreviewContext::new(picker, preview_script, event_tx),
            ui_context: UiContext { layout: vec![] },
            commandline_context,
            watcher,
            watched_paths,
            last_stdout: None,
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

    pub fn config_ref(&self) -> &AppConfig {
        &self.config
    }
    pub fn config_mut(&mut self) -> &mut AppConfig {
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

    pub fn get_search_context(&self) -> Option<&MatchContext> {
        self.search_context.as_ref()
    }
    pub fn set_search_context(&mut self, context: MatchContext) {
        self.search_context = Some(context);
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
    pub fn load_preview(&mut self, backend: &AppBackend, path: path::PathBuf) {
        // always load image without cache
        self.preview_context_mut().set_image_preview(None);
        self.preview_context
            .load_preview_image(self, backend, path.clone());

        let previews = self.preview_context_mut().previews_mut();
        if previews.get(path.as_path()).is_none() {
            // add to loading state
            previews.insert(path.clone(), PreviewFileState::Loading);
            self.preview_context
                .load_preview_script(self, backend, path);
        }
    }
}
