use std;
use std::cmp;
use std::fs;
use std::time;

use joshuto::structs;

#[derive(Debug)]
pub enum SortType {
    SortNatural(SortStruct),
    SortMtime(SortStruct),
}

impl SortType {
    pub fn compare_func(&self) -> fn (&structs::JoshutoDirEntry, &structs::JoshutoDirEntry) -> std::cmp::Ordering
    {
        match *self {
            SortType::SortNatural(ref ss) => {
                if ss.folders_first && !ss.case_sensitive && !ss.reverse {
                    sort_natural_dir_first_case_insensitive
                } else {
                    sort_natural_dir_first_case_insensitive
                }
            }
            SortType::SortMtime(_) => {
                sort_mtime_dir_first_case_insensitive
            }
        }
    }

    pub fn filter_func(&self) -> fn(Result<fs::DirEntry, std::io::Error>) -> Option<structs::JoshutoDirEntry>
    {
        match *self {
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
        }
    }

    pub fn show_hidden(&self) -> bool
    {
        match *self {
            SortType::SortNatural(ref ss) => {
                ss.show_hidden
            },
            SortType::SortMtime(ref ss) => {
                ss.show_hidden
            },
        }
    }

    pub fn set_show_hidden(&mut self, show_hidden: bool)
    {
        match self {
            SortType::SortNatural(ref mut ss) => {
                ss.show_hidden = show_hidden;
            },
            SortType::SortMtime(ref mut ss) => {
                ss.show_hidden = show_hidden;
            },
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

fn compare_string_ordering(str1: &str, str2: &str) -> cmp::Ordering
{
    if str1 <= str2 {
        cmp::Ordering::Less
    } else {
        cmp::Ordering::Greater
    }
}

pub fn sort_natural_dir_first_case_insensitive(file1 : &structs::JoshutoDirEntry,
        file2 : &structs::JoshutoDirEntry) -> cmp::Ordering
{
    fn compare(file1: &fs::DirEntry, file2: &fs::DirEntry)
            -> Result<cmp::Ordering, std::io::Error>
    {
        let f1_meta: fs::Metadata = file1.metadata()?;
        let f2_meta: fs::Metadata = file2.metadata()?;

        if f1_meta.is_dir() && f2_meta.is_dir() {
            let f1_name : std::string::String =
                file1.file_name().into_string().unwrap().to_lowercase();
            let f2_name : std::string::String =
                file2.file_name().into_string().unwrap().to_lowercase();
            Ok(compare_string_ordering(&f1_name, &f2_name))
        } else if f1_meta.is_dir() {
            Ok(cmp::Ordering::Less)
        } else if f2_meta.is_dir() {
            Ok(cmp::Ordering::Greater)
        } else {
            let f1_name : std::string::String =
                file1.file_name().as_os_str().to_str().unwrap().to_lowercase();
            let f2_name : std::string::String =
                file2.file_name().as_os_str().to_str().unwrap().to_lowercase();
            Ok(compare_string_ordering(&f1_name, &f2_name))
        }
    }
    compare(&file1.entry, &file2.entry).unwrap_or(cmp::Ordering::Less)
}

pub fn sort_mtime_dir_first_case_insensitive(file1 : &structs::JoshutoDirEntry,
        file2 : &structs::JoshutoDirEntry) -> cmp::Ordering
{
    fn compare(file1: &fs::DirEntry, file2: &fs::DirEntry)
            -> Result<cmp::Ordering, std::io::Error>
    {
        let f1_meta: fs::Metadata = file1.metadata()?;
        let f2_meta: fs::Metadata = file2.metadata()?;

        if f1_meta.is_dir() && f2_meta.is_dir() {
            let f1_mtime: time::SystemTime = f1_meta.modified()?;
            let f2_mtime: time::SystemTime = f2_meta.modified()?;
            Ok(if f1_mtime <= f2_mtime {
                    cmp::Ordering::Less
                } else {
                    cmp::Ordering::Greater
            })
        } else if f1_meta.is_dir() {
            Ok(cmp::Ordering::Less)
        } else if f2_meta.is_dir() {
            Ok(cmp::Ordering::Greater)
        } else {
            let f1_mtime: time::SystemTime = f1_meta.modified()?;
            let f2_mtime: time::SystemTime = f2_meta.modified()?;
            Ok(if f1_mtime <= f2_mtime {
                    cmp::Ordering::Less
                } else {
                    cmp::Ordering::Greater
            })
        }
    }
    compare(&file1.entry, &file2.entry).unwrap_or(cmp::Ordering::Less)
}

