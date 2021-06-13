use std::{fs, io, path, time};

#[derive(Clone, Debug)]
pub enum FileType {
    Directory,
    File,
}

#[derive(Clone, Debug)]
pub enum LinkType {
    Normal,
    Symlink(String),
}

#[derive(Clone, Debug)]
pub struct JoshutoMetadata {
    _len: u64,
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
        let metadata = fs::metadata(path)?;

        let _len = metadata.len();
        let _modified = metadata.modified()?;
        let _permissions = metadata.permissions();
        let _file_type = match metadata.file_type().is_dir() {
            true => FileType::Directory,
            false => FileType::File,
        };
        let _link_type = match symlink_metadata.file_type().is_symlink() {
            true => {
                let mut link = "".to_string();

                if let Ok(path) = fs::read_link(path) {
                    if let Some(s) = path.to_str() {
                        link = s.to_string();
                    }
                }
                LinkType::Symlink(link)
            }
            false => LinkType::Normal,
        };

        #[cfg(unix)]
        let uid = symlink_metadata.uid();
        #[cfg(unix)]
        let gid = symlink_metadata.gid();
        #[cfg(unix)]
        let mode = symlink_metadata.mode();

        Ok(Self {
            _len,
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
}
