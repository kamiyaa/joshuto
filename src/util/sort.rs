use std::cmp;
use std::fs;
use std::time;

use serde_derive::Deserialize;

use crate::fs::JoshutoDirEntry;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum SortType {
    Lexical,
    Mtime,
    Natural,
    Size,
}

impl SortType {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "lexical" => Some(SortType::Lexical),
            "mtime" => Some(SortType::Mtime),
            "natural" => Some(SortType::Natural),
            "size" => Some(SortType::Size),
            _ => None,
        }
    }
    pub const fn as_str(&self) -> &str {
        match *self {
            SortType::Lexical => "lexical",
            SortType::Mtime => "mtime",
            SortType::Natural => "natural",
            SortType::Size => "size",
        }
    }
}

impl std::fmt::Display for SortType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct SortOption {
    pub directories_first: bool,
    pub case_sensitive: bool,
    pub reverse: bool,
    pub sort_method: SortType,
}

impl SortOption {
    pub fn compare(&self, f1: &JoshutoDirEntry, f2: &JoshutoDirEntry) -> cmp::Ordering {
        if self.directories_first {
            let f1_isdir = f1.file_path().is_dir();
            let f2_isdir = f2.file_path().is_dir();

            if f1_isdir && !f2_isdir {
                return cmp::Ordering::Less;
            } else if !f1_isdir && f2_isdir {
                return cmp::Ordering::Greater;
            }
        }

        let mut res = match self.sort_method {
            SortType::Lexical => {
                let f1_name = f1.file_name();
                let f2_name = f2.file_name();
                if self.case_sensitive {
                    f1_name.cmp(&f2_name)
                } else {
                    let f1_name = f1_name.to_lowercase();
                    let f2_name = f2_name.to_lowercase();
                    f1_name.cmp(&f2_name)
                }
            }
            SortType::Natural => {
                let f1_name = f1.file_name();
                let f2_name = f2.file_name();
                if self.case_sensitive {
                    alphanumeric_sort::compare_str(&f1_name, &f2_name)
                } else {
                    let f1_name = f1_name.to_lowercase();
                    let f2_name = f2_name.to_lowercase();
                    alphanumeric_sort::compare_str(&f1_name, &f2_name)
                }
            }
            SortType::Mtime => mtime_sort(f1, f2),
            SortType::Size => size_sort(f1, f2),
        };

        if self.reverse {
            res = match res {
                cmp::Ordering::Less => cmp::Ordering::Greater,
                cmp::Ordering::Greater => cmp::Ordering::Less,
                s => s,
            };
        }
        res
    }
}

impl std::default::Default for SortOption {
    fn default() -> Self {
        SortOption {
            directories_first: true,
            case_sensitive: false,
            reverse: false,
            sort_method: SortType::Natural,
        }
    }
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
        Ok(f1_mtime.cmp(&f2_mtime))
        // Ok(if f1_mtime >= f2_mtime {
        //     cmp::Ordering::Less
        // } else {
        //     cmp::Ordering::Greater
        // })
    }
    compare(&file1, &file2).unwrap_or(cmp::Ordering::Equal)
}

fn size_sort(file1: &JoshutoDirEntry, file2: &JoshutoDirEntry) -> cmp::Ordering {
    file1.metadata.len().cmp(&file2.metadata.len())
}
