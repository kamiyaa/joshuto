use std::cmp;
use std::collections::VecDeque;
use std::fs;
use std::time;

use serde::Deserialize;

use crate::fs::JoshutoDirEntry;

use super::sort::SortOption;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum SortType {
    Lexical,
    Mtime,
    Natural,
    Size,
    Ext,
}

impl SortType {
    pub fn from_str(s: &str) -> Option<Self> {
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
        match &self {
            SortType::Natural => natural_sort(f1, f2, sort_option),
            SortType::Lexical => lexical_sort(f1, f2, sort_option),
            SortType::Size => size_sort(f1, f2),
            SortType::Mtime => mtime_sort(f1, f2),
            SortType::Ext => ext_sort(f1, f2),
        }
    }
}

impl std::fmt::Display for SortType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct SortTypes {
    pub list: VecDeque<SortType>,
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

impl std::default::Default for SortTypes {
    fn default() -> Self {
        let list: VecDeque<SortType> = vec![
            SortType::Natural,
            SortType::Lexical,
            SortType::Size,
            SortType::Ext,
            SortType::Mtime,
        ]
        .into_iter()
        .collect();

        Self { list }
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
    }
    compare(file1, file2).unwrap_or(cmp::Ordering::Equal)
}

fn size_sort(file1: &JoshutoDirEntry, file2: &JoshutoDirEntry) -> cmp::Ordering {
    file1.metadata.len().cmp(&file2.metadata.len())
}

fn ext_sort(file1: &JoshutoDirEntry, file2: &JoshutoDirEntry) -> cmp::Ordering {
    let f1_ext = file1.get_ext();
    let f2_ext = file2.get_ext();
    alphanumeric_sort::compare_str(f1_ext, f2_ext)
}

fn lexical_sort(
    f1: &JoshutoDirEntry,
    f2: &JoshutoDirEntry,
    sort_option: &SortOption,
) -> cmp::Ordering {
    let f1_name = f1.file_name();
    let f2_name = f2.file_name();
    if sort_option.case_sensitive {
        f1_name.cmp(f2_name)
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
        alphanumeric_sort::compare_str(f1_name, f2_name)
    } else {
        let f1_name = f1_name.to_lowercase();
        let f2_name = f2_name.to_lowercase();
        alphanumeric_sort::compare_str(f1_name, f2_name)
    }
}
