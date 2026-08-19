use std::io::{self, stdout, Write};

use ratatui::backend::TermionBackend;
use ratatui::termion::input::MouseTerminal;
use ratatui::termion::raw::{IntoRawMode, RawTerminal};
use ratatui::termion::screen::AlternateScreen;
use ratatui::termion::screen::IntoAlternateScreen;
use ratatui::widgets::Widget;

use crate::utils::format::clear_screen;

/// The raw-mode alternate-screen terminal, with or without mouse event capture enabled.
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
/// The concrete ratatui terminal type joshuto renders to.
pub type TuiTerminal = ratatui::Terminal<TermionBackend<Screen>>;

/// Owns the terminal, which can be temporarily released (e.g. to hand control to a subprocess)
/// and restored.
pub struct AppBackend {
    pub terminal: Option<TuiTerminal>,
    pub mouse_support: bool,
}

impl AppBackend {
    /// Enters raw/alternate-screen mode and constructs the terminal backend.
    pub fn new(mouse_support: bool) -> io::Result<Self> {
        let alt_screen = Screen::new(mouse_support)?;
        // clears the screen of artifacts
        clear_screen();

        let backend = TermionBackend::new(alt_screen);
        let mut terminal = ratatui::Terminal::new(backend)?;
        terminal.hide_cursor()?;
        Ok(Self {
            mouse_support,
            terminal: Some(terminal),
        })
    }

    /// Renders `widget` to fill the full terminal area for one frame.
    pub fn render<W>(&mut self, widget: W)
    where
        W: Widget,
    {
        let _ = self.terminal_mut().draw(|frame| {
            let rect = frame.area();
            frame.render_widget(widget, rect);
        });
    }

    /// Returns the underlying terminal. Panics if the terminal has been dropped via
    /// [`terminal_drop`](Self::terminal_drop) and not yet restored.
    pub fn terminal_ref(&self) -> &TuiTerminal {
        self.terminal.as_ref().unwrap()
    }

    /// Returns a mutable reference to the underlying terminal. Panics if the terminal has been
    /// dropped via [`terminal_drop`](Self::terminal_drop) and not yet restored.
    pub fn terminal_mut(&mut self) -> &mut TuiTerminal {
        self.terminal.as_mut().unwrap()
    }

    // For when we need to launch a terminal application
    /// Releases the terminal (exiting raw/alternate-screen mode), e.g. to hand control to a
    /// subprocess. Call [`terminal_restore`](Self::terminal_restore) afterward.
    pub fn terminal_drop(&mut self) {
        let _ = self.terminal.take();
        let _ = stdout().flush();
    }

    // For when we need to restore joshuto
    /// Re-enters raw/alternate-screen mode after a [`terminal_drop`](Self::terminal_drop).
    pub fn terminal_restore(&mut self) -> io::Result<()> {
        let mut new_backend = Self::new(self.mouse_support)?;
        std::mem::swap(&mut self.terminal, &mut new_backend.terminal);
        Ok(())
    }
}
