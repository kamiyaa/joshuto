use std::{fs, io, path, time};

#[derive(Clone, Debug)]
pub struct JoshutoMetadata {
    pub len: u64,
    pub modified: time::SystemTime,
    pub permissions: fs::Permissions,
    pub file_type: fs::FileType,
    #[cfg(unix)]
    pub uid: u32,
    #[cfg(unix)]
    pub gid: u32,
}

impl JoshutoMetadata {
    pub fn from(path: &path::Path) -> Result<Self, io::Error> {
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;

        let metadata = fs::symlink_metadata(path)?;

        let len = metadata.len();
        let modified = metadata.modified()?;
        let permissions = metadata.permissions();
        let file_type = metadata.file_type();

        #[cfg(unix)]
        let uid = metadata.uid();
        #[cfg(unix)]
        let gid = metadata.gid();

        Ok(JoshutoMetadata {
            len,
            modified,
            permissions,
            file_type,
            #[cfg(unix)]
            uid,
            #[cfg(unix)]
            gid,
        })
    }
}
