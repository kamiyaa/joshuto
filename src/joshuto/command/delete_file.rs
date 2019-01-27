extern crate ncurses;

use std::path;
use std::fs;

use joshuto::command;
use joshuto::command::JoshutoCommand;
use joshuto::command::JoshutoRunnable;
use joshuto::config::keymap;
use joshuto::context::JoshutoContext;
use joshuto::preview;
use joshuto::ui;

#[derive(Clone, Debug)]
pub struct DeleteFiles;

impl DeleteFiles {
    pub fn new() -> Self { DeleteFiles }
    pub const fn command() -> &'static str { "delete_files" }

    pub fn remove_files(paths: Vec<path::PathBuf>)
    {
        for path in &paths {
            if let Ok(metadata) = fs::symlink_metadata(path) {
                if metadata.is_dir() {
                    fs::remove_dir_all(&path).unwrap();
                } else {
                    fs::remove_file(&path).unwrap();
                }
            }
        }
    }
}

impl JoshutoCommand for DeleteFiles {}

impl std::fmt::Display for DeleteFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for DeleteFiles {
    fn execute(&self, context: &mut JoshutoContext)
    {
        ui::wprint_msg(&context.views.bot_win, "Delete selected files? (Y/n)");
        ncurses::timeout(-1);
        ncurses::doupdate();

        let ch: i32 = ncurses::getch();
        if ch == 'y' as i32 || ch == keymap::ENTER as i32 {
            if let Some(s) = context.tabs[context.curr_tab_index].curr_list.as_ref() {
                if let Some(paths) = command::collect_selected_paths(s) {
                    Self::remove_files(paths);
                }
            }
            ui::wprint_msg(&context.views.bot_win, "Deleted files");

            let curr_tab = &mut context.tabs[context.curr_tab_index];
            curr_tab.reload_contents(&context.config_t.sort_type);
            curr_tab.refresh(&context.views, &context.config_t,
                &context.username, &context.hostname);
        } else {
            let curr_tab = &context.tabs[context.curr_tab_index];
            curr_tab.refresh_file_status(&context.views.bot_win);
            curr_tab.refresh_path_status(&context.views.top_win,
                    &context.username, &context.hostname);
        }
        ncurses::doupdate();
    }
}
