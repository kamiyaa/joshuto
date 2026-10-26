use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

/// Git status of a single entry, expressed in `git status --short` `XY` form.
///
/// `index` is the staged (index) column and `worktree` is the unstaged
/// (working tree) column. A space means "no change" in that column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GitFileStatus {
    pub index: char,
    pub worktree: char,
}

impl GitFileStatus {
    pub fn new(index: char, worktree: char) -> Self {
        Self { index, worktree }
    }

    pub fn is_untracked(&self) -> bool {
        self.index == '?' && self.worktree == '?'
    }

    pub fn is_deleted(&self) -> bool {
        self.index == 'D' || self.worktree == 'D'
    }

    /// Only the index column carries a change (fully staged).
    pub fn is_staged_only(&self) -> bool {
        self.index != ' ' && !self.is_untracked() && self.worktree == ' '
    }
}

impl std::fmt::Display for GitFileStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.index, self.worktree)
    }
}

/// Compute the `git status --short` indicators for the direct children of
/// `dir`, keyed by entry file name (as it appears in the directory listing).
///
/// Returns `None` when `dir` is not inside a git work tree (or `git` is
/// unavailable); individual changes that don't map to an existing child entry
/// (e.g. deleted files) are simply omitted. Changes nested inside a child
/// directory are aggregated onto that directory entry.
pub fn dir_git_statuses(dir: &Path) -> Option<HashMap<String, GitFileStatus>> {
    let prefix = git_show_prefix(dir)?;

    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["status", "--porcelain=v1", "-z", "--no-renames", "--", "."])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Some(parse_git_status(&stdout, &prefix))
}

/// `git rev-parse --show-prefix` -> path of `dir` relative to the repo root
/// (with a trailing `/`, or empty at the root). `None` if `dir` is not in a repo.
fn git_show_prefix(dir: &Path) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["rev-parse", "--show-prefix"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(
        String::from_utf8_lossy(&output.stdout)
            .trim_end_matches(['\n', '\r'])
            .to_string(),
    )
}

