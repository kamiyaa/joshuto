use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;

use crate::context::WorkerContext;
use crate::util::format;

pub struct TuiWorker<'a> {
    pub context: &'a WorkerContext,
}

impl<'a> TuiWorker<'a> {
    pub fn new(context: &'a WorkerContext) -> Self {
        Self { context }
    }
}

impl<'a> Widget for TuiWorker<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.context.worker_ref() {
            Some(io_obs) => {
                if let Some(progress) = io_obs.progress.as_ref() {
                    let op_str = progress.kind().actioning_str();

                    let processed_size = format::file_size_to_string(progress.bytes_processed());
                    let total_size = format::file_size_to_string(progress.total_bytes());

                    let msg = format!(
                        "{} ({}/{}) ({}/{}) {:?}",
                        op_str,
                        progress.files_processed() + 1,
                        progress.total_files(),
                        processed_size,
                        total_size,
                        io_obs.dest_path()
                    );

                    let style = Style::default();
                    buf.set_stringn(0, 2, msg, area.width as usize, style);

                    // draw a progress bar
                    let progress_bar_width = (progress.files_processed() as f32
                        / progress.total_files() as f32
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
                    for (i, worker) in self.context.iter().enumerate() {
                        let msg = format!(
                            "{:02} {} {} items {:?}",
                            i + 1,
                            worker.kind(),
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
