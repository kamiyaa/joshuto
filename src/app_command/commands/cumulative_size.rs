use std::collections::HashSet;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path;

use crate::error::AppResult;
use crate::state::AppState;
use crate::utils::format::file_size_to_string;

/// Compute the total apparent size in bytes of every non-directory entry rooted at `root`.
///
/// - Symlinks are not followed; their own size is included instead.
/// - Hard-linked files are counted once.
/// - Directories on a different filesystem than `root` are not entered (like `du -x`).
/// - Directories themselves contribute nothing, whether or not they are readable.
///
/// I/O errors on individual entries are silently skipped, matching `du`.
fn compute_size(root: &path::Path) -> u64 {
    let Ok(root_meta) = fs::symlink_metadata(root) else {
        return 0;
    };
    if !root_meta.is_dir() {
        return root_meta.len();
    }

    let root_dev = root_meta.dev();
    let mut seen_inodes: HashSet<(u64, u64)> = HashSet::new();
    let mut dirs = vec![root.to_path_buf()];
    let mut total: u64 = 0;

    while let Some(dir) = dirs.pop() {
        let Ok(read_dir) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in read_dir.flatten() {
            // DirEntry::metadata() stats relative to the open directory and never follows
            // symlinks, so it's cheaper than fs::symlink_metadata() on a full path
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            let Ok(metadata) = entry.metadata() else {
                continue;
            };
            if file_type.is_dir() {
                if metadata.dev() == root_dev {
                    dirs.push(entry.path());
                }
                continue;
            }
            if metadata.nlink() > 1 && !seen_inodes.insert((metadata.dev(), metadata.ino())) {
                continue;
            }
            total = total.saturating_add(metadata.len());
        }
    }
    total
}

/// Implements `calculate_cumulative_size`: computes and displays the total recursive size of
/// the selected entries (or the current entry, if none selected).
pub fn calculate_cumulative_size(app_state: &mut AppState) -> AppResult {
    let targets: Vec<(path::PathBuf, String)> =
        match app_state.state.tab_state.curr_tab_ref().curr_list_ref() {
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

    if let Some(list) = app_state.state.tab_state.curr_tab_mut().curr_list_mut() {
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
    app_state.state.message_queue.push_info(msg);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hard_links_counted_once_and_symlinks_not_followed() {
        let dir = std::env::temp_dir().join(format!("joshuto-test-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(dir.join("sub")).unwrap();
        fs::write(dir.join("sub/file"), b"hello").unwrap();
        fs::hard_link(dir.join("sub/file"), dir.join("link")).unwrap();
        std::os::unix::fs::symlink("..", dir.join("sub/up")).unwrap();
        let symlink_len = fs::symlink_metadata(dir.join("sub/up")).unwrap().len();

        let size = compute_size(&dir);
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(size, 5 + symlink_len);
    }
}
