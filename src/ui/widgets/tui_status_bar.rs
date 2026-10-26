use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Gauge, Paragraph, Widget, Wrap};

use crate::config::TabDisplayOption;
use crate::fs::AppDirectoryList;
use crate::state::{AppState, Message};
use crate::tasks::FileOperationProgress;

use super::TuiFooter;

/// Widest the worker progress bar gets.
const PROGRESS_BAR_MAX_WIDTH: u16 = 24;
/// Narrowest terminal that still gets a progress bar next to the status text.
const PROGRESS_BAR_MIN_AREA_WIDTH: u16 = 40;

/// The bottom status line. Shows, in priority order: the oldest queued message, else the IO
/// worker's message, else the regular file-info footer. While an IO task runs, a progress bar
/// is drawn at the right edge regardless of which text is shown.
pub struct TuiStatusBar<'a> {
    message: Option<&'a Message>,
    progress: Option<&'a FileOperationProgress>,
    dirlist: &'a AppDirectoryList,
    tab_options: &'a TabDisplayOption,
}

impl<'a> TuiStatusBar<'a> {
    pub fn new(
        app_state: &'a AppState,
        dirlist: &'a AppDirectoryList,
        tab_options: &'a TabDisplayOption,
    ) -> Self {
        let state = &app_state.state;
        let message = state
            .message_queue
            .current_message()
            .or(state.io_worker_message.as_ref());
        Self {
            message,
            progress: state.worker_state.progress.as_ref().map(|s| &s.progress),
            dirlist,
            tab_options,
        }
    }
}

impl Widget for TuiStatusBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let progress = self.progress;
        let (text_area, bar_area) = match progress {
            Some(_) if area.width >= PROGRESS_BAR_MIN_AREA_WIDTH => {
                let bar_width = PROGRESS_BAR_MAX_WIDTH.min(area.width / 4);
                let [text_area, _, bar_area] = Layout::horizontal([
                    Constraint::Min(0),
                    Constraint::Length(1),
                    Constraint::Length(bar_width),
                ])
                .areas(area);
                (text_area, Some(bar_area))
            }
            _ => (area, None),
        };

        match self.message {
            Some(msg) => Paragraph::new(Span::styled(msg.content.as_str(), msg.style))
                .wrap(Wrap { trim: true })
                .render(text_area, buf),
            None => TuiFooter::new(self.dirlist, self.tab_options).render(text_area, buf),
        }

        if let (Some(progress), Some(bar_area)) = (progress, bar_area) {
            Gauge::default()
                .gauge_style(Style::default().fg(Color::Yellow).bg(Color::DarkGray))
                .ratio(progress_ratio(progress))
                .render(bar_area, buf);
        }
    }
}

/// Fraction of the task done, by bytes when sizes are known, otherwise by file count.
fn progress_ratio(progress: &FileOperationProgress) -> f64 {
    let ratio = if progress.total_bytes > 0 {
        progress.bytes_processed as f64 / progress.total_bytes as f64
    } else if progress.total_files > 0 {
        progress.files_processed as f64 / progress.total_files as f64
    } else {
        0.0
    };
    ratio.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::widgets::Widget;

    use crate::config::{DisplayOption, TabDisplayOption};
    use crate::fs::AppDirectoryList;
    use crate::state::Message;
    use crate::tasks::{FileOperation, FileOperationProgress};

    use super::TuiStatusBar;

    const WIDTH: u16 = 80;

    fn half_done() -> FileOperationProgress {
        FileOperationProgress {
            kind: FileOperation::Copy,
            current_file: PathBuf::new(),
            files_processed: 1,
            total_files: 2,
            bytes_processed: 50,
            total_bytes: 100,
        }
    }

    fn render(message: Option<&Message>, progress: Option<&FileOperationProgress>) -> String {
        let tab_options = TabDisplayOption::default();
        let dirlist = AppDirectoryList::from_path(
            PathBuf::from("/"),
            &DisplayOption::default(),
            &tab_options,
        )
        .unwrap();
        let area = Rect::new(0, 0, WIDTH, 1);
        let mut buf = Buffer::empty(area);
        TuiStatusBar {
            message,
            progress,
            dirlist: &dirlist,
            tab_options: &tab_options,
        }
        .render(area, &mut buf);
        (0..WIDTH).map(|x| buf[(x, 0)].symbol()).collect()
    }

    #[test]
    fn running_task_shows_message_with_progress_bar() {
        let msg = Message::info("copying".to_string());
        let line = render(Some(&msg), Some(&half_done()));
        assert!(line.starts_with("copying"));
        assert!(line.contains("50%"), "no progress bar in {line:?}");
    }

    #[test]
    fn no_task_shows_message_without_progress_bar() {
        let msg = Message::success("successfully copied".to_string());
        let line = render(Some(&msg), None);
        assert!(line.starts_with("successfully copied"));
        assert!(!line.contains('%'));
    }

    #[test]
    fn no_message_falls_back_to_footer() {
        let line = render(None, None);
        assert!(!line.trim().is_empty(), "footer not drawn");
        assert!(!line.contains('%'));
    }
}
