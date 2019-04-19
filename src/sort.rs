use std::cmp;
use std::fs;
use std::time;

use crate::structs;

#[derive(Debug, Clone)]
pub enum SortType {
    SortNatural,
    SortMtime,
}

#[derive(Debug, Clone)]
pub struct SortOption {
    pub show_hidden: bool,
    pub directories_first: bool,
    pub case_sensitive: bool,
    pub reverse: bool,
    pub sort_method: SortType,
}

impl SortOption {
    pub fn compare_func(
        &self,
    ) -> impl Fn(&structs::JoshutoDirEntry, &structs::JoshutoDirEntry) -> std::cmp::Ordering {
        let base_cmp = match self.sort_method {
            SortType::SortNatural => {
                if self.case_sensitive {
                    natural_sort
                } else {
                    natural_sort_case_insensitive
                }
            }
            SortType::SortMtime => mtime_sort,
        };

        let rev_cmp = if self.reverse {
            reverse_ordering
        } else {
            dummy_reverse
        };
        let dir_cmp = if self.directories_first {
            dir_first
        } else {
            dummy_dir_first
        };

        move |f1, f2| dir_cmp(f1, f2).unwrap_or_else(|| rev_cmp(base_cmp(f1, f2)))
    }

    pub fn filter_func(&self) -> fn(&Result<fs::DirEntry, std::io::Error>) -> bool {
        if self.show_hidden {
            no_filter
        } else {
            filter_hidden
        }
    }
}

const fn no_filter(_: &Result<fs::DirEntry, std::io::Error>) -> bool {
    true
}

fn filter_hidden(result: &Result<fs::DirEntry, std::io::Error>) -> bool {
    match result {
        Err(_) => false,
        Ok(entry) => {
            let file_name = entry.file_name();
            if let Some(file_name) = file_name.to_str() {
                !file_name.starts_with(".")
            } else {
                false
            }
        }
    }
}

pub fn map_entry_default(
    result: Result<fs::DirEntry, std::io::Error>,
) -> Option<structs::JoshutoDirEntry> {
    match result {
        Ok(direntry) => match structs::JoshutoDirEntry::from(&direntry) {
            Ok(s) => Some(s),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

const fn dummy_dir_first(
    _: &structs::JoshutoDirEntry,
    _: &structs::JoshutoDirEntry,
) -> Option<cmp::Ordering> {
    None
}

fn dir_first(
    f1: &structs::JoshutoDirEntry,
    f2: &structs::JoshutoDirEntry,
) -> Option<cmp::Ordering> {
    let f1_isdir = f1.path.is_dir();
    let f2_isdir = f2.path.is_dir();

    if f1_isdir && !f2_isdir {
        Some(cmp::Ordering::Less)
    } else if !f1_isdir && f2_isdir {
        Some(cmp::Ordering::Greater)
    } else {
        None
    }
}

const fn dummy_reverse(c: cmp::Ordering) -> cmp::Ordering {
    c
}

fn reverse_ordering(c: cmp::Ordering) -> cmp::Ordering {
    match c {
        cmp::Ordering::Less => cmp::Ordering::Greater,
        cmp::Ordering::Greater => cmp::Ordering::Less,
        x => x,
    }
}

fn natural_sort_case_insensitive(
    f1: &structs::JoshutoDirEntry,
    f2: &structs::JoshutoDirEntry,
) -> cmp::Ordering {
    let f1_name = f1.file_name_as_string.to_lowercase();
    let f2_name = f2.file_name_as_string.to_lowercase();
    if f1_name <= f2_name {
        cmp::Ordering::Less
    } else {
        cmp::Ordering::Greater
    }
}

fn natural_sort(f1: &structs::JoshutoDirEntry, f2: &structs::JoshutoDirEntry) -> cmp::Ordering {
    if f1.file_name <= f2.file_name {
        cmp::Ordering::Less
    } else {
        cmp::Ordering::Greater
    }
}

fn mtime_sort(file1: &structs::JoshutoDirEntry, file2: &structs::JoshutoDirEntry) -> cmp::Ordering {
    fn compare(
        file1: &structs::JoshutoDirEntry,
        file2: &structs::JoshutoDirEntry,
    ) -> Result<cmp::Ordering, std::io::Error> {
        let f1_meta: fs::Metadata = std::fs::metadata(&file1.path)?;
        let f2_meta: fs::Metadata = std::fs::metadata(&file2.path)?;

        let f1_mtime: time::SystemTime = f1_meta.modified()?;
        let f2_mtime: time::SystemTime = f2_meta.modified()?;

        Ok(if f1_mtime >= f2_mtime {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        })
    }
    compare(&file1, &file2).unwrap_or(cmp::Ordering::Less)
}
