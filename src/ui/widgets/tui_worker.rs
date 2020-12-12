use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Paragraph, Widget, Wrap};

use crate::context::JoshutoContext;
use crate::io::FileOp;
use crate::ui::widgets::TuiTopBar;

pub struct TuiWorker<'a> {
    pub context: &'a JoshutoContext,
}

impl<'a> TuiWorker<'a> {
    pub fn new(context: &'a JoshutoContext) -> Self {
        Self { context }
    }
}

impl<'a> Widget for TuiWorker<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let f_size = area;

        let topbar_width = f_size.width;

        let curr_tab = self.context.tab_context_ref().curr_tab_ref();
        let rect = Rect {
            x: 0,
            y: 0,
            width: topbar_width,
            height: 1,
        };
        TuiTopBar::new(curr_tab.pwd()).render(rect, buf);

        match self.context.worker_ref() {
            Some(io_obs) => match io_obs.progress.as_ref() {
                Some(progress) => {
                    let msg = match progress.kind() {
                        FileOp::Copy => format!(
                            "Copying ({}/{}) {:?} -> {:?}",
                            progress.index() + 1,
                            progress.len(),
                            io_obs.src_path(),
                            io_obs.dest_path()
                        ),
                        FileOp::Cut => format!(
                            "Moving ({}/{}) {:?} -> {:?}",
                            progress.index() + 1,
                            progress.len(),
                            io_obs.src_path(),
                            io_obs.dest_path()
                        ),
                    };
                    let style = Style::default();
                    buf.set_stringn(0, 2, msg, area.width as usize, style);
                    let style = Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD);
                    buf.set_stringn(0, 4, "Queue:", area.width as usize, style);
                    let style = Style::default();
                    for (i, worker) in self.context.worker_iter().enumerate() {
                        let msg = match worker.kind() {
                            FileOp::Copy => format!(
                                "{:02} Copy {} items {:?} -> {:?}",
                                i + 1,
                                worker.paths.len(),
                                worker.paths[0].parent().unwrap(),
                                worker.dest
                            ),
                            FileOp::Cut => format!(
                                "{:02} Moving {} items {:?} -> {:?}",
                                i + 1,
                                worker.paths.len(),
                                worker.paths[0].parent().unwrap(),
                                worker.dest
                            ),
                        };
                        buf.set_stringn(0, (4 + i + 2) as u16, msg, area.width as usize, style);
                    }
                }
                _ => {}
            },
            _ => {
                let style = Style::default();
                buf.set_stringn(0, 2, "No operations running", area.width as usize, style);
                let style = Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD);
                buf.set_stringn(0, 4, "Queue:", area.width as usize, style);
            }
        }
    }
}
