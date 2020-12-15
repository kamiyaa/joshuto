use std::fs;
use std::path;

use termion::event::Key;

use crate::context::JoshutoContext;
use crate::history::DirectoryHistory;
use crate::ui::widgets::TuiPrompt;
use crate::ui::TuiBackend;
use crate::util::load_child::LoadChild;

use super::reload;

pub fn remove_files<'a, I>(paths: I) -> std::io::Result<()>
where
    I: Iterator<Item = &'a path::Path>,
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

fn delete_files(context: &mut JoshutoContext, backend: &mut TuiBackend) -> std::io::Result<()> {
    let tab_index = context.tab_context_ref().get_index();
    let paths = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(s) => s.get_selected_paths(),
        None => vec![],
    };
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

    if ch == Key::Char('y') || ch == Key::Char('\n') {
        if paths_len > 1 {
            let ch = {
                let prompt_str = "Are you sure? (y/N)";
                let mut prompt = TuiPrompt::new(prompt_str);
                prompt.get_key(backend, context)
            };
            if ch == Key::Char('y') {
                remove_files(paths.iter().map(|p| p.as_path()))?;
                reload::reload(context, tab_index)?;
                let msg = format!("Deleted {} files", paths_len);
                context.push_msg(msg);
            }
        } else {
            remove_files(paths.iter().map(|p| p.as_path()))?;
            reload::reload(context, tab_index)?;
            let msg = format!("Deleted {} files", paths_len);
            context.push_msg(msg);
        }
    }
    Ok(())
}

pub fn delete_selected_files(
    context: &mut JoshutoContext,
    backend: &mut TuiBackend,
) -> std::io::Result<()> {
    delete_files(context, backend)?;

    let options = context.config_ref().sort_option.clone();
    let curr_path = context.tab_context_ref().curr_tab_ref().pwd().to_path_buf();
    for tab in context.tab_context_mut().iter_mut() {
        tab.history_mut().reload(&curr_path, &options)?;
    }
    LoadChild::load_child(context)?;
    Ok(())
}
