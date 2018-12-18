
use std;
use joshuto;
use joshuto::sort;
use joshuto::structs;
use std::collections::HashMap;

#[derive(Debug)]
pub struct JoshutoRawConfig {
    pub show_hidden : String,
    pub sort_type : String,
    pub column_ratio : [usize; 3],
}

impl JoshutoRawConfig {
    pub fn new() -> Self
    {
        JoshutoRawConfig {
            show_hidden: "false".to_string(),
            sort_type: "natural".to_string(),
            column_ratio : [1, 3, 4],
        }
    }

    pub fn to_config(self) -> JoshutoConfig
    {
        let show_hidden: bool;
        let sort_type: sort::SortType;
        let column_ratio = (self.column_ratio[0], self.column_ratio[1], self.column_ratio[2]);

        if self.show_hidden.to_lowercase() == "true" {
            show_hidden = true;
        } else {
            show_hidden = false;
        }
        let sort_struct = sort::SortStruct {
                show_hidden,
                folders_first: false,
                case_sensitive: false,
                reverse: false,
            };

        if self.sort_type.to_lowercase() == "natural" {
            sort_type = sort::SortType::SortNatural(sort_struct);
        } else {
            sort_type = sort::SortType::SortNatural(sort_struct);
        }


        JoshutoConfig {
            sort_type,
            column_ratio,
            mimetypes: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct JoshutoConfig {
    pub sort_type : joshuto::sort::SortType,
    pub column_ratio : (usize, usize, usize),
    pub mimetypes : HashMap<String, String>,
}
