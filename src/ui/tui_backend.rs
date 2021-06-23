use std::io::stdout;
use std::io::Write;

use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders, Widget};

#[cfg(feature = "mouse")]
use termion::input::MouseTerminal;

use crate::util::display::DisplayOption;

trait New {
    fn new() -> std::io::Result<Self>
    where
        Self: Sized;
}

#[cfg(feature = "mouse")]
type Screen = MouseTerminal<AlternateScreen<RawTerminal<std::io::Stdout>>>;
#[cfg(feature = "mouse")]
impl New for Screen {
    fn new() -> std::io::Result<Self> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let alt_screen = MouseTerminal::from(AlternateScreen::from(stdout));
        return Ok(alt_screen);
    }
}
#[cfg(not(feature = "mouse"))]
type Screen = AlternateScreen<RawTerminal<std::io::Stdout>>;
#[cfg(not(feature = "mouse"))]
impl New for Screen {
    fn new() -> std::io::Result<Self> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let alt_screen = AlternateScreen::from(stdout);
        Ok(alt_screen)
    }
}

pub type JoshutoTerminal = tui::Terminal<TermionBackend<Screen>>;

pub struct TuiBackend {
    pub terminal: Option<JoshutoTerminal>,
}

impl TuiBackend {
    pub fn new() -> std::io::Result<Self> {
        let mut alt_screen = Screen::new()?;
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

pub fn build_layout(
    area: Rect,
    constraints: &[Constraint; 3],
    display_options: &DisplayOption,
) -> Vec<Rect> {
    let layout_rect = if display_options.show_borders() {
        let area = Rect {
            y: area.top() + 1,
            height: area.height - 2,
            ..area
        };

        let block = Block::default().borders(Borders::ALL);
        let inner = block.inner(area);

        let layout_rect = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints.as_ref())
            .split(inner);

        let block = Block::default().borders(Borders::RIGHT);
        let inner1 = block.inner(layout_rect[0]);

        let block = Block::default().borders(Borders::LEFT);
        let inner3 = block.inner(layout_rect[2]);

        vec![inner1, layout_rect[1], inner3]
    } else {
        let mut layout_rect = Layout::default()
            .direction(Direction::Horizontal)
            .vertical_margin(1)
            .constraints(constraints.as_ref())
            .split(area);

        layout_rect[0] = Rect {
            width: layout_rect[0].width - 1,
            ..layout_rect[0]
        };
        layout_rect[1] = Rect {
            width: layout_rect[1].width - 1,
            ..layout_rect[1]
        };
        layout_rect
    };
    layout_rect
}
