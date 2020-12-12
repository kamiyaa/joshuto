use std::{fs, path, process, time};

#[derive(Clone, Debug)]
pub enum FileType {
    Directory,
    Symlink(String),
    File,
}

#[derive(Clone, Debug)]
pub struct JoshutoMetadata {
    pub len: u64,
    pub modified: time::SystemTime,
    pub permissions: fs::Permissions,
    pub file_type: FileType,
    pub mimetype: Option<String>,
    #[cfg(unix)]
    pub uid: u32,
    #[cfg(unix)]
    pub gid: u32,
    #[cfg(unix)]
    pub mode: u32,
}

impl JoshutoMetadata {
    pub fn from(path: &path::Path) -> std::io::Result<Self> {
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;

        let metadata = fs::symlink_metadata(path)?;

        let len = metadata.len();
        let modified = metadata.modified()?;
        let permissions = metadata.permissions();
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
            len,
            modified,
            permissions,
            file_type,
            mimetype,
            #[cfg(unix)]
            uid,
            #[cfg(unix)]
            gid,
            #[cfg(unix)]
            mode,
        })
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
