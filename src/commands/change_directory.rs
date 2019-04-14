use std::path;

use crate::commands;

use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::preview;
use crate::ui;
use crate::window::JoshutoView;
use commands::{JoshutoCommand, JoshutoRunnable};

#[derive(Clone, Debug)]
pub struct ChangeDirectory {
    path: path::PathBuf,
}

impl ChangeDirectory {
    pub fn new(path: path::PathBuf) -> Self {
        ChangeDirectory { path }
    }
    pub const fn command() -> &'static str {
        "cd"
    }

    pub fn change_directory(
        path: &path::PathBuf,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        let curr_tab = &mut context.tabs[context.curr_tab_index];

        match std::env::set_current_dir(path.as_path()) {
            Ok(_) => curr_tab.curr_path = path.clone(),
            Err(e) => {
                return Err(JoshutoError::IO(e));
            }
        }

        let parent_list = curr_tab.parent_list.take();
        curr_tab.history.put_back(parent_list);
        let curr_list = curr_tab.curr_list.take();
        curr_tab.history.put_back(curr_list);

        curr_tab
            .history
            .populate_to_root(&curr_tab.curr_path, &context.config_t.sort_option);

        curr_tab.curr_list = match curr_tab
            .history
            .pop_or_create(&curr_tab.curr_path, &context.config_t.sort_option)
        {
            Ok(s) => Some(s),
            Err(e) => {
                ui::wprint_err(&view.bot_win, e.to_string().as_str());
                None
            }
        };

        if let Some(parent) = curr_tab.curr_path.parent() {
            curr_tab.parent_list = match curr_tab
                .history
                .pop_or_create(&parent, &context.config_t.sort_option)
            {
                Ok(s) => Some(s),
                Err(e) => {
                    ui::wprint_err(&view.bot_win, e.to_string().as_str());
                    None
                }
            };
        }

        curr_tab.refresh(
            view,
            &context.config_t,
            &context.username,
            &context.hostname,
        );
        Ok(())
    }
}

impl JoshutoCommand for ChangeDirectory {}

impl std::fmt::Display for ChangeDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", Self::command(), self.path.to_str().unwrap())
    }
}

impl JoshutoRunnable for ChangeDirectory {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        let res = Self::change_directory(&self.path, context, view);
        preview::preview_file(
            &mut context.tabs[context.curr_tab_index],
            &view,
            &context.config_t,
        );
        ncurses::doupdate();
        res
    }
}
