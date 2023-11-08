use ratatui::text::Span;
use std::isize;
use std::path::Path;

use crate::config::clean::theme::tab::TabTheme;
use crate::util::string::UnicodeTruncate;
use crate::HOME_DIR;
use unicode_width::UnicodeWidthStr;

// This file provides stuff to factor a tab-line from a list of paths.
//
// The tab-line logic uses these basic cases:
//
// * Case 0: Not enough space for anything useful. ⇒ Empty tab line
// * Case 1: All [1..n] tabs fit in long form
// * If only one tab exists, then...
//   * Case 2: The only tab did _not_ fit in long form (would have been Case 1)...
//     * Case 2a: Single tab fits in short form
//     * Case 2b: Single tab fits only in short form, without pre- and postfix
//     * Case 2c: Single tab fits only without pre- and postfix, and further
//                shortened with an ellipsis
// * If more than one tab exist, then...
//   * Case 3: The active tab fits in long form together with the others in short form
//   * Case 4: Not all tabs can be shown; Scrolling needed...
//     * Case 4a: The active tab fits between the scroll tags in long form
//     * Case 4b: The active tab fits between the scroll tags only in short form
//     * Case 4c: The active tab fits only without scroll tags and pre- and postfix
//     * Case 4d: The active tab fits only without scroll tags and pre- and postfix
//               and further shortened with ellipsis

pub struct TabLabel {
    long: String,
    short: String,
}

impl TabLabel {
    fn from_path(path: &Path) -> TabLabel {
        let mut full_path_str = path.as_os_str().to_str().unwrap().to_string();
        if let Some(home_dir) = HOME_DIR.as_ref() {
            let home_dir_str = home_dir.to_string_lossy().into_owned();
            full_path_str = full_path_str.replace(&home_dir_str, "~");
        }
        // eprintln!("full_path_str: {:?}", full_path_str);
        let last = Path::new(&full_path_str)
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();
        TabLabel {
            long: full_path_str,
            short: last,
        }
    }
}

#[derive(PartialEq, Debug)]
enum TabBarElement {
    // Note: The Tab-Elements also store the index, eventhough it's not used
    // for the tab-bar rendering at all. The reason is that this would allow
    // an easier implementation for other features later on, like changing
    // tabs by mouse clicks.
    // It's a valuable information that comes for free here.
    DividerII,
    DividerIA,
    DividerAI,
    PrefixI,
    PostfixI,
    PrefixA,
    PostfixA,
    TabI(usize, String), // index, label-string
    TabA(usize, String), // index, label-string
    ScrollFrontPrefix,
    ScrollFrontPostfix,
    ScrollFront(usize), // number of hidden tabs
    ScrollBackPrefix,
    ScrollBackPostfix,
    ScrollBack(usize), // number of hidden tab
    PaddingPrefix,
    PaddingPostfix,
    PaddingFill(usize), // width of padding
}

fn check_fit_and_build_sequence(
    labels: &[String],
    available_width: usize,
    extra_width: usize,
    current_index: usize,
) -> Option<Vec<TabBarElement>> {
    if labels.iter().map(|l| l.width()).sum::<usize>() + extra_width <= available_width {
        let r = labels
            .iter()
            .enumerate()
            .flat_map(|(ix, l)| {
                let mut section = Vec::with_capacity(4);
                if ix > 0 {
                    if ix == current_index {
                        section.push(TabBarElement::DividerIA);
                    } else if ix - 1 == current_index {
                        section.push(TabBarElement::DividerAI);
                    } else {
                        section.push(TabBarElement::DividerII);
                    }
                };
                if ix == current_index {
                    section.push(TabBarElement::PrefixA);
                    section.push(TabBarElement::TabA(ix, l.to_string()));
                    section.push(TabBarElement::PostfixA);
                } else {
                    section.push(TabBarElement::PrefixI);
                    section.push(TabBarElement::TabI(ix, l.to_string()));
                    section.push(TabBarElement::PostfixI);
                }
                section
            })
            .collect();
        Some(r)
    } else {
        None
    }
}

