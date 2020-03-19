use std::cmp;
use std::fs;
use std::time;

use crate::fs::JoshutoDirEntry;

use alphanumeric_sort::compare_str;
use serde_derive::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum SortType {
    Lexical,
    Mtime,
    Natural,
}

impl SortType {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "lexical" => Some(SortType::Lexical),
            "mtime" => Some(SortType::Mtime),
            "natural" => Some(SortType::Natural),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SortOption {
    pub show_hidden: bool,
    pub directories_first: bool,
    pub case_sensitive: bool,
    pub reverse: bool,
    pub sort_method: SortType,
}

impl SortOption {
    pub fn compare_func(&self) -> impl Fn(&JoshutoDirEntry, &JoshutoDirEntry) -> cmp::Ordering {
        let base_cmp = match self.sort_method {
            SortType::Natural => {
                if self.case_sensitive {
                    natural_sort
                } else {
                    natural_sort_case_insensitive
                }
            }
            SortType::Lexical => {
                if self.case_sensitive {
                    natural_sort
                } else {
                    natural_sort_case_insensitive
                }
            }
            SortType::Mtime => mtime_sort,
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

        move |f1, f2| dir_cmp(f1, f2).then_with(|| rev_cmp(base_cmp(f1, f2)))
    }

    pub fn filter_func(&self) -> fn(&Result<fs::DirEntry, std::io::Error>) -> bool {
        if self.show_hidden {
            no_filter
        } else {
            filter_hidden
        }
    }
}

impl std::default::Default for SortOption {
    fn default() -> Self {
        SortOption {
            show_hidden: false,
            directories_first: true,
            case_sensitive: false,
            reverse: false,
            sort_method: SortType::Natural,
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
                !file_name.starts_with('.')
            } else {
                false
            }
        }
    }
}

const fn dummy_dir_first(_: &JoshutoDirEntry, _: &JoshutoDirEntry) -> cmp::Ordering {
    cmp::Ordering::Equal
}

fn dir_first(f1: &JoshutoDirEntry, f2: &JoshutoDirEntry) -> cmp::Ordering {
    let f1_isdir = f1.file_path().is_dir();
    let f2_isdir = f2.file_path().is_dir();

    if f1_isdir && !f2_isdir {
        cmp::Ordering::Less
    } else if !f1_isdir && f2_isdir {
        cmp::Ordering::Greater
    } else {
        cmp::Ordering::Equal
    }
}

const fn dummy_reverse(c: cmp::Ordering) -> cmp::Ordering {
    c
}

fn reverse_ordering(c: cmp::Ordering) -> cmp::Ordering {
    c.reverse()
}

fn natural_sort_case_insensitive(f1: &JoshutoDirEntry, f2: &JoshutoDirEntry) -> cmp::Ordering {
    let f1_name = f1.file_name().to_lowercase();
    let f2_name = f2.file_name().to_lowercase();
    compare_str(&f1_name, &f2_name)
}

fn natural_sort(f1: &JoshutoDirEntry, f2: &JoshutoDirEntry) -> cmp::Ordering {
    compare_str(f1.file_name(), f2.file_name())
}

fn mtime_sort(file1: &JoshutoDirEntry, file2: &JoshutoDirEntry) -> cmp::Ordering {
    fn compare(
        file1: &JoshutoDirEntry,
        file2: &JoshutoDirEntry,
    ) -> Result<cmp::Ordering, std::io::Error> {
        let f1_meta: fs::Metadata = std::fs::metadata(file1.file_path())?;
        let f2_meta: fs::Metadata = std::fs::metadata(file2.file_path())?;

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
