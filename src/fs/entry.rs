use std::{fs, io, path};

use crate::fs::JoshutoMetadata;

#[derive(Clone)]
pub struct JoshutoDirEntry {
    name: String,
    path: path::PathBuf,
    pub metadata: JoshutoMetadata,
    selected: bool,
    marked: bool,
}

impl JoshutoDirEntry {
    pub fn from(direntry: &fs::DirEntry) -> Result<Self, io::Error> {
        let name = match direntry.file_name().into_string() {
            Ok(s) => s,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed converting OsString to String",
                ))
            }
        };

        let path = direntry.path();
        let metadata = JoshutoMetadata::from(&path)?;

        let dir_entry = JoshutoDirEntry {
            name,
            path,
            metadata,
            selected: false,
            marked: false,
        };
        Ok(dir_entry)
    }

    pub fn file_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn file_path(&self) -> &path::PathBuf {
        &self.path
    }

    /*
        pub fn is_marked(&self) -> bool {
            self.marked
        }

        pub fn set_marked(&mut self, marked: bool) {
            self.marked = marked;
        }
    */

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}

impl std::fmt::Debug for JoshutoDirEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "JoshutoDirEntry {{\n\tfile_name: {:?}, \n\tpath: {:?} \n}}",
            self.name, self.path
        )
    }
}