fn factor_tab_bar_sequence(
    available_width: usize,
    tab_records: &Vec<&TabLabel>,
    current_index: usize,
    config: &TabTheme,
) -> Vec<TabBarElement> {
    //##################################################################
    //## Case 0: less available width than 2 units -> just show nothing
    //##################################################################
    if available_width < 2 {
        return vec![];
    }
    //## Not case 0. Let's continue...

    let tab_num = tab_records.len();
    let labels_are_indexed = tab_num > 1;
    let extra_width = config.inference.tab_divider_length * (tab_num - 1)
        + config.inference.active_tab_extra_width
        + config.inference.inactive_tab_extra_width * (tab_num - 1);

    let all_labels_as_long: Vec<_> = tab_records
        .iter()
        .enumerate()
        .map(|(ix, &r)| {
            if labels_are_indexed {
                format!("{}: {}", ix + 1, &r.long)
            } else {
                String::from(&r.long)
            }
        })
        .collect();

    //##############################################################
    //## Case 1: all tabs fit with long form label
    //##############################################################
    if let Some(result) = check_fit_and_build_sequence(
        &all_labels_as_long,
        available_width,
        extra_width,
        current_index,
    ) {
        return result;
    }
    //## Not case 1. Let's continue...

    //##############################################################
    //## Case 2: single tab that does _not_ fit in long form
    //##############################################################
    if tab_num == 1 {
        let label = String::from(&tab_records[0].short);
        if label.width() as isize
            <= available_width as isize - config.inference.active_tab_extra_width as isize
        {
            return vec![
                TabBarElement::PrefixA,
                TabBarElement::TabA(0, label),
                TabBarElement::PostfixA,
            ];
        }
        if label.width() <= available_width {
            return vec![TabBarElement::TabA(0, label)];
        }
        return vec![TabBarElement::TabA(
            0,
            format!("{}…", label.trunc(available_width - 1)),
        )];
    }
    //## Not case 2. Let's continue...

    let all_labels_as_short: Vec<_> = tab_records
        .iter()
        .enumerate()
        .map(|(ix, &r)| {
            if labels_are_indexed {
                format!("{}: {}", ix + 1, &r.short)
            } else {
                String::from(&r.short)
            }
        })
        .collect();

    let all_labels_as_short_except_current: Vec<String> = all_labels_as_short
        .iter()
        .zip(all_labels_as_long.iter())
        .enumerate()
        .map(|(ix, (short, long))| {
            if ix == current_index {
                long.to_string()
            } else {
                short.to_string()
            }
        })
        .collect();

    //####################################################################
    //## Case 3: active tab fits in long form and all others in short form
    //#####################################################################
    if let Some(result) = check_fit_and_build_sequence(
        &all_labels_as_short_except_current,
        available_width,
        extra_width,
        current_index,
    ) {
        return result;
    }
    //## Not case 3. Let's continue...

    //####################################################################
    //## Case 4: more than one tab and the tabs don't fit at once
    //##         (=> we need scrolling)
    //####################################################################
    let scroll_tags_width = config.inference.calc_scroll_tags_width(tab_num);
    let scrollable_width = if scroll_tags_width > available_width {
        0
    } else {
        available_width - scroll_tags_width
    };

    let carousel_labels = if all_labels_as_long[current_index].width()
        + config.inference.active_tab_extra_width
        <= scrollable_width
    {
        //## Case 4a: active tab fits in long form between scroll tags
        &all_labels_as_short_except_current
    } else if all_labels_as_short[current_index].width() + config.inference.active_tab_extra_width
        <= scrollable_width
    {
        //## Case 4b: active tab fits only in short form between scroll tags
        &all_labels_as_short
    } else if all_labels_as_short[current_index].width() <= available_width {
        //## Case 4c: active tab fits only without scroll tags and pre- and postfix
        return vec![TabBarElement::TabA(
            current_index,
            String::from(&all_labels_as_short[current_index]),
        )];
    } else {
        //## Case 4d: active only fits when shortened with ellipsis
        return vec![TabBarElement::TabA(
            current_index,
            format!(
                "{}…",
                &all_labels_as_short[current_index].trunc(available_width - 1)
            ),
        )];
    };

    //####################################################################
    //## Case 4a/b: Sub-set of tabs shown with scroll tags
    //####################################################################
    // Preparing mutable variables for the following looping alogorithm.
    // If we end up here, we know that at least the scroll tags and the label of the
    // current tab fit into the available space.
    let mut remaining_width = scrollable_width as isize;
    let mut tab_bar_elements: Vec<TabBarElement> = Vec::new();
    let mut ix_front = if current_index >= 1 {
        Some(current_index - 1)
    } else {
        None
    };
    let mut ix_back = Some(current_index);
    let mut try_to_take_next_from_front = false;
    let mut remaining_left: Option<usize> = None;
    let mut remaining_right: Option<usize> = None;

    // Loop until the visible space of the tab-bar is full or until we run out of tabs
    loop {
        // Pick the next tab alternating from before and after the current tab as long as
        // another tab is available from the respective side.
        let (taking_from_front, ix_to_take) = match (ix_front, ix_back) {
            (Some(ixf), Some(ixb)) => (
                try_to_take_next_from_front,
                if try_to_take_next_from_front {
                    ixf
                } else {
                    ixb
                },
            ),
            (Some(ixf), None) => (true, ixf), //only tabs before current to be added
            (None, Some(ixb)) => (false, ixb), //only tabs after current to be added
            (None, None) => break,            //for both sides hold, either there are no more
                                               //tabs or the next tab does not fit anymore
        };
        try_to_take_next_from_front = !try_to_take_next_from_front;
        let is_active_tab = ix_to_take == current_index;

        // Pick this loop's tab that shall be added to the tab-bar if it still fits
        if let Some(label_to_add) = carousel_labels.get(ix_to_take) {
            let needed_width = (if is_active_tab {
                config.inference.active_tab_extra_width
            } else {
                config.inference.inactive_tab_extra_width + config.inference.tab_divider_length
            } + label_to_add.width()) as isize;
            if needed_width <= remaining_width {
                // This next tab still fits into the available space; let's add it...
                remaining_width -= needed_width;
                if taking_from_front {
                    // Next tab is added to the front.
                    // This cannot be the active tab because we start adding from the back side.
                    assert!(!is_active_tab);
                    // Add the tab-bar elements...
                    tab_bar_elements.insert(
                        0,
                        if ix_to_take + 1 == current_index {
                            TabBarElement::DividerIA
                        } else {
                            TabBarElement::DividerII
                        },
                    );
                    tab_bar_elements.insert(0, TabBarElement::PostfixI);
                    tab_bar_elements
                        .insert(0, TabBarElement::TabI(ix_to_take, label_to_add.to_string()));
                    tab_bar_elements.insert(0, TabBarElement::PrefixI);
                    // Prepare the next loop...
                    if ix_to_take > 0 {
                        ix_front = Some(ix_to_take - 1);
                    } else {
                        ix_front = None;
                    }
                } else {
                    // Next tab is added to the back.
                    // Add the tab-bar elements...
                    if !is_active_tab {
                        tab_bar_elements.push(
                            if ix_to_take > 0 && ix_to_take - 1 == current_index {
                                TabBarElement::DividerAI
                            } else {
                                TabBarElement::DividerII
                            },
                        );
                    }
                    if is_active_tab {
                        tab_bar_elements.push(TabBarElement::PrefixA);
                        tab_bar_elements
                            .push(TabBarElement::TabA(ix_to_take, label_to_add.to_string()));
                        tab_bar_elements.push(TabBarElement::PostfixA);
                    } else {
                        tab_bar_elements.push(TabBarElement::PrefixI);
                        tab_bar_elements
                            .push(TabBarElement::TabI(ix_to_take, label_to_add.to_string()));
                        tab_bar_elements.push(TabBarElement::PostfixI);
                    }
                    // Prepare the next loop...
                    if ix_to_take < tab_num - 1 {
                        ix_back = Some(ix_to_take + 1);
                    } else {
                        ix_back = None;
                    }
                }
            } else {
                // This next tab does NOT fit anymore.
                // This must not be the active tab though, as this case was handeled before.
                assert!(!is_active_tab);
                // Break the processing of the current side,
                // and store the number of remaining tabs for the respective scroll tag.
                if taking_from_front {
                    remaining_left = Some(ix_to_take + 1);
                    ix_front = None;
                } else {
                    remaining_right = Some(tab_num - ix_to_take);
                    ix_back = None;
                }
            }
        } else {
            // The index for the label to pick must always exists.
            // Otherwise, there would be a logical bug in this algorithm.
            unreachable!();
        }
    } // End of loop building up the scrollable tab-bar.
      // The visible elements for the scrollable bar are composed.
      // Only the scroll-tags on each end of the bar and the padding are missing.
      // Let's add them...
      // left scroll tag...
    tab_bar_elements.insert(0, TabBarElement::ScrollFrontPostfix);
    tab_bar_elements.insert(
        0,
        TabBarElement::ScrollFront(if let Some(remains) = remaining_left {
            remains
        } else {
            0
        }),
    );
    tab_bar_elements.insert(0, TabBarElement::ScrollFrontPrefix);
    // padding...
    if remaining_width > 0 {
        tab_bar_elements.push(TabBarElement::PaddingPrefix);
        if remaining_width > 2 {
            tab_bar_elements.push(TabBarElement::PaddingFill(remaining_width as usize - 2));
        }
        if remaining_width > 1 {
            tab_bar_elements.push(TabBarElement::PaddingPostfix);
        }
    }
    // right scroll tag...
    tab_bar_elements.push(TabBarElement::ScrollBackPrefix);
    tab_bar_elements.push(TabBarElement::ScrollBack(
        if let Some(remains) = remaining_right {
            remains
        } else {
            0
        },
    ));
    tab_bar_elements.push(TabBarElement::ScrollBackPostfix);

    // That was it...
    tab_bar_elements
}

