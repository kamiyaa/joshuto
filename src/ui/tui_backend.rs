use std::io::stdout;
use std::io::Write;

use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::widgets::Widget;

pub struct TuiBackend {
    pub terminal:
        Option<tui::Terminal<TermionBackend<AlternateScreen<RawTerminal<std::io::Stdout>>>>>,
}

impl TuiBackend {
    pub fn new() -> std::io::Result<Self> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = tui::Terminal::new(backend)?;
        terminal.hide_cursor()?;
        Ok(Self {
            terminal: Some(terminal),
        })
    }

    pub fn render<W>(&mut self, widget: &mut W)
    where
        W: Widget,
    {
        self.terminal_mut().draw(|mut frame| {
            let rect = frame.size();
            widget.render(&mut frame, rect);
        });
    }

    pub fn terminal_mut(
        &mut self,
    ) -> &mut tui::Terminal<TermionBackend<AlternateScreen<RawTerminal<std::io::Stdout>>>> {
        self.terminal.as_mut().unwrap()
    }

    pub fn terminal_drop(&mut self) {
        let _ = self.terminal.take();
        stdout().flush();
    }

    pub fn terminal_restore(&mut self) -> std::io::Result<()> {
        let mut new_backend = TuiBackend::new()?;
        std::mem::swap(&mut self.terminal, &mut new_backend.terminal);
        Ok(())
    }
}
