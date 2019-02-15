extern crate ncurses;

use joshuto::commands::JoshutoRunnable;
use joshuto::context::JoshutoContext;
use joshuto::preview;
use joshuto::ui;
use joshuto::JoshutoCommand;

#[derive(Clone, Debug)]
pub struct ParentDirectory;

impl ParentDirectory {
    pub fn new() -> Self {
        ParentDirectory
    }
    pub const fn command() -> &'static str {
        "parent_directory"
    }

    pub fn parent_directory(context: &mut JoshutoContext) {
        if context.curr_tab_mut().curr_path.pop() == false {
            return;
        }

        match std::env::set_current_dir(&context.curr_tab_ref().curr_path) {
            Ok(_) => {
                {
                    let curr_tab = &mut context.tabs[context.curr_tab_index];

                    let curr_list = curr_tab.curr_list.take();
                    curr_tab.history.put_back(curr_list);
                    let parent_list = curr_tab.parent_list.take();
                    curr_tab.curr_list = parent_list;

                    match curr_tab.curr_path.parent() {
                        Some(parent) => {
                            curr_tab.parent_list = match curr_tab
                                .history
                                .pop_or_create(&parent, &context.config_t.sort_type)
                            {
                                Ok(s) => Some(s),
                                Err(e) => {
                                    ui::wprint_err(&context.views.left_win, e.to_string().as_str());
                                    None
                                }
                            };
                        }
                        None => {
                            ncurses::werase(context.views.left_win.win);
                            ncurses::wnoutrefresh(context.views.left_win.win);
                        }
                    }
                    curr_tab.refresh(
                        &context.views,
                        &context.config_t,
                        &context.username,
                        &context.hostname,
                    );
                }
                preview::preview_file(context);
            }
            Err(e) => {
                ui::wprint_err(&context.views.bot_win, e.to_string().as_str());
            }
        };
        ncurses::doupdate();
    }
}

impl JoshutoCommand for ParentDirectory {}

impl std::fmt::Display for ParentDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for ParentDirectory {
    fn execute(&self, context: &mut JoshutoContext) {
        Self::parent_directory(context);
    }
}
