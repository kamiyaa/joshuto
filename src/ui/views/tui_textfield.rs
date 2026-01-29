use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::str::FromStr;
use std::{env, fs};

use rustyline::completion::{Candidate, FilenameCompleter, Pair};
use rustyline::history::SearchDirection;
use rustyline::line_buffer::{self, ChangeListener, DeleteListener, Direction, LineBuffer};
use rustyline::{At, Word};

use lazy_static::lazy_static;
use ratatui::layout::{Position, Rect};
use ratatui::termion::event::{Event, Key};
use ratatui::widgets::Clear;
use unicode_width::UnicodeWidthStr;

use crate::constants::command_name::COMMANDS;
use crate::run::process_event;
use crate::traits::app_execute::{CommandCompletion, InteractiveExecute};
use crate::types::command::Command;
use crate::types::completion_kind::CompletionKind;
use crate::types::event::AppEvent;
use crate::types::state::AppState;
use crate::ui::views::TuiView;
use crate::ui::widgets::{TuiMenu, TuiMultilineText};
use crate::ui::AppBackend;

lazy_static! {
    static ref EXECUTABLES: Vec<&'static str> = {
        let mut vec = vec![];

        if let Some(path) = env::var_os("PATH") {
            for path in env::split_paths(&path) {
                let Ok(files) = fs::read_dir(path) else {
                    continue;
                };

                for file in files {
                    let Ok(file) = file else {
                        continue;
                    };

                    let Ok(metadata) = file.metadata() else {
                        continue;
                    };

                    #[cfg(unix)]
                    if metadata.mode() & 0o100 == 0 {
                        // if file is not executable
                        continue;
                    }

                    if let Some(basename) = file.path().file_name() {
                        let boxed = Box::new(basename.to_string_lossy().to_string());
                        vec.push(Box::leak(boxed).as_str());
                    }
                }
            }
        }

        vec
    };
}

// Might need to be implemented in the future
#[derive(Clone, Debug)]
pub struct DummyListener {}

impl DeleteListener for DummyListener {
    fn delete(&mut self, _idx: usize, _string: &str, _dir: Direction) {}
}

impl ChangeListener for DummyListener {
    fn insert_char(&mut self, _idx: usize, _c: char) {}

    fn insert_str(&mut self, _idx: usize, _string: &str) {}

    fn replace(&mut self, _idx: usize, _old: &str, _new: &str) {}
}

struct CompletionTracker {
    pub index: usize,
    pub pos: usize,
    pub _original: String,
    pub candidates: Vec<Pair>,
}

impl CompletionTracker {
    pub fn new(index: usize, pos: usize, candidates: Vec<Pair>, _original: String) -> Self {
        CompletionTracker {
            index,
            pos,
            _original,
            candidates,
        }
    }
}

pub struct CursorInfo {
    pub x: usize,
    pub y: usize,
}

#[derive(Default)]
pub struct TuiTextField<'a> {
    _prompt: &'a str,
    _prefix: &'a str,
    _suffix: &'a str,
    _menu_items: Vec<&'a str>,
}

impl<'a> TuiTextField<'a> {
    pub fn menu_items<I>(&mut self, items: I) -> &mut Self
    where
        I: Iterator<Item = &'a str>,
    {
        self._menu_items = items.collect();
        self
    }

    pub fn prompt(&mut self, prompt: &'a str) -> &mut Self {
        self._prompt = prompt;
        self
    }

    pub fn prefix(&mut self, prefix: &'a str) -> &mut Self {
        self._prefix = prefix;
        self
    }

    pub fn suffix(&mut self, suffix: &'a str) -> &mut Self {
        self._suffix = suffix;
        self
    }

