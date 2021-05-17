use std::cmp;
use std::fs;
use std::time;

use serde_derive::Deserialize;

use crate::fs::JoshutoDirEntry;

#[derive(Clone, Debug)]
pub struct SortTypes {
    pub list: std::collections::LinkedList<SortType>,
}

impl SortTypes {
    pub fn reorganize(&mut self, st: SortType) {
        self.list.push_front(st);
        self.list.pop_back();
    }

    pub fn cmp(
        &self,
        f1: &JoshutoDirEntry,
        f2: &JoshutoDirEntry,
        sort_option: &SortOption,
    ) -> cmp::Ordering {
        for st in &self.list {
            let res = st.cmp(f1, f2, sort_option);
            if res != cmp::Ordering::Equal {
                return res;
            }
        }
        cmp::Ordering::Equal
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum SortType {
    Lexical,
    Mtime,
    Natural,
    Size,
    Ext,
}

impl SortType {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "lexical" => Some(SortType::Lexical),
            "mtime" => Some(SortType::Mtime),
            "natural" => Some(SortType::Natural),
            "size" => Some(SortType::Size),
            "ext" => Some(SortType::Ext),
            _ => None,
        }
    }
    pub const fn as_str(&self) -> &str {
        match *self {
            SortType::Lexical => "lexical",
            SortType::Mtime => "mtime",
            SortType::Natural => "natural",
            SortType::Size => "size",
            SortType::Ext => "ext",
        }
    }
    pub fn cmp(
        &self,
        f1: &JoshutoDirEntry,
        f2: &JoshutoDirEntry,
        sort_option: &SortOption,
    ) -> cmp::Ordering {
        let res = match &self {
            SortType::Natural => natural_sort(f1, f2, sort_option),
            SortType::Lexical => lexical_sort(f1, f2, sort_option),
            SortType::Size => size_sort(f1, f2),
            SortType::Mtime => mtime_sort(f1, f2),
            SortType::Ext => ext_sort(f1, f2),
        };
        return res;
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
    pub sort_methods: SortTypes,
}

impl SortOption {
    pub fn set_sort_method(&mut self, method: SortType) {
        self.sort_methods.reorganize(method);
    }

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

        // let mut res = self.sort_method.cmp(f1, f2, &self);
        let mut res = self.sort_methods.cmp(f1, f2, &self);
        if self.reverse {
            res = match res {
                cmp::Ordering::Less => cmp::Ordering::Greater,
                cmp::Ordering::Greater => cmp::Ordering::Less,
                s => s,
            };
        };
        res
    }
}

impl std::default::Default for SortOption {
    fn default() -> Self {
        let mut sort_methods = std::collections::LinkedList::new();
        sort_methods.push_back(SortType::Ext);
        sort_methods.push_back(SortType::Size);
        sort_methods.push_back(SortType::Mtime);
        sort_methods.push_back(SortType::Lexical);
        sort_methods.push_back(SortType::Natural);
        SortOption {
            directories_first: true,
            case_sensitive: false,
            reverse: false,
            sort_method: SortType::Natural,
            sort_methods: SortTypes { list: sort_methods },
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

        Ok(if f1_mtime >= f2_mtime {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        })
    }
    compare(&file1, &file2).unwrap_or(cmp::Ordering::Less)
}

fn size_sort(file1: &JoshutoDirEntry, file2: &JoshutoDirEntry) -> cmp::Ordering {
    file1.metadata.len().cmp(&file2.metadata.len())
}

fn ext_sort(file1: &JoshutoDirEntry, file2: &JoshutoDirEntry) -> cmp::Ordering {
    let f1_ext = file1.get_ext();
    let f2_ext = file2.get_ext();
    alphanumeric_sort::compare_str(&f1_ext, &f2_ext)
}

fn lexical_sort(
    f1: &JoshutoDirEntry,
    f2: &JoshutoDirEntry,
    sort_option: &SortOption,
) -> cmp::Ordering {
    let f1_name = f1.file_name();
    let f2_name = f2.file_name();
    if sort_option.case_sensitive {
        f1_name.cmp(&f2_name)
    } else {
        let f1_name = f1_name.to_lowercase();
        let f2_name = f2_name.to_lowercase();
        f1_name.cmp(&f2_name)
    }
}

fn natural_sort(
    f1: &JoshutoDirEntry,
    f2: &JoshutoDirEntry,
    sort_option: &SortOption,
) -> cmp::Ordering {
    let f1_name = f1.file_name();
    let f2_name = f2.file_name();
    if sort_option.case_sensitive {
        alphanumeric_sort::compare_str(&f1_name, &f2_name)
    } else {
        let f1_name = f1_name.to_lowercase();
        let f2_name = f2_name.to_lowercase();
        alphanumeric_sort::compare_str(&f1_name, &f2_name)
    }
}
