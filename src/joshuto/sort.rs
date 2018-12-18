use std;
use std::cmp;
use std::fs;

use joshuto::structs;

#[derive(Debug)]
pub enum SortType {
    SortNatural(SortStruct),
    SortMtime(SortStruct),
}

impl SortType {
    pub fn compare_func(&self) -> fn (&structs::JoshutoDirEntry, &structs::JoshutoDirEntry) -> std::cmp::Ordering
    {
        match (*self) {
            SortType::SortNatural(ref ss) => {
                sort_dir_first
            }
            SortType::SortMtime(ref ss) => {
                sort_dir_first
            }
        }
    }

    pub fn filter_func(&self) -> fn(Result<fs::DirEntry, std::io::Error>) -> Option<structs::JoshutoDirEntry>
    {
        match (*self) {
            SortType::SortNatural(ref ss) => {
                if ss.show_hidden {
                    filter_default
                } else {
                    filter_hidden_files
                }
            },
            SortType::SortMtime(ref ss) => {
                if ss.show_hidden {
                    filter_default
                } else {
                    filter_hidden_files
                }
            },
            _ => {
                filter_hidden_files
            },
        }
    }

    pub fn show_hidden(&self) -> bool
    {
        match (*self) {
            SortType::SortNatural(ref ss) => {
                ss.show_hidden
            },
            SortType::SortMtime(ref ss) => {
                ss.show_hidden
            },
            _ => true,
        }
    }
}

#[derive(Debug)]
pub struct SortStruct {
    pub show_hidden: bool,
    pub folders_first: bool,
    pub case_sensitive: bool,
    pub reverse: bool,
}

impl SortStruct {

    pub fn new() -> Self
    {
        SortStruct {
            show_hidden: false,
            folders_first: true,
            case_sensitive: false,
            reverse: false
        }
    }
}

fn filter_default(result : Result<fs::DirEntry, std::io::Error>) -> Option<structs::JoshutoDirEntry>
{
    match result {
        Ok(direntry) => {
            let dir_entry = structs::JoshutoDirEntry {
                entry : direntry,
                selected : false,
                marked : false,
            };
            Some(dir_entry)
        },
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}

fn filter_hidden_files(result : Result<fs::DirEntry, std::io::Error>) -> Option<structs::JoshutoDirEntry>
{
    match result {
        Ok(direntry) => {
            match direntry.file_name().into_string() {
                Ok(file_name) => {
                    if file_name.starts_with(".") {
                        None
                    } else {
                        let dir_entry = structs::JoshutoDirEntry {
                            entry : direntry,
                            selected : false,
                            marked : false,
                        };
                        Some(dir_entry)
                    }
                },
                Err(_e) => {
                    None
                },
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}


/* sort by directory first, incase-sensitive */
pub fn sort_dir_first(file1 : &structs::JoshutoDirEntry,
        file2 : &structs::JoshutoDirEntry) -> cmp::Ordering
{
    fn res_ordering(file1 : &fs::DirEntry, file2 : &fs::DirEntry) -> Result<cmp::Ordering, std::io::Error> {

        let f1_meta = file1.metadata()?;
        let f2_meta = file2.metadata()?;

        if f1_meta.is_dir() && f2_meta.is_dir() {
            let f1_name : std::string::String =
                file1.file_name().as_os_str().to_str().unwrap().to_lowercase();
            let f2_name : std::string::String =
                file2.file_name().as_os_str().to_str().unwrap().to_lowercase();
            if f1_name <= f2_name {
                Ok(cmp::Ordering::Less)
            } else {
                Ok(cmp::Ordering::Greater)
            }
        } else if f1_meta.is_dir() {
            Ok(cmp::Ordering::Less)
        } else if f2_meta.is_dir() {
            Ok(cmp::Ordering::Greater)
        } else {
            let f1_name : std::string::String =
                file1.file_name().as_os_str().to_str().unwrap().to_lowercase();
            let f2_name : std::string::String =
                file2.file_name().as_os_str().to_str().unwrap().to_lowercase();
            if f1_name <= f2_name {
                Ok(cmp::Ordering::Less)
            } else {
                Ok(cmp::Ordering::Greater)
            }
        }
    }
    res_ordering(&file1.entry, &file2.entry).unwrap_or(cmp::Ordering::Less)
}
