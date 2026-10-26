use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path;

/// Get total bytes and number of files inside a list of files/folders.
///
/// Symlinks are never followed, matching how copy/cut treat them.
/// Unreadable subdirectories are skipped since the result is only used for progress reporting.
pub fn query_number_of_items(paths: &[path::PathBuf]) -> io::Result<(usize, u64)> {
    let mut total_bytes = 0;
    let mut total_files = 0;

    let mut dirs: VecDeque<path::PathBuf> = VecDeque::new();
    for path in paths.iter() {
        let metadata = path.symlink_metadata()?;
        total_files += 1;
        if metadata.is_dir() {
            dirs.push_back(path.clone());
            total_bytes += 1;
        } else {
            total_bytes += metadata.len();
        }
    }

    while let Some(dir) = dirs.pop_front() {
        let Ok(read_dir) = fs::read_dir(dir) else {
            continue;
        };
        for entry in read_dir.flatten() {
            let Ok(metadata) = entry.metadata() else {
                continue;
            };
            total_files += 1;
            if metadata.is_dir() {
                dirs.push_back(entry.path());
                total_bytes += 1;
            } else {
                total_bytes += metadata.len();
            }
        }
    }
    Ok((total_files, total_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symlink_loop_is_not_followed() {
        let dir = std::env::temp_dir().join(format!("joshuto-test-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(dir.join("sub")).unwrap();
        fs::write(dir.join("sub/file"), b"hello").unwrap();
        std::os::unix::fs::symlink("..", dir.join("sub/up")).unwrap();

        let res = query_number_of_items(std::slice::from_ref(&dir));
        fs::remove_dir_all(&dir).unwrap();

        // dir, sub, file, up
        assert_eq!(res.unwrap().0, 4);
    }
}
