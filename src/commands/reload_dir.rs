use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::window::JoshutoView;

#[derive(Clone, Debug)]
pub struct ReloadDirList;

impl ReloadDirList {
    pub fn new() -> Self {
        ReloadDirList
    }
    pub const fn command() -> &'static str {
        "reload_dir_list"
    }

    pub fn reload(context: &mut JoshutoContext, view: &JoshutoView) -> Result<(), std::io::Error> {
        let curr_tab = &mut context.tabs[context.curr_tab_index];
        let dir_len = curr_tab.curr_list.contents.len();
        match curr_tab.curr_list.index {
            None => {}
            Some(s) => {
                curr_tab.curr_list.pagestate.update_page_state(
                    s,
                    view.mid_win.rows,
                    dir_len,
                    context.config_t.scroll_offset,
                );
                curr_tab.reload_contents(&context.config_t.sort_option)?;
                curr_tab.refresh(view, &context.config_t);
            }
        }
        Ok(())
    }
}

impl JoshutoCommand for ReloadDirList {}

impl std::fmt::Display for ReloadDirList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for ReloadDirList {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        match Self::reload(context, view) {
            Ok(_) => {
                ncurses::doupdate();
                Ok(())
            }
            Err(e) => Err(JoshutoError::IO(e)),
        }
    }
}