fn factor_tab_bar_spans_from_sequence<'a>(
    tab_bar_elements: Vec<TabBarElement>,
    config: &TabTheme,
) -> Vec<Span<'a>> {
    tab_bar_elements
        .into_iter()
        .map(|e| match e {
            TabBarElement::PrefixA => {
                Span::styled(String::from(&config.chars.prefix_a), config.styles.prefix_a)
            }
            TabBarElement::PostfixA => Span::styled(
                String::from(&config.chars.postfix_a),
                config.styles.postfix_a,
            ),
            TabBarElement::TabA(_ix, s) => Span::styled(s, config.styles.tab_a),
            TabBarElement::PrefixI => {
                Span::styled(String::from(&config.chars.prefix_i), config.styles.prefix_i)
            }
            TabBarElement::PostfixI => Span::styled(
                String::from(&config.chars.postfix_i),
                config.styles.postfix_i,
            ),
            TabBarElement::TabI(_ix, s) => Span::styled(s, config.styles.tab_i),
            TabBarElement::DividerII => Span::styled(
                String::from(&config.chars.divider),
                config.styles.divider_ii,
            ),
            TabBarElement::DividerAI => Span::styled(
                String::from(&config.chars.divider),
                config.styles.divider_ai,
            ),
            TabBarElement::DividerIA => Span::styled(
                String::from(&config.chars.divider),
                config.styles.divider_ia,
            ),
            TabBarElement::ScrollFront(s) => Span::styled(
                format!(
                    "{}{}{}",
                    &config.chars.scroll_front_prestring, s, &config.chars.scroll_front_poststring,
                ),
                config.styles.scroll_front,
            ),
            TabBarElement::ScrollFrontPrefix => Span::styled(
                String::from(&config.chars.scroll_front_prefix),
                config.styles.scroll_front_prefix,
            ),
            TabBarElement::ScrollFrontPostfix => Span::styled(
                String::from(&config.chars.scroll_front_postfix),
                config.styles.scroll_front_postfix,
            ),
            TabBarElement::ScrollBack(s) => Span::styled(
                format!(
                    "{}{}{}",
                    &config.chars.scroll_back_prestring, s, &config.chars.scroll_back_poststring,
                ),
                config.styles.scroll_back,
            ),
            TabBarElement::ScrollBackPrefix => Span::styled(
                String::from(&config.chars.scroll_back_prefix),
                config.styles.scroll_back_prefix,
            ),
            TabBarElement::ScrollBackPostfix => Span::styled(
                String::from(&config.chars.scroll_back_postfix),
                config.styles.scroll_back_postfix,
            ),
            TabBarElement::PaddingPrefix => Span::styled(
                String::from(config.chars.padding_prefix),
                config.styles.padding_prefix,
            ),
            TabBarElement::PaddingPostfix => Span::styled(
                String::from(config.chars.padding_postfix),
                config.styles.padding_postfix,
            ),
            TabBarElement::PaddingFill(n) => Span::styled(
                String::from(config.chars.padding_fill).repeat(n),
                config.styles.padding_fill,
            ),
        })
        .collect()
}

