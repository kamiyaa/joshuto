use serde_derive::Deserialize;
use std::{fs, io, path, time};

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug, Deserialize)]
pub enum FileType {
    Directory,
    File,
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        *self == Self::Directory
    }
    #[allow(dead_code)]
    pub fn is_file(&self) -> bool {
        *self == Self::File
    }
}

#[derive(Clone, Debug)]
pub enum LinkType {
    Normal,
    Symlink(String, bool), // link target, link validity
}

#[derive(Clone, Debug)]
pub struct JoshutoMetadata {
    _len: u64,
    _directory_size: Option<usize>,
    _modified: time::SystemTime,
    _permissions: fs::Permissions,
    _file_type: FileType,
    _link_type: LinkType,
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

        let symlink_metadata = fs::symlink_metadata(path)?;
        let metadata = fs::metadata(path);
        let (_len, _modified, _permissions) = match metadata.as_ref() {
            Ok(m) => (m.len(), m.modified()?, m.permissions()),
            Err(_) => (
                symlink_metadata.len(),
                symlink_metadata.modified()?,
                symlink_metadata.permissions(),
            ),
        };

        let (_file_type, _directory_size) = match metadata.as_ref() {
            Ok(m) if m.file_type().is_dir() => (FileType::Directory, None),
            _ => (FileType::File, None),
        };

        let _link_type = if symlink_metadata.file_type().is_symlink() {
            let mut link = "".to_string();

            if let Ok(path) = fs::read_link(path) {
                if let Some(s) = path.to_str() {
                    link = s.to_string();
                }
            }

            let exists = path.exists();
            LinkType::Symlink(link, exists)
        } else {
            LinkType::Normal
        };

        #[cfg(unix)]
        let uid = symlink_metadata.uid();
        #[cfg(unix)]
        let gid = symlink_metadata.gid();
        #[cfg(unix)]
        let mode = symlink_metadata.mode();

        Ok(Self {
            _len,
            _directory_size,
            _modified,
            _permissions,
            _file_type,
            _link_type,
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

    pub fn directory_size(&self) -> Option<usize> {
        self._directory_size
    }

    pub fn update_directory_size(&mut self, size: usize) {
        self._directory_size = Some(size);
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

    pub fn link_type(&self) -> &LinkType {
        &self._link_type
    }

    pub fn is_dir(&self) -> bool {
        self._file_type == FileType::Directory
    }
}
