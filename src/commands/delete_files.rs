use std::fs;
use std::io::{self, Write};
use std::path;

use termion::clear;
use termion::cursor::Goto;

use crate::commands::{JoshutoCommand, JoshutoRunnable, ReloadDirList};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;
use crate::util::event::Event;

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

    fn delete_files(context: &mut JoshutoContext, backend: &mut TuiBackend) -> std::io::Result<()> {
        let curr_tab = &mut context.tabs[context.curr_tab_index];
        let paths = match curr_tab.curr_list_ref() {
            Some(s) => s.get_selected_paths(),
            None => Vec::new(),
        };

        if paths.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "no files selected",
            ));
        }

        let frame = backend.terminal.get_frame();
        let f_size = frame.size();

        let termion_terminal = backend.terminal.backend_mut();

        write!(
            termion_terminal,
            "{}Delete {} files? (y/N){}",
            Goto(1, f_size.height),
            paths.len(),
            clear::AfterCursor
        );

        io::stdout().flush().ok();

        let mut ch = termion::event::Key::Char('n');
        while let Ok(evt) = context.events.next() {
            match evt {
                Event::Input(key) => {
                    if key == termion::event::Key::Char('y')
                        || key == termion::event::Key::Char('\n')
                    {
                        ch = termion::event::Key::Char('y');
                    }
                    break;
                }
                _ => {}
            }
        }

        if ch == termion::event::Key::Char('y') {
            if paths.len() > 1 {
                write!(
                    termion_terminal,
                    "{}Are you sure? (Y/n){}",
                    Goto(1, f_size.height),
                    clear::AfterCursor
                );

                io::stdout().flush().ok();

                while let Ok(evt) = context.events.next() {
                    match evt {
                        Event::Input(key) => {
                            ch = key;
                            break;
                        }
                        _ => {}
                    }
                }
            } else {
                ch = termion::event::Key::Char('y');
            }
        }

        if ch == termion::event::Key::Char('y') {
            Self::remove_files(&paths)?;
            ReloadDirList::reload(context.curr_tab_index, context)?;
        }
        write!(
            termion_terminal,
            "{}{}",
            Goto(1, f_size.height),
            clear::AfterCursor
        );
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
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        Self::delete_files(context, backend)?;
        Ok(())
    }
}
