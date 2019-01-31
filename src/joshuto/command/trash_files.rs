extern crate ncurses;
extern crate fs_extra;

use std::path;
use std::fs;

use joshuto::command;
use joshuto::command::JoshutoCommand;
use joshuto::command::JoshutoRunnable;
use joshuto::config::keymap;
use joshuto::context::JoshutoContext;
use joshuto::preview;
use joshuto::ui;

fn get_trash_dir() -> path::PathBuf {
    xdg::BaseDirectories::new().unwrap().get_data_home().join("Trash/files")
}

#[derive(Clone, Debug)]
pub struct TrashFiles;

impl TrashFiles {
    pub fn new() -> Self { TrashFiles }
    pub const fn command() -> &'static str { "trash_files" }

    pub fn trash_files(paths: Vec<path::PathBuf>)
    {
        let trash_dir = get_trash_dir();
        fs::create_dir_all(&trash_dir);
        fs_extra::move_items(&paths, trash_dir, &fs_extra::dir::CopyOptions::new());
    }
}

impl JoshutoCommand for TrashFiles {}

impl std::fmt::Display for TrashFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for TrashFiles {
    fn execute(&self, context: &mut JoshutoContext)
    {
        ui::wprint_msg(&context.views.bot_win, "Trash selected files? (Y/n)");
        ncurses::timeout(-1);
        ncurses::doupdate();

        let ch: i32 = ncurses::getch();
        if ch == 'y' as i32 || ch == keymap::ENTER as i32 {
            if let Some(s) = context.tabs[context.curr_tab_index].curr_list.as_ref() {
                if let Some(paths) = command::collect_selected_paths(s) {
                    Self::trash_files(paths);
                }
            }
            ui::wprint_msg(&context.views.bot_win, "Trashed files");

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
