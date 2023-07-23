use std::{collections::HashMap, path::PathBuf};

use ratatui::layout::Constraint;

use crate::config::option::LineMode;
use crate::config::option::SortOption;

#[derive(Clone, Copy, Debug)]
pub enum DisplayMode {
    Default,
    HSplit,
}

pub const fn default_column_ratio() -> (usize, usize, usize) {
    (1, 3, 4)
}

/// Display options globally valid for Joshuto (for all tabs)
#[derive(Clone, Debug)]
pub struct DisplayOption {
    pub _mode: DisplayMode,
    pub _automatically_count_files: bool,
    pub _collapse_preview: bool,
    pub _scroll_offset: usize,
    pub _show_borders: bool,
    pub _show_hidden: bool,
    pub _show_icons: bool,
    pub _tilde_in_titlebar: bool,
    pub _line_nums: LineNumberStyle,
    pub column_ratio: (usize, usize, usize),
    pub default_layout: [Constraint; 3],
    pub no_preview_layout: [Constraint; 3],
    pub default_tab_display_option: TabDisplayOption,
}

/// Display options valid pre JoshutoDirList in a JoshutoTab
#[derive(Clone, Debug, Default)]
pub struct DirListDisplayOptions {
    filter_string: String,
    depth: u8,
}

/// Display options valid per JoshutoTab
#[derive(Clone, Debug, Default)]
pub struct TabDisplayOption {
    pub dirlist_options: HashMap<PathBuf, DirListDisplayOptions>,
    pub sort_options: SortOption,
    pub linemode: LineMode,
}

#[derive(Clone, Copy, Debug)]
pub enum LineNumberStyle {
    None,
    Relative,
    Absolute,
}

impl LineNumberStyle {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "absolute" => Some(Self::Absolute),
            "relative" => Some(Self::Relative),
            "none" => Some(Self::None),
            _ => None,
        }
    }
}

impl DirListDisplayOptions {
    pub fn set_filter_string(&mut self, pattern: &str) {
        self.filter_string = pattern.to_owned();
    }

    pub fn filter_string_ref(&self) -> &str {
        &self.filter_string
    }

    pub fn set_depth(&mut self, depth: u8) {
        self.depth = depth;
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }
}

impl TabDisplayOption {
    pub fn sort_options_ref(&self) -> &SortOption {
        &self.sort_options
    }

    pub fn sort_options_mut(&mut self) -> &mut SortOption {
        &mut self.sort_options
    }

    pub fn dirlist_options_ref(&self, path: &PathBuf) -> Option<&DirListDisplayOptions> {
        self.dirlist_options.get(path)
    }

    pub fn dirlist_options_mut(&mut self, path: &PathBuf) -> &mut DirListDisplayOptions {
        if !self.dirlist_options.contains_key(path) {
            self.dirlist_options
                .insert(path.to_owned(), Default::default());
        }
        self.dirlist_options.get_mut(path).unwrap()
    }
}

impl DisplayOption {
    pub fn mode(&self) -> DisplayMode {
        self._mode
    }

    pub fn automatically_count_files(&self) -> bool {
        self._automatically_count_files
    }

    pub fn collapse_preview(&self) -> bool {
        self._collapse_preview
    }

    pub fn scroll_offset(&self) -> usize {
        self._scroll_offset
    }

    pub fn show_borders(&self) -> bool {
        self._show_borders
    }

    pub fn show_hidden(&self) -> bool {
        self._show_hidden
    }

    pub fn show_icons(&self) -> bool {
        self._show_icons
    }

    pub fn set_show_hidden(&mut self, show_hidden: bool) {
        self._show_hidden = show_hidden;
    }

    pub fn tilde_in_titlebar(&self) -> bool {
        self._tilde_in_titlebar
    }

    pub fn line_nums(&self) -> LineNumberStyle {
        self._line_nums
    }

    pub fn set_line_nums(&mut self, style: LineNumberStyle) {
        self._line_nums = style;
    }

    pub fn filter_func(
        &self,
    ) -> fn(&walkdir::DirEntry, &DisplayOption, &DirListDisplayOptions) -> bool {
        filter
    }
}

impl std::default::Default for DisplayOption {
    fn default() -> Self {
        let column_ratio = default_column_ratio();

        let total = (column_ratio.0 + column_ratio.1 + column_ratio.2) as u32;
        let default_layout = [
            Constraint::Ratio(column_ratio.0 as u32, total),
            Constraint::Ratio(column_ratio.1 as u32, total),
            Constraint::Ratio(column_ratio.2 as u32, total),
        ];
        let no_preview_layout = [
            Constraint::Ratio(column_ratio.0 as u32, total),
            Constraint::Ratio(column_ratio.1 as u32 + column_ratio.2 as u32, total),
            Constraint::Ratio(0, total),
        ];

        Self {
            _mode: DisplayMode::Default,
            _automatically_count_files: false,
            _collapse_preview: true,
            column_ratio,
            _scroll_offset: 4,
            _show_borders: true,
            _show_hidden: false,
            _show_icons: false,
            _tilde_in_titlebar: true,
            _line_nums: LineNumberStyle::None,
            default_layout,
            no_preview_layout,
            default_tab_display_option: TabDisplayOption::default(),
        }
    }
}

fn has_str(entry: &walkdir::DirEntry, pat: &str) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| {
            s.to_ascii_lowercase()
                .as_str()
                .contains(pat.to_ascii_lowercase().as_str())
        })
        .unwrap_or(false)
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn filter(
    entry: &walkdir::DirEntry,
    opt: &DisplayOption,
    dirlist_opts: &DirListDisplayOptions,
) -> bool {
    if opt.show_hidden() && dirlist_opts.filter_string_ref().is_empty() {
        true
    } else if dirlist_opts.filter_string_ref().is_empty() {
        !is_hidden(entry)
    } else if opt.show_hidden() || !is_hidden(entry) {
        has_str(entry, dirlist_opts.filter_string_ref())
    } else {
        false
    }
}
