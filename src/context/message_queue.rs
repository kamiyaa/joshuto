use std::collections::VecDeque;

use ratatui::style::{Color, Style};

pub struct Message {
    pub content: String,
    pub style: Style,
}

impl Message {
    pub fn new(content: String, style: Style) -> Self {
        Self { content, style }
    }
}

#[derive(Default)]
pub struct MessageQueue {
    contents: VecDeque<Message>,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_info(&mut self, msg: String) {
        let message = Message::new(msg, Style::default().fg(Color::Yellow));
        self.push_msg(message);
    }
    pub fn push_success(&mut self, msg: String) {
        let message = Message::new(msg, Style::default().fg(Color::Green));
        self.push_msg(message);
    }
    pub fn push_error(&mut self, msg: String) {
        let message = Message::new(msg, Style::default().fg(Color::Red));
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
