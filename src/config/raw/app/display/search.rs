use serde::Deserialize;

fn default_string_case_sensitivity() -> String {
    "insensitive".to_string()
}

fn default_glob_case_sensitivity() -> String {
    "sensitive".to_string()
}

fn default_regex_case_sensitivity() -> String {
    "sensitive".to_string()
}

fn default_fzf_case_sensitivity() -> String {
    "insensitive".to_string()
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchOptionRaw {
    #[serde(default = "default_string_case_sensitivity")]
    pub string_case_sensitivity: String,

    #[serde(default = "default_glob_case_sensitivity")]
    pub glob_case_sensitivity: String,

    #[serde(default = "default_regex_case_sensitivity")]
    pub regex_case_sensitivity: String,

    #[serde(default = "default_fzf_case_sensitivity")]
    pub fzf_case_sensitivity: String,
}

impl std::default::Default for SearchOptionRaw {
    fn default() -> Self {
        SearchOptionRaw {
            string_case_sensitivity: default_string_case_sensitivity(),
            glob_case_sensitivity: default_glob_case_sensitivity(),
            regex_case_sensitivity: default_regex_case_sensitivity(),
            fzf_case_sensitivity: default_fzf_case_sensitivity(),
        }
    }
}
