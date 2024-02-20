use std::io::{self, stdout, Write};

use ratatui::backend::TermionBackend;
use ratatui::widgets::Widget;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use termion::screen::IntoAlternateScreen;

use termion::input::MouseTerminal;

trait New {
    fn new() -> io::Result<Self>
    where
        Self: Sized;
}

pub enum Screen {
    WithMouse(MouseTerminal<AlternateScreen<RawTerminal<std::io::Stdout>>>),
    WithoutMouse(AlternateScreen<RawTerminal<std::io::Stdout>>),
}

impl Screen {
    // Returns alternate screen
    fn new(mouse_support: bool) -> io::Result<Self> {
        let stdout = io::stdout().into_raw_mode()?;
        if mouse_support {
            Ok(Self::WithMouse(MouseTerminal::from(
                stdout.into_alternate_screen().unwrap(),
            )))
        } else {
            Ok(Self::WithoutMouse(stdout.into_alternate_screen().unwrap()))
        }
    }
}

impl Write for Screen {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Screen::WithMouse(t) => t.write(buf),
            Screen::WithoutMouse(t) => t.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Screen::WithMouse(t) => t.flush(),
            Screen::WithoutMouse(t) => t.flush(),
        }
    }
}

// pub type TuiBackend = TermionBackend<Screen>;
pub type TuiTerminal = ratatui::Terminal<TermionBackend<Screen>>;

pub struct AppBackend {
    pub terminal: Option<TuiTerminal>,
}

impl AppBackend {
    pub fn new(mouse_support: bool) -> io::Result<Self> {
        let mut alt_screen = Screen::new(mouse_support)?;
        // clears the screen of artifacts
        write!(alt_screen, "{}", termion::clear::All)?;

        let backend = TermionBackend::new(alt_screen);
        let mut terminal = ratatui::Terminal::new(backend)?;
        terminal.hide_cursor()?;
        Ok(Self {
            terminal: Some(terminal),
        })
    }

    pub fn render<W>(&mut self, widget: W)
    where
        W: Widget,
    {
        let _ = self.terminal_mut().draw(|frame| {
            let rect = frame.size();
            frame.render_widget(widget, rect);
        });
    }

    pub fn terminal_ref(&self) -> &TuiTerminal {
        self.terminal.as_ref().unwrap()
    }

    pub fn terminal_mut(&mut self) -> &mut TuiTerminal {
        self.terminal.as_mut().unwrap()
    }

    pub fn terminal_drop(&mut self) {
        let _ = self.terminal.take();
        let _ = stdout().flush();
    }

    pub fn terminal_restore(&mut self, mouse_support: bool) -> io::Result<()> {
        let mut new_backend = Self::new(mouse_support)?;
        std::mem::swap(&mut self.terminal, &mut new_backend.terminal);
        Ok(())
    }
}
