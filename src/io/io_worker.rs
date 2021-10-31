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

#[derive(Clone, Copy, Debug, Default)]
pub struct IoWorkerOptions {
    pub overwrite: bool,
    pub skip_exist: bool,
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
    _files_processed: usize,
    _total_files: usize,
    _bytes_processed: u64,
    _total_bytes: u64,
}

impl IoWorkerProgress {
    pub fn new(
        _kind: FileOp,
        _files_processed: usize,
        _total_files: usize,
        _bytes_processed: u64,
        _total_bytes: u64,
    ) -> Self {
        Self {
            _kind,
            _files_processed,
            _total_files,
            _bytes_processed,
            _total_bytes,
        }
    }

    pub fn kind(&self) -> FileOp {
        self._kind
    }

    pub fn files_processed(&self) -> usize {
        self._files_processed
    }

    pub fn set_files_processed(&mut self, files_processed: usize) {
        self._files_processed = files_processed;
    }

    pub fn total_files(&self) -> usize {
        self._total_files
    }

    pub fn bytes_processed(&self) -> u64 {
        self._bytes_processed
    }

    pub fn set_bytes_processed(&mut self, _bytes_processed: u64) {
        self._bytes_processed = _bytes_processed;
    }

    pub fn total_bytes(&self) -> u64 {
        self._total_bytes
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

    pub fn start(&self, tx: mpsc::Sender<IoWorkerProgress>) -> io::Result<IoWorkerProgress> {
        match self.kind() {
            FileOp::Cut => self.paste_cut(tx),
            FileOp::Copy => self.paste_copy(tx),
        }
    }

    fn query_number_of_items(&self) -> io::Result<(usize, u64)> {
        let mut total_bytes = 0;
        let mut total_files = 0;

        let mut dirs: VecDeque<path::PathBuf> = VecDeque::new();
        for path in self.paths.iter() {
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

    fn paste_copy(&self, tx: mpsc::Sender<IoWorkerProgress>) -> io::Result<IoWorkerProgress> {
        let (total_files, total_bytes) = self.query_number_of_items()?;
        let mut progress = IoWorkerProgress::new(self.kind(), 0, total_files, 0, total_bytes);
        for path in self.paths.iter() {
            let _ = tx.send(progress.clone());
            recursive_copy(
                self.options,
                path.as_path(),
                self.dest.as_path(),
                tx.clone(),
                &mut progress,
            )?;
        }
        Ok(progress)
    }

    fn paste_cut(&self, tx: mpsc::Sender<IoWorkerProgress>) -> io::Result<IoWorkerProgress> {
        let (total_files, total_bytes) = self.query_number_of_items()?;
        let mut progress = IoWorkerProgress::new(self.kind(), 0, total_files, 0, total_bytes);

        for path in self.paths.iter() {
            let _ = tx.send(progress.clone());
            recursive_cut(
                self.options,
                path.as_path(),
                self.dest.as_path(),
                tx.clone(),
                &mut progress,
            )?;
        }
        Ok(progress)
    }
}

pub fn recursive_copy(
    options: IoWorkerOptions,
    src: &path::Path,
    dest: &path::Path,
    tx: mpsc::Sender<IoWorkerProgress>,
    progress: &mut IoWorkerProgress,
) -> io::Result<()> {
    let mut dest_buf = dest.to_path_buf();
    if let Some(s) = src.file_name() {
        dest_buf.push(s);
    }
    if !options.overwrite {
        rename_filename_conflict(&mut dest_buf);
    }
    let file_type = fs::symlink_metadata(src)?.file_type();
    if file_type.is_dir() {
        match fs::create_dir(dest_buf.as_path()) {
            Ok(_) => {},
            e => if !options.overwrite {
                return e;
            },
        }
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            recursive_copy(
                options,
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
        progress.set_files_processed(progress.files_processed() + 1);
        Ok(())
    } else if file_type.is_symlink() {
        let link_path = fs::read_link(src)?;
        std::os::unix::fs::symlink(link_path, dest_buf)?;
        progress.set_files_processed(progress.files_processed() + 1);
        Ok(())
    } else {
        Ok(())
    }
}

pub fn recursive_cut(
    options: IoWorkerOptions,
    src: &path::Path,
    dest: &path::Path,
    tx: mpsc::Sender<IoWorkerProgress>,
    progress: &mut IoWorkerProgress,
) -> io::Result<()> {
    let mut dest_buf = dest.to_path_buf();
    if let Some(s) = src.file_name() {
        dest_buf.push(s);
    }
    if !options.overwrite {
        rename_filename_conflict(&mut dest_buf);
    }
    let metadata = fs::symlink_metadata(src)?;
    let file_type = metadata.file_type();

    match fs::rename(src, dest_buf.as_path()) {
        Ok(_) => {
            let bytes_processed = progress.bytes_processed() + metadata.len();
            progress.set_bytes_processed(bytes_processed);
            progress.set_files_processed(progress.files_processed() + 1);
            Ok(())
        }
        Err(e) if e.kind() == io::ErrorKind::Other => {
            if file_type.is_dir() {
                fs::create_dir(dest_buf.as_path())?;
                for entry in fs::read_dir(src)? {
                    let entry_path = entry?.path();
                    recursive_cut(
                        options,
                        entry_path.as_path(),
                        dest_buf.as_path(),
                        tx.clone(),
                        progress,
                    )?;
                }
                fs::remove_dir(src)?;
            } else if file_type.is_symlink() {
                let link_path = fs::read_link(src)?;
                std::os::unix::fs::symlink(link_path, dest_buf)?;
                fs::remove_file(src)?;
                let processed = progress.bytes_processed() + metadata.len();
                progress.set_bytes_processed(processed);
                progress.set_files_processed(progress.files_processed() + 1);
            } else {
                let processed = progress.bytes_processed() + fs::copy(src, dest_buf.as_path())?;
                fs::remove_file(src)?;
                progress.set_bytes_processed(processed);
                progress.set_files_processed(progress.files_processed() + 1);
            }
            Ok(())
        }
        e => e,
    }
}
