use tui::buffer::Buffer;
use tui::layout::{Constraint, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Cell, Row, Table, Widget};

use crate::commands::CommandKeybind;
use crate::config::AppKeyMapping;
use crate::context::AppContext;
use termion::event::{Event, Key};

use lazy_static::lazy_static;

lazy_static! {
    static ref COMMENT_STYLE: Style = Style::default().add_modifier(Modifier::REVERSED);
    static ref DEFAULT_STYLE: Style = Style::default();
    static ref HEADER_STYLE: Style = Style::default().fg(Color::Yellow);
    static ref KEY_STYLE: Style = Style::default().fg(Color::Red);
    static ref COMMAND_STYLE: Style = Style::default().fg(Color::Green);
}

const TITLE: &str = "Keybindings";
const FOOTER: &str = "Press <ESC> to return and Q to exit";

pub struct TuiHelp<'a> {
    context: &'a mut AppContext,
    keymap: &'a AppKeyMapping,
}

impl<'a> TuiHelp<'a> {
    pub fn new(context: &'a mut AppContext, keymap: &'a AppKeyMapping) -> TuiHelp<'a> {
        TuiHelp { context, keymap }
    }
}

impl<'a> Widget for TuiHelp<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut offset = self.context.page_type_ref().get_offset().unwrap();
        // Subtracting 2 because we'll draw a title at the top and some
        // additional information at the bottom of the page
        let height = (area.bottom() - area.top() - 2) as i16;
        let width = area.right() - area.left();
        let keymap_table = get_keymap_table(self.keymap);
        let max_offset = Ord::max(keymap_table.len() as i16 - height + 2, 0) as u8;
        if offset > max_offset {
            self.context.page_type_mut().set_offset(max_offset).unwrap();
            offset = max_offset;
        }
        let keymap_table = Vec::from(&keymap_table[(offset as usize)..]);

        let keybindings_area = Rect::new(0, 1, width, height as u16);
        let mut keybindings_buffer = Buffer::default();
        keybindings_buffer.resize(keybindings_area);
        let widths = [
            Constraint::Length((width as f32 * 0.12) as u16),
            Constraint::Length((width as f32 * 0.50) as u16),
            Constraint::Length((width as f32 * 0.38) as u16),
        ];
        let table_widget = Table::new(keymap_table)
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
        buf.set_stringn(
            0,
            (height + 1) as u16,
            format!("{:^w$}", FOOTER, w = width as usize),
            width as usize,
            *COMMENT_STYLE,
        );
    }
}

// This function is needed because we cannot access Row items, which
// means that we won't be able to sort binds if we create Rows directly
fn get_keymap_table(keymap: &AppKeyMapping) -> Vec<Row> {
    let raw_rows = get_raw_keymap_table(keymap);
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

fn get_raw_keymap_table(keymap: &AppKeyMapping) -> Vec<[String; 3]> {
    let mut rows = Vec::new();
    for (event, bind) in keymap.as_ref() {
        let key = key_event_to_string(event);
        let (command, comment) = match bind {
            CommandKeybind::SimpleKeybind(command) => (format!("{}", command), command.comment()),
            CommandKeybind::CompositeKeybind(sub_keymap) => {
                let mut sub_rows = get_raw_keymap_table(sub_keymap);
                for sub_row in sub_rows.iter_mut() {
                    sub_row[0] = key.clone() + &sub_row[0];
                }
                rows.append(&mut sub_rows);
                continue;
            }
        };
        rows.push([key, command, comment.to_string()]);
    }
    rows.sort_by_cached_key(|x| x[1].clone());
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
