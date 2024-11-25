use std::collections::VecDeque;

use ratatui::style::{Color, Style};
use crate::{THEME_T};

#[derive(Clone, Debug)]
pub struct Message {
    pub content: String,
    pub style: Style,
}

impl Message {
    pub fn new(content: String, style: Style) -> Self {
        Self { content, style }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MessageQueue {
    pub contents: VecDeque<Message>,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_info(&mut self, msg: String) {
        let style = Style::default()
            .fg(THEME_T.msg_info.fg)
            .bg(THEME_T.msg_info.bg)
            .add_modifier(THEME_T.msg_info.modifier);

        let message = Message::new(msg, style);
        self.push_msg(message);
    }
    pub fn push_success(&mut self, msg: String) {
        let style = Style::default()
            .fg(THEME_T.msg_success.fg)
            .bg(THEME_T.msg_success.bg)
            .add_modifier(THEME_T.msg_success.modifier);

        let message = Message::new(msg, style);
        self.push_msg(message);
    }
    pub fn push_error(&mut self, msg: String) {
        let style = Style::default()
            .fg(THEME_T.msg_error.fg)
            .bg(THEME_T.msg_error.bg)
            .add_modifier(THEME_T.msg_error.modifier);

        let message = Message::new(msg, style);
        self.push_msg(message);
    }

    pub fn pop_front(&mut self) -> Option<Message> {
        self.contents.pop_front()
    }
    pub fn current_message(&self) -> Option<&Message> {
        self.contents.front()
    }

    fn push_msg(&mut self, msg: Message) {
        self.contents.push_back(msg);
    }
}
