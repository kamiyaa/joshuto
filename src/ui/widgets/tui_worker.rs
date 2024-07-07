use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Widget;

use crate::types::state::WorkerState;
use crate::utils::format;
use crate::workers::io::IoWorkerObserver;

pub struct TuiWorker<'a> {
    pub app_state: &'a WorkerState,
}

impl<'a> TuiWorker<'a> {
    pub fn new(app_state: &'a WorkerState) -> Self {
        Self { app_state }
    }
}

impl<'a> Widget for TuiWorker<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 7 {
            return;
        }
        match self.app_state.worker_ref() {
            Some(observer) => {
                let current_area = Rect {
                    y: area.y + 1,
                    ..area
                };
                TuiCurrentWorker::new(observer).render(current_area, buf);

                // draw queued up work
                let style = Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD);
                buf.set_stringn(0, 6, "Queue:", area.width as usize, style);

                let queue_area = Rect {
                    y: area.y + 7,
                    ..area
                };
                TuiWorkerQueue::new(self.app_state).render(queue_area, buf);
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
}

impl<'a> TuiCurrentWorker<'a> {
    pub fn new(observer: &'a IoWorkerObserver) -> Self {
        Self { observer }
    }
}

impl<'a> Widget for TuiCurrentWorker<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let top = area.top();
        let left = area.left();
        let right = area.right();

        let progress = &self.observer.progress;

        let op_str = progress.kind.actioning_str();

        let processed_size = format::file_size_to_string(progress.bytes_processed);
        let total_size = format::file_size_to_string(progress.total_bytes);

        let msg = format!(
            "{} ({}/{}) ({}/{}) {:?}",
            op_str,
            progress.files_processed + 1,
            progress.total_files,
            processed_size,
            total_size,
            self.observer.dest_path(),
        );
        buf.set_stringn(left, top, msg, right as usize, Style::default());

        buf.set_stringn(
            left,
            top + 1,
            format!("{}", progress.current_file.to_string_lossy()),
            right as usize,
            Style::default(),
        );

        // draw a progress bar
        let progress_bar_width = (progress.files_processed as f32 / progress.total_files as f32
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
    pub app_state: &'a WorkerState,
}

impl<'a> TuiWorkerQueue<'a> {
    pub fn new(app_state: &'a WorkerState) -> Self {
        Self { app_state }
    }
}

impl<'a> Widget for TuiWorkerQueue<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let top = area.top();
        let left = area.left();
        let right = area.right();
        let width = right - left;

        let style = Style::default();

        for (i, worker) in self.app_state.iter().enumerate() {
            let msg = format!(
                "{:02} {} {} items {:?}",
                i + 1,
                worker.get_operation_type(),
                worker.paths.len(),
                worker.dest
            );
            buf.set_stringn(left, (top as usize + i) as u16, msg, width as usize, style);
        }
    }
}
