use std::{fs, io, path};

use crate::structs::JoshutoMetadata;

#[derive(Clone)]
pub struct JoshutoDirEntry {
    pub file_name: String,
    pub path: path::PathBuf,
    pub metadata: JoshutoMetadata,
    pub selected: bool,
    pub marked: bool,
}

impl JoshutoDirEntry {
    pub fn from(direntry: &fs::DirEntry) -> Result<Self, io::Error> {
        let file_name = match direntry.file_name().into_string() {
            Ok(s) => s,
            Err(e) => return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed converting OsString to String",
                    )),
        };

        let path = direntry.path();
        let metadata = JoshutoMetadata::from(&path)?;

        let dir_entry = JoshutoDirEntry {
            file_name,
            path,
            metadata,
            selected: false,
            marked: false,
        };
        Ok(dir_entry)
    }
}

impl std::fmt::Debug for JoshutoDirEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "JoshutoDirEntry {{\n\tfile_name: {:?}, \n\tpath: {:?} \n}}",
            self.file_name, self.path)
    }
}
