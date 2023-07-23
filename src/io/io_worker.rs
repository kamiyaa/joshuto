use std::fs;
use std::io;
use std::path;
use std::sync::mpsc;

#[cfg(unix)]
use std::os::unix;

use crate::io::{FileOperation, FileOperationOptions, FileOperationProgress};
use crate::util::fs::query_number_of_items;
use crate::util::name_resolution::rename_filename_conflict;

#[derive(Clone, Debug)]
pub struct IoWorkerThread {
    _kind: FileOperation,
    pub options: FileOperationOptions,
    pub paths: Vec<path::PathBuf>,
    pub dest: path::PathBuf,
}

impl IoWorkerThread {
    pub fn new(
        _kind: FileOperation,
        paths: Vec<path::PathBuf>,
        dest: path::PathBuf,
        options: FileOperationOptions,
    ) -> Self {
        Self {
            _kind,
            options,
            paths,
            dest,
        }
    }

    pub fn kind(&self) -> FileOperation {
        self._kind
    }

    pub fn start(
        &self,
        tx: mpsc::Sender<FileOperationProgress>,
    ) -> io::Result<FileOperationProgress> {
        match self.kind() {
            FileOperation::Cut => self.paste_cut(tx),
            FileOperation::Copy => self.paste_copy(tx),
            FileOperation::Symlink { relative: false } => self.paste_link_absolute(tx),
            FileOperation::Symlink { relative: true } => self.paste_link_relative(tx),
            FileOperation::Delete => self.delete(tx),
        }
    }

    fn paste_copy(
        &self,
        tx: mpsc::Sender<FileOperationProgress>,
    ) -> io::Result<FileOperationProgress> {
        let (total_files, total_bytes) = query_number_of_items(&self.paths)?;
        let mut progress = FileOperationProgress::new(
            self.kind(),
            self.paths[0].to_path_buf(),
            0,
            total_files,
            0,
            total_bytes,
        );
        for path in self.paths.iter() {
            let _ = tx.send(progress.clone());
            recursive_copy(
                &tx,
                path.as_path(),
                self.dest.as_path(),
                self.options,
                &mut progress,
            )?;
        }
        Ok(progress)
    }

    fn paste_cut(
        &self,
        tx: mpsc::Sender<FileOperationProgress>,
    ) -> io::Result<FileOperationProgress> {
        let (total_files, total_bytes) = query_number_of_items(&self.paths)?;
        let mut progress = FileOperationProgress::new(
            self.kind(),
            self.paths[0].to_path_buf(),
            0,
            total_files,
            0,
            total_bytes,
        );
        for path in self.paths.iter() {
            let _ = tx.send(progress.clone());
            recursive_cut(
                &tx,
                path.as_path(),
                self.dest.as_path(),
                self.options,
                &mut progress,
            )?;
        }
        Ok(progress)
    }

    fn paste_link_absolute(
        &self,
        tx: mpsc::Sender<FileOperationProgress>,
    ) -> io::Result<FileOperationProgress> {
        let total_files = self.paths.len();
        let total_bytes = total_files as u64;
        let mut progress = FileOperationProgress::new(
            self.kind(),
            self.paths[0].to_path_buf(),
            0,
            total_files,
            0,
            total_bytes,
        );

        #[cfg(unix)]
        for src in self.paths.iter() {
            let _ = tx.send(progress.clone());
            let mut dest_buf = self.dest.to_path_buf();
            if let Some(s) = src.file_name() {
                dest_buf.push(s);
            }
            if !self.options.overwrite {
                rename_filename_conflict(&mut dest_buf);
            }
            unix::fs::symlink(src, &dest_buf)?;
            progress.set_files_processed(progress.files_processed() + 1);
            progress.set_bytes_processed(progress.bytes_processed() + 1);
        }
        Ok(progress)
    }