    pub fn get_input(
        &mut self,
        app_state: &mut AppState,
        backend: &mut AppBackend,
        listener: &mut DummyListener,
    ) -> Option<String> {
        let mut line_buffer = line_buffer::LineBuffer::with_capacity(255);
        let completer = FilenameCompleter::new();

        let mut completion_tracker: Option<CompletionTracker> = None;

        let char_idx = self._prefix.chars().map(|c| c.len_utf8()).sum();

        line_buffer.insert_str(0, self._prefix, listener);
        line_buffer.insert_str(line_buffer.len(), self._suffix, listener);
        line_buffer.set_pos(char_idx);

        let terminal = backend.terminal_mut();
        let _ = terminal.show_cursor();

        let mut curr_history_index = app_state.state.commandline_state_ref().history_ref().len();

        loop {
            terminal
                .draw(|frame| {
                    let area: Rect = frame.area();
                    if area.height == 0 {
                        return;
                    }
                    // redraw view
                    {
                        let mut view = TuiView::new(app_state);
                        view.show_bottom_status = false;
                        frame.render_widget(view, area);
                    }

                    let area_width = area.width as usize;
                    let buffer_str = line_buffer.as_str();
                    let cursor_xpos = line_buffer.pos();

                    let line_str = format!("{}{}", self._prompt, buffer_str);
                    let multiline = TuiMultilineText::new(line_str.as_str(), area_width);
                    let multiline_height = multiline.height();

                    // render menu
                    if !self._menu_items.is_empty() {
                        let menu_widget = TuiMenu::new(self._menu_items.as_slice());
                        let menu_len = menu_widget.len();
                        let menu_y = if menu_len + 1 > area.height as usize {
                            0
                        } else {
                            (area.height as usize - menu_len - 1) as u16
                        };

                        let menu_rect = Rect {
                            x: 0,
                            y: menu_y - multiline_height as u16,
                            width: area.width,
                            height: menu_len as u16 + 1,
                        };
                        frame.render_widget(Clear, menu_rect);
                        frame.render_widget(menu_widget, menu_rect);
                    }

                    let multiline_rect = Rect {
                        x: 0,
                        y: area.height - multiline_height as u16,
                        width: area.width,
                        height: multiline_height as u16,
                    };
                    let mut cursor_info = CursorInfo {
                        x: 0,
                        y: area.height as usize,
                    };

                    // get cursor render position
                    let cursor_prefix_width =
                        buffer_str[0..cursor_xpos].width() + self._prompt.len();
                    let y_offset = cursor_prefix_width / area_width;
                    cursor_info.y = area.height as usize - multiline_height + y_offset;
                    cursor_info.x = cursor_prefix_width % area_width + y_offset;

                    // render multiline textfield
                    frame.render_widget(Clear, multiline_rect);
                    frame.render_widget(multiline, multiline_rect);

                    // render cursor
                    frame.set_cursor_position(Position::new(
                        cursor_info.x as u16,
                        cursor_info.y as u16,
                    ));
                })
                .expect("Failed to draw");

            let event = match app_state.poll_event() {
                Ok(event) => event,
                Err(_) => continue,
            };
            match event {
                AppEvent::TerminalEvent(Event::Key(key)) => {
                    let dirty = match key {
                        Key::Backspace => {
                            if line_buffer.is_empty() {
                                let _ = terminal.hide_cursor();
                                return None;
                            }

                            let res = line_buffer.backspace(1, listener);
                            if let Ok(command) = Command::from_str(line_buffer.as_str()) {
                                command.interactive_execute(app_state)
                            }
                            res
                        }
                        Key::Delete => {
                            if line_buffer.is_empty() {
                                let _ = terminal.hide_cursor();
                                return None;
                            }
                            line_buffer.delete(1, listener).is_some()
                        }
                        Key::Home => line_buffer.move_home(),
                        Key::End => line_buffer.move_end(),
                        Key::Up => {
                            curr_history_index = curr_history_index.saturating_sub(1);
                            line_buffer.move_home();
                            line_buffer.kill_line(listener);
                            if let Ok(Some(s)) = app_state
                                .state
                                .commandline_state_ref()
                                .history_ref()
                                .get(curr_history_index, SearchDirection::Forward)
                            {
                                line_buffer.insert_str(0, &s.entry, listener);
                                line_buffer.move_end();
                            }
                            true
                        }
                        Key::Down => {
                            curr_history_index = if curr_history_index
                                < app_state.state.commandline_state_ref().history_ref().len()
                            {
                                curr_history_index + 1
                            } else {
                                curr_history_index
                            };
                            line_buffer.move_home();
                            line_buffer.kill_line(listener);
                            if let Ok(Some(s)) = app_state
                                .state
                                .commandline_state_ref()
                                .history_ref()
                                .get(curr_history_index, SearchDirection::Reverse)
                            {
                                line_buffer.insert_str(0, &s.entry, listener);
                                line_buffer.move_end();
                            }
                            true
                        }
                        Key::Esc => {
                            let _ = terminal.hide_cursor();
                            return None;
                        }
                        Key::Char('\t') => {
                            autocomplete(
                                &mut line_buffer,
                                &mut completion_tracker,
                                &completer,
                                listener,
                                false,
                            );
                            false
                        }
                        Key::BackTab => {
                            autocomplete(
                                &mut line_buffer,
                                &mut completion_tracker,
                                &completer,
                                listener,
                                true,
                            );
                            false
                        }

                        // Current `completion_tracker` should be dropped
                        // only if we moved to another word
                        Key::Ctrl('a') => moved_to_another_word(&mut line_buffer, |line_buffer| {
                            line_buffer.move_home()
                        }),
                        Key::Ctrl('e') => moved_to_another_word(&mut line_buffer, |line_buffer| {
                            line_buffer.move_end()
                        }),
                        Key::Ctrl('f') | Key::Right => {
                            moved_to_another_word(&mut line_buffer, |line_buffer| {
                                line_buffer.move_forward(1)
                            })
                        }
                        Key::Ctrl('b') | Key::Left => {
                            moved_to_another_word(&mut line_buffer, |line_buffer| {
                                line_buffer.move_backward(1)
                            })
                        }
                        Key::Alt('f') => moved_to_another_word(&mut line_buffer, |line_buffer| {
                            line_buffer.move_to_next_word(At::Start, Word::Vi, 1)
                        }),
                        Key::Alt('b') => moved_to_another_word(&mut line_buffer, |line_buffer| {
                            line_buffer.move_to_prev_word(Word::Vi, 1)
                        }),

                        Key::Ctrl('w') => line_buffer.delete_prev_word(Word::Vi, 1, listener),
                        Key::Ctrl('u') => line_buffer.discard_line(listener),
                        Key::Ctrl('h') => line_buffer.backspace(1, listener),
                        Key::Ctrl('d') => line_buffer.delete(1, listener).is_some(),
                        Key::Char('\n') => {
                            break;
                        }
                        Key::Char(c) => {
                            let dirty = line_buffer.insert(c, 1, listener).is_some();

                            if let Ok(command) = Command::from_str(line_buffer.as_str()) {
                                command.interactive_execute(app_state)
                            }
                            dirty
                        }
                        _ => false,
                    };
                    if dirty {
                        completion_tracker.take();
                    }
                    app_state.flush_event();
                }
                AppEvent::TerminalEvent(_) => {
                    app_state.flush_event();
                }
                event => process_event::process_noninteractive(event, app_state),
            };
        }
        let _ = terminal.hide_cursor();

        if line_buffer.as_str().is_empty() {
            None
        } else {
            let input_string = line_buffer.to_string();
            Some(input_string)
        }
    }
}

