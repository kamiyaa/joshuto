extern crate fs_extra;
extern crate ncurses;

use std;
use std::fmt;
use std::path;
use std::process;

use joshuto;
use joshuto::command;
use joshuto::ui;

#[derive(Clone, Debug)]
pub struct ChangeDirectory {
    path: path::PathBuf,
}

impl ChangeDirectory {
    pub fn new(path: path::PathBuf) -> Self
    {
        ChangeDirectory {
            path,
        }
    }
    pub const fn command() -> &'static str { "cd" }
}

impl command::JoshutoCommand for ChangeDirectory {}

impl std::fmt::Display for ChangeDirectory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{} {}", Self::command(), self.path.to_str().unwrap())
    }
}

impl command::Runnable for ChangeDirectory {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        if !self.path.exists() {
            ui::wprint_err(&context.views.bot_win, "Error: No such file or directory");
            ncurses::doupdate();
            return;
        }
        let curr_tab = &mut context.tabs[context.tab_index];

        match std::env::set_current_dir(self.path.as_path()) {
            Ok(_) => {
                curr_tab.curr_path = self.path.clone();
            },
            Err(e) => {
                ui::wprint_err(&context.views.bot_win, e.to_string().as_str());
                return;
            }
        }

        {
            curr_tab.history.populate_to_root(&curr_tab.curr_path, &context.config_t.sort_type);

            let parent_list = curr_tab.parent_list.take();
            curr_tab.history.put_back(parent_list);

            let curr_list = curr_tab.curr_list.take();
            curr_tab.history.put_back(curr_list);
        }

        curr_tab.curr_list = match curr_tab.history.pop_or_create(&curr_tab.curr_path,
                    &context.config_t.sort_type) {
            Ok(s) => {
                Some(s)
            },
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            },
        };

        if let Some(parent) = curr_tab.curr_path.parent() {
            curr_tab.parent_list = match curr_tab.history.pop_or_create(&parent, &context.config_t.sort_type) {
                Ok(s) => { Some(s) },
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                },
            };
        }

        ui::redraw_view(&context.config_t, &context.theme_t,
                &context.views.left_win, curr_tab.parent_list.as_mut());
        ui::redraw_view_detailed(&context.config_t, &context.theme_t,
                &context.views.mid_win, curr_tab.curr_list.as_mut());

        ui::redraw_status(&context.theme_t, &context.views,
                curr_tab.curr_list.as_ref(),
                &curr_tab.curr_path,
                &context.username, &context.hostname);

        ncurses::doupdate();
    }
}
