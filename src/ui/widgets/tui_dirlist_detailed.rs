use std::cmp::{min, Ordering};

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Widget;

use crate::config::clean::app::display::line_mode::LineMode;
use crate::config::clean::app::display::line_number::LineNumberStyle;
use crate::config::clean::app::display::tab::TabDisplayOption;
use crate::config::clean::app::display::DisplayOption;
use crate::fs::{FileType, JoshutoDirEntry, JoshutoDirList, LinkType};
use crate::util::string::UnicodeTruncate;
use crate::util::style;
use crate::util::{format, unix};
use unicode_width::UnicodeWidthStr;

const MIN_LEFT_LABEL_WIDTH: i32 = 15;

const ELLIPSIS: &str = "…";

pub struct TuiDirListDetailed<'a> {
    dirlist: &'a JoshutoDirList,
    display_options: &'a DisplayOption,
    tab_display_options: &'a TabDisplayOption,
    pub focused: bool,
}
impl<'a> TuiDirListDetailed<'a> {
    pub fn new(
        dirlist: &'a JoshutoDirList,
        display_options: &'a DisplayOption,
        tab_display_options: &'a TabDisplayOption,
        focused: bool,
    ) -> Self {
        Self {
            dirlist,
            display_options,
            tab_display_options,
            focused,
        }
    }
}

impl<'a> Widget for TuiDirListDetailed<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 4 || area.height < 1 {
            return;
        }

        let x = area.left();
        let y = area.top();
        let curr_index = match self.dirlist.get_index() {
            Some(i) => i,
            None => {
                let style = Style::default().bg(Color::Red).fg(Color::White);
                buf.set_stringn(x, y, "empty", area.width as usize, style);
                return;
            }
        };

        let drawing_width = area.width as usize;
        let skip_dist = self.dirlist.first_index_for_viewport();
        let line_num_style = self.display_options.line_nums();
        // Length (In chars) of the last entry's index on current page.
        // Using this to align all elements
        let max_index_length = (skip_dist
            + min(self.dirlist.len() - skip_dist, area.height as usize))
        .to_string()
        .len();

        let space_fill = " ".repeat(drawing_width);

        // draw every entry
        self.dirlist
            .iter()
            .skip(skip_dist)
            .enumerate()
            .take(area.height as usize)
            .for_each(|(i, entry)| {
                let ix = skip_dist + i;

                let style = if !self.focused {
                    style::entry_style(entry)
                } else if ix == curr_index {
                    style::entry_style(entry).add_modifier(Modifier::REVERSED)
                } else {
                    style::entry_style(entry)
                };

                buf.set_string(x, y + i as u16, space_fill.as_str(), style);

                let mut prefix = if entry.is_selected() {
                    " ".to_string()
                } else {
                    "".to_string()
                };
                let line_number_prefix = match line_num_style {
                    LineNumberStyle::None => "".to_string(),
                    _ if ix == curr_index => format!("{:<1$} ", curr_index + 1, max_index_length),
                    LineNumberStyle::Absolute => format!("{:1$} ", ix + 1, max_index_length),
                    LineNumberStyle::Relative => format!(
                        "{:1$} ",
                        (curr_index as i16 - ix as i16).abs(),
                        max_index_length
                    ),
                };
                prefix.push_str(&line_number_prefix);

                print_entry(
                    buf,
                    entry,
                    style,
                    (x + 1, y + i as u16),
                    self.tab_display_options.linemode,
                    drawing_width - 1,
                    &prefix,
                );
            });
    }
}

fn get_entry_size_string(entry: &JoshutoDirEntry) -> String {
    match entry.metadata.file_type() {
        FileType::Directory => entry
            .metadata
            .directory_size()
            .map(|n| n.to_string())
            .unwrap_or_default(),
        FileType::File => format::file_size_to_string(entry.metadata.len()),
    }
}

