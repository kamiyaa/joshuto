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
use crate::types::event::AppEvent;
use crate::types::io::FileOperationProgress;
use crate::types::io::IoTaskStat;
use crate::types::io::{FileOperation, FileOperationOptions, IoTask, IoTaskProgressMessage};
use crate::utils::fs::query_number_of_items;
use crate::utils::name_resolution::rename_filename_conflict;

pub fn process_io_tasks(
    event_rx: mpsc::Receiver<IoTask>,
    event_tx: mpsc::Sender<AppEvent>,
) -> AppResult<()> {
    while let Ok(io_task) = event_rx.recv() {
        let res = process_io_task(&io_task, &event_tx);
        let event = AppEvent::IoTaskResult(res);
        let _ = event_tx.send(event);
    }
    Ok(())
}

pub fn process_io_task(io_task: &IoTask, event_tx: &mpsc::Sender<AppEvent>) -> AppResult<()> {
    let (total_files, total_bytes) = query_number_of_items(io_task.paths.as_slice())?;
    let src = io_task.paths[0].parent().unwrap().to_path_buf();
    let dest = io_task.dest.clone();

    let operation_progress = FileOperationProgress {
        kind: io_task.operation,
        current_file: io_task.paths[0].clone(),
        total_files,
        files_processed: 0,
        total_bytes,
        bytes_processed: 0,
    };

    let io_stat = IoTaskStat::new(operation_progress, src, dest);
    let event = AppEvent::IoTaskStart(io_stat);
    let _ = event_tx.send(event);

    let res = match io_task.get_operation_type() {
        FileOperation::Cut => paste_cut(io_task, event_tx),
        FileOperation::Copy => paste_copy(io_task, event_tx),
        FileOperation::Symlink { relative: false } => paste_link_absolute(io_task, event_tx),
        FileOperation::Symlink { relative: true } => paste_link_relative(io_task, event_tx),
        FileOperation::Delete => delete(io_task, event_tx),
    };
    res?;
    Ok(())
}

fn paste_copy(task: &IoTask, tx: &mpsc::Sender<AppEvent>) -> AppResult<()> {
    for path in task.paths.iter() {
        recursive_copy(tx, path.as_path(), task.dest.as_path(), task.options)?;
    }
    Ok(())
}

fn paste_cut(task: &IoTask, tx: &mpsc::Sender<AppEvent>) -> AppResult<()> {
    for path in task.paths.iter() {
        recursive_cut(tx, path.as_path(), task.dest.as_path(), task.options)?;
    }
    Ok(())
}

fn paste_link_absolute(task: &IoTask, tx: &mpsc::Sender<AppEvent>) -> AppResult<()> {
    #[cfg(unix)]
    for src in task.paths.iter() {
        let event = IoTaskProgressMessage::FileStart {
            file_path: src.to_path_buf(),
        };
        let _ = tx.send(AppEvent::IoTaskProgress(event));
        let mut dest_buf = task.dest.to_path_buf();
        if let Some(s) = src.file_name() {
            dest_buf.push(s);
        }
        if !task.options.overwrite {
            rename_filename_conflict(&mut dest_buf);
        }
        unix::fs::symlink(src, &dest_buf)?;
        let event = IoTaskProgressMessage::FileComplete { file_size: 1 };
        let _ = tx.send(AppEvent::IoTaskProgress(event));
    }
    Ok(())
}

