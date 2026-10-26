use std::thread;
use std::{io, path};
use std::time::SystemTime;

use crate::config::{DisplayOption, TabDisplayOption};
use crate::fs::AppDirectoryList;
use crate::state::AppState;
use crate::tasks::AppEvent;

/// Status of a directory preview being generated on a background thread.
#[derive(Debug, Clone)]
pub enum PreviewDirState {
    Loading,
    Error { message: String },
}

/// Namespace for spawning background directory-preview loads.
pub struct Background {}

impl Background {
    /// Spawns a background thread that reads `dir_path` and posts an [`AppEvent::PreviewDir`]
    /// with the result. If the current tab already has a listing for `dir_path` that isn't
    /// marked stale, the thread first checks the directory's mtime and skips the re-read when
    /// it hasn't changed. Even that check stays off the main thread, since a single `stat` can
    /// block for seconds on a spun-down disk.
    pub fn refresh(app_state: &mut AppState, dir_path: path::PathBuf) -> thread::JoinHandle<()> {
        let cached_modified = app_state
            .state
            .tab_state_ref()
            .curr_tab_ref()
            .history_ref()
            .get(dir_path.as_path())
            .filter(|list| !list.need_update)
            .map(|list| list.metadata.modified());
        Self::spawn(app_state, dir_path, cached_modified)
    }

    fn spawn(
        app_state: &mut AppState,
        dir_path: path::PathBuf,
        cached_modified: Option<SystemTime>,
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
            let dir_res = read_if_changed(dir_path, cached_modified, &options, &tab_options);
            let res = AppEvent::PreviewDir {
                id: tab_id,
                path: path_clone,
                res: Box::new(dir_res),
            };
            let _ = event_tx.send(res);
        })
    }
}

/// Reads `dir_path`, unless `cached_modified` is given and the directory hasn't been modified
/// since then, in which case returns `Ok(None)`.
fn read_if_changed(
    dir_path: path::PathBuf,
    cached_modified: Option<SystemTime>,
    options: &DisplayOption,
    tab_options: &TabDisplayOption,
) -> io::Result<Option<AppDirectoryList>> {
    let unchanged = cached_modified.is_some_and(|cached| {
        // follow symlinks, like `AppFileMetadata::from` does for the cached listing;
        // if the stat fails, keep showing the cached listing
        std::fs::metadata(&dir_path)
            .and_then(|m| m.modified())
            .map(|m| m <= cached)
            .unwrap_or(true)
    });
    if unchanged {
        return Ok(None);
    }
    AppDirectoryList::from_path(dir_path, options, tab_options).map(Some)
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use crate::config::{DisplayOption, TabDisplayOption};

    use super::read_if_changed;

    fn temp_dir() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("joshuto-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&dir).unwrap();
        std::fs::write(dir.join("a"), "").unwrap();
        dir
    }

    #[test]
    fn unchanged_directory_is_not_reread() {
        let dir = temp_dir();
        let far_future = SystemTime::now() + Duration::from_secs(3600);
        let res = read_if_changed(
            dir.clone(),
            Some(far_future),
            &DisplayOption::default(),
            &TabDisplayOption::default(),
        );
        assert!(matches!(res, Ok(None)));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn changed_or_uncached_directory_is_read() {
        let dir = temp_dir();
        for cached in [None, Some(SystemTime::UNIX_EPOCH)] {
            let res = read_if_changed(
                dir.clone(),
                cached,
                &DisplayOption::default(),
                &TabDisplayOption::default(),
            );
            assert_eq!(res.unwrap().unwrap().contents.len(), 1);
        }
        std::fs::remove_dir_all(dir).unwrap();
    }
}
