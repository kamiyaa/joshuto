use std::fs;
use std::path;
use std::sync::mpsc;

use super::rename_filename_conflict;

#[derive(Clone, Copy, Debug)]
pub enum FileOp {
    Cut,
    Copy,
}

#[derive(Clone, Debug)]
pub struct IOWorkerOptions {
    pub overwrite: bool,
    pub skip_exist: bool,
}

impl std::default::Default for IOWorkerOptions {
    fn default() -> Self {
        Self {
            overwrite: false,
            skip_exist: false,
        }
    }
}

impl std::fmt::Display for IOWorkerOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "overwrite={} skip_exist={}",
            self.overwrite, self.skip_exist
        )
    }
}

#[derive(Clone, Debug)]
pub struct IOWorkerProgress {
    _kind: FileOp,
    _index: usize,
    _len: usize,
    _processed: u64,
}

impl IOWorkerProgress {
    pub fn new(_kind: FileOp, _index: usize, _len: usize, _processed: u64) -> Self {
        Self {
            _kind,
            _index,
            _len,
            _processed,
        }
    }

    pub fn kind(&self) -> FileOp {
        self._kind
    }

    pub fn index(&self) -> usize {
        self._index
    }

    pub fn set_index(&mut self, _index: usize) {
        self._index = _index;
    }

    pub fn len(&self) -> usize {
        self._len
    }

    pub fn processed(&self) -> u64 {
        self._processed
    }

    pub fn set_processed(&mut self, _processed: u64) {
        self._processed = _processed;
    }
}

#[derive(Debug)]
pub struct IOWorkerThread {
    _kind: FileOp,
    pub options: IOWorkerOptions,
    pub paths: Vec<path::PathBuf>,
    pub dest: path::PathBuf,
}

impl IOWorkerThread {
    pub fn new(
        _kind: FileOp,
        paths: Vec<path::PathBuf>,
        dest: path::PathBuf,
        options: IOWorkerOptions,
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

    pub fn start(&self, tx: mpsc::Sender<IOWorkerProgress>) -> std::io::Result<IOWorkerProgress> {
        match self.kind() {
            FileOp::Cut => self.paste_cut(tx),
            FileOp::Copy => self.paste_copy(tx),
        }
    }

    fn paste_copy(&self, tx: mpsc::Sender<IOWorkerProgress>) -> std::io::Result<IOWorkerProgress> {
        let mut progress = IOWorkerProgress::new(self.kind(), 0, self.paths.len(), 0);
        for (i, path) in self.paths.iter().enumerate() {
            progress.set_index(i);
            let _ = tx.send(progress.clone());
            self.recursive_copy(
                path.as_path(),
                self.dest.as_path(),
                tx.clone(),
                &mut progress,
            )?;
        }
        Ok(IOWorkerProgress::new(
            self.kind(),
            self.paths.len(),
            self.paths.len(),
            progress.processed(),
        ))
    }

    fn recursive_copy(
        &self,
        src: &path::Path,
        dest: &path::Path,
        tx: mpsc::Sender<IOWorkerProgress>,
        progress: &mut IOWorkerProgress,
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
                self.recursive_copy(
                    entry_path.as_path(),
                    dest_buf.as_path(),
                    tx.clone(),
                    progress,
                )?;
                let _ = tx.send(progress.clone());
            }
            Ok(())
        } else if file_type.is_file() {
            let processed = progress.processed() + fs::copy(src, dest_buf)?;
            progress.set_processed(processed);
            Ok(())
        } else if file_type.is_symlink() {
            let link_path = fs::read_link(src)?;
            std::os::unix::fs::symlink(link_path, dest_buf)?;
            Ok(())
        } else {
            Ok(())
        }
    }

    fn paste_cut(&self, tx: mpsc::Sender<IOWorkerProgress>) -> std::io::Result<IOWorkerProgress> {
        let mut progress = IOWorkerProgress::new(self.kind(), 0, self.paths.len(), 0);
        for (i, path) in self.paths.iter().enumerate() {
            progress.set_index(i);
            let _ = tx.send(progress.clone());
            self.recursive_cut(
                path.as_path(),
                self.dest.as_path(),
                tx.clone(),
                &mut progress,
            )?;
        }
        Ok(IOWorkerProgress::new(
            self.kind(),
            self.paths.len(),
            self.paths.len(),
            progress.processed(),
        ))
    }

    pub fn recursive_cut(
        &self,
        src: &path::Path,
        dest: &path::Path,
        tx: mpsc::Sender<IOWorkerProgress>,
        progress: &mut IOWorkerProgress,
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
                    let processed = progress.processed() + metadata.len();
                    progress.set_processed(processed);
                }
                Err(_) => {
                    fs::create_dir(dest_buf.as_path())?;
                    for entry in fs::read_dir(src)? {
                        let entry_path = entry?.path();
                        self.recursive_cut(
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
                let processed = progress.processed() + metadata.len();
                progress.set_processed(processed);
            }
        } else if file_type.is_symlink() {
            let link_path = fs::read_link(src)?;
            std::os::unix::fs::symlink(link_path, dest_buf)?;
            fs::remove_file(src)?;
            let processed = progress.processed() + metadata.len();
            progress.set_processed(processed);
        }
        Ok(())
    }
}