pub fn factor_tab_bar_spans<'a>(
    available_width: usize,
    tab_paths: &[&'a Path],
    current_index: usize,
    config: &TabTheme,
) -> Vec<Span<'a>> {
    let reps: Vec<TabLabel> = tab_paths.iter().map(|p| TabLabel::from_path(p)).collect();
    let rep_refs: Vec<&TabLabel> = reps.iter().collect();
    let tab_bar_elements =
        factor_tab_bar_sequence(available_width, &rep_refs, current_index, config);
    let tab_bar = factor_tab_bar_spans_from_sequence(tab_bar_elements, config);
    tab_bar
}

#[cfg(test)]
mod tests_facator_tab_bar_sequence {
    use crate::config::clean::theme::tab::TabTheme;
    use crate::config::raw::theme::tab::TabThemeRaw;

    use super::{factor_tab_bar_sequence, TabBarElement, TabLabel};

    fn test_config() -> TabTheme {
        let raw = TabThemeRaw {
            ..Default::default()
        };
        TabTheme::from(raw)
    }

    #[test]
    /// If there is less available width than 2,
    /// an empty tab line is returned.
    fn too_little_available_width_for_anything() {
        // Given
        let tabs = vec![TabLabel {
            long: "/foo/a".to_string(),
            short: "a".to_string(),
        }];
        // When
        let elements = factor_tab_bar_sequence(1, &tabs.iter().collect(), 0, &test_config());
        // Then
        assert_eq!(Vec::<TabBarElement>::new(), elements)
    }

