use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

use super::rename_filename_conflict;

#[derive(Clone, Copy, Debug)]
pub enum FileOp {
    Cut,
    Copy,
}

#[derive(Clone, Debug)]
pub struct IOWorkerOptions {
    pub kind: FileOp,
    pub overwrite: bool,
    pub skip_exist: bool,
}

impl std::default::Default for IOWorkerOptions {
    fn default() -> Self {
        Self {
            kind: FileOp::Copy,
            overwrite: false,
            skip_exist: false,
        }
    }
}

pub struct IOWorkerObserver {
    pub handle: thread::JoinHandle<()>,
    pub src: PathBuf,
    pub dest: PathBuf,
}

impl IOWorkerObserver {
    pub fn new(handle: thread::JoinHandle<()>, src: PathBuf, dest: PathBuf) -> Self {
        Self { handle, src, dest }
    }

    pub fn join(self) {
        self.handle.join();
    }
}

pub struct IOWorkerThread {
    pub options: IOWorkerOptions,
    pub paths: Vec<PathBuf>,
    pub dest: PathBuf,
}

impl IOWorkerThread {
    pub fn new(options: IOWorkerOptions, paths: Vec<PathBuf>, dest: PathBuf) -> Self {
        Self {
            options,
            paths,
            dest,
        }
    }

    pub fn start(&self, tx: mpsc::Sender<u64>) -> std::io::Result<u64> {
        match self.options.kind {
            FileOp::Cut => self.paste_cut(tx),
            FileOp::Copy => self.paste_copy(tx),
        }
    }

    fn paste_copy(&self, tx: mpsc::Sender<u64>) -> std::io::Result<u64> {
        let mut total = 0;
        for path in self.paths.iter() {
            total += self.recursive_copy(self.dest.as_path(), path.as_path())?;
            tx.send(total);
        }
        Ok(total)
    }

    fn recursive_copy(&self, dest: &Path, src: &Path) -> std::io::Result<u64> {
        let mut dest_buf = dest.to_path_buf();
        if let Some(s) = src.file_name() {
            dest_buf.push(s);
        }
        rename_filename_conflict(&mut dest_buf);
        let file_type = fs::symlink_metadata(src)?.file_type();
        if file_type.is_dir() {
            fs::create_dir(dest_buf.as_path())?;
            let mut total = 0;
            for entry in fs::read_dir(src)? {
                let entry = entry?;
                let entry_path = entry.path();
                total += self.recursive_copy(dest_buf.as_path(), entry_path.as_path())?;
            }
            Ok(total)
        } else if file_type.is_file() {
            fs::copy(src, dest_buf)
        } else if file_type.is_symlink() {
            let link_path = fs::read_link(src)?;
            std::os::unix::fs::symlink(link_path, dest_buf)?;
            Ok(0)
        } else {
            Ok(0)
        }
    }

    fn paste_cut(&self, tx: mpsc::Sender<u64>) -> std::io::Result<u64> {
        let mut total = 0;
        for path in self.paths.iter() {
            total += self.recursive_cut(self.dest.as_path(), path.as_path())?;
            tx.send(total);
        }
        Ok(total)
    }

    pub fn recursive_cut(&self, dest: &Path, src: &Path) -> std::io::Result<u64> {
        let mut dest_buf = dest.to_path_buf();
        if let Some(s) = src.file_name() {
            dest_buf.push(s);
        }
        rename_filename_conflict(&mut dest_buf);
        let metadata = fs::symlink_metadata(src)?;
        let file_type = metadata.file_type();
        if file_type.is_dir() {
            match fs::rename(src, dest_buf.as_path()) {
                Ok(_) => Ok(metadata.len()),
                Err(_) => {
                    let mut total = 0;
                    fs::create_dir(dest_buf.as_path())?;
                    for entry in fs::read_dir(src)? {
                        let entry = entry?;
                        let entry_path = entry.path();
                        total += self.recursive_cut(dest_buf.as_path(), entry_path.as_path())?;
                    }
                    fs::remove_dir(src)?;
                    Ok(total)
                }
            }
        } else if file_type.is_file() {
            if fs::rename(src, dest_buf.as_path()).is_err() {
                fs::copy(src, dest_buf.as_path())?;
                fs::remove_file(src)?;
            }
            Ok(metadata.len())
        } else if file_type.is_symlink() {
            let link_path = fs::read_link(src)?;
            std::os::unix::fs::symlink(link_path, dest_buf)?;
            fs::remove_file(src)?;
            Ok(metadata.len())
        } else {
            Ok(0)
        }
    }
}
