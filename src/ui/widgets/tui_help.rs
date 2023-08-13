use lazy_static::lazy_static;

use termion::event::{Event, Key};

use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Cell, Row, Table, Widget};

use crate::config::KeyMapping;
use crate::key_command::traits::CommandComment;
use crate::key_command::CommandKeybind;

lazy_static! {
    static ref COMMENT_STYLE: Style = Style::default().add_modifier(Modifier::REVERSED);
    static ref DEFAULT_STYLE: Style = Style::default();
    static ref HEADER_STYLE: Style = Style::default().fg(Color::Yellow);
    static ref KEY_STYLE: Style = Style::default().fg(Color::Green);
    static ref COMMAND_STYLE: Style = Style::default().fg(Color::Blue);
}

const TITLE: &str = "Keybindings";
const FOOTER: &str = "Press <ESC> to return, / to search, 1,2,3 to sort";

pub struct TuiHelp<'a> {
    // This keymap is constructed with get_keymap_table function
    keymap: &'a [Row<'a>],
    offset: &'a mut u8,
    search_query: &'a str,
}

impl<'a> TuiHelp<'a> {
    pub fn new(keymap: &'a [Row], offset: &'a mut u8, search_query: &'a str) -> TuiHelp<'a> {
        TuiHelp {
            keymap,
            offset,
            search_query,
        }
    }
}

impl<'a> Widget for TuiHelp<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Subtracting 2 because we'll draw a title at the top and some
        // additional information at the bottom of the page
        let height = (area.bottom() - area.top() - 2) as i16;
        let width = area.right() - area.left();
        let max_offset = Ord::max(self.keymap.len() as i16 - height + 2, 0) as u8;
        if *self.offset > max_offset {
            *self.offset = max_offset;
        }
        let keymap = Vec::from(&self.keymap[(*self.offset as usize)..]);

        let keybindings_area = Rect::new(0, 1, width, height as u16);
        let mut keybindings_buffer = Buffer::default();
        keybindings_buffer.resize(keybindings_area);
        let widths = [
            Constraint::Length((width as f32 * 0.12) as u16),
            Constraint::Length((width as f32 * 0.50) as u16),
            Constraint::Length((width as f32 * 0.38) as u16),
        ];
        let table_widget = Table::new(keymap)
            .header(
                Row::new(vec!["Key", "Command", "Description"])
                    .style(*HEADER_STYLE)
                    .bottom_margin(1),
            )
            .widths(&widths)
            .column_spacing(1);

        table_widget.render(keybindings_area, &mut keybindings_buffer);
        buf.merge(&keybindings_buffer);
        buf.set_stringn(
            0,
            0,
            format!("{:^w$}", TITLE, w = width as usize),
            width as usize,
            *COMMENT_STYLE,
        );

        let footer = if self.search_query.is_empty() {
            format!("{:^w$}", FOOTER, w = width as usize)
        } else {
            format!("{:<w$}", self.search_query, w = width as usize)
        };
        buf.set_stringn(
            0,
            (height + 1) as u16,
            &footer,
            footer.len(),
            *COMMENT_STYLE,
        );
    }
}

// Translates output from 'get_raw_keymap_table' into format,
// readable by TUI table widget
pub fn get_keymap_table<'a>(
    keymap: &'a KeyMapping,
    search_query: &'a str,
    sort_by: usize,
) -> Vec<Row<'a>> {
    let raw_rows = get_raw_keymap_table(keymap, search_query, sort_by);
    let mut rows = Vec::new();
    for row in raw_rows {
        rows.push(Row::new(vec![
            Cell::from(row[0].clone()).style(*KEY_STYLE),
            Cell::from(row[1].clone()).style(*COMMAND_STYLE),
            Cell::from(row[2].clone()).style(*DEFAULT_STYLE),
        ]));
    }
    rows
}

// This function is needed because we cannot access Row items, which
// means that we won't be able to sort binds if we create Rows directly
pub fn get_raw_keymap_table<'a>(
    keymap: &'a KeyMapping,
    search_query: &'a str,
    sort_by: usize,
) -> Vec<[String; 3]> {
    let search_query_lowercase = &String::from(search_query).to_lowercase();
    let mut rows = Vec::new();
    for (event, bind) in keymap.iter() {
        let key = key_event_to_string(event);
        let (command, comment) = match bind {
            CommandKeybind::SimpleKeybind {
                command,
                description: None,
            } => (format!("{}", command), command.comment()),
            CommandKeybind::SimpleKeybind {
                command,
                description: Some(desc),
            } => (format!("{}", command), desc.as_str()),
            CommandKeybind::CompositeKeybind(sub_keymap) => {
                let mut sub_rows = get_raw_keymap_table(sub_keymap, "", sort_by);
                for _ in 0..sub_rows.len() {
                    let mut sub_row = sub_rows.pop().unwrap();
                    sub_row[0] = key.clone() + &sub_row[0];
                    if sub_row[0].contains(search_query)
                        || sub_row[1].contains(search_query)
                        || sub_row[2].to_lowercase().contains(search_query_lowercase)
                    {
                        rows.push(sub_row)
                    }
                }
                continue;
            }
        };
        if key.contains(search_query)
            || command.contains(search_query)
            || comment.to_lowercase().contains(search_query_lowercase)
        {
            rows.push([key, command, comment.to_string()]);
        }
    }
    rows.sort_by_cached_key(|x| x[sort_by].clone());
    rows
}

fn key_event_to_string(event: &Event) -> String {
    match event {
        Event::Key(key) => match key {
            Key::Backspace => "Backspace".to_string(),
            Key::Left => "Left".to_string(),
            Key::Right => "Right".to_string(),
            Key::Up => "Up".to_string(),
            Key::Down => "Down".to_string(),
            Key::Home => "Home".to_string(),
            Key::End => "End".to_string(),
            Key::PageUp => "PageUp".to_string(),
            Key::PageDown => "PageDown".to_string(),
            Key::BackTab => "BackTab".to_string(),
            Key::Delete => "Delete".to_string(),
            Key::Insert => "Insert".to_string(),
            Key::Esc => "Esc".to_string(),
            Key::F(n) => format!("F{}", n),
            Key::Char(chr) => match chr {
                ' ' => "Space".to_string(),
                '\t' => "Tab".to_string(),
                '\n' => "Enter".to_string(),
                chr => chr.to_string(),
            },
            Key::Alt(chr) => format!("Alt+{}", chr),
            Key::Ctrl(chr) => format!("Ctrl+{}", chr),
            _ => "".to_string(),
        },
        _ => "".to_string(),
    }
}