/// Parse NUL-separated `git status --porcelain=v1 -z` records (paths are
/// relative to the repo root) into per-entry statuses relative to the directory
/// described by `prefix`.
fn parse_git_status(raw: &str, prefix: &str) -> HashMap<String, GitFileStatus> {
    let mut direct: HashMap<String, GitFileStatus> = HashMap::new();
    // For nested changes: (has_tracked_change, has_untracked) per child directory.
    let mut dir_agg: HashMap<String, (bool, bool)> = HashMap::new();

    for record in raw.split('\0') {
        // Each record is `XY<space>PATH`; the shortest meaningful one is 4 bytes.
        if record.len() < 4 {
            continue;
        }
        let bytes = record.as_bytes();
        let index = bytes[0] as char;
        let worktree = bytes[1] as char;
        // bytes[2] is the separating space; the path is the remainder.
        let path = &record[3..];

        let rel = match path.strip_prefix(prefix) {
            Some(rel) => rel.trim_end_matches('/'),
            None => continue,
        };
        if rel.is_empty() {
            continue;
        }

        match rel.find('/') {
            // Direct child (a file, or an untracked directory collapsed to `dir/`).
            None => {
                direct.insert(rel.to_string(), GitFileStatus::new(index, worktree));
            }
            // A change nested inside a child directory: aggregate onto it.
            Some(i) => {
                let name = rel[..i].to_string();
                let untracked = index == '?' && worktree == '?';
                let agg = dir_agg.entry(name).or_insert((false, false));
                if untracked {
                    agg.1 = true;
                } else {
                    agg.0 = true;
                }
            }
        }
    }

    for (name, (tracked, _untracked)) in dir_agg {
        // An explicit status for the directory itself takes precedence.
        direct.entry(name).or_insert_with(|| {
            if tracked {
                GitFileStatus::new(' ', 'M')
            } else {
                GitFileStatus::new('?', '?')
            }
        });
    }

    direct
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(parts: &[&str]) -> String {
        parts.join("\0")
    }

    #[test]
    fn direct_children_at_repo_root() {
        let raw = rec(&[" M tracked.txt", "?? untracked.txt", "A  staged.txt"]);
        let map = parse_git_status(&raw, "");
        assert_eq!(map["tracked.txt"], GitFileStatus::new(' ', 'M'));
        assert_eq!(map["untracked.txt"], GitFileStatus::new('?', '?'));
        assert_eq!(map["staged.txt"], GitFileStatus::new('A', ' '));
    }

    #[test]
    fn nested_tracked_change_aggregates_to_subdir() {
        let raw = rec(&[" M sub/deep/inner.txt"]);
        let map = parse_git_status(&raw, "");
        assert_eq!(map["sub"], GitFileStatus::new(' ', 'M'));
        assert!(!map.contains_key("inner.txt"));
    }

    #[test]
    fn collapsed_untracked_directory_is_a_direct_child() {
        let raw = rec(&["?? deep/"]);
        let map = parse_git_status(&raw, "");
        assert_eq!(map["deep"], GitFileStatus::new('?', '?'));
    }

    #[test]
    fn nested_untracked_only_marks_subdir_untracked() {
        let raw = rec(&["?? sub/a.txt", "?? sub/b.txt"]);
        let map = parse_git_status(&raw, "");
        assert_eq!(map["sub"], GitFileStatus::new('?', '?'));
    }

    #[test]
    fn nested_tracked_takes_precedence_over_untracked() {
        let raw = rec(&["?? sub/a.txt", " M sub/b.txt"]);
        let map = parse_git_status(&raw, "");
        assert_eq!(map["sub"], GitFileStatus::new(' ', 'M'));
    }

    #[test]
    fn prefix_is_stripped_when_listing_a_subdirectory() {
        // Paths from porcelain are root-relative; we list `sub/`.
        let raw = rec(&[" M sub/inner.txt", "?? sub/deep/"]);
        let map = parse_git_status(&raw, "sub/");
        assert_eq!(map["inner.txt"], GitFileStatus::new(' ', 'M'));
        assert_eq!(map["deep"], GitFileStatus::new('?', '?'));
    }

    // --- end-to-end against a real `git` invocation ---

    fn git(dir: &Path, args: &[&str]) -> bool {
        Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn git_available() -> bool {
        Command::new("git")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    #[test]
    fn end_to_end_against_real_repo() {
        if !git_available() {
            eprintln!("skipping: git not available");
            return;
        }

        let root = std::env::temp_dir().join(format!("joshuto-git-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("sub/deep")).unwrap();

        std::fs::write(root.join("tracked.txt"), "a\n").unwrap();
        std::fs::write(root.join("sub/inner.txt"), "b\n").unwrap();

        assert!(git(&root, &["init", "-q"]));
        // ensure commits work in throwaway environments without global config
        assert!(git(&root, &["config", "user.email", "t@t"]));
        assert!(git(&root, &["config", "user.name", "t"]));
        assert!(git(&root, &["add", "."]));
        assert!(git(&root, &["commit", "-qm", "init"]));

        // a worktree modification, a new untracked file, and a nested change
        std::fs::write(root.join("tracked.txt"), "a\nchange\n").unwrap();
        std::fs::write(root.join("untracked.txt"), "c\n").unwrap();
        std::fs::write(root.join("sub/deep/new.txt"), "d\n").unwrap();

        // from the repo root
        let map = dir_git_statuses(&root).expect("dir is in a repo");
        assert_eq!(map["tracked.txt"], GitFileStatus::new(' ', 'M'));
        assert_eq!(map["untracked.txt"], GitFileStatus::new('?', '?'));
        // nested untracked change aggregates onto the `sub` directory entry
        assert_eq!(map["sub"], GitFileStatus::new('?', '?'));

        // from the subdirectory: prefix stripping must yield child-relative names
        let sub_map = dir_git_statuses(&root.join("sub")).expect("dir is in a repo");
        assert_eq!(sub_map["deep"], GitFileStatus::new('?', '?'));
        assert!(!sub_map.contains_key("sub"));

        // outside any repo -> None
        let outside = std::env::temp_dir();
        let _ = dir_git_statuses(&outside); // may or may not be in a repo; just must not panic

        let _ = std::fs::remove_dir_all(&root);
    }
}
