use std::fs;
use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable, ReloadDirList};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui;
use crate::window::JoshutoView;

use crate::KEYMAP_T;

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

    fn delete_files(context: &mut JoshutoContext, view: &JoshutoView) -> std::io::Result<()> {
        ui::wprint_msg(&view.bot_win, "Delete selected files? (Y/n)");
        ncurses::doupdate();

        let curr_tab = &mut context.tabs[context.curr_tab_index];
        let paths = curr_tab.curr_list.get_selected_paths();
        if paths.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "no files selected",
            ));
        }
        if paths.len() > 1 {
            ui::wprint_msg(&view.bot_win, "Are you sure? (y/N)");
        } else {
        }
        Self::remove_files(&paths)?;
        ui::wprint_msg(&view.bot_win, "Deleted files");
        ReloadDirList::reload(context.curr_tab_index, context)?;
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
    fn execute(&self, context: &mut JoshutoContext, view: &JoshutoView) -> JoshutoResult<()> {
        Self::delete_files(context, view)?;
        let curr_tab = &mut context.tabs[context.curr_tab_index];
        curr_tab.refresh_curr(&view.mid_win, &context.config_t);
        if context.config_t.show_preview {
            curr_tab.refresh_preview(&view.right_win, &context.config_t);
        }
        curr_tab.refresh_path_status(&view.top_win, &context.config_t);
        curr_tab.refresh_file_status(&view.bot_win);
        ncurses::doupdate();
        Ok(())
    }
}
