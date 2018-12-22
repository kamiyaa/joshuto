extern crate whoami;

use joshuto;
use joshuto::sort;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct JoshutoRawConfig {
    show_hidden: Option<bool>,
    sort_type: Option<String>,
    column_ratio: Option<[usize; 3]>,
    mimetypes: Option<HashMap<String, Vec<Vec<String>>>>,
}

impl JoshutoRawConfig {
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        JoshutoRawConfig {
            show_hidden: Some(false),
            sort_type: Some(String::from("natural")),
            column_ratio: Some([1, 3, 4]),
            mimetypes: None,
        }
    }

    pub fn flatten(self) -> JoshutoConfig
    {
        let username : String = whoami::username();
        let hostname : String = whoami::hostname();

        let column_ratio = match self.column_ratio {
            Some(s) => (s[0], s[1], s[2]),
            None => (1, 3, 4),
            };

        let show_hidden: bool = match self.show_hidden {
            Some(s) => s,
            None => false,
            };

        let sort_struct = sort::SortStruct {
                show_hidden,
                folders_first: true,
                case_sensitive: false,
                reverse: false,
            };

        let sort_type: sort::SortType = match self.sort_type {
            Some(s) => {
                match s.as_str() {
                    "natural" => sort::SortType::SortNatural(sort_struct),
                    "mtime" => sort::SortType::SortMtime(sort_struct),
                    _ => sort::SortType::SortNatural(sort_struct),
                }
            }
            _ => sort::SortType::SortNatural(sort_struct),
            };

        let mimetypes = match self.mimetypes {
            Some(s) => s,
            None => HashMap::new(),
            };

        JoshutoConfig {
            username,
            hostname,
            sort_type,
            column_ratio,
            mimetypes,
        }
    }
}

#[derive(Debug)]
pub struct JoshutoConfig {
    pub username: String,
    pub hostname: String,
    pub sort_type: joshuto::sort::SortType,
    pub column_ratio: (usize, usize, usize),
    pub mimetypes: HashMap<String, Vec<Vec<String>>>,
}

impl JoshutoConfig {

    pub fn new() -> Self
    {
        let sort_struct = sort::SortStruct {
                show_hidden: false,
                folders_first: true,
                case_sensitive: false,
                reverse: false,
            };
        let sort_type = sort::SortType::SortNatural(sort_struct);

        let username : String = whoami::username();
        let hostname : String = whoami::hostname();

        JoshutoConfig {
            username,
            hostname,
            sort_type,
            column_ratio: (1, 3, 4),
            mimetypes: HashMap::new(),
        }
    }
}
