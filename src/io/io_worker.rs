use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path;
use std::sync::mpsc;

use crate::util::name_resolution::rename_filename_conflict;

#[derive(Clone, Copy, Debug)]
pub enum FileOp {
    Cut,
    Copy,
}

#[derive(Clone, Debug)]
pub struct IoWorkerOptions {
    pub overwrite: bool,
    pub skip_exist: bool,
}

impl std::default::Default for IoWorkerOptions {
    fn default() -> Self {
        Self {
            overwrite: false,
            skip_exist: false,
        }
    }
}

impl std::fmt::Display for IoWorkerOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "overwrite={} skip_exist={}",
            self.overwrite, self.skip_exist
        )
    }
}

#[derive(Clone, Debug)]
pub struct IoWorkerProgress {
    _kind: FileOp,
    _completed: usize,
    _len: usize,
    _bytes_processed: u64,
}

impl IoWorkerProgress {
    pub fn new(_kind: FileOp, _completed: usize, _len: usize, _bytes_processed: u64) -> Self {
        Self {
            _kind,
            _completed,
            _len,
            _bytes_processed,
        }
    }

    pub fn kind(&self) -> FileOp {
        self._kind
    }

    pub fn completed(&self) -> usize {
        self._completed
    }

    pub fn increment_completed(&mut self) {
        self._completed += 1;
    }

    pub fn len(&self) -> usize {
        self._len
    }

    pub fn bytes_processed(&self) -> u64 {
        self._bytes_processed
    }

    pub fn set_bytes_processed(&mut self, _bytes_processed: u64) {
        self._bytes_processed = _bytes_processed;
    }
}

#[derive(Debug)]
pub struct IoWorkerThread {
    _kind: FileOp,
    pub options: IoWorkerOptions,
    pub paths: Vec<path::PathBuf>,
    pub dest: path::PathBuf,
}

impl IoWorkerThread {
    pub fn new(
        _kind: FileOp,
        paths: Vec<path::PathBuf>,
        dest: path::PathBuf,
        options: IoWorkerOptions,
    ) -> Self {
        Self {
            _kind,
            options,
            paths,
            dest,
        }
    }

    pub fn kind(&self) -> FileOp {
        self._kind
    }

    pub fn start(&self, tx: mpsc::Sender<IoWorkerProgress>) -> std::io::Result<IoWorkerProgress> {
        match self.kind() {
            FileOp::Cut => self.paste_cut(tx),
            FileOp::Copy => self.paste_copy(tx),
        }
    }

    fn query_number_of_items(&self) -> io::Result<usize> {
        let mut dirs: VecDeque<path::PathBuf> = VecDeque::new();
        for path in self.paths.iter() {
            let metadata = path.symlink_metadata()?;
            if metadata.is_dir() {
                dirs.push_back(path.clone());
            }
        }

        let mut total = self.paths.len() - dirs.len();

        while let Some(dir) = dirs.pop_front() {
            for entry in fs::read_dir(dir)? {
                let path = entry?.path();
                if path.is_dir() {
                    dirs.push_back(path);
                } else {
                    total += 1;
                }
            }
        }
        Ok(total)
    }

    fn paste_copy(&self, tx: mpsc::Sender<IoWorkerProgress>) -> std::io::Result<IoWorkerProgress> {
        let num_items = self.query_number_of_items()?;
        let mut progress = IoWorkerProgress::new(self.kind(), 0, num_items, 0);
        for path in self.paths.iter() {
            let _ = tx.send(progress.clone());
            recursive_copy(
                path.as_path(),
                self.dest.as_path(),
                tx.clone(),
                &mut progress,
            )?;
        }
        Ok(IoWorkerProgress::new(
            self.kind(),
            self.paths.len(),
            self.paths.len(),
            progress.bytes_processed(),
        ))
    }

    fn paste_cut(&self, tx: mpsc::Sender<IoWorkerProgress>) -> std::io::Result<IoWorkerProgress> {
        let num_items = self.query_number_of_items()?;
        let mut progress = IoWorkerProgress::new(self.kind(), 0, num_items, 0);
        for path in self.paths.iter() {
            let _ = tx.send(progress.clone());
            recursive_cut(
                path.as_path(),
                self.dest.as_path(),
                tx.clone(),
                &mut progress,
            )?;
        }
        Ok(IoWorkerProgress::new(
            self.kind(),
            self.paths.len(),
            self.paths.len(),
            progress.bytes_processed(),
        ))
    }
}

pub fn recursive_copy(
    src: &path::Path,
    dest: &path::Path,
    tx: mpsc::Sender<IoWorkerProgress>,
    progress: &mut IoWorkerProgress,
) -> std::io::Result<()> {
    let mut dest_buf = dest.to_path_buf();
    if let Some(s) = src.file_name() {
        dest_buf.push(s);
    }
    rename_filename_conflict(&mut dest_buf);
    let file_type = fs::symlink_metadata(src)?.file_type();
    if file_type.is_dir() {
        fs::create_dir(dest_buf.as_path())?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            recursive_copy(
                entry_path.as_path(),
                dest_buf.as_path(),
                tx.clone(),
                progress,
            )?;
            let _ = tx.send(progress.clone());
        }
        Ok(())
    } else if file_type.is_file() {
        let bytes_processed = progress.bytes_processed() + fs::copy(src, dest_buf)?;
        progress.set_bytes_processed(bytes_processed);
        progress.increment_completed();
        Ok(())
    } else if file_type.is_symlink() {
        let link_path = fs::read_link(src)?;
        std::os::unix::fs::symlink(link_path, dest_buf)?;
        progress.increment_completed();
        Ok(())
    } else {
        Ok(())
    }
}

pub fn recursive_cut(
    src: &path::Path,
    dest: &path::Path,
    tx: mpsc::Sender<IoWorkerProgress>,
    progress: &mut IoWorkerProgress,
) -> std::io::Result<()> {
    let mut dest_buf = dest.to_path_buf();
    if let Some(s) = src.file_name() {
        dest_buf.push(s);
    }
    rename_filename_conflict(&mut dest_buf);
    let metadata = fs::symlink_metadata(src)?;
    let file_type = metadata.file_type();
    if file_type.is_dir() {
        match fs::rename(src, dest_buf.as_path()) {
            Ok(_) => {
                let processed = progress.bytes_processed() + metadata.len();
                progress.set_bytes_processed(processed);
            }
            Err(_) => {
                fs::create_dir(dest_buf.as_path())?;
                for entry in fs::read_dir(src)? {
                    let entry_path = entry?.path();
                    recursive_cut(
                        entry_path.as_path(),
                        dest_buf.as_path(),
                        tx.clone(),
                        progress,
                    )?;
                }
                fs::remove_dir(src)?;
            }
        }
    } else if file_type.is_file() {
        if fs::rename(src, dest_buf.as_path()).is_err() {
            fs::copy(src, dest_buf.as_path())?;
            fs::remove_file(src)?;
            let processed = progress.bytes_processed() + metadata.len();
            progress.set_bytes_processed(processed);
        }
        progress.increment_completed();
    } else if file_type.is_symlink() {
        let link_path = fs::read_link(src)?;
        std::os::unix::fs::symlink(link_path, dest_buf)?;
        fs::remove_file(src)?;
        let processed = progress.bytes_processed() + metadata.len();
        progress.set_bytes_processed(processed);
        progress.increment_completed();
    }
    Ok(())
}