    fn paste_link_relative(
        &self,
        tx: mpsc::Sender<FileOperationProgress>,
    ) -> io::Result<FileOperationProgress> {
        let total_files = self.paths.len();
        let total_bytes = total_files as u64;
        let mut progress = FileOperationProgress::new(
            self.kind(),
            self.paths[0].to_path_buf(),
            0,
            total_files,
            0,
            total_bytes,
        );

        #[cfg(unix)]
        for src in self.paths.iter() {
            let _ = tx.send(progress.clone());
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

            progress.set_files_processed(progress.files_processed() + 1);
            progress.set_bytes_processed(progress.bytes_processed() + 1);
        }
        Ok(progress)
    }

    fn delete(
        &self,
        _tx: mpsc::Sender<FileOperationProgress>,
    ) -> io::Result<FileOperationProgress> {
        let (total_files, total_bytes) = query_number_of_items(&self.paths)?;
        let progress = FileOperationProgress::new(
            self.kind(),
            self.paths[0].to_path_buf(),
            total_files,
            total_files,
            total_bytes,
            total_bytes,
        );
        #[cfg(feature = "recycle_bin")]
        if self.options.permanently {
            remove_files(&self.paths)?;
        } else {
            trash_files(&self.paths)?;
        }
        #[cfg(not(feature = "recycle_bin"))]
        {
            remove_files(&self.paths)?;
        }

        Ok(progress)
    }
}

pub fn recursive_copy(
    tx: &mpsc::Sender<FileOperationProgress>,
    src: &path::Path,
    dest: &path::Path,
    options: FileOperationOptions,
    progress: &mut FileOperationProgress,
) -> io::Result<()> {
    let mut dest_buf = dest.to_path_buf();
    if let Some(s) = src.file_name() {
        dest_buf.push(s);
    }
    if !options.overwrite {
        rename_filename_conflict(&mut dest_buf);
    }

    progress.set_current_file(src.to_path_buf());

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
            recursive_copy(
                tx,
                entry_path.as_path(),
                dest_buf.as_path(),
                options,
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
    tx: &mpsc::Sender<FileOperationProgress>,
    src: &path::Path,
    dest: &path::Path,
    options: FileOperationOptions,
    progress: &mut FileOperationProgress,
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

    progress.set_current_file(src.to_path_buf());

    match fs::rename(src, dest_buf.as_path()) {
        Ok(_) => {
            let bytes_processed = progress.bytes_processed() + metadata.len();
            progress.set_bytes_processed(bytes_processed);
            progress.set_files_processed(progress.files_processed() + 1);
            Ok(())
        }
        Err(_e) => {
            if file_type.is_dir() {
                fs::create_dir(dest_buf.as_path())?;
                for entry in fs::read_dir(src)? {
                    let entry_path = entry?.path();
                    recursive_cut(
                        tx,
                        entry_path.as_path(),
                        dest_buf.as_path(),
                        options,
                        progress,
                    )?;
                    let _ = tx.send(progress.clone());
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
    }
}

#[cfg(feature = "recycle_bin")]
fn trash_error_to_io_error(err: ::Error) -> std::io::Error {
    match err {
        trash::Error::Unknown { description } => {
            std::io::Error::new(std::io::ErrorKind::Other, description)
        }
        trash::Error::TargetedRoot => {
            std::io::Error::new(std::io::ErrorKind::Other, "Targeted Root")
        }
        _ => std::io::Error::new(std::io::ErrorKind::Other, "Unknown Error"),
    }
}

fn remove_files<P>(paths: &[P]) -> std::io::Result<()>
where
    P: AsRef<path::Path>,
{
    for path in paths {
        if let Ok(metadata) = fs::symlink_metadata(path) {
            if metadata.is_dir() {
                fs::remove_dir_all(path)?;
            } else {
                fs::remove_file(path)?;
            }
        }
    }
    Ok(())
}

#[cfg(feature = "recycle_bin")]
fn trash_files<P>(paths: &[P]) -> std::io::Result<()>
where
    P: AsRef<path::Path>,
{
    for path in paths {
        if let Err(e) = trash::delete(path) {
            return Err(trash_error_to_io_error(e));
        }
    }
    Ok(())
}
