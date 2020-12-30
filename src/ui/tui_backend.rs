use std::io::stdout;
use std::io::Write;

use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::widgets::Widget;

pub type JoshutoTerminal =
    tui::Terminal<TermionBackend<MouseTerminal<AlternateScreen<RawTerminal<std::io::Stdout>>>>>;

pub struct TuiBackend {
    pub terminal: Option<JoshutoTerminal>,
}

impl TuiBackend {
    pub fn new() -> std::io::Result<Self> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let mut alt_screen = MouseTerminal::from(AlternateScreen::from(stdout));
        // clears the screen of artifacts
        write!(alt_screen, "{}", termion::clear::All)?;

        let backend = TermionBackend::new(alt_screen);
        let mut terminal = tui::Terminal::new(backend)?;
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

    pub fn terminal_mut(&mut self) -> &mut JoshutoTerminal {
        self.terminal.as_mut().unwrap()
    }

    pub fn terminal_drop(&mut self) {
        let _ = self.terminal.take();
        let _ = stdout().flush();
    }

    pub fn terminal_restore(&mut self) -> std::io::Result<()> {
        let mut new_backend = TuiBackend::new()?;
        std::mem::swap(&mut self.terminal, &mut new_backend.terminal);
        Ok(())
    }
}
