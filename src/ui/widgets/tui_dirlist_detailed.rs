use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;

use crate::fs::{FileType, JoshutoDirEntry, JoshutoDirList, LinkType};
use crate::util::format;
use crate::util::string::UnicodeTruncate;
use crate::util::style;
use unicode_width::UnicodeWidthStr;

const MIN_LEFT_LABEL_WIDTH: i32 = 15;

const ELLIPSIS: &str = "…";

pub struct TuiDirListDetailed<'a> {
    dirlist: &'a JoshutoDirList,
}

impl<'a> TuiDirListDetailed<'a> {
    pub fn new(dirlist: &'a JoshutoDirList) -> Self {
        Self { dirlist }
    }
}

impl<'a> Widget for TuiDirListDetailed<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 4 || area.height < 1 {
            return;
        }

        let x = area.left();
        let y = area.top();
        let curr_index = match self.dirlist.index {
            Some(i) => i,
            None => {
                let style = Style::default().bg(Color::Red).fg(Color::White);
                buf.set_stringn(x, y, "empty", area.width as usize, style);
                return;
            }
        };

        let drawing_width = area.width as usize;
        let skip_dist = curr_index / area.height as usize * area.height as usize;

        // draw every entry
        self.dirlist
            .iter()
            .skip(skip_dist)
            .enumerate()
            .take(area.height as usize)
            .for_each(|(i, entry)| {
                let style = style::entry_style(entry);
                print_entry(buf, entry, style, (x + 1, y + i as u16), drawing_width - 1);
            });

        // draw selected entry in a different style
        let screen_index = curr_index % area.height as usize;

        let entry = self.dirlist.curr_entry_ref().unwrap();
        let style = style::entry_style(entry).add_modifier(Modifier::REVERSED);

        let space_fill = " ".repeat(drawing_width);
        buf.set_string(x, y + screen_index as u16, space_fill.as_str(), style);

        print_entry(
            buf,
            entry,
            style,
            (x + 1, y + screen_index as u16),
            drawing_width - 1,
        );
    }
}

fn print_entry(
    buf: &mut Buffer,
    entry: &JoshutoDirEntry,
    style: Style,
    (x, y): (u16, u16),
    drawing_width: usize,
) {
    let size_string = match entry.metadata.file_type() {
        FileType::Directory => String::from(""),
        FileType::File => format::file_size_to_string(entry.metadata.len()),
    };
    let symlink_string = match entry.metadata.link_type() {
        LinkType::Normal => "",
        LinkType::Symlink(_) => "-> ",
    };
    let left_label_original = entry.label();
    let right_label_original = format!(" {}{}", symlink_string, size_string);
    let (left_label, right_label) =
        factor_labels_for_entry(left_label_original, right_label_original, drawing_width);
    let right_width = right_label.width();
    buf.set_stringn(x, y, left_label, drawing_width, style);
    buf.set_stringn(
        x + drawing_width as u16 - right_width as u16,
        y,
        right_label,
        drawing_width,
        style,
    );
}

fn factor_labels_for_entry(
    left_label_original: &str,
    right_label_original: String,
    drawing_width: usize,
) -> (String, String) {
    let left_width_remainder = drawing_width as i32 - right_label_original.width() as i32;
    let width_remainder = left_width_remainder as i32 - left_label_original.width() as i32;
    if width_remainder >= 0 {
        (
            left_label_original.to_string(),
            right_label_original.to_string(),
        )
    } else {
        if left_label_original.width() <= drawing_width {
            (left_label_original.to_string(), "".to_string())
        } else if left_width_remainder < MIN_LEFT_LABEL_WIDTH {
            (
                trim_file_label(left_label_original, drawing_width),
                "".to_string(),
            )
        } else {
            (
                trim_file_label(left_label_original, left_width_remainder as usize),
                right_label_original.to_string(),
            )
        }
    }
}

pub fn trim_file_label(name: &str, drawing_width: usize) -> String {
    // pre-condition: string name is longer than width
    let (stem, extension) = match name.rfind('.') {
        None => (name, ""),
        Some(i) => name.split_at(i),
    };
    if drawing_width < 1 {
        String::from("")
    } else if stem.is_empty() || extension.is_empty() {
        let full = format!("{}{}", stem, extension);
        let mut truncated = full.trunc(drawing_width - 1);
        truncated.push_str(ELLIPSIS);
        truncated
    } else {
        let ext_width = extension.width();
        if ext_width > drawing_width {
            // file ext does not fit
            ELLIPSIS.to_string()
        } else if ext_width == drawing_width {
            extension.to_string().replacen('.', ELLIPSIS, 1)
        } else {
            let stem_width = drawing_width - ext_width;
            let truncated_stem = stem.trunc(stem_width - 1);
            format!("{}{}{}", truncated_stem, ELLIPSIS, extension)
        }
    }
}

