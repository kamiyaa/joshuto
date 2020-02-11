use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, SelectableList, Text, Widget};
use unicode_width::UnicodeWidthStr;

use crate::context::JoshutoContext;
// use crate::fs::JoshutoDirList;

pub struct TuiBackend {
    pub terminal: tui::Terminal<TermionBackend<AlternateScreen<RawTerminal<std::io::Stdout>>>>,
}

impl TuiBackend {
    pub fn new() -> std::io::Result<Self> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = tui::Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn render(&mut self, context: &JoshutoContext) {
        let curr_tab = context.curr_tab_ref();

        let curr_list = curr_tab.curr_list_ref();
        let parent_list = curr_tab.parent_list_ref();
        let child_list = curr_tab.child_list_ref();

        self.terminal.draw(|mut f| {
            let f_size = f.size();

            let constraints = match child_list {
                Some(_) => [
                    Constraint::Ratio(1, 6),
                    Constraint::Ratio(2, 6),
                    Constraint::Ratio(3, 6),
                ],
                None => [
                    Constraint::Ratio(1, 6),
                    Constraint::Ratio(5, 6),
                    Constraint::Ratio(0, 6),
                ],
            };
            let layout_rect = Layout::default()
                .direction(Direction::Horizontal)
                .vertical_margin(1)
                .constraints(constraints.as_ref())
                .split(f_size);

            if let Some(curr_list) = parent_list.as_ref() {
                let list_name = "parent_list";
                let list_style = Style::default().fg(tui::style::Color::LightBlue);
                let selected_style = Style::default()
                    .fg(tui::style::Color::LightBlue)
                    .modifier(tui::style::Modifier::REVERSED);

                SelectableList::default()
                    .block(Block::default().borders(Borders::ALL).title(list_name))
                    .items(&curr_list.contents)
                    .style(list_style)
                    .highlight_style(selected_style)
                    .select(curr_list.index)
                    .render(&mut f, layout_rect[0]);
            }

            if let Some(curr_list) = curr_list.as_ref() {
                let list_name = "curr_list";
                let list_style = Style::default().fg(tui::style::Color::LightBlue);
                let selected_style = Style::default()
                    .fg(tui::style::Color::LightBlue)
                    .modifier(tui::style::Modifier::REVERSED);

                SelectableList::default()
                    .block(Block::default().borders(Borders::ALL).title(list_name))
                    .items(&curr_list.contents)
                    .style(list_style)
                    .highlight_style(selected_style)
                    .select(curr_list.index)
                    .render(&mut f, layout_rect[1]);
            }

            if let Some(curr_list) = child_list.as_ref() {
                let list_name = "child_list";
                let list_style = Style::default().fg(tui::style::Color::LightBlue);
                let selected_style = Style::default()
                    .fg(tui::style::Color::LightBlue)
                    .modifier(tui::style::Modifier::REVERSED);

                SelectableList::default()
                    .block(Block::default().borders(Borders::ALL).title(list_name))
                    .items(&curr_list.contents)
                    .style(list_style)
                    .highlight_style(selected_style)
                    .select(curr_list.index)
                    .render(&mut f, layout_rect[2]);
            }
        });
    }
}

pub fn rerender(context: &mut JoshutoContext) {}
