use std::path::PathBuf;

use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use crate::ui::TuiBackend;

pub fn cursor_move(new_index: usize, context: &mut JoshutoContext) {
    let mut new_index = new_index;
    let curr_tab = &mut context.tabs[context.curr_tab_index];

    let mut path: Option<PathBuf> = None;

    if let Some(curr_list) = curr_tab.curr_list_mut() {
        if let Some(_) = curr_list.index {
            let dir_len = curr_list.contents.len();
            if new_index >= dir_len {
                new_index = dir_len - 1;
            }
            curr_list.index = Some(new_index);

            let entry = &curr_list.contents[new_index];
            path = Some(entry.file_path().clone())
        }
    }

    // get preview
    if let Some(path) = path {
        if path.is_dir() {
            curr_tab
                .history
                .create_or_update(path.as_path(), &context.config_t.sort_option);
        }
    }
}

#[derive(Clone, Debug)]
pub struct CursorMoveStub {}

impl CursorMoveStub {
    pub fn new() -> Self {
        Self {}
    }
    pub const fn command() -> &'static str {
        "cursor_move_stub"
    }
}

impl JoshutoCommand for CursorMoveStub {}

impl std::fmt::Display for CursorMoveStub {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for CursorMoveStub {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        let new_index = match context.curr_tab_ref().curr_list_ref() {
            Some(curr_list) => curr_list.index,
            None => None,
        };

        if let Some(s) = new_index {
            cursor_move(s, context)
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMoveDown {
    movement: usize,
}

impl CursorMoveDown {
    pub fn new(movement: usize) -> Self {
        Self { movement }
    }
    pub const fn command() -> &'static str {
        "cursor_move_up"
    }
}

impl JoshutoCommand for CursorMoveDown {}

impl std::fmt::Display for CursorMoveDown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", Self::command(), self.movement)
    }
}

impl JoshutoRunnable for CursorMoveDown {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        let movement = match context.curr_tab_ref().curr_list_ref() {
            Some(curr_list) => curr_list.index.map(|idx| idx + self.movement),
            None => None,
        };

        if let Some(s) = movement {
            cursor_move(s, context)
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMoveUp {
    movement: usize,
}

impl CursorMoveUp {
    pub fn new(movement: usize) -> Self {
        Self { movement }
    }
    pub const fn command() -> &'static str {
        "cursor_move_down"
    }
}

impl JoshutoCommand for CursorMoveUp {}

impl std::fmt::Display for CursorMoveUp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", Self::command(), self.movement)
    }
}

impl JoshutoRunnable for CursorMoveUp {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        let movement = match context.curr_tab_ref().curr_list_ref() {
            Some(curr_list) => curr_list.index.map(|idx| {
                if idx > self.movement {
                    idx - self.movement
                } else {
                    0
                }
            }),
            None => None,
        };

        if let Some(s) = movement {
            cursor_move(s, context)
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMovePageUp;

impl CursorMovePageUp {
    pub fn new() -> Self {
        Self
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
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        let half_page = {
            match backend.terminal.as_ref().unwrap().size() {
                Ok(rect) => rect.height as usize - 2,
                _ => 10,
            }
        };

        let movement = match context.curr_tab_ref().curr_list_ref() {
            Some(curr_list) => {
                curr_list
                    .index
                    .map(|idx| if idx > half_page { idx - half_page } else { 0 })
            }
            None => None,
        };

        if let Some(s) = movement {
            cursor_move(s, context);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMovePageDown;

impl CursorMovePageDown {
    pub fn new() -> Self {
        Self
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
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        let half_page = {
            match backend.terminal.as_ref().unwrap().size() {
                Ok(rect) => rect.height as usize - 2,
                _ => 10,
            }
        };

        let movement = match context.curr_tab_ref().curr_list_ref() {
            Some(curr_list) => {
                let dir_len = curr_list.contents.len();
                curr_list.index.map(|idx| {
                    if idx + half_page > dir_len - 1 {
                        dir_len - 1
                    } else {
                        idx + half_page
                    }
                })
            }
            None => None,
        };

        if let Some(s) = movement {
            cursor_move(s, context);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMoveHome;

impl CursorMoveHome {
    pub fn new() -> Self {
        Self
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
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        let movement: Option<usize> = match context.curr_tab_ref().curr_list_ref() {
            Some(curr_list) => {
                let len = curr_list.contents.len();
                if len == 0 {
                    None
                } else {
                    Some(0)
                }
            }
            None => None,
        };

        if let Some(s) = movement {
            cursor_move(s, context);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMoveEnd;

impl CursorMoveEnd {
    pub fn new() -> Self {
        Self
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
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        let movement: Option<usize> = match context.curr_tab_ref().curr_list_ref() {
            Some(curr_list) => {
                let len = curr_list.contents.len();
                if len == 0 {
                    None
                } else {
                    Some(len - 1)
                }
            }
            None => None,
        };

        if let Some(s) = movement {
            cursor_move(s, context);
        }
        Ok(())
    }
}
