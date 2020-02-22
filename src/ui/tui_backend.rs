use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::widgets::Widget;

pub struct TuiBackend {
    pub terminal: tui::Terminal<TermionBackend<AlternateScreen<RawTerminal<std::io::Stdout>>>>,
}

impl TuiBackend {
    pub fn new() -> std::io::Result<Self> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = tui::Terminal::new(backend)?;
        terminal.hide_cursor()?;
        Ok(Self { terminal })
    }

    pub fn render<W>(&mut self, widget: &mut W)
    where
        W: Widget,
    {
        self.terminal.draw(|mut frame| {
            let rect = frame.size();
            widget.render(&mut frame, rect);
        });
    }
}
