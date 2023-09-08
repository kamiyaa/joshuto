use std::cmp;

use crate::{config::raw::app::display::sort::SortOptionRaw, fs::JoshutoDirEntry};

use super::sort_type::{SortType, SortTypes};

#[derive(Clone, Debug)]
pub struct SortOption {
    pub directories_first: bool,
    pub case_sensitive: bool,
    pub reverse: bool,
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
        let mut res = self.sort_methods.cmp(f1, f2, self);
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
        SortOption {
            directories_first: true,
            case_sensitive: false,
            reverse: false,
            sort_methods: SortTypes::default(),
        }
    }
}

impl From<SortOptionRaw> for SortOption {
    fn from(raw: SortOptionRaw) -> Self {
        let sort_method = raw
            .sort_method
            .and_then(|s| SortType::from_str(&s))
            .unwrap_or(SortType::Natural);

        let mut sort_methods = SortTypes::default();
        sort_methods.reorganize(sort_method);

        Self {
            directories_first: raw.directories_first,
            case_sensitive: raw.case_sensitive,
            reverse: raw.reverse,
            sort_methods,
        }
    }
}
