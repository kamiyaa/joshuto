use std::{fs, io, path, time};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FileType {
    Directory,
    File,
    // Unix specific
    Link,
    Socket,
    Block,
    Character,
    Pipe,
}

#[allow(clippy::unnecessary_cast)]
const LIBC_FILE_VALS: [(u32, FileType); 7] = [
    (libc::S_IFREG as u32 >> 9, FileType::File),
    (libc::S_IFDIR as u32 >> 9, FileType::Directory),
    (libc::S_IFLNK as u32 >> 9, FileType::Link),
    (libc::S_IFSOCK as u32 >> 9, FileType::Socket),
    (libc::S_IFBLK as u32 >> 9, FileType::Block),
    (libc::S_IFCHR as u32 >> 9, FileType::Character),
    (libc::S_IFIFO as u32 >> 9, FileType::Pipe),
];

impl From<u32> for FileType {
    fn from(value: u32) -> Self {
        Self::from_mode(value)
    }
}

impl FileType {
    pub fn from_mode(mode: u32) -> Self {
        let mode_shifted = mode >> 9;
        for (val, ch) in LIBC_FILE_VALS.iter() {
            if mode_shifted & (u32::MAX - 1) == *val {
                return *ch;
            }
        }
        FileType::File
    }
}

#[derive(Clone, Debug)]
pub enum LinkType {
    Normal,
    Symlink { target: String, valid: bool },
}

#[derive(Clone, Debug)]
pub struct JoshutoMetadata {
    pub len: u64,
    pub directory_size: Option<usize>,
    pub modified: time::SystemTime,
    pub accessed: time::SystemTime,
    pub created: time::SystemTime,
    pub permissions: fs::Permissions,
    pub file_type: FileType,
    pub link_type: LinkType,
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

        let directory_size = None;
        let file_type = match metadata.as_ref() {
            Ok(metadata) => FileType::from_mode(metadata.mode()),
            _ => FileType::File,
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

    pub fn file_type(&self) -> FileType {
        self.file_type
    }

    pub fn link_type(&self) -> &LinkType {
        &self.link_type
    }

    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::Directory
    }
}
