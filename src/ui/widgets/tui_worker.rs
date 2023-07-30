use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Widget;

use crate::context::WorkerContext;
use crate::io::{FileOperationProgress, IoWorkerObserver};
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
        if area.height < 7 {
            return;
        }
        match self.context.worker_ref() {
            Some(io_obs) => {
                if let Some(progress) = io_obs.progress.as_ref() {
                    let current_area = Rect {
                        y: area.y + 1,
                        ..area
                    };
                    TuiCurrentWorker::new(io_obs, progress).render(current_area, buf);
                }

                // draw queued up work
                let style = Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD);
                buf.set_stringn(0, 6, "Queue:", area.width as usize, style);

                let queue_area = Rect {
                    y: area.y + 7,
                    ..area
                };
                TuiWorkerQueue::new(self.context).render(queue_area, buf);
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

pub struct TuiCurrentWorker<'a> {
    pub observer: &'a IoWorkerObserver,
    pub progress: &'a FileOperationProgress,
}

impl<'a> TuiCurrentWorker<'a> {
    pub fn new(observer: &'a IoWorkerObserver, progress: &'a FileOperationProgress) -> Self {
        Self { observer, progress }
    }
}

impl<'a> Widget for TuiCurrentWorker<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let top = area.top();
        let left = area.left();
        let right = area.right();

        let op_str = self.progress.kind().actioning_str();

        let processed_size = format::file_size_to_string(self.progress.bytes_processed());
        let total_size = format::file_size_to_string(self.progress.total_bytes());

        let msg = format!(
            "{} ({}/{}) ({}/{}) {:?}",
            op_str,
            self.progress.files_processed() + 1,
            self.progress.total_files(),
            processed_size,
            total_size,
            self.observer.dest_path(),
        );
        buf.set_stringn(left, top, msg, right as usize, Style::default());

        if let Some(file_name) = self
            .progress
            .current_file()
            .file_name()
            .map(|s| s.to_string_lossy())
        {
            buf.set_stringn(
                left,
                top + 1,
                format!("{}", file_name),
                right as usize,
                Style::default(),
            );
        }

        // draw a progress bar
        let progress_bar_width = (self.progress.files_processed() as f32
            / self.progress.total_files() as f32
            * area.width as f32) as usize;
        let progress_bar_space = " ".repeat(progress_bar_width);
        let progress_bar_style = Style::default().bg(Color::Blue);
        buf.set_stringn(
            left,
            top + 2,
            progress_bar_space,
            right as usize,
            progress_bar_style,
        );
    }
}

pub struct TuiWorkerQueue<'a> {
    pub context: &'a WorkerContext,
}

impl<'a> TuiWorkerQueue<'a> {
    pub fn new(context: &'a WorkerContext) -> Self {
        Self { context }
    }
}

impl<'a> Widget for TuiWorkerQueue<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let top = area.top();
        let left = area.left();
        let right = area.right();
        let width = right - left;

        let style = Style::default();

        for (i, worker) in self.context.iter().enumerate() {
            let msg = format!(
                "{:02} {} {} items {:?}",
                i + 1,
                worker.kind(),
                worker.paths.len(),
                worker.dest
            );
            buf.set_stringn(left, (top as usize + i) as u16, msg, width as usize, style);
        }
    }
}
