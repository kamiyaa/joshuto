use std::fs;
use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable, ReloadDirList};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
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

    pub fn remove_files(paths: Vec<path::PathBuf>) -> Result<(), std::io::Error> {
        for path in &paths {
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

    fn delete_files(
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), std::io::Error> {
        ui::wprint_msg(&view.bot_win, "Delete selected files? (Y/n)");
        ncurses::timeout(-1);
        ncurses::doupdate();

        let curr_tab = &mut context.tabs[context.curr_tab_index];
        let mut ch = ncurses::getch();
        if ch == 'y' as i32 || ch == KEYMAP_T.enter {
            if let Some(paths) = curr_tab.curr_list.get_selected_paths() {
                if paths.len() > 1 {
                    ui::wprint_msg(&view.bot_win, "Are you sure? (y/N)");
                    ncurses::doupdate();
                    ch = ncurses::getch();
                } else {
                    ch = 'y' as i32;
                }
                if ch == 'y' as i32 {
                    Self::remove_files(paths)?;
                    ui::wprint_msg(&view.bot_win, "Deleted files");
                    ReloadDirList::reload(context.curr_tab_index, context, view)?;
                }
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
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        let res = Self::delete_files(context, view);
        let curr_tab = &mut context.tabs[context.curr_tab_index];
        curr_tab.refresh(view, &context.config_t);
        ncurses::doupdate();
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(JoshutoError::IO(e)),
        }
    }
}