fn paste_link_relative(task: &IoTask, tx: &mpsc::Sender<AppEvent>) -> AppResult<()> {
    #[cfg(unix)]
    for src in task.paths.iter() {
        let event = IoTaskProgressMessage::FileStart {
            file_path: src.to_path_buf(),
        };
        let _ = tx.send(AppEvent::IoTaskProgress(event));
        let mut dest_buf = task.dest.to_path_buf();
        if let Some(s) = src.file_name() {
            dest_buf.push(s);
        }
        if !task.options.overwrite {
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

        let event = IoTaskProgressMessage::FileComplete { file_size: 1 };
        let _ = tx.send(AppEvent::IoTaskProgress(event));
    }
    Ok(())
}

fn delete(task: &IoTask, tx: &mpsc::Sender<AppEvent>) -> AppResult<()> {
    if task.options.permanently {
        remove_files(&task.paths, tx)?;
    } else {
        trash_files(&task.paths, tx)?;
    }
    Ok(())
}

pub fn recursive_copy(
    tx: &mpsc::Sender<AppEvent>,
    src: &path::Path,
    dest: &path::Path,
    options: FileOperationOptions,
) -> io::Result<()> {
    let event = IoTaskProgressMessage::FileStart {
        file_path: src.to_path_buf(),
    };
    let _ = tx.send(AppEvent::IoTaskProgress(event));

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
        let event = IoTaskProgressMessage::FileComplete { file_size: 1 };
        let _ = tx.send(AppEvent::IoTaskProgress(event));

        Ok(())
    } else if file_type.is_file() {
        let bytes_processed = fs::copy(src, dest_buf)?;
        let event = IoTaskProgressMessage::FileComplete {
            file_size: bytes_processed,
        };
        let _ = tx.send(AppEvent::IoTaskProgress(event));

        Ok(())
    } else if file_type.is_symlink() {
        let link_path = fs::read_link(src)?;
        std::os::unix::fs::symlink(link_path, dest_buf)?;
        let event = IoTaskProgressMessage::FileComplete { file_size: 1 };
        let _ = tx.send(AppEvent::IoTaskProgress(event));

        Ok(())
    } else {
        Ok(())
    }
}

pub fn recursive_cut(
    tx: &mpsc::Sender<AppEvent>,
    src: &path::Path,
    dest: &path::Path,
    options: FileOperationOptions,
) -> io::Result<()> {
    let event = IoTaskProgressMessage::FileStart {
        file_path: src.to_path_buf(),
    };
    let _ = tx.send(AppEvent::IoTaskProgress(event));

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
            let event = IoTaskProgressMessage::FileComplete {
                file_size: bytes_processed,
            };
            let _ = tx.send(AppEvent::IoTaskProgress(event));
        }
        Err(_err) => {
            if file_type.is_dir() {
                fs::create_dir(dest_buf.as_path())?;
                for entry in fs::read_dir(src)? {
                    let entry_path = entry?.path();
                    recursive_cut(tx, entry_path.as_path(), dest_buf.as_path(), options)?;
                }
                fs::remove_dir(src)?;
                let event = IoTaskProgressMessage::FileComplete { file_size: 1 };
                let _ = tx.send(AppEvent::IoTaskProgress(event));
            } else if file_type.is_symlink() {
                let link_path = fs::read_link(src)?;
                std::os::unix::fs::symlink(link_path, dest_buf)?;
                fs::remove_file(src)?;

                let bytes_processed = metadata.len();
                let event = IoTaskProgressMessage::FileComplete {
                    file_size: bytes_processed,
                };
                let _ = tx.send(AppEvent::IoTaskProgress(event));
            } else {
                let bytes_processed = fs::copy(src, dest_buf.as_path())?;
                fs::remove_file(src)?;

                let event = IoTaskProgressMessage::FileComplete {
                    file_size: bytes_processed,
                };
                let _ = tx.send(AppEvent::IoTaskProgress(event));
            }
        }
    }
    Ok(())
}

fn remove_files<P>(paths: &[P], tx: &mpsc::Sender<AppEvent>) -> std::io::Result<()>
where
    P: AsRef<path::Path>,
{
    for path in paths {
        if let Ok(metadata) = fs::symlink_metadata(path) {
            let event = IoTaskProgressMessage::FileStart {
                file_path: path.as_ref().to_path_buf(),
            };
            let _ = tx.send(AppEvent::IoTaskProgress(event));

            if metadata.is_dir() {
                fs::remove_dir_all(path)?;
            } else {
                fs::remove_file(path)?;
            }
            let bytes_processed = metadata.len();
            let event = IoTaskProgressMessage::FileComplete {
                file_size: bytes_processed,
            };
            let _ = tx.send(AppEvent::IoTaskProgress(event));
        }
    }
    Ok(())
}

fn trash_files<P>(paths: &[P], tx: &mpsc::Sender<AppEvent>) -> AppResult
where
    P: AsRef<path::Path>,
{
    for path in paths {
        let event = IoTaskProgressMessage::FileStart {
            file_path: path.as_ref().to_path_buf(),
        };
        let _ = tx.send(AppEvent::IoTaskProgress(event));

        trash_file(path)?;
        let event = IoTaskProgressMessage::FileComplete { file_size: 1 };
        let _ = tx.send(AppEvent::IoTaskProgress(event));
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
