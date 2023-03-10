use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path;

pub fn query_number_of_items(paths: &[path::PathBuf]) -> io::Result<(usize, u64)> {
    let mut total_bytes = 0;
    let mut total_files = 0;

    let mut dirs: VecDeque<path::PathBuf> = VecDeque::new();
    for path in paths.iter() {
        let metadata = path.symlink_metadata()?;
        if metadata.is_dir() {
            dirs.push_back(path.clone());
        } else {
            let metadata = path.symlink_metadata()?;
            total_bytes += metadata.len();
            total_files += 1;
        }
    }

    while let Some(dir) = dirs.pop_front() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                dirs.push_back(path);
            } else {
                let metadata = path.symlink_metadata()?;
                total_bytes += metadata.len();
                total_files += 1;
            }
        }
    }
    Ok((total_files, total_bytes))
}
