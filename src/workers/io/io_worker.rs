use std::fs;
use std::io;
use std::path;
use std::process::{Command, Stdio};
use std::sync::mpsc;

#[cfg(unix)]
use std::os::unix;

use crate::error::AppError;
use crate::error::AppErrorKind;
use crate::error::AppResult;
use crate::utils::name_resolution::rename_filename_conflict;

use super::{FileOperation, FileOperationOptions, IoWorkerProgressMessage};

#[derive(Clone, Debug)]
pub struct IoWorkerThread {
    pub operation: FileOperation,
    pub options: FileOperationOptions,
    pub paths: Vec<path::PathBuf>,
    pub dest: path::PathBuf,
}

impl IoWorkerThread {
    pub fn new(
        operation: FileOperation,
        paths: Vec<path::PathBuf>,
        dest: path::PathBuf,
        options: FileOperationOptions,
    ) -> Self {
        Self {
            operation,
            options,
            paths,
            dest,
        }
    }

    pub fn get_operation_type(&self) -> FileOperation {
        self.operation
    }

    pub fn start(&self, tx: mpsc::Sender<IoWorkerProgressMessage>) -> AppResult<()> {
        match self.get_operation_type() {
            FileOperation::Cut => self.paste_cut(tx),
            FileOperation::Copy => self.paste_copy(tx),
            FileOperation::Symlink { relative: false } => self.paste_link_absolute(tx),
            FileOperation::Symlink { relative: true } => self.paste_link_relative(tx),
            FileOperation::Delete => self.delete(tx),
        }
    }

    fn paste_copy(&self, tx: mpsc::Sender<IoWorkerProgressMessage>) -> AppResult<()> {
        for path in self.paths.iter() {
            recursive_copy(&tx, path.as_path(), self.dest.as_path(), self.options)?;
        }
        Ok(())
    }

    fn paste_cut(&self, tx: mpsc::Sender<IoWorkerProgressMessage>) -> AppResult<()> {
        for path in self.paths.iter() {
            recursive_cut(&tx, path.as_path(), self.dest.as_path(), self.options)?;
        }
        Ok(())
    }

    fn paste_link_absolute(&self, tx: mpsc::Sender<IoWorkerProgressMessage>) -> AppResult<()> {
        #[cfg(unix)]
        for src in self.paths.iter() {
            let _ = tx.send(IoWorkerProgressMessage::FileStart {
                file_path: src.to_path_buf(),
            });
            let mut dest_buf = self.dest.to_path_buf();
            if let Some(s) = src.file_name() {
                dest_buf.push(s);
            }
            if !self.options.overwrite {
                rename_filename_conflict(&mut dest_buf);
            }
            unix::fs::symlink(src, &dest_buf)?;
            let _ = tx.send(IoWorkerProgressMessage::FileComplete { file_size: 1 });
        }
        Ok(())
    }

    fn paste_link_relative(&self, tx: mpsc::Sender<IoWorkerProgressMessage>) -> AppResult<()> {
        #[cfg(unix)]
        for src in self.paths.iter() {
            let _ = tx.send(IoWorkerProgressMessage::FileStart {
                file_path: src.to_path_buf(),
            });
            let mut dest_buf = self.dest.to_path_buf();
            if let Some(s) = src.file_name() {
                dest_buf.push(s);
            }
            if !self.options.overwrite {
                rename_filename_conflict(&mut dest_buf);
            }
            let mut src_components = src.components();
            let mut dest_components = dest_buf.components();

            // skip to where the two paths diverge
            let mut non_relative_path = path::PathBuf::new();
            for (s, d) in src_components.by_ref().zip(dest_components.by_ref()) {
                if s != d {
                    non_relative_path.push(s);
                    break;
                }
            }

            let mut relative_path = path::PathBuf::new();
            for _ in dest_components {
                relative_path.push("..");
            }
            relative_path.push(non_relative_path);
            for s in src_components {
                relative_path.push(s);
            }
            unix::fs::symlink(relative_path, &dest_buf)?;

            let _ = tx.send(IoWorkerProgressMessage::FileComplete { file_size: 1 });
        }
        Ok(())
    }

    fn delete(&self, tx: mpsc::Sender<IoWorkerProgressMessage>) -> AppResult<()> {
        if self.options.permanently {
            remove_files(&self.paths, tx)?;
        } else {
            trash_files(&self.paths, tx)?;
        }
        Ok(())
    }
}

