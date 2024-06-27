use std::{fs, io, path, time};

#[derive(Clone, Debug, PartialEq, Eq)]
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
    Symlink { target: String, valid: bool },
}

#[derive(Clone, Debug)]
pub struct JoshutoMetadata {
    len: u64,
    directory_size: Option<usize>,
    modified: time::SystemTime,
    accessed: time::SystemTime,
    created: time::SystemTime,
    permissions: fs::Permissions,
    file_type: FileType,
    link_type: LinkType,
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
        let (len, modified, accessed, created, permissions) = match metadata.as_ref() {
            Ok(m) => (
                m.len(),
                m.modified()?,
                m.accessed()?,
                m.created()?,
                m.permissions(),
            ),
            Err(_) => (
                symlink_metadata.len(),
                symlink_metadata.modified()?,
                symlink_metadata.accessed()?,
                symlink_metadata.created()?,
                symlink_metadata.permissions(),
            ),
        };

        let (file_type, directory_size) = match metadata.as_ref() {
            Ok(m) if m.file_type().is_dir() => (FileType::Directory, None),
            _ => (FileType::File, None),
        };

        let link_type = if symlink_metadata.file_type().is_symlink() {
            let mut link = "".to_string();

            if let Ok(path) = fs::read_link(path) {
                if let Some(s) = path.to_str() {
                    link = s.to_string();
                }
            }

            let exists = path.exists();
            LinkType::Symlink {
                target: link,
                valid: exists,
            }
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
            len,
            directory_size,
            modified,
            accessed,
            created,
            permissions,
            file_type,
            link_type,
            #[cfg(unix)]
            uid,
            #[cfg(unix)]
            gid,
            #[cfg(unix)]
            mode,
        })
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn directory_size(&self) -> Option<usize> {
        self.directory_size
    }

    pub fn update_directory_size(&mut self, size: usize) {
        self.directory_size = Some(size);
    }

    pub fn modified(&self) -> time::SystemTime {
        self.modified
    }

    pub fn accessed(&self) -> time::SystemTime {
        self.accessed
    }

    pub fn created(&self) -> time::SystemTime {
        self.created
    }

    pub fn permissions_ref(&self) -> &fs::Permissions {
        &self.permissions
    }

    pub fn permissions_mut(&mut self) -> &mut fs::Permissions {
        &mut self.permissions
    }

    pub fn file_type(&self) -> &FileType {
        &self.file_type
    }

    pub fn link_type(&self) -> &LinkType {
        &self.link_type
    }

    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::Directory
    }
}