fn autocomplete(
    line_buffer: &mut LineBuffer,
    completion_tracker: &mut Option<CompletionTracker>,
    completer: &FilenameCompleter,
    listener: &mut DummyListener,
    backwards: bool,
) {
    // If we are in the middle of a word, move to the end of it,
    // so we don't split it with autocompletion.
    move_to_the_end(line_buffer);

    match completion_tracker {
        Some(ct) => {
            if backwards && ct.index == 0 {
                return;
            }
            if !backwards && ct.index + 1 >= ct.candidates.len() {
                return;
            }

            ct.index = if backwards {
                ct.index.checked_sub(1).unwrap_or(ct.candidates.len() - 1)
            } else {
                ct.index + 1
            };

            let pos = ct.pos;
            let first = ct.candidates[ct.index].display();

            line_buffer.set_pos(pos);
            line_buffer.kill_buffer(listener);
            line_buffer.insert_str(pos, first, listener);
            line_buffer.move_end();
        }
        None => {
            if let Some((pos, mut candidates)) = get_autocomplete_candidates(completer, line_buffer)
            {
                if !candidates.is_empty() {
                    candidates.sort_by(|x, y| {
                        x.display()
                            .partial_cmp(y.display())
                            .unwrap_or(std::cmp::Ordering::Less)
                    });

                    let index = if backwards { candidates.len() - 1 } else { 0 };
                    let pos = line_buffer.rfind('/').map(|p| p + 1).unwrap_or(pos);
                    let ct =
                        CompletionTracker::new(index, pos, candidates, line_buffer.to_string());
                    let first = ct.candidates[index].display();

                    line_buffer.set_pos(pos);
                    line_buffer.kill_buffer(listener);
                    line_buffer.insert_str(pos, first, listener);
                    line_buffer.move_end();

                    *completion_tracker = Some(ct);
                }
            }
        }
    }
}

