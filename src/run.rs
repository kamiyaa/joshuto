use crate::commands::numbered_command;
use crate::config::AppKeyMapping;
use crate::context::{AppContext, QuitType};
use crate::event::AppEvent;
use crate::key_command::{AppExecute, CommandKeybind};
use crate::preview::preview_default;
use crate::tab::JoshutoTab;
use crate::ui;
use crate::ui::views;
use crate::ui::views::TuiView;
use crate::ui::PreviewArea;
use crate::util::input;
use crate::util::to_string::ToString;

use std::path::{Path, PathBuf};
use std::process;
use std::thread;
use termion::event::{Event, Key};
use tui::layout::Rect;

pub fn run(
    backend: &mut ui::TuiBackend,
    context: &mut AppContext,
    keymap_t: AppKeyMapping,
) -> std::io::Result<()> {
    let curr_path = std::env::current_dir()?;
    {
        // Initialize an initial tab
        let tab = JoshutoTab::new(curr_path, context.config_ref().display_options_ref())?;
        context.tab_context_mut().push_tab(tab);

        // trigger a preview of child
        preview_default::load_preview(context, backend);
    }

    let mut preview_area: Option<PreviewArea> = None;

    while context.quit == QuitType::DoNot {
        backend.render(TuiView::new(context));

        {
            let config = context.config_ref();
            let preview_options = config.preview_options_ref();
            if let Ok(area) = backend.terminal_ref().size() {
                preview_area = process_preview_on_change(
                    &context,
                    area,
                    preview_area,
                    preview_options.preview_shown_hook_script.as_ref(),
                    preview_options.preview_removed_hook_script.as_ref(),
                );
            }
        }

        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };

        match event {
            AppEvent::Termion(Event::Mouse(event)) => {
                input::process_mouse(event, context, backend, &keymap_t);
                preview_default::load_preview(context, backend);
            }
            AppEvent::Termion(key) => {
                if context.message_queue_ref().current_message().is_some() {
                    context.message_queue_mut().pop_front();
                }
                match key {
                    // in the event where mouse input is not supported
                    // but we still want to register scroll
                    Event::Unsupported(s) => {
                        input::process_unsupported(context, backend, &keymap_t, s);
                    }
                    Event::Key(Key::Char(c)) if c.is_numeric() && c != '0' => {
                        if let Err(e) =
                            numbered_command::numbered_command(c, context, backend, &keymap_t)
                        {
                            context.message_queue_mut().push_error(e.to_string());
                        }
                    }
                    key => match keymap_t.as_ref().get(&key) {
                        None => {
                            context
                                .message_queue_mut()
                                .push_info(format!("Unmapped input: {}", key.to_string()));
                        }
                        Some(CommandKeybind::SimpleKeybind(command)) => {
                            if let Err(e) = command.execute(context, backend, &keymap_t) {
                                context.message_queue_mut().push_error(e.to_string());
                            }
                        }
                        Some(CommandKeybind::CompositeKeybind(m)) => {
                            let cmd = input::get_input_while_composite(backend, context, m);

                            if let Some(command) = cmd {
                                if let Err(e) = command.execute(context, backend, &keymap_t) {
                                    context.message_queue_mut().push_error(e.to_string());
                                }
                            }
                        }
                    },
                }
                preview_default::load_preview(context, backend);
                context.flush_event();
            }
            event => input::process_noninteractive(event, context),
        }
        context.update_watcher();
    }
    Ok(())
}

fn process_preview_on_change(
    context: &AppContext,
    area: Rect,
    old_preview_area: Option<PreviewArea>,
    preview_shown_hook_script: Option<&PathBuf>,
    preview_removed_hook_script: Option<&PathBuf>,
) -> Option<PreviewArea> {
    let area = Rect {
        y: area.top() + 1,
        height: area.height - 2,
        ..area
    };

    let constraints = views::get_constraints(&context);
    let config = context.config_ref();
    let display_options = config.display_options_ref();
    let layout = if display_options.show_borders() {
        views::calculate_layout_with_borders(area, constraints)
    } else {
        views::calculate_layout(area, constraints)
    };
    let new_preview_area = views::calculate_preview(&context, layout[2]);

    match new_preview_area.as_ref() {
        Some(new) => {
            let should_preview = if let Some(old) = old_preview_area {
                new.file_preview_path != old.file_preview_path
            } else {
                true
            };
            if should_preview {
                if let Some(hook_script) = preview_shown_hook_script {
                    let hook_script = hook_script.to_path_buf();
                    let new2 = new.clone();
                    let _ = thread::spawn(move || {
                        let _ = process::Command::new(hook_script.as_path())
                            .arg(new2.file_preview_path.as_path())
                            .arg(new2.preview_area.x.to_string())
                            .arg(new2.preview_area.y.to_string())
                            .arg(new2.preview_area.width.to_string())
                            .arg(new2.preview_area.height.to_string())
                            .status();
                    });
                }
            }
        }
        None => {
            if let Some(hook_script) = preview_shown_hook_script {
                let hook_script = hook_script.to_path_buf();
                let _ = thread::spawn(|| {
                    let _ = process::Command::new(hook_script).status();
                });
            }
        }
    }
    new_preview_area
}
