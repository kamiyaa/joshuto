#[inline(always)]
pub fn truncate_by_char(s: &String, max_chars: usize) -> String {
    match s.char_indices().nth(max_chars) {
        None => String::from(s),
        Some((idx, _)) => s[..idx].to_string(),
    }
}

#[cfg(test)]
mod string_truncation {
    use super::truncate_by_char;

    #[test]
    fn truncate_despite_several_multibyte_chars() {
        assert_eq!(
            truncate_by_char(&String::from("✗a⮋b⬎"), 3),
            String::from("✗a⮋")
        );
    }
}
