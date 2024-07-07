use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

///Truncates a string to width, less or equal to the specified one.
///
///In case the point of truncation falls into a full-width character,
///the returned string will be shorter than the given `width`.
///Otherwise, it will be equal.
pub trait UnicodeTruncate {
    fn trunc(&self, width: usize) -> String;
}

impl UnicodeTruncate for str {
    #[inline]
    fn trunc(&self, width: usize) -> String {
        if self.width() <= width {
            String::from(self)
        } else {
            let mut length: usize = 0;
            let mut result = String::new();
            for grapheme in self.graphemes(true) {
                let grapheme_length = grapheme.width();
                length += grapheme_length;
                if length > width {
                    break;
                };
                result.push_str(grapheme);
            }
            result
        }
    }
}

#[cfg(test)]
mod tests_trunc {
    use super::UnicodeTruncate;

    #[test]
    fn truncate_correct_despite_several_multibyte_chars() {
        assert_eq!(String::from("r͂o͒͜w̾").trunc(2), String::from("r͂o͒͜"));
    }

    #[test]
    fn truncate_at_end_returns_complete_string() {
        assert_eq!(String::from("r͂o͒͜w̾").trunc(3), String::from("r͂o͒͜w̾"));
    }

    #[test]
    fn truncate_behind_end_returns_complete_string() {
        assert_eq!(String::from("r͂o͒͜w̾").trunc(4), String::from("r͂o͒͜w̾"));
    }

    #[test]
    fn truncate_at_zero_returns_empty_string() {
        assert_eq!(String::from("r͂o͒͜w̾").trunc(0), String::from(""));
    }

    #[test]
    fn truncate_correct_despite_fullwidth_character() {
        assert_eq!(String::from("a🌕bc").trunc(4), String::from("a🌕b"));
    }

    #[test]
    fn truncate_within_fullwidth_character_truncates_before_the_character() {
        assert_eq!(String::from("a🌕").trunc(2), String::from("a"));
    }
}
