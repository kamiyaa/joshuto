use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::preview;
use crate::window::JoshutoView;

pub struct CursorMove;

impl CursorMove {
    pub fn cursor_move(mut new_index: usize, context: &mut JoshutoContext, view: &JoshutoView) {
        let curr_tab = &mut context.tabs[context.curr_tab_index];

        if let Some(curr_list) = curr_tab.curr_list.as_mut() {
            match curr_list.index {
                None => {}
                Some(index) => {
                    let dir_len = curr_list.contents.len();
                    if new_index >= dir_len {
                        new_index = dir_len - 1;
                        if index == dir_len - 1 {
                            return;
                        }
                    }
                    curr_list.index = Some(new_index);
                }
            }
        }
        curr_tab.refresh_curr(&view.mid_win, context.config_t.scroll_offset);
        curr_tab.refresh_file_status(&view.bot_win);
        curr_tab.refresh_path_status(
            &view.top_win,
            &context.username,
            &context.hostname,
            context.config_t.tilde_in_titlebar,
        );
        preview::preview_file(curr_tab, &view, &context.config_t);
        ncurses::doupdate();
    }
}

#[derive(Clone, Debug)]
pub struct CursorMoveInc {
    movement: usize,
}

impl CursorMoveInc {
    pub fn new(movement: usize) -> Self {
        CursorMoveInc { movement }
    }
    pub const fn command() -> &'static str {
        "cursor_move_increment"
    }
}

impl JoshutoCommand for CursorMoveInc {}

impl std::fmt::Display for CursorMoveInc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", Self::command(), self.movement)
    }
}

impl JoshutoRunnable for CursorMoveInc {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        let mut movement: Option<usize> = None;
        {
            let curr_tab = context.curr_tab_mut();
            if let Some(curr_list) = curr_tab.curr_list.as_ref() {
                movement = curr_list.index.map(|x| x + self.movement);
            }
        }
        if let Some(s) = movement {
            CursorMove::cursor_move(s, context, view)
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMoveDec {
    movement: usize,
}

impl CursorMoveDec {
    pub fn new(movement: usize) -> Self {
        CursorMoveDec { movement }
    }
    pub const fn command() -> &'static str {
        "cursor_move_increment"
    }
}

impl JoshutoCommand for CursorMoveDec {}

impl std::fmt::Display for CursorMoveDec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", Self::command(), self.movement)
    }
}

impl JoshutoRunnable for CursorMoveDec {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        let mut movement: Option<usize> = None;
        {
            let curr_tab = context.curr_tab_mut();
            if let Some(curr_list) = curr_tab.curr_list.as_ref() {
                movement = curr_list.index.map(|x| {
                    if x > self.movement {
                        x - self.movement
                    } else {
                        0
                    }
                });
            }
        }
        if let Some(s) = movement {
            CursorMove::cursor_move(s, context, view);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMovePageUp;

impl CursorMovePageUp {
    pub fn new() -> Self {
        CursorMovePageUp
    }
    pub const fn command() -> &'static str {
        "cursor_move_page_up"
    }
}

impl JoshutoCommand for CursorMovePageUp {}

impl std::fmt::Display for CursorMovePageUp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for CursorMovePageUp {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        let movement: Option<usize> = {
            let curr_tab = context.curr_tab_mut();
            if let Some(curr_list) = curr_tab.curr_list.as_ref() {
                let half_page = view.mid_win.cols as usize / 2;
                curr_list
                    .index
                    .map(|x| if x > half_page { x - half_page } else { 0 })
            } else {
                None
            }
        };

        if let Some(s) = movement {
            CursorMove::cursor_move(s, context, view);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMovePageDown;

impl CursorMovePageDown {
    pub fn new() -> Self {
        CursorMovePageDown
    }
    pub const fn command() -> &'static str {
        "cursor_move_page_down"
    }
}

impl JoshutoCommand for CursorMovePageDown {}

impl std::fmt::Display for CursorMovePageDown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for CursorMovePageDown {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        let movement: Option<usize> = {
            let curr_tab = &mut context.tabs[context.curr_tab_index];
            if let Some(curr_list) = curr_tab.curr_list.as_ref() {
                let dir_len = curr_list.contents.len();
                let half_page = view.mid_win.cols as usize / 2;
                curr_list.index.map(|x| {
                    if x + half_page > dir_len - 1 {
                        dir_len - 1
                    } else {
                        x + half_page
                    }
                })
            } else {
                None
            }
        };

        if let Some(s) = movement {
            CursorMove::cursor_move(s, context, view);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMoveHome;

impl CursorMoveHome {
    pub fn new() -> Self {
        CursorMoveHome
    }
    pub const fn command() -> &'static str {
        "cursor_move_home"
    }
}

impl JoshutoCommand for CursorMoveHome {}

impl std::fmt::Display for CursorMoveHome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for CursorMoveHome {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        let movement: Option<usize> = {
            let curr_tab = context.curr_tab_mut();
            if let Some(curr_list) = curr_tab.curr_list.as_ref() {
                if curr_list.contents.len() == 0 {
                    None
                } else {
                    Some(0)
                }
            } else {
                None
            }
        };

        if let Some(s) = movement {
            CursorMove::cursor_move(s, context, view);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMoveEnd;

impl CursorMoveEnd {
    pub fn new() -> Self {
        CursorMoveEnd
    }
    pub const fn command() -> &'static str {
        "cursor_move_end"
    }
}

impl JoshutoCommand for CursorMoveEnd {}

impl std::fmt::Display for CursorMoveEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for CursorMoveEnd {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        let movement: Option<usize> = {
            let curr_tab = context.curr_tab_mut();
            if let Some(curr_list) = curr_tab.curr_list.as_ref() {
                let dir_len = curr_list.contents.len();
                Some(dir_len - 1)
            } else {
                None
            }
        };

        if let Some(s) = movement {
            CursorMove::cursor_move(s, context, view);
        }
        Ok(())
    }
}
