/// Search and selection options globally valid for Joshuto (for all tabs)
#[derive(Clone, Debug)]
pub struct SearchOption {
    pub _case_sensitivity: CaseSensitivity,
}

#[derive(Clone, Copy, Debug)]
pub enum CaseSensitivity {
    Insensitive,
    Sensitive,
    Smart,
}

impl SearchOption {
    pub fn case_sensitivity(&self) -> CaseSensitivity {
        self._case_sensitivity
    }
}

impl std::default::Default for SearchOption {
    fn default() -> Self {
        Self {
            _case_sensitivity: CaseSensitivity::Insensitive,
        }
    }
}