fn print_entry(
    buf: &mut Buffer,
    entry: &JoshutoDirEntry,
    style: Style,
    (x, y): (u16, u16),
    linemode: LineMode,
    drawing_width: usize,
    prefix: &str,
) {
    let symlink_string = match entry.metadata.link_type() {
        LinkType::Normal => "",
        LinkType::Symlink { .. } => "-> ",
    };
    let left_label_original = entry.label();
    let right_label_original = format!(
        " {}{} ",
        symlink_string,
        linemode
            .iter_names()
            .map(|f| match f.0 {
                "size" => get_entry_size_string(entry),
                "mtime" => format::mtime_to_string(entry.metadata.modified()),
                "user" => unix::uid_to_string(entry.metadata.uid).unwrap_or("unknown".into()),
                "group" => unix::gid_to_string(entry.metadata.gid).unwrap_or("unknown".into()),
                "perm" => unix::mode_to_string(entry.metadata.mode),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .join(" ")
    );

    // draw prefix first
    let prefix_width = prefix.width();
    buf.set_stringn(x, y, prefix, prefix_width, Style::default());
    let x = x + prefix_width as u16;

    // factor left_label and right_label
    let drawing_width = drawing_width - prefix_width;
    let (left_label, right_label) = factor_labels_for_entry(
        left_label_original,
        right_label_original.as_str(),
        drawing_width,
    );

    // Draw labels
    buf.set_stringn(x, y, left_label, drawing_width, style);
    buf.set_stringn(
        x + drawing_width as u16 - right_label.width() as u16,
        y,
        right_label,
        drawing_width,
        style,
    );
}

fn factor_labels_for_entry<'a>(
    left_label_original: &'a str,
    right_label_original: &'a str,
    drawing_width: usize,
) -> (String, &'a str) {
    let left_label_original_width = left_label_original.width();
    let right_label_original_width = right_label_original.width();

    let left_width_remainder = drawing_width as i32 - right_label_original_width as i32;
    let width_remainder = left_width_remainder - left_label_original_width as i32;

    if drawing_width == 0 {
        ("".to_string(), "")
    } else if width_remainder >= 0 {
        (left_label_original.to_string(), right_label_original)
    } else if left_width_remainder < MIN_LEFT_LABEL_WIDTH {
        (
            if left_label_original.width() as i32 <= left_width_remainder {
                trim_file_label(left_label_original, drawing_width)
            } else {
                left_label_original.to_string()
            },
            "",
        )
    } else {
        (
            trim_file_label(left_label_original, left_width_remainder as usize),
            right_label_original,
        )
    }
}

pub fn trim_file_label(name: &str, drawing_width: usize) -> String {
    // pre-condition: string name is longer than width
    let (stem, extension) = match name.rfind('.') {
        None => (name, ""),
        Some(i) => name.split_at(i),
    };
    if drawing_width < 1 {
        "".to_string()
    } else if stem.is_empty() || extension.is_empty() {
        let full = format!("{}{}", stem, extension);
        let mut truncated = full.trunc(drawing_width - 1);
        truncated.push_str(ELLIPSIS);
        truncated
    } else {
        let ext_width = extension.width();
        match ext_width.cmp(&drawing_width) {
            Ordering::Less => {
                let stem_width = drawing_width - ext_width;
                let truncated_stem = stem.trunc(stem_width - 1);
                format!("{}{}{}", truncated_stem, ELLIPSIS, extension)
            }
            Ordering::Equal => extension.replacen('.', ELLIPSIS, 1),
            Ordering::Greater => {
                // file ext does not fit
                let stem_width = drawing_width;
                let truncated_stem = stem.trunc(stem_width - 3);
                format!("{}{}.{}", truncated_stem, ELLIPSIS, ELLIPSIS)
            }
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
            ("".to_string(), ""),
            factor_labels_for_entry(left, right, 0)
        );
    }

    #[test]
    fn nothing_changes_if_all_labels_fit_easily() {
        let left = "foo.ext";
        let right = "right";
        assert_eq!(
            (left.to_string(), right),
            factor_labels_for_entry(left, right, 20)
        );
    }

    #[test]
    fn nothing_changes_if_all_labels_just_fit() {
        let left = "foo.ext";
        let right = "right";
        assert_eq!(
            (left.to_string(), right),
            factor_labels_for_entry(left, right, 12)
        );
    }

    #[test]
    fn right_label_omitted_if_left_label_would_need_to_be_shortened_below_min_left_label_width() {
        let left = "foobarbazfo.ext";
        let right = "right";
        assert!(left.chars().count() as i32 == MIN_LEFT_LABEL_WIDTH);
        assert_eq!(
            ("foobarbazfo.ext".to_string(), ""),
            factor_labels_for_entry(left, right, MIN_LEFT_LABEL_WIDTH as usize)
        );
    }

    #[test]
    fn right_label_is_kept_if_left_label_is_not_shortened_below_min_left_label_width() {
        let left = "foobarbazfoobarbaz.ext";
        let right = "right";
        assert!(left.chars().count() as i32 > MIN_LEFT_LABEL_WIDTH + right.chars().count() as i32);
        assert_eq!(
            ("foobarbazf….ext".to_string(), right),
            factor_labels_for_entry(
                left,
                right,
                MIN_LEFT_LABEL_WIDTH as usize + right.chars().count()
            )
        );
    }

    #[test]
    // regression
    fn file_name_which_is_smaller_or_equal_drawing_width_does_not_cause_right_label_to_be_omitted()
    {
        let left = "foooooobaaaaaaarbaaaaaaaaaz";
        let right = "right";
        assert!(left.chars().count() as i32 > MIN_LEFT_LABEL_WIDTH);
        assert_eq!(
            ("foooooobaaaaaaarbaaaa…".to_string(), right),
            factor_labels_for_entry(left, right, left.chars().count())
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
    fn if_the_extension_doesnt_fit_show_stem_with_double_ellipse() {
        let label = "12345678.12345678910";
        assert_eq!("12345….…".to_string(), trim_file_label(label, 8));
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