fn moved_to_another_word<F, Any>(line_buffer: &mut LineBuffer, action: F) -> bool
where
    F: FnOnce(&mut LineBuffer) -> Any,
{
    let old_pos = line_buffer.pos();
    action(line_buffer);
    let new_pos = line_buffer.pos();

    let left_pos = usize::min(old_pos, new_pos);
    let right_pos = usize::max(old_pos, new_pos);

    line_buffer.as_str()[left_pos..right_pos].contains(' ')
}

// We shout take into account the fact that `pos` returns a
// *byte position*, while we need to move by characters.
fn move_to_the_end(line_buffer: &mut LineBuffer) {
    let mut curr_pos = line_buffer.pos();
    let mut found = false;
    let buffer = line_buffer.to_string();
    for value in buffer.chars() {
        if !found {
            if curr_pos != 0 {
                curr_pos -= value.len_utf8();
                continue;
            }
            found = true;
        }
        if value == ' ' {
            break;
        }
        if !line_buffer.move_forward(1) {
            break;
        }
    }
}

fn pair_from_str(s: &str) -> Pair {
    Pair {
        display: s.to_string(),
        replacement: s.to_string(),
    }
}

fn start_of_word(line_buffer: &LineBuffer) -> usize {
    line_buffer
        .as_str()
        .split_at(line_buffer.pos())
        .0
        .rfind(' ')
        .map(|x| x + 1)
        .unwrap_or(0)
}

fn complete_word(candidates: &[&str], partial: &str) -> Vec<Pair> {
    candidates
        .iter()
        .filter(|entry| entry.starts_with(partial))
        .map(|entry| pair_from_str(entry))
        .collect()
}

fn get_autocomplete_candidates(
    completer: &FilenameCompleter,
    line_buffer: &mut LineBuffer,
) -> Option<(usize, Vec<Pair>)> {
    let res = line_buffer.as_str().split_once(' ');

    match res {
        None => Some((0, complete_word(&COMMANDS, line_buffer.as_str()))),
        Some((command, arg)) => {
            // We want to autocomplete a command if we are inside it.
            if line_buffer.pos() <= command.len() {
                return Some((0, complete_word(&COMMANDS, command)));
            }
            let completion_kind = Command::completion_kind(command)?;
            match completion_kind {
                CompletionKind::Bin => {
                    if !arg.starts_with("./") && !arg.starts_with('/') {
                        return Some((
                            start_of_word(line_buffer),
                            complete_word(&EXECUTABLES, arg),
                        ));
                    }

                    let (start, list) = completer
                        .complete_path(line_buffer.as_str(), line_buffer.pos())
                        .ok()?;

                    #[cfg(unix)]
                    let list = list
                        .into_iter()
                        .filter(|pair| {
                            // keep entry only if it's executable
                            if let Ok(file) = PathBuf::from(pair.replacement()).metadata() {
                                file.mode() & 0o100 != 0
                            } else {
                                false
                            }
                        })
                        .collect();

                    Some((start, list))
                }

                CompletionKind::Custom(v) => {
                    Some((start_of_word(line_buffer), complete_word(&v, arg)))
                }

                CompletionKind::File => completer
                    .complete_path(line_buffer.as_str(), line_buffer.pos())
                    .ok(),

                CompletionKind::Dir(flags) => {
                    let (start, list) = completer
                        .complete_path(line_buffer.as_str(), line_buffer.pos())
                        .ok()?;

                    let mut new_list: Vec<Pair> = list
                        .into_iter()
                        .filter(|pair| PathBuf::from(pair.replacement()).is_dir())
                        .collect();

                    if let Some(flags) = flags {
                        new_list.append(&mut flags.iter().map(|f| pair_from_str(f)).collect())
                    }
                    Some((start, new_list))
                }
            }
        }
    }
}