pub fn recursive_copy(
    tx: &mpsc::Sender<IoWorkerProgressMessage>,
    src: &path::Path,
    dest: &path::Path,
    options: FileOperationOptions,
) -> io::Result<()> {
    let _ = tx.send(IoWorkerProgressMessage::FileStart {
        file_path: src.to_path_buf(),
    });

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
            Ok(_) => {}
            e => {
                if !options.overwrite {
                    return e;
                }
            }
        }
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            recursive_copy(tx, entry_path.as_path(), dest_buf.as_path(), options)?;
        }
        let _ = tx.send(IoWorkerProgressMessage::FileComplete { file_size: 1 });
        Ok(())
    } else if file_type.is_file() {
        let bytes_processed = fs::copy(src, dest_buf)?;
        let _ = tx.send(IoWorkerProgressMessage::FileComplete {
            file_size: bytes_processed,
        });
        Ok(())
    } else if file_type.is_symlink() {
        let link_path = fs::read_link(src)?;
        std::os::unix::fs::symlink(link_path, dest_buf)?;
        let _ = tx.send(IoWorkerProgressMessage::FileComplete { file_size: 1 });
        Ok(())
    } else {
        Ok(())
    }
}

pub fn recursive_cut(
    tx: &mpsc::Sender<IoWorkerProgressMessage>,
    src: &path::Path,
    dest: &path::Path,
    options: FileOperationOptions,
) -> io::Result<()> {
    let _ = tx.send(IoWorkerProgressMessage::FileStart {
        file_path: src.to_path_buf(),
    });

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
            let bytes_processed = metadata.len();
            let _ = tx.send(IoWorkerProgressMessage::FileComplete {
                file_size: bytes_processed,
            });
            Ok(())
        }
        Err(_e) => {
            if file_type.is_dir() {
                fs::create_dir(dest_buf.as_path())?;
                for entry in fs::read_dir(src)? {
                    let entry_path = entry?.path();
                    recursive_cut(tx, entry_path.as_path(), dest_buf.as_path(), options)?;
                }
                fs::remove_dir(src)?;
                let _ = tx.send(IoWorkerProgressMessage::FileComplete { file_size: 1 });
            } else if file_type.is_symlink() {
                let link_path = fs::read_link(src)?;
                std::os::unix::fs::symlink(link_path, dest_buf)?;
                fs::remove_file(src)?;

                let bytes_processed = metadata.len();
                let _ = tx.send(IoWorkerProgressMessage::FileComplete {
                    file_size: bytes_processed,
                });
            } else {
                let bytes_processed = fs::copy(src, dest_buf.as_path())?;
                fs::remove_file(src)?;

                let _ = tx.send(IoWorkerProgressMessage::FileComplete {
                    file_size: bytes_processed,
                });
            }
            Ok(())
        }
    }
}

fn remove_files<P>(paths: &[P], tx: mpsc::Sender<IoWorkerProgressMessage>) -> std::io::Result<()>
where
    P: AsRef<path::Path>,
{
    for path in paths {
        if let Ok(metadata) = fs::symlink_metadata(path) {
            let _ = tx.send(IoWorkerProgressMessage::FileStart {
                file_path: path.as_ref().to_path_buf(),
            });
            if metadata.is_dir() {
                fs::remove_dir_all(path)?;
            } else {
                fs::remove_file(path)?;
            }
            let bytes_processed = metadata.len();
            let _ = tx.send(IoWorkerProgressMessage::FileComplete {
                file_size: bytes_processed,
            });
        }
    }
    Ok(())
}

fn trash_files<P>(paths: &[P], tx: mpsc::Sender<IoWorkerProgressMessage>) -> AppResult
where
    P: AsRef<path::Path>,
{
    for path in paths {
        let _ = tx.send(IoWorkerProgressMessage::FileStart {
            file_path: path.as_ref().to_path_buf(),
        });
        trash_file(path)?;
        let _ = tx.send(IoWorkerProgressMessage::FileComplete { file_size: 1 });
    }
    Ok(())
}

fn trash_file<P>(file_path: P) -> AppResult
where
    P: AsRef<path::Path>,
{
    let file_path_str = file_path
        .as_ref()
        .as_os_str()
        .to_string_lossy()
        .replace('\'', "'\\''");

    let clipboards = [
        ("gio trash", format!("gio trash -- '{}'", file_path_str)),
        ("trash-put", format!("trash-put '{}'", file_path_str)),
        ("trash", format!("trash '{}'", file_path_str)),
        ("gtrash put", format!("gtrash put -- '{}'", file_path_str)),
    ];

    for (_, cmd) in clipboards.iter() {
        let status = Command::new("sh")
            .args(["-c", cmd.as_str()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        match status {
            Ok(s) if s.success() => return Ok(()),
            _ => {}
        }
    }
    Err(AppError::new(
        AppErrorKind::Trash,
        "Failed to trash file".to_string(),
    ))
}
