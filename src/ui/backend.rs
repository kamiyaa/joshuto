use std::io::{self, stdout, Write};

use ratatui::backend::TermionBackend;
use ratatui::widgets::Widget;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use termion::screen::IntoAlternateScreen;

#[cfg(feature = "mouse")]
use termion::input::MouseTerminal;

trait New {
    fn new() -> io::Result<Self>
    where
        Self: Sized;
}

#[cfg(feature = "mouse")]
type Screen = MouseTerminal<AlternateScreen<RawTerminal<std::io::Stdout>>>;
#[cfg(feature = "mouse")]
impl New for Screen {
    // Returns alternate screen
    fn new() -> io::Result<Self> {
        let stdout = io::stdout().into_raw_mode()?;
        Ok(MouseTerminal::from(stdout.into_alternate_screen().unwrap()))
    }
}
#[cfg(not(feature = "mouse"))]
type Screen = AlternateScreen<RawTerminal<std::io::Stdout>>;
#[cfg(not(feature = "mouse"))]
impl New for Screen {
    // Returns alternate screen
    fn new() -> io::Result<Self> {
        let stdout = std::io::stdout().into_raw_mode()?;
        Ok(stdout.into_alternate_screen().unwrap())
    }
}

// pub type TuiBackend = TermionBackend<Screen>;
pub type TuiTerminal = ratatui::Terminal<TermionBackend<Screen>>;

pub struct AppBackend {
    pub terminal: Option<TuiTerminal>,
}

impl AppBackend {
    pub fn new() -> io::Result<Self> {
        let mut alt_screen = Screen::new()?;
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

    pub fn terminal_restore(&mut self) -> io::Result<()> {
        let mut new_backend = Self::new()?;
        std::mem::swap(&mut self.terminal, &mut new_backend.terminal);
        Ok(())
    }
}
