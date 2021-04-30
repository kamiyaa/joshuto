use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;

use crate::context::AppContext;
use crate::io::FileOp;

pub struct TuiWorker<'a> {
    pub context: &'a AppContext,
}

impl<'a> TuiWorker<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        Self { context }
    }
}

impl<'a> Widget for TuiWorker<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.context.worker_ref() {
            Some(io_obs) => {
                if let Some(progress) = io_obs.progress.as_ref() {
                    let op_str = match progress.kind() {
                        FileOp::Copy => "Copying",
                        FileOp::Cut => "Moving",
                    };
                    let msg = format!(
                        "{} ({}/{}) {:?}",
                        op_str,
                        progress.completed() + 1,
                        progress.len(),
                        io_obs.dest_path()
                    );
                    let style = Style::default();
                    buf.set_stringn(0, 2, msg, area.width as usize, style);

                    // draw a progress bar
                    let progress_bar_width = (progress.completed() as f32 / progress.len() as f32
                        * area.width as f32) as usize;
                    let progress_bar_space = " ".repeat(progress_bar_width);
                    let style = Style::default().bg(Color::Blue);
                    buf.set_stringn(0, 3, progress_bar_space, area.width as usize, style);

                    // draw queued up work
                    let style = Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD);
                    buf.set_stringn(0, 5, "Queue:", area.width as usize, style);

                    let style = Style::default();
                    for (i, worker) in self.context.worker_iter().enumerate() {
                        let op_str = match worker.kind() {
                            FileOp::Copy => "Copy",
                            FileOp::Cut => "Move",
                        };
                        let msg = format!(
                            "{:02} {} {} items {:?}",
                            i + 1,
                            op_str,
                            worker.paths.len(),
                            worker.dest
                        );
                        buf.set_stringn(0, (5 + i + 2) as u16, msg, area.width as usize, style);
                    }
                }
            }
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
