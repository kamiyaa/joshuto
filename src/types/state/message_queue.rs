use std::collections::VecDeque;

use ratatui::style::{Color, Style};

/// A single status-bar message with its display style.
#[derive(Clone, Debug)]
pub struct Message {
    pub content: String,
    pub style: Style,
}

impl Message {
    /// Builds a message with the given text and style.
    pub fn new(content: String, style: Style) -> Self {
        Self { content, style }
    }
}

/// A FIFO queue of status-bar messages awaiting display.
#[derive(Clone, Debug, Default)]
pub struct MessageQueue {
    pub contents: VecDeque<Message>,
}

impl MessageQueue {
    /// Creates an empty message queue.
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues an informational (yellow) message.
    pub fn push_info(&mut self, msg: String) {
        let message = Message::new(msg, Style::default().fg(Color::Yellow));
        self.push_msg(message);
    }
    /// Queues a success (green) message.
    pub fn push_success(&mut self, msg: String) {
        let message = Message::new(msg, Style::default().fg(Color::Green));
        self.push_msg(message);
    }
    /// Queues an error (red) message.
    pub fn push_error(&mut self, msg: String) {
        let message = Message::new(msg, Style::default().fg(Color::Red));
        self.push_msg(message);
    }

    /// Removes and returns the oldest queued message.
    pub fn pop_front(&mut self) -> Option<Message> {
        self.contents.pop_front()
    }
    /// Returns the oldest queued message without removing it.
    pub fn current_message(&self) -> Option<&Message> {
        self.contents.front()
    }

    fn push_msg(&mut self, msg: Message) {
        self.contents.push_back(msg);
    }
}