    #[test]
    /// All tabs - even a single one - has a prefix and a postfix.
    /// In this test case, a single tab just fits exactly to the last character
    /// (`[/foo/a]` is exactly a width of 8).
    fn one_tab_that_fits() {
        // Given
        let tabs = vec![TabLabel {
            long: "/foo/a".to_string(),
            short: "a".to_string(),
        }];
        // When
        let elements = factor_tab_bar_sequence(8, &tabs.iter().collect(), 0, &test_config());
        // Then
        assert_eq!(
            vec![
                TabBarElement::PrefixA,
                TabBarElement::TabA(0, "/foo/a".to_string()),
                TabBarElement::PostfixA,
            ],
            elements
        )
    }

    #[test]
    /// If there's only a single tab, and the long label does not fit, the short version
    /// is used instead.
    /// In this test case, the available width is too short by one unit, so the short
    /// version of the label will be shown
    /// (`[a]`).
    fn one_tab_that_fits_only_in_short_form() {
        // Given
        let tabs = vec![TabLabel {
            long: "/foo/a".to_string(),
            short: "a".to_string(),
        }];
        // When
        let elements = factor_tab_bar_sequence(7, &tabs.iter().collect(), 0, &test_config());
        // Then
        assert_eq!(
            vec![
                TabBarElement::PrefixA,
                TabBarElement::TabA(0, "a".to_string()),
                TabBarElement::PostfixA,
            ],
            elements
        )
    }

    #[test]
    /// If there's only a single tab, and the long label does not fit,
    /// and the short version does not fit with the prefix and postfix,
    /// the prefix and postfix are omitted.
    /// In this test case, the available width is too short by one unit
    /// to use the short form with pre- and post-fix.
    /// (`aaaaaaa`).
    fn one_tab_that_fits_only_in_short_form_without_prepostfixes() {
        // Given
        let tabs = vec![TabLabel {
            long: "/foo/a".to_string(),
            short: "aaaaaaa".to_string(),
        }];
        // When
        let elements = factor_tab_bar_sequence(7, &tabs.iter().collect(), 0, &test_config());
        // Then
        assert_eq!(
            vec![TabBarElement::TabA(0, "aaaaaaa".to_string()),],
            elements
        )
    }

    #[test]
    /// If there's only a single tab, and the long label does not fit,
    /// and the short version does not fit even without the prefix and postfix,
    /// the prefix and postfix are omitted and the label is shortened with an ellipsis.
    /// In this test case, the available width is too short by one unit
    /// to use the short form without pre- and post-fix.
    /// (`aaaaa…`).
    fn case_2c_one_tab_that_does_not_fit_unless_further_shortened() {
        // Given
        let tabs = vec![TabLabel {
            long: "/foo/a".to_string(),
            short: "aaaaaaa".to_string(),
        }];
        // When
        let elements = factor_tab_bar_sequence(6, &tabs.iter().collect(), 0, &test_config());
        // Then
        assert_eq!(
            vec![TabBarElement::TabA(0, "aaaaa…".to_string()),],
            elements
        )
    }

    #[test]
    /// If there are multiple tabs, a delimiter is put between each two
    /// adjacent tabs and the label of each tab is prefixed with the tab's index,
    /// starting counting at 1.
    /// In this test case, the tab line just fits exactly into the available width
    /// (`[1: /foo/a]| 2: /foo/b ` has exactly a width of 23 )
    fn case_1_two_tabs_that_fit() {
        // Given
        let tabs = vec![
            TabLabel {
                long: "/foo/a".to_string(),
                short: "a".to_string(),
            },
            TabLabel {
                long: "/foo/b".to_string(),
                short: "b".to_string(),
            },
        ];
        // When
        let elements = factor_tab_bar_sequence(23, &tabs.iter().collect(), 0, &test_config());
        // Then
        assert_eq!(
            vec![
                TabBarElement::PrefixA,
                TabBarElement::TabA(0, "1: /foo/a".to_string()),
                TabBarElement::PostfixA,
                TabBarElement::DividerAI,
                TabBarElement::PrefixI,
                TabBarElement::TabI(1, "2: /foo/b".to_string()),
                TabBarElement::PostfixI,
            ],
            elements
        )
    }

