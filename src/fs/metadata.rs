use std::{fs, io, path, time};

#[derive(Clone, Debug)]
pub enum FileType {
    Directory,
    Symlink(String),
    File,
}

#[derive(Clone, Debug)]
pub struct JoshutoMetadata {
    _len: u64,
    _modified: time::SystemTime,
    _permissions: fs::Permissions,
    _file_type: FileType,
    #[cfg(unix)]
    pub uid: u32,
    #[cfg(unix)]
    pub gid: u32,
    #[cfg(unix)]
    pub mode: u32,
}

impl JoshutoMetadata {
    pub fn from(path: &path::Path) -> io::Result<Self> {
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;

        let metadata = fs::symlink_metadata(path)?;

        let _len = metadata.len();
        let _modified = metadata.modified()?;
        let _permissions = metadata.permissions();
        let file_type = metadata.file_type();

        let file_type = if file_type.is_dir() {
            FileType::Directory
        } else if file_type.is_symlink() {
            let mut link = "".to_string();

            if let Ok(path) = fs::read_link(path) {
                if let Some(s) = path.to_str() {
                    link = s.to_string();
                }
            }
            FileType::Symlink(link)
        } else {
            FileType::File
        };

        #[cfg(unix)]
        let uid = metadata.uid();
        #[cfg(unix)]
        let gid = metadata.gid();
        #[cfg(unix)]
        let mode = metadata.mode();

        Ok(Self {
            _len,
            _modified,
            _permissions,
            _file_type: file_type,
            #[cfg(unix)]
            uid,
            #[cfg(unix)]
            gid,
            #[cfg(unix)]
            mode,
        })
    }

    pub fn len(&self) -> u64 {
        self._len
    }

    pub fn modified(&self) -> time::SystemTime {
        self._modified
    }

    pub fn permissions_ref(&self) -> &fs::Permissions {
        &self._permissions
    }

    pub fn permissions_mut(&mut self) -> &mut fs::Permissions {
        &mut self._permissions
    }

    pub fn file_type(&self) -> &FileType {
        &self._file_type
    }
}
