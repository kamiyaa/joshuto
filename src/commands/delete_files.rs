use std::fs;
use std::path;

use termion::event::Key;

use crate::commands::{JoshutoCommand, JoshutoRunnable, ReloadDirList};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use crate::ui::widgets::TuiPrompt;
use crate::ui::TuiBackend;
use crate::util::load_child::LoadChild;

#[derive(Clone, Debug)]
pub struct DeleteFiles;

impl DeleteFiles {
    pub fn new() -> Self {
        DeleteFiles
    }
    pub const fn command() -> &'static str {
        "delete_files"
    }

    pub fn remove_files(paths: &[&path::PathBuf]) -> std::io::Result<()> {
        for path in paths {
            if let Ok(metadata) = fs::symlink_metadata(path) {
                if metadata.is_dir() {
                    fs::remove_dir_all(&path)?;
                } else {
                    fs::remove_file(&path)?;
                }
            }
        }
        Ok(())
    }

    fn delete_files(context: &mut JoshutoContext, backend: &mut TuiBackend) -> std::io::Result<()> {
        let curr_tab = &context.tabs[context.curr_tab_index];
        let paths = match curr_tab.curr_list_ref() {
            Some(s) => s.get_selected_paths(),
            None => Vec::new(),
        };
        let paths_len = paths.len();

        if paths_len == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "no files selected",
            ));
        }

        let ch = {
            let prompt_str = format!("Delete {} files? (Y/n)", paths_len);
            let mut prompt = TuiPrompt::new(&prompt_str);
            prompt.get_key(backend, &context)
        };

        if ch == Key::Char('y') || ch == Key::Char('\n') {
            if paths_len > 1 {
                let ch = {
                    let prompt_str = "Are you sure? (y/N)";
                    let mut prompt = TuiPrompt::new(prompt_str);
                    prompt.get_key(backend, &context)
                };
                if ch == Key::Char('y') {
                    Self::remove_files(&paths)?;
                    ReloadDirList::reload(context.curr_tab_index, context)?;
                    let msg = format!("Deleted {} files", paths_len);
                    context.message_queue.push_back(msg);
                }
            } else {
                Self::remove_files(&paths)?;
                ReloadDirList::reload(context.curr_tab_index, context)?;
                let msg = format!("Deleted {} files", paths_len);
                context.message_queue.push_back(msg);
            }
        }
        Ok(())
    }
}

impl JoshutoCommand for DeleteFiles {}

impl std::fmt::Display for DeleteFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for DeleteFiles {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        Self::delete_files(context, backend)?;

        let options = &context.config_t.sort_option;
        let curr_path = context.tabs[context.curr_tab_index].curr_path.clone();
        for tab in context.tabs.iter_mut() {
            tab.history.reload(&curr_path, options)?;
        }
        LoadChild::load_child(context)?;
        Ok(())
    }
}
