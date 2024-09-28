use std::{fs, io, path, time};

use nix::sys::stat::{Mode, SFlag};

#[cfg(target_os = "macos")]
use nix::sys::stat::mode_t;

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

impl From<SFlag> for FileType {
    fn from(value: SFlag) -> Self {
        Self::from_mode(value)
    }
}

impl FileType {
    pub fn from_mode(mode: SFlag) -> Self {
        match mode {
            SFlag::S_IFBLK => FileType::Block,
            SFlag::S_IFCHR => FileType::Character,
            SFlag::S_IFDIR => FileType::Directory,
            SFlag::S_IFIFO => FileType::Pipe,
            SFlag::S_IFLNK => FileType::Link,
            SFlag::S_IFSOCK => FileType::Socket,
            _ => FileType::File,
        }
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
    pub mode: Mode,
    pub file_type: FileType,
    pub link_type: LinkType,
    #[cfg(unix)]
    pub uid: u32,
    #[cfg(unix)]
    pub gid: u32,
}

impl JoshutoMetadata {
    pub fn from(path: &path::Path) -> io::Result<Self> {
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;

        let symlink_metadata = fs::symlink_metadata(path)?;
        let metadata = fs::metadata(path);
        let (len, modified, accessed) = match metadata.as_ref() {
            Ok(m) => (m.len(), m.modified()?, m.accessed()?),
            Err(_) => (
                symlink_metadata.len(),
                symlink_metadata.modified()?,
                symlink_metadata.accessed()?,
            ),
        };

        let directory_size = None;
        let (file_type, mode) = match metadata.as_ref() {
            Ok(metadata) => {
                let metadata_mode = metadata.mode();
                #[cfg(target_os = "macos")]
                let sflag = SFlag::from_bits_truncate(metadata_mode as mode_t);

                #[cfg(not(target_os = "macos"))]
                let sflag = SFlag::from_bits_truncate(metadata_mode);

                #[cfg(target_os = "macos")]
                let mode = Mode::from_bits_truncate(metadata_mode as mode_t);

                #[cfg(not(target_os = "macos"))]
                let mode = Mode::from_bits_truncate(metadata_mode);

                (FileType::from_mode(sflag), mode)
            }
            _ => (FileType::File, Mode::empty()),
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

        Ok(Self {
            len,
            directory_size,
            modified,
            accessed,
            mode,
            file_type,
            link_type,
            #[cfg(unix)]
            uid,
            #[cfg(unix)]
            gid,
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