    #[test]
    /// If there are multiple tabs and not all fit with their long-form labels,
    /// the inactive tabs are shown with their short-form labels.
    /// In this test case, the tab line's available width is just one unit to short for the long
    /// form, so the inactive tab will be shown with its shortened label
    /// (`[1: /foo/a]| 2: b `).
    fn case_3_two_tabs_fit_shortened() {
        // Given
        let tabs = vec![
            TabLabel {
                long: "/foo/a".to_string(),
                short: "a".to_string(),
            },
            TabLabel {
                long: "/foo/b".to_string(),
                short: "b".to_string(),
            },
        ];
        // When
        let elements = factor_tab_bar_sequence(22, &tabs.iter().collect(), 0, &test_config());
        // Then
        assert_eq!(
            vec![
                TabBarElement::PrefixA,
                TabBarElement::TabA(0, "1: /foo/a".to_string()),
                TabBarElement::PostfixA,
                TabBarElement::DividerAI,
                TabBarElement::PrefixI,
                TabBarElement::TabI(1, "2: b".to_string()),
                TabBarElement::PostfixI,
            ],
            elements
        )
    }

    #[test]
    /// If there are multiple tabs and scrolling is needed,
    /// when the long form of the active tab does not fit in between the scroll tags,
    /// the short form is used also for the current tab.
    ///
    /// This test would result in this tab-bar (dots are padding):
    /// `«0 [1: long_name_a]···· 1»`
    fn multiple_tabs_but_active_one_does_only_fit_in_short_form_with_scroll_tags() {
        // Given
        let tabs = vec![
            TabLabel {
                long: "/foo/long_name_a".to_string(),
                short: "long_name_a".to_string(),
            },
            TabLabel {
                long: "/foo/long_name_b".to_string(),
                short: "long_name_b".to_string(),
            },
        ];
        // When
        let elements = factor_tab_bar_sequence(
            16 + 3 + 2 + 6 - 1, //label + tab-index + pre/postfix + scroll tags - 1 to make it not fit
            &tabs.iter().collect(),
            0,
            &test_config(),
        );
        // Then
        assert_eq!(
            vec![
                TabBarElement::ScrollFrontPrefix,
                TabBarElement::ScrollFront(0),
                TabBarElement::ScrollFrontPostfix,
                TabBarElement::PrefixA,
                TabBarElement::TabA(0, "1: long_name_a".to_string()),
                TabBarElement::PostfixA,
                TabBarElement::PaddingPrefix,
                TabBarElement::PaddingFill(2),
                TabBarElement::PaddingPostfix,
                TabBarElement::ScrollBackPrefix,
                TabBarElement::ScrollBack(1),
                TabBarElement::ScrollBackPostfix,
            ],
            elements
        )
    }

    #[test]
    /// If there are multiple tabs and scrolling is needed,
    /// when even the short form of the active tab does not fit in between the scroll tags,
    /// the scroll tags and the tab's pre- and postfix are omitted.
    ///
    /// This test would result in this tab-bar (dots are padding):
    /// `1: long_name_a`
    fn multiple_tabs_but_active_one_does_not_fit_in_short_form_with_scroll_tags() {
        // Given
        let tabs = vec![
            TabLabel {
                long: "/foo/long_name_a".to_string(),
                short: "long_name_a".to_string(),
            },
            TabLabel {
                long: "/foo/long_name_b".to_string(),
                short: "long_name_b".to_string(),
            },
        ];
        // When
        let elements = factor_tab_bar_sequence(
            21, //label + tab-index + pre/postfix + scroll tags - 1 to make it not fit
            &tabs.iter().collect(),
            0,
            &test_config(),
        );
        // Then
        assert_eq!(
            vec![TabBarElement::TabA(0, "1: long_name_a".to_string()),],
            elements
        )
    }

