use std::cmp;
use std::fs;
use std::time;

use serde::{Deserialize, Serialize};

use crate::config::sort_option_raw::SortOptionRaw;
use crate::fs::JoshutoDirEntry;

use crate::types::option::sort::{SortMethod, SortMethodList};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SortOption {
    pub directories_first: bool,
    pub case_sensitive: bool,
    pub reverse: bool,
    pub sort_methods: SortMethodList,
}

impl SortOption {
    pub fn set_sort_method(&mut self, method: SortMethod) {
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

        for sort_method in self.sort_methods.list.iter() {
            let mut cmp_res = match sort_method {
                SortMethod::Ext => ext_sort(f1, f2),
                SortMethod::Lexical => {
                    let f1_name = f1.file_name();
                    let f2_name = f2.file_name();
                    if self.case_sensitive {
                        f1_name.cmp(f2_name)
                    } else {
                        let f1_name = f1_name.to_lowercase();
                        let f2_name = f2_name.to_lowercase();
                        f1_name.cmp(&f2_name)
                    }
                }
                SortMethod::Mtime => mtime_sort(f1, f2),
                SortMethod::Natural => {
                    let f1_name = f1.file_name();
                    let f2_name = f2.file_name();
                    if self.case_sensitive {
                        alphanumeric_sort::compare_str(f1_name, f2_name)
                    } else {
                        let f1_name = f1_name.to_lowercase();
                        let f2_name = f2_name.to_lowercase();
                        alphanumeric_sort::compare_str(f1_name, f2_name)
                    }
                }
                SortMethod::Size => size_sort(f1, f2),
            };

            if self.reverse {
                cmp_res = cmp_res.reverse();
            }

            if cmp_res != cmp::Ordering::Equal {
                return cmp_res;
            }
        }
        cmp::Ordering::Equal
    }
}

impl std::default::Default for SortOption {
    fn default() -> Self {
        SortOption {
            directories_first: true,
            case_sensitive: false,
            reverse: false,
            sort_methods: SortMethodList::default(),
        }
    }
}

impl From<SortOptionRaw> for SortOption {
    fn from(raw: SortOptionRaw) -> Self {
        let sort_method = raw
            .sort_method
            .and_then(|s| SortMethod::from_str(&s))
            .unwrap_or(SortMethod::Natural);
        let mut sort_methods = SortMethodList::default();
        sort_methods.reorganize(sort_method);

        Self {
            directories_first: raw.directories_first,
            case_sensitive: raw.case_sensitive,
            reverse: raw.reverse,
            sort_methods,
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
    }
    compare(file1, file2).unwrap_or(cmp::Ordering::Equal)
}

fn size_sort(file1: &JoshutoDirEntry, file2: &JoshutoDirEntry) -> cmp::Ordering {
    file1.metadata.len().cmp(&file2.metadata.len())
}

fn ext_sort(file1: &JoshutoDirEntry, file2: &JoshutoDirEntry) -> cmp::Ordering {
    let f1_ext = file1.ext().unwrap_or_default();
    let f2_ext = file2.ext().unwrap_or_default();
    alphanumeric_sort::compare_str(f1_ext, f2_ext)
}