#[cfg(test)]
mod test_factor_labels {
    use super::{factor_labels_for_entry, MIN_LEFT_LABEL_WIDTH};

    #[test]
    fn both_labels_empty_if_drawing_width_zero() {
        let left = "foo.ext";
        let right = "right";
        assert_eq!(
            ("".to_string(), "".to_string()),
            factor_labels_for_entry(left, right.to_string(), 0)
        );
    }

    #[test]
    fn nothing_changes_if_all_labels_fit_easily() {
        let left = "foo.ext";
        let right = "right";
        assert_eq!(
            (left.to_string(), right.to_string()),
            factor_labels_for_entry(left, right.to_string(), 20)
        );
    }

    #[test]
    fn nothing_changes_if_all_labels_just_fit() {
        let left = "foo.ext";
        let right = "right";
        assert_eq!(
            (left.to_string(), right.to_string()),
            factor_labels_for_entry(left, right.to_string(), 12)
        );
    }

    #[test]
    fn right_label_omitted_if_left_label_would_need_to_be_shortened_below_min_left_label_width() {
        let left = "foobarbazfo.ext";
        let right = "right";
        assert!(left.chars().count() as i32 == MIN_LEFT_LABEL_WIDTH);
        assert_eq!(
            (left.to_string(), "".to_string()),
            factor_labels_for_entry(left, right.to_string(), MIN_LEFT_LABEL_WIDTH as usize)
        );
    }

    #[test]
    fn right_label_is_kept_if_left_label_is_not_shortened_below_min_left_label_width() {
        let left = "foobarbazfoobarbaz.ext";
        let right = "right";
        assert!(left.chars().count() as i32 > MIN_LEFT_LABEL_WIDTH + right.chars().count() as i32);
        assert_eq!(
            ("foobarbazf….ext".to_string(), right.to_string()),
            factor_labels_for_entry(
                left,
                right.to_string(),
                MIN_LEFT_LABEL_WIDTH as usize + right.chars().count()
            )
        );
    }
}

#[cfg(test)]
mod test_trim_file_label {
    use super::trim_file_label;

    #[test]
    fn dotfiles_get_an_ellipsis_at_the_end_if_they_dont_fit() {
        let label = ".joshuto";
        assert_eq!(".jos…".to_string(), trim_file_label(label, 5));
    }

    #[test]
    fn dotless_files_get_an_ellipsis_at_the_end_if_they_dont_fit() {
        let label = "Desktop";
        assert_eq!("Desk…".to_string(), trim_file_label(label, 5));
    }

    #[test]
    fn if_the_extension_doesnt_fit_just_an_ellipses_is_shown() {
        let label = "foo.ext";
        assert_eq!("…".to_string(), trim_file_label(label, 2));
    }

    #[test]
    fn if_just_the_extension_fits_its_shown_with_an_ellipsis_instead_of_a_dot() {
        let left = "foo.ext";
        assert_eq!("…ext".to_string(), trim_file_label(left, 4));
    }

    #[test]
    fn if_the_extension_fits_the_stem_is_truncated_with_an_appended_ellipsis_1() {
        let left = "foo.ext";
        assert_eq!("….ext".to_string(), trim_file_label(left, 5));
    }

    #[test]
    fn if_the_extension_fits_the_stem_is_truncated_with_an_appended_ellipsis_2() {
        let left = "foo.ext";
        assert_eq!("f….ext".to_string(), trim_file_label(left, 6));
    }

    #[test]
    fn if_the_name_is_truncated_after_a_full_width_character_the_ellipsis_is_shown_correctly() {
        let left = "🌕🌕🌕";
        assert_eq!("🌕…".to_string(), trim_file_label(left, 4));
    }

    #[test]
    fn if_the_name_is_truncated_within_a_full_width_character_the_ellipsis_is_shown_correctly() {
        let left = "🌕🌕🌕";
        assert_eq!("🌕🌕…".to_string(), trim_file_label(left, 5));
    }
}
