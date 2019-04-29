use std::{ffi, fs, io, path};

use crate::structs::JoshutoMetadata;

#[derive(Clone)]
pub struct JoshutoDirEntry {
    pub file_name: ffi::OsString,
    pub file_name_as_string: String,
    pub path: path::PathBuf,
    pub metadata: JoshutoMetadata,
    pub selected: bool,
    pub marked: bool,
}

impl JoshutoDirEntry {
    pub fn from(direntry: &fs::DirEntry) -> Result<Self, io::Error> {
        let file_name = direntry.file_name();
        let file_name_as_string: String = match file_name.clone().into_string() {
            Ok(s) => s,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Failed to get file_name",
                ));
            }
        };
        let path = direntry.path();
        let metadata = JoshutoMetadata::from(&path)?;

        let dir_entry = JoshutoDirEntry {
            file_name,
            file_name_as_string,
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
        write!(f, "JoshutoDirEntry {{\n\tfile_name: {:?}, \n\tfile_name_as_string: {}, \n\tpath: {:?} \n}}",
            self.file_name, self.file_name_as_string, self.path)
    }
}
