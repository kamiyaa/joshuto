use std::{fs, io, path, process, time};

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
    pub mimetype: Option<String>,
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

        let mut mimetype = None;
        if let FileType::File = file_type {
            #[cfg(feature = "file_mimetype")]
            {
                mimetype = file_mimetype(path)
            }
        }

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
            mimetype,
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

fn file_mimetype(path: &path::Path) -> Option<String> {
    let output = process::Command::new("file")
        .args(&["-Lb", "--mime-type"])
        .arg(path)
        .output();

    match output {
        Ok(s) => {
            if s.status.success() {
                match String::from_utf8(s.stdout) {
                    Ok(s) => Some(s),
                    Err(_) => None,
                }
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
