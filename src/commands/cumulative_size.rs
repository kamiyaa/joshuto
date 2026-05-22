use std::fs;
use std::path;

use crate::error::AppResult;
use crate::types::state::AppState;
use crate::utils::format::file_size_to_string;

/// Recursively compute the total size in bytes of every regular file rooted
/// at `path`. Symlinks are not followed; their own size is included instead.
/// I/O errors on individual entries are silently skipped, matching `du`.
fn compute_size(path: &path::Path) -> u64 {
    let symlink_meta = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => return 0,
    };

    let file_type = symlink_meta.file_type();
    if file_type.is_symlink() {
        return symlink_meta.len();
    }
    if !file_type.is_dir() {
        return symlink_meta.len();
    }

    let read_dir = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(_) => return symlink_meta.len(),
    };

    let mut total: u64 = 0;
    for entry in read_dir.flatten() {
        total = total.saturating_add(compute_size(&entry.path()));
    }
    total
}

pub fn calculate_cumulative_size(app_state: &mut AppState) -> AppResult {
    let targets: Vec<(path::PathBuf, String)> = match app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
    {
        Some(list) => list
            .selected_or_current()
            .into_iter()
            .map(|e| (e.file_path().to_path_buf(), e.file_name().to_string()))
            .collect(),
        None => Vec::new(),
    };

    if targets.is_empty() {
        return Ok(());
    }

    let mut total: u64 = 0;
    let mut sizes: Vec<(String, u64)> = Vec::with_capacity(targets.len());
    for (path, name) in &targets {
        let size = compute_size(path);
        total = total.saturating_add(size);
        sizes.push((name.clone(), size));
    }

    if let Some(list) = app_state
        .state
        .tab_state_mut()
        .curr_tab_mut()
        .curr_list_mut()
    {
        for (name, size) in &sizes {
            if let Some(entry) = list.iter_mut().find(|e| e.file_name() == name.as_str()) {
                entry.metadata.update_cumulative_size(*size);
            }
        }
    }

    let msg = if sizes.len() == 1 {
        format!(
            "Size of {}: {}",
            sizes[0].0.trim(),
            file_size_to_string(total).trim()
        )
    } else {
        format!(
            "Cumulative size of {} items: {}",
            sizes.len(),
            file_size_to_string(total).trim()
        )
    };
    app_state.state.message_queue_mut().push_info(msg);

    Ok(())
}
