use std::cmp;
use std::fs;
use std::time;

use joshuto::structs;

#[derive(Debug, Clone)]
pub struct SortOption {
    pub show_hidden: bool,
    pub directories_first: bool,
    pub case_sensitive: bool,
    pub reverse: bool,
}

#[derive(Debug, Clone)]
pub enum SortType {
    SortNatural(SortOption),
    SortMtime(SortOption),
}

impl SortType {
    pub fn compare_func(
        &self,
    ) -> fn(&structs::JoshutoDirEntry, &structs::JoshutoDirEntry) -> std::cmp::Ordering {
        match *self {
            SortType::SortNatural(ref ss) => {
                if ss.directories_first && !ss.case_sensitive && !ss.reverse {
                    SortNatural::dir_first_case_insensitive
                } else if ss.directories_first && ss.case_sensitive && !ss.reverse {
                    SortNatural::dir_first
                } else {
                    SortNatural::default_sort
                }
            }
            SortType::SortMtime(ref ss) => {
                if ss.directories_first && !ss.reverse {
                    SortMtime::dir_first
                } else {
                    SortMtime::default_sort
                }
            }
        }
    }

    pub fn filter_func(
        &self,
    ) -> fn(Result<fs::DirEntry, std::io::Error>) -> Option<structs::JoshutoDirEntry> {
        match *self {
            SortType::SortNatural(ref ss) => {
                if ss.show_hidden {
                    filter_default
                } else {
                    filter_hidden_files
                }
            }
            SortType::SortMtime(ref ss) => {
                if ss.show_hidden {
                    filter_default
                } else {
                    filter_hidden_files
                }
            }
        }
    }

    pub fn show_hidden(&self) -> bool {
        match *self {
            SortType::SortNatural(ref ss) => ss.show_hidden,
            SortType::SortMtime(ref ss) => ss.show_hidden,
        }
    }

    pub fn set_show_hidden(&mut self, show_hidden: bool) {
        match self {
            SortType::SortNatural(ref mut ss) => {
                ss.show_hidden = show_hidden;
            }
            SortType::SortMtime(ref mut ss) => {
                ss.show_hidden = show_hidden;
            }
        }
    }
}

fn filter_default(
    result: Result<fs::DirEntry, std::io::Error>,
) -> Option<structs::JoshutoDirEntry> {
    match result {
        Ok(direntry) => match structs::JoshutoDirEntry::from(&direntry) {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("error: {:?}", e);
                None
            }
        },
        Err(_) => None,
    }
}

fn filter_hidden_files(
    result: Result<fs::DirEntry, std::io::Error>,
) -> Option<structs::JoshutoDirEntry> {
    match result {
        Ok(direntry) => match direntry.file_name().into_string() {
            Ok(file_name) => {
                if file_name.starts_with(".") {
                    None
                } else {
                    match structs::JoshutoDirEntry::from(&direntry) {
                        Ok(s) => Some(s),
                        Err(e) => {
                            eprintln!("error: {:?}", e);
                            None
                        }
                    }
                }
            }
            Err(_) => None,
        },
        Err(_) => None,
    }
}

pub struct SortNatural {}
impl SortNatural {
    pub fn dir_first_case_insensitive(
        file1: &structs::JoshutoDirEntry,
        file2: &structs::JoshutoDirEntry,
    ) -> cmp::Ordering {
        let f1_isdir = file1.path.is_dir();
        let f2_isdir = file2.path.is_dir();

        if f1_isdir && !f2_isdir {
            cmp::Ordering::Less
        } else if !f1_isdir && f2_isdir {
            cmp::Ordering::Greater
        } else {
            let f1_name = file1.file_name_as_string.to_lowercase();
            let f2_name = file2.file_name_as_string.to_lowercase();
            if f1_name <= f2_name {
                cmp::Ordering::Less
            } else {
                cmp::Ordering::Greater
            }
        }
    }

    pub fn dir_first(
        file1: &structs::JoshutoDirEntry,
        file2: &structs::JoshutoDirEntry,
    ) -> cmp::Ordering {
        let f1_isdir = file1.path.is_dir();
        let f2_isdir = file2.path.is_dir();

        if f1_isdir && !f2_isdir {
            cmp::Ordering::Less
        } else if !f1_isdir && f2_isdir {
            cmp::Ordering::Greater
        } else {
            Self::default_sort(file1, file2)
        }
    }

    pub fn default_sort(
        file1: &structs::JoshutoDirEntry,
        file2: &structs::JoshutoDirEntry,
    ) -> cmp::Ordering {
        if file1.file_name <= file2.file_name {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        }
    }
}

pub struct SortMtime {}
impl SortMtime {
    pub fn dir_first(
        file1: &structs::JoshutoDirEntry,
        file2: &structs::JoshutoDirEntry,
    ) -> cmp::Ordering {
        fn compare(
            file1: &structs::JoshutoDirEntry,
            file2: &structs::JoshutoDirEntry,
        ) -> Result<cmp::Ordering, std::io::Error> {
            let f1_isdir = file1.path.is_dir();
            let f2_isdir = file2.path.is_dir();

            if f1_isdir && !f2_isdir {
                Ok(cmp::Ordering::Less)
            } else if !f1_isdir && f2_isdir {
                Ok(cmp::Ordering::Greater)
            } else {
                Ok(SortMtime::default_sort(file1, file2))
            }
        }
        compare(&file1, &file2).unwrap_or(cmp::Ordering::Less)
    }

    pub fn default_sort(
        file1: &structs::JoshutoDirEntry,
        file2: &structs::JoshutoDirEntry,
    ) -> cmp::Ordering {
        fn compare(
            file1: &structs::JoshutoDirEntry,
            file2: &structs::JoshutoDirEntry,
        ) -> Result<cmp::Ordering, std::io::Error> {
            let f1_meta: fs::Metadata = std::fs::metadata(&file1.path)?;
            let f2_meta: fs::Metadata = std::fs::metadata(&file2.path)?;

            let f1_mtime: time::SystemTime = f1_meta.modified()?;
            let f2_mtime: time::SystemTime = f2_meta.modified()?;

            Ok(if f1_mtime <= f2_mtime {
                cmp::Ordering::Less
            } else {
                cmp::Ordering::Greater
            })
        }
        compare(&file1, &file2).unwrap_or(cmp::Ordering::Less)
    }
}
