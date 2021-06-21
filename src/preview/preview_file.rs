use std::io;
use std::path;
use std::process::{Command, Output};

use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders};

use crate::context::AppContext;
use crate::ui::TuiBackend;

pub fn preview_path_with_script(
    context: &AppContext,
    backend: &mut TuiBackend,
    p: path::PathBuf,
) -> io::Result<Output> {
    let preview_options = context.config_ref().preview_options_ref();
    let config = context.config_ref();

    match preview_options.preview_script.as_ref() {
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            "No preview script specified",
        )),
        Some(script) => {
            let area = backend.terminal.as_ref().unwrap().size().unwrap();

            let constraints: &[Constraint; 3] = &config.display_options_ref().default_layout;

            let layout_rect = if config.display_options_ref().show_borders() {
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

                let block = Block::default().borders(Borders::LEFT);
                let inner3 = block.inner(layout_rect[2]);
                inner3
            } else {
                let layout_rect = Layout::default()
                    .direction(Direction::Horizontal)
                    .vertical_margin(1)
                    .constraints(constraints.as_ref())
                    .split(area);
                layout_rect[2]
            };

            let file_full_path = p.as_path();
            let preview_width = layout_rect.width;
            let preview_height = layout_rect.height;
            let image_cache = 0;
            let preview_image = if preview_options.preview_images { 1 } else { 0 };

            // spawn preview process
            Command::new(script)
                .arg(file_full_path)
                .arg(preview_width.to_string())
                .arg(preview_height.to_string())
                .arg(image_cache.to_string())
                .arg(preview_image.to_string())
                .output()
        }
    }
}

pub fn preview_with_script(context: &AppContext, backend: &mut TuiBackend) -> io::Result<Output> {
    let curr_tab = context.tab_context_ref().curr_tab_ref();
    let child_list = curr_tab.child_list_ref();

    let preview_options = context.config_ref().preview_options_ref();

    let config = context.config_ref();

    match child_list.and_then(|list| list.curr_entry_ref()) {
        None => Err(io::Error::new(io::ErrorKind::Other, "No file to preview")),
        Some(entry) => preview_path_with_script(context, backend, entry.file_path().to_path_buf()),
    }
}
