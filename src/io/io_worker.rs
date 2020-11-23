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
    pub kind: FileOp,
    pub index: usize,
    pub len: usize,
    pub processed: u64,
}

#[derive(Debug)]
pub struct IOWorkerThread {
    pub kind: FileOp,
    pub options: IOWorkerOptions,
    pub paths: Vec<path::PathBuf>,
    pub dest: path::PathBuf,
}

impl IOWorkerThread {
    pub fn new(
        kind: FileOp,
        paths: Vec<path::PathBuf>,
        dest: path::PathBuf,
        options: IOWorkerOptions,
    ) -> Self {
        Self {
            kind,
            options,
            paths,
            dest,
        }
    }

    pub fn start(&self, tx: mpsc::Sender<IOWorkerProgress>) -> std::io::Result<IOWorkerProgress> {
        match self.kind {
            FileOp::Cut => self.paste_cut(tx),
            FileOp::Copy => self.paste_copy(tx),
        }
    }

    fn paste_copy(&self, tx: mpsc::Sender<IOWorkerProgress>) -> std::io::Result<IOWorkerProgress> {
        let mut progress = IOWorkerProgress {
            kind: self.kind,
            index: 0,
            processed: 0,
            len: self.paths.len(),
        };
        for (i, path) in self.paths.iter().enumerate() {
            progress.index = i;
            tx.send(progress.clone());
            self.recursive_copy(
                path.as_path(),
                self.dest.as_path(),
                tx.clone(),
                &mut progress,
            )?;
        }
        Ok(IOWorkerProgress {
            kind: self.kind,
            index: self.paths.len(),
            processed: progress.processed,
            len: self.paths.len(),
        })
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
                tx.send(progress.clone());
            }
            Ok(())
        } else if file_type.is_file() {
            progress.processed += fs::copy(src, dest_buf)?;
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
        let mut progress = IOWorkerProgress {
            kind: self.kind,
            index: 0,
            processed: 0,
            len: self.paths.len(),
        };
        for (i, path) in self.paths.iter().enumerate() {
            progress.index = i;
            tx.send(progress.clone());
            self.recursive_cut(
                path.as_path(),
                self.dest.as_path(),
                tx.clone(),
                &mut progress,
            )?;
        }
        Ok(IOWorkerProgress {
            kind: self.kind,
            index: self.paths.len(),
            processed: progress.processed,
            len: self.paths.len(),
        })
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
                    progress.processed += metadata.len();
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
                progress.processed += metadata.len();
            }
        } else if file_type.is_symlink() {
            let link_path = fs::read_link(src)?;
            std::os::unix::fs::symlink(link_path, dest_buf)?;
            fs::remove_file(src)?;
            progress.processed += metadata.len();
        }
        Ok(())
    }
}