    #[test]
    /// If there are multiple tabs and scrolling is needed,
    /// when even the short form alone does not fit in the available width,
    /// only the fitting part of the short form with an ellipsis is shown.
    ///
    /// This test would result in this tab-bar (dots are padding):
    /// `1: long…`
    fn multiple_tabs_but_active_one_does_not_fit_in_short_even_without_scroll_tags() {
        // Given
        let tabs = vec![
            TabLabel {
                long: "/foo/long_name_a".to_string(),
                short: "long_name_a".to_string(),
            },
            TabLabel {
                long: "/foo/long_name_b".to_string(),
                short: "long_name_b".to_string(),
            },
        ];
        // When
        let elements = factor_tab_bar_sequence(
            8, //label + tab-index + pre/postfix + scroll tags - 1 to make it not fit
            &tabs.iter().collect(),
            0,
            &test_config(),
        );
        // Then
        assert_eq!(
            vec![TabBarElement::TabA(0, "1: long…".to_string()),],
            elements
        )
    }

    /// Scenario, used for the next four test cases.
    fn _scenario_three_tabs(
        available_width: usize,
        current_index: usize,
        expected: Vec<TabBarElement>,
    ) {
        // Given
        let tabs = vec![
            TabLabel {
                long: "/foo/long_name_a".to_string(),
                short: "long_name_a".to_string(),
            },
            TabLabel {
                long: "/foo/long_name_b".to_string(),
                short: "long_name_b".to_string(),
            },
            TabLabel {
                long: "/foo/long_name_c".to_string(),
                short: "long_name_c".to_string(),
            },
        ];
        // When
        let elements = factor_tab_bar_sequence(
            available_width,
            &tabs.iter().collect(),
            current_index,
            &test_config(),
        );
        // Then
        assert_eq!(expected, elements)
    }

    #[test]
    /// When one or more tabs don't fit anymore into the bar, “scroll tags” are added to both ends,
    /// showing the number of hidden tabs for each side.
    ///
    /// The bar starts with the current tab, and then adds more tabs one by one to each
    /// side alternating. The first inacive tab added is added to the front.
    ///
    /// In this test case, there are three tabs with the first on active which would produce a
    /// tab-line like this if there is enough space:
    /// `[1: /foo/long_name_a]| 2: long_name_b | 3: long_name_c ` (width 55)
    ///
    /// But we choose a available width which is to short.
    /// It's just long enough to fit the two tabs and the scroll tags.
    /// So, this example results in a tab-bar like this:
    /// `«0·[1: /foo/long_name_a]|·2: long_name_b··1»` (width 44)
    fn three_tabs_with_overflow_on_the_right_w_no_padding() {
        _scenario_three_tabs(
            44,
            0,
            vec![
                TabBarElement::ScrollFrontPrefix,
                TabBarElement::ScrollFront(0),
                TabBarElement::ScrollFrontPostfix,
                TabBarElement::PrefixA,
                TabBarElement::TabA(0, "1: /foo/long_name_a".to_string()),
                TabBarElement::PostfixA,
                TabBarElement::DividerAI,
                TabBarElement::PrefixI,
                TabBarElement::TabI(1, "2: long_name_b".to_string()),
                TabBarElement::PostfixI,
                TabBarElement::ScrollBackPrefix,
                TabBarElement::ScrollBack(1),
                TabBarElement::ScrollBackPostfix,
            ],
        )
    }

    // When scroll tags are shown, the right scroll tag stays right-aligned.
    // The gap between the rightmost tab and the right scroll tab is filled with a padding section.
    //
    // In this test case, we use the same tab-situation as before, but the available
    // width is only one unit to short for all three tabs to be shown (54 units).
    // So, there will be a gap of 10 units to be filled with padding.
    // Like the other elements, also the padding has a prefix and postfix.
    // The middle (“fill”) character is repeated to fill the gap.
    #[test]
    fn three_tabs_with_overflow_on_the_right_w_long_padding() {
        _scenario_three_tabs(
            54,
            0,
            vec![
                TabBarElement::ScrollFrontPrefix,
                TabBarElement::ScrollFront(0),
                TabBarElement::ScrollFrontPostfix,
                TabBarElement::PrefixA,
                TabBarElement::TabA(0, "1: /foo/long_name_a".to_string()),
                TabBarElement::PostfixA,
                TabBarElement::DividerAI,
                TabBarElement::PrefixI,
                TabBarElement::TabI(1, "2: long_name_b".to_string()),
                TabBarElement::PostfixI,
                TabBarElement::PaddingPrefix,
                TabBarElement::PaddingFill(8),
                TabBarElement::PaddingPostfix,
                TabBarElement::ScrollBackPrefix,
                TabBarElement::ScrollBack(1),
                TabBarElement::ScrollBackPostfix,
            ],
        )
    }

