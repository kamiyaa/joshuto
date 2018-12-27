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
                if ss.sort_directories_first && !ss.sort_case_sensitive && !ss.sort_reverse {
                    SortNatural::dir_first_case_insensitive
                } else if ss.sort_directories_first && ss.sort_case_sensitive && !ss.sort_reverse {
                    SortNatural::dir_first
                } else {
                    SortNatural::default_sort
                }
            }
            SortType::SortMtime(ref ss) => {
                if ss.sort_directories_first && !ss.sort_reverse {
                    SortMtime::dir_first
                } else {
                    SortMtime::default_sort
                }
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
    pub sort_directories_first: bool,
    pub sort_case_sensitive: bool,
    pub sort_reverse: bool,
}

fn filter_default(result : Result<fs::DirEntry, std::io::Error>) -> Option<structs::JoshutoDirEntry>
{
    match result {
        Ok(direntry) => {
            let dir_entry = structs::JoshutoDirEntry {
                entry: direntry,
                selected: false,
                marked: false,
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
                            entry: direntry,
                            selected: false,
                            marked: false,
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

pub struct SortNatural {}
impl SortNatural {
    pub fn dir_first_case_insensitive(file1 : &structs::JoshutoDirEntry,
        file2 : &structs::JoshutoDirEntry) -> cmp::Ordering
    {
        let f1_entry = &file1.entry;
        let f2_entry = &file2.entry;

        let f1_path = f1_entry.path();
        let f2_path = f2_entry.path();

        if f1_path.is_dir() && !f2_path.is_dir() {
            cmp::Ordering::Less
        } else if !f1_path.is_dir() && f2_path.is_dir() {
            cmp::Ordering::Greater
        } else {
            let f1_name = f1_entry.file_name().into_string().unwrap().to_lowercase();
            let f2_name = f2_entry.file_name().into_string().unwrap().to_lowercase();
            if f1_name <= f2_name {
                cmp::Ordering::Less
            } else {
                cmp::Ordering::Greater
            }
        }
    }

    pub fn dir_first(file1 : &structs::JoshutoDirEntry,
        file2 : &structs::JoshutoDirEntry) -> cmp::Ordering
    {
        let f1_entry = &file1.entry;
        let f2_entry = &file2.entry;

        let f1_path = f1_entry.path();
        let f2_path = f2_entry.path();

        if f1_path.is_dir() && !f2_path.is_dir() {
            cmp::Ordering::Less
        } else if !f1_path.is_dir() && f2_path.is_dir() {
            cmp::Ordering::Greater
        } else {
            let f1_name = f1_entry.file_name();
            let f2_name = f2_entry.file_name();
            if f1_name <= f2_name {
                cmp::Ordering::Less
            } else {
                cmp::Ordering::Greater
            }
        }
    }

    pub fn default_sort(file1 : &structs::JoshutoDirEntry,
        file2 : &structs::JoshutoDirEntry) -> cmp::Ordering
    {
        let f1_entry = &file1.entry;
        let f2_entry = &file2.entry;

        let f1_name = f1_entry.file_name();
        let f2_name = f2_entry.file_name();
        if f1_name <= f2_name {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        }
    }
}

pub struct SortMtime {}
impl SortMtime {
    pub fn dir_first(file1 : &structs::JoshutoDirEntry,
            file2 : &structs::JoshutoDirEntry) -> cmp::Ordering
    {
        fn compare(file1: &fs::DirEntry, file2: &fs::DirEntry)
                -> Result<cmp::Ordering, std::io::Error>
        {
            let f1_path = file1.path();
            let f2_path = file2.path();

            if f1_path.is_dir() && !f2_path.is_dir() {
                Ok(cmp::Ordering::Less)
            } else if !f1_path.is_dir() && f2_path.is_dir() {
                Ok(cmp::Ordering::Greater)
            } else {
                let f1_meta: fs::Metadata = file1.metadata()?;
                let f2_meta: fs::Metadata = file2.metadata()?;

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

    pub fn default_sort(file1 : &structs::JoshutoDirEntry,
            file2 : &structs::JoshutoDirEntry) -> cmp::Ordering
    {
        fn compare(file1: &fs::DirEntry, file2: &fs::DirEntry)
                -> Result<cmp::Ordering, std::io::Error>
        {
            let f1_meta: fs::Metadata = file1.metadata()?;
            let f2_meta: fs::Metadata = file2.metadata()?;

            let f1_mtime: time::SystemTime = f1_meta.modified()?;
            let f2_mtime: time::SystemTime = f2_meta.modified()?;

            Ok(if f1_mtime <= f2_mtime {
                    cmp::Ordering::Less
                } else {
                    cmp::Ordering::Greater
            })
        }
        compare(&file1.entry, &file2.entry).unwrap_or(cmp::Ordering::Less)
    }
}

