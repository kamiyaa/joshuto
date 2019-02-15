#[derive(Clone, Debug)]
pub struct JoshutoPageState {
    pub start: usize,
    pub end: usize,
}

impl JoshutoPageState {
    pub fn new() -> Self {
        JoshutoPageState { start: 0, end: 0 }
    }

    pub fn update_page_state(
        &mut self,
        index: usize,
        win_rows: i32,
        vec_len: usize,
        offset: usize,
    ) {
        if self.end != win_rows as usize + self.start {
            self.end = self.start + win_rows as usize;
        }
        if self.end > vec_len {
            self.end = vec_len
        }

        if self.start + offset >= index {
            self.start = if index as usize <= offset {
                0
            } else {
                index as usize - offset
            };
            self.end = if self.start + win_rows as usize >= vec_len {
                vec_len
            } else {
                self.start + win_rows as usize
            };
            self.start = if self.end <= win_rows as usize {
                0
            } else {
                self.end - win_rows as usize
            };
        }
        if self.end <= index + offset {
            self.end = if index as usize + offset >= vec_len {
                vec_len
            } else {
                index as usize + offset
            };
            self.start = if self.end <= win_rows as usize {
                0
            } else {
                self.end - win_rows as usize
            };
            self.end = if self.start + win_rows as usize >= vec_len {
                vec_len
            } else {
                self.start + win_rows as usize
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shorter_than_viewable_region() {
        let mut page_state = JoshutoPageState::new();
        let index = 5;
        let win_rows = 37;
        let vec_len = 12;
        let offset = 6;
        page_state.update_page_state(index, win_rows, vec_len, offset);
        assert_eq!(0, page_state.start);
        assert_eq!(12, page_state.end);
        let index = 30;
        let win_rows = 37;
        let vec_len = 12;
        let offset = 6;
        page_state.update_page_state(index, win_rows, vec_len, offset);
        assert_eq!(0, page_state.start);
        assert_eq!(12, page_state.end);
    }
    #[test]
    fn test_01() {
        let mut page_state = JoshutoPageState::new();
        let index = 3;
        let win_rows = 25;
        let vec_len = 40;
        let offset = 6;
        page_state.update_page_state(index, win_rows, vec_len, offset);
        assert_eq!(0, page_state.start);
        assert_eq!(25, page_state.end);
    }
    #[test]
    fn test_02() {
        let mut page_state = JoshutoPageState::new();
        let index = 12;
        let win_rows = 25;
        let vec_len = 40;
        let offset = 6;
        page_state.update_page_state(index, win_rows, vec_len, offset);
        assert_eq!(0, page_state.start);
        assert_eq!(25, page_state.end);
    }
    #[test]
    fn test_inside_top_offset() {
        let mut page_state = JoshutoPageState::new();
        page_state.start = 10;
        let index = 12;
        let win_rows = 25;
        let vec_len = 40;
        let offset = 6;
        page_state.update_page_state(index, win_rows, vec_len, offset);
        assert_eq!(6, page_state.start);
        assert_eq!(31, page_state.end);
    }
    #[test]
    fn test_inside_bottom_offset() {
        let mut page_state = JoshutoPageState::new();
        page_state.start = 36;
        let index = 34;
        let win_rows = 25;
        let vec_len = 40;
        let offset = 6;
        page_state.update_page_state(index, win_rows, vec_len, offset);
        assert_eq!(15, page_state.start);
        assert_eq!(40, page_state.end);
    }
    #[test]
    fn test_small_size() {
        let mut page_state = JoshutoPageState::new();
        let index = 3;
        let win_rows = 6;
        let vec_len = 6;
        let offset = 6;
        page_state.update_page_state(index, win_rows, vec_len, offset);
        assert_eq!(0, page_state.start);
        assert_eq!(6, page_state.end);
        let index = 0;
        let win_rows = 6;
        let vec_len = 6;
        let offset = 6;
        page_state.update_page_state(index, win_rows, vec_len, offset);
        assert_eq!(0, page_state.start);
        assert_eq!(6, page_state.end);
        let index = 6;
        let win_rows = 6;
        let vec_len = 6;
        let offset = 6;
        page_state.update_page_state(index, win_rows, vec_len, offset);
        assert_eq!(0, page_state.start);
        assert_eq!(6, page_state.end);
    }
    #[test]
    fn test_negative() {
        let mut page_state = JoshutoPageState::new();
        page_state.start = 10;
        page_state.end = 5;
        let index = 7;
        let win_rows = 30;
        let vec_len = 35;
        let offset = 6;
        page_state.update_page_state(index, win_rows, vec_len, offset);
        assert_eq!(1, page_state.start);
        assert_eq!(31, page_state.end);
    }
}
