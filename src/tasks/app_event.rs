use std::fmt::Debug;
use std::io;
use std::path;
use std::sync::mpsc;

use ratatui::termion::event::Event;
use uuid::Uuid;

use crate::error::AppResult;
use crate::fs::AppDirectoryList;
use crate::tasks::io::IoTaskProgressMessage;
use crate::tasks::io::IoTaskStat;
use crate::tasks::preview::preview_data::PreviewData;

/// Sending half of the app event channel.
pub type AppEventSender = mpsc::Sender<AppEvent>;
/// Receiving half of the app event channel.
pub type AppEventReceiver = mpsc::Receiver<AppEvent>;

/// Every kind of asynchronous event the main loop can receive: terminal input, background IO
/// task updates, forked-process completion, preview results, terminal resize, and filesystem
/// changes.
#[derive(Debug)]
pub enum AppEvent {
    // User input events
    TerminalEvent(Event),

    // background IO worker events
    NewIoTask,
    IoTaskStart(IoTaskStat),
    IoTaskProgress(IoTaskProgressMessage),
    IoTaskResult(AppResult),

    // forked process events
    ChildProcessComplete(u32),

    // preview thread events
    /// A background directory read finished. `Ok(None)` means the directory hadn't changed
    /// since the cached listing was read, so it wasn't re-read.
    PreviewDir {
        id: Uuid,
        path: path::PathBuf,
        res: Box<io::Result<Option<AppDirectoryList>>>,
    },
    PreviewFile {
        path: path::PathBuf,
        res: io::Result<PreviewData>,
    },
    // terminal size change events
    Signal(i32),
    // filesystem change events
    Filesystem(notify::Event),
}
