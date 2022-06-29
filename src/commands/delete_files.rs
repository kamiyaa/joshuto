use std::fs;
use std::path;

use termion::event::Key;

use crate::context::AppContext;
use crate::history::DirectoryHistory;
use crate::ui::widgets::TuiPrompt;
use crate::ui::AppBackend;

use super::reload;

fn trash_error_to_io_error(err: trash::Error) -> std::io::Error {
    match err {
        trash::Error::Unknown { description } => {
            std::io::Error::new(std::io::ErrorKind::Other, description)
        }
        trash::Error::TargetedRoot => {
            std::io::Error::new(std::io::ErrorKind::Other, "Targeted Root")
        }
        _ => std::io::Error::new(std::io::ErrorKind::Other, "Unknown Error"),
    }
}

pub fn remove_files<P>(paths: &[P]) -> std::io::Result<()>
where
    P: AsRef<path::Path>,
{
    for path in paths {
        if let Ok(metadata) = fs::symlink_metadata(path) {
            if metadata.is_dir() {
                fs::remove_dir_all(&path)?;
            } else {
                fs::remove_file(&path)?;
            }
        }
    }
    Ok(())
}

pub fn trash_files<P>(paths: &[P]) -> std::io::Result<()>
where
    P: AsRef<path::Path>,
{
    for path in paths {
        if let Err(e) = trash::delete(path) {
            return Err(trash_error_to_io_error(e));
        }
    }
    Ok(())
}

fn delete_files(context: &mut AppContext, backend: &mut AppBackend) -> std::io::Result<()> {
    let delete_func = if context.config_ref().use_trash {
        trash_files
    } else {
        remove_files
    };

    let tab_index = context.tab_context_ref().index;
    let paths = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|s| s.get_selected_paths())
        .unwrap_or_else(Vec::new);
    let paths_len = paths.len();
    if paths_len == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "no files selected",
        ));
    }

    let ch = {
        let prompt_str = format!("Delete {} files? (Y/n)", paths_len);
        let mut prompt = TuiPrompt::new(&prompt_str);
        prompt.get_key(backend, context)
    };

    match ch {
        Key::Char('Y') | Key::Char('y') | Key::Char('\n') => {
            let confirm_delete = if paths_len > 1 {
                // prompt user again for deleting multiple files
                let ch = {
                    let prompt_str = "Are you sure? (y/N)";
                    let mut prompt = TuiPrompt::new(prompt_str);
                    prompt.get_key(backend, context)
                };
                ch == Key::Char('y')
            } else {
                true
            };
            if confirm_delete {
                delete_func(&paths)?;

                // remove directory previews
                for tab in context.tab_context_mut().iter_mut() {
                    for p in &paths {
                        tab.history_mut().remove(p.as_path());
                    }
                }
                reload::reload(context, tab_index)?;
                let msg = format!("Deleted {} files", paths_len);
                context.message_queue_mut().push_success(msg);
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

pub fn delete_selected_files(
    context: &mut AppContext,
    backend: &mut AppBackend,
) -> std::io::Result<()> {
    let _ = delete_files(context, backend)?;

    let options = context.config_ref().display_options_ref().clone();
    let curr_path = context.tab_context_ref().curr_tab_ref().cwd().to_path_buf();
    for tab in context.tab_context_mut().iter_mut() {
        tab.history_mut().reload(&curr_path, &options)?;
    }
    Ok(())
}
