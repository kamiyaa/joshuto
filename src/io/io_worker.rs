use std::fs;
use std::path;
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

#[derive(Debug)]
pub struct IOWorkerObserver {
    msg: Option<String>,
    pub handle: thread::JoinHandle<()>,
    src: path::PathBuf,
    dest: path::PathBuf,
}

impl IOWorkerObserver {
    pub fn new(handle: thread::JoinHandle<()>, src: path::PathBuf, dest: path::PathBuf) -> Self {
        Self {
            handle,
            src,
            dest,
            msg: None,
        }
    }

    pub fn join(self) {
        self.handle.join();
    }
    pub fn set_msg(&mut self, msg: String) {
        self.msg = Some(msg)
    }
    pub fn get_msg(&self) -> Option<&String> {
        self.msg.as_ref()
    }
    pub fn clear_msg(&mut self) {
        self.msg = None
    }
    pub fn get_src_path(&self) -> &path::Path {
        self.src.as_path()
    }
    pub fn get_dest_path(&self) -> &path::Path {
        self.dest.as_path()
    }
}

#[derive(Debug)]
pub struct IOWorkerThread {
    pub options: IOWorkerOptions,
    pub paths: Vec<path::PathBuf>,
    pub dest: path::PathBuf,
}

impl IOWorkerThread {
    pub fn new(options: IOWorkerOptions, paths: Vec<path::PathBuf>, dest: path::PathBuf) -> Self {
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
            total += self.recursive_copy(path.as_path(), self.dest.as_path())?;
            tx.send(total);
        }
        Ok(total)
    }

    fn recursive_copy(&self, src: &path::Path, dest: &path::Path) -> std::io::Result<u64> {
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
                total += self.recursive_copy(entry_path.as_path(), dest_buf.as_path())?;
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
            total += self.recursive_cut(path.as_path(), self.dest.as_path())?;
            tx.send(total);
        }
        Ok(total)
    }

    pub fn recursive_cut(&self, src: &path::Path, dest: &path::Path) -> std::io::Result<u64> {
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
                Err(e) => {
                    let mut total = 0;
                    fs::create_dir(dest_buf.as_path())?;
                    for entry in fs::read_dir(src)? {
                        let entry = entry?;
                        let entry_path = entry.path();
                        total += self.recursive_cut(entry_path.as_path(), dest_buf.as_path())?;
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
