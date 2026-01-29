use ratatui::termion::event::{Event, Key, MouseButton, MouseEvent};

pub fn str_to_event(s: &str) -> Option<Event> {
    if let Some(k) = str_to_key(s) {
        Some(Event::Key(k))
    } else {
        str_to_mouse(s).map(Event::Mouse)
    }
}

pub fn str_to_key(s: &str) -> Option<Key> {
    if s.is_empty() {
        return None;
    }

    let key = match s {
        "backspace" => Some(Key::Backspace),
        "backtab" => Some(Key::BackTab),
        "arrow_left" => Some(Key::Left),
        "arrow_right" => Some(Key::Right),
        "arrow_up" => Some(Key::Up),
        "arrow_down" => Some(Key::Down),
        "home" => Some(Key::Home),
        "end" => Some(Key::End),
        "page_up" => Some(Key::PageUp),
        "page_down" => Some(Key::PageDown),
        "delete" => Some(Key::Delete),
        "insert" => Some(Key::Insert),
        "escape" => Some(Key::Esc),
        "f1" => Some(Key::F(1)),
        "f2" => Some(Key::F(2)),
        "f3" => Some(Key::F(3)),
        "f4" => Some(Key::F(4)),
        "f5" => Some(Key::F(5)),
        "f6" => Some(Key::F(6)),
        "f7" => Some(Key::F(7)),
        "f8" => Some(Key::F(8)),
        "f9" => Some(Key::F(9)),
        "f10" => Some(Key::F(10)),
        "f11" => Some(Key::F(11)),
        "f12" => Some(Key::F(12)),
        _ => None,
    };

    if key.is_some() {
        return key;
    }

    if s.starts_with("ctrl+") {
        let ch = s.chars().nth("ctrl+".len());
        let key = ch.map(Key::Ctrl);
        return key;
    } else if s.starts_with("alt+") {
        let ch = s.chars().nth("alt+".len());
        let key = ch.map(Key::Alt);
        return key;
    } else if s.len() == 1 {
        let ch = s.chars().next();
        let key = ch.map(Key::Char);
        return key;
    }
    None
}

pub fn str_to_mouse(s: &str) -> Option<MouseEvent> {
    match s {
        "scroll_up" => Some(MouseEvent::Press(MouseButton::WheelUp, 0, 0)),
        "scroll_down" => Some(MouseEvent::Press(MouseButton::WheelDown, 0, 0)),
        _ => None,
    }
}