    // The padding section has a prefix, a postfix, and a fill-sequence in the middle.
    // However, this is only possible when the padding has at least a width of 3 chars.
    // If the padding is only one char wide, only the prefix is shown.
    // If the padding is only two chars wide, only the prefix and postfix are shown.
    //
    // In the next two test cases, we use the same tab-situation as before, but the available
    // width has a value so that the padding is only one or two characters long.
    #[test]
    fn three_tabs_with_overflow_on_the_right_w_one_char_padding() {
        _scenario_three_tabs(
            45,
            0,
            vec![
                TabBarElement::ScrollFrontPrefix,
                TabBarElement::ScrollFront(0),
                TabBarElement::ScrollFrontPostfix,
                TabBarElement::PrefixA,
                TabBarElement::TabA(0, "1: /foo/long_name_a".to_string()),
                TabBarElement::PostfixA,
                TabBarElement::DividerAI,
                TabBarElement::PrefixI,
                TabBarElement::TabI(1, "2: long_name_b".to_string()),
                TabBarElement::PostfixI,
                TabBarElement::PaddingPrefix,
                TabBarElement::ScrollBackPrefix,
                TabBarElement::ScrollBack(1),
                TabBarElement::ScrollBackPostfix,
            ],
        )
    }

    #[test]
    fn three_tabs_with_overflow_on_the_right_w_two_char_padding() {
        _scenario_three_tabs(
            46,
            0,
            vec![
                TabBarElement::ScrollFrontPrefix,
                TabBarElement::ScrollFront(0),
                TabBarElement::ScrollFrontPostfix,
                TabBarElement::PrefixA,
                TabBarElement::TabA(0, "1: /foo/long_name_a".to_string()),
                TabBarElement::PostfixA,
                TabBarElement::DividerAI,
                TabBarElement::PrefixI,
                TabBarElement::TabI(1, "2: long_name_b".to_string()),
                TabBarElement::PostfixI,
                TabBarElement::PaddingPrefix,
                TabBarElement::PaddingPostfix,
                TabBarElement::ScrollBackPrefix,
                TabBarElement::ScrollBack(1),
                TabBarElement::ScrollBackPostfix,
            ],
        )
    }

    #[test]
    /// When scrolling is needed, tabs are added alternating to the left and to the right of the
    /// current tab. The algorithm starts on the left. So, if there are three tabs, and the middle
    /// one is the current one, and there is only enough space for two tabs, the first and the
    /// second (current) tab will be shown. The tab right of the current tab will be the one that
    /// is hidden.
    fn adding_tabs_to_front_has_precedence_over_adding_to_right() {
        _scenario_three_tabs(
            44,
            1,
            vec![
                TabBarElement::ScrollFrontPrefix,
                TabBarElement::ScrollFront(0),
                TabBarElement::ScrollFrontPostfix,
                TabBarElement::PrefixI,
                TabBarElement::TabI(0, "1: long_name_a".to_string()),
                TabBarElement::PostfixI,
                TabBarElement::DividerIA,
                TabBarElement::PrefixA,
                TabBarElement::TabA(1, "2: /foo/long_name_b".to_string()),
                TabBarElement::PostfixA,
                TabBarElement::ScrollBackPrefix,
                TabBarElement::ScrollBack(1),
                TabBarElement::ScrollBackPostfix,
            ],
        )
    }

    #[test]
    /// This tests uses the same three-tab screnario as the ones before, but uses the last tab as
    /// the current one. The available space is again just enough for two tabs.
    /// As a result, the first tab will now be hidden and the front scroll tag will indicate that
    /// hidden tab.
    fn overflowing_on_the_left_side() {
        _scenario_three_tabs(
            44,
            2,
            vec![
                TabBarElement::ScrollFrontPrefix,
                TabBarElement::ScrollFront(1),
                TabBarElement::ScrollFrontPostfix,
                TabBarElement::PrefixI,
                TabBarElement::TabI(1, "2: long_name_b".to_string()),
                TabBarElement::PostfixI,
                TabBarElement::DividerIA,
                TabBarElement::PrefixA,
                TabBarElement::TabA(2, "3: /foo/long_name_c".to_string()),
                TabBarElement::PostfixA,
                TabBarElement::ScrollBackPrefix,
                TabBarElement::ScrollBack(0),
                TabBarElement::ScrollBackPostfix,
            ],
        )
    }
}
