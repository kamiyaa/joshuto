# keymap.toml

This file is for mapping keyboard keys to commands.

```toml
# keymapping for default view
[default_view]
keymap = [
    { keys = [ "T" ], command = "new_tab" },
    # ...
]

# keymapping for task view
[task_view]
keymap = [
    # ...
]

# keymapping for help view
[help_view]
keymap = [
    # ...
]
```

For more examples, take a look at [config/keymap.toml](https://github.com/kamiyaa/joshuto/blob/main/config/keymap.toml)

# Keys available:

To combine keys with Ctrl and Alt, simply have `ctrl+key`/`alt+key`
where `key` is a valid key.

In addition to the standard alphabet, Joshuto currently also support
the following keys.

```sh
backspace
backtab     # this is shift+tab
arrow_left
arrow_right
arrow_up
arrow_down
home
end
page_up
page_down
delete
insert
escape
f1
f2
f3
f4
f5
f6
f7
f8
f9
f10
f11
f12
```

# Commands available:

- [General](#general)
- [Navigation](#navigation)
- [Tabs](#tabs)
- [Search](#search)
- [Integration](#integration)

## General

### `quit`: quit joshuto

- **will not quit** if there are pending IO work (paste jobs)
- `quit`
  - exit code 0
- `quit --force`: does **_NOT_** wait for pending IO work
  - exit code 100
- `quit --output-current-directory`: if `--output-file` argument is set, output the current directory to it
  - exit code 101
- `quit --output-selected-files`: if `--output-file` argument is set, output the selected files to it
  - exit code 102

The following is a bash snippet on how to integrate with `quit`

```bash
function joshuto() {
	ID="$$"
	mkdir -p /tmp/$USER
	OUTPUT_FILE="/tmp/$USER/joshuto-cwd-$ID"
	env joshuto --output-file "$OUTPUT_FILE" $@
	exit_code=$?

	case "$exit_code" in
		# regular exit
		0)
			;;
		# output contains current directory
		101)
			JOSHUTO_CWD=$(cat "$OUTPUT_FILE")
			cd "$JOSHUTO_CWD"
			;;
		# output selected files
		102)
			;;
		*)
			echo "Exit code: $exit_code"
			;;
	esac
}
```

### `:`: opens the command prompt

- this does not execute the command, but merely sets the text to it
- Example: `:cd /` will open up the command prompt with `cd /` already written

### `shell`: runs a shell command

- `%s` is substituted by a list of all selected files or by the file under the cursor, if none is selected
- When running the external program, the directory shown in Joshuto is set as “working directory”,
  the file names substituted for `%s` are given without path.
- Example: `:shell touch file.txt` will create a file called `file.txt`
- Example for `keymap.toml`: To open all selected files with `nvim`, one can add a keybinding like this:
  ```toml
  keymap = [ //..
     { keys = [ "e", "v" ], command = "shell nvim %s" }
  ]
  ```

### `spawn`: runs a shell command in the background

- Supports `%s`, just like the `shell` command.
- Example for `keymap.toml`: To open all selected files or directories with `sxiv`,
  one can add a keybinding like this:
  ```toml
  keymap = [ //..
     { keys = [ "i" ], command = "spawn sxiv -t %s" }
  ]
  ```

### `sort`: change the sort method

- `sort lexical`: sort lexically (`10.txt` comes before `2.txt`)
- `sort natural`: sort naturally (`2.txt` comes before `10.txt`)
- `sort mtime`: sort via last modified time
- `sort reverse`: reverse the sorting

### `linemode`: change the line-mode (textual representation of files and directories in the “current view”)

- `linemode size`: show the entry’s size (bytes for files, number of entries for directories) (default) <sup>✻</sup>
- `linemode mtime`: show the entry’s modified time (aka. “_mtime_”) <sup>✻</sup>
- `linemode sizemtime`: show the entry’s size and modified time <sup>✻</sup>

<sup>✻: file- or directory-name is shown on left, the respective meta-data is shown on the right, preceded by a symlink indicator</sup>

### `show_workers`: show the pending IO operations and the current progress

- press `escape` to exit view

### `toggle_hidden`: toggle hidden files

### `line_nums`: switch displaying of entry numbers

- `line_nums 0` or `line_nums none`: disable displaying
- `line_nums 1` or `line_nums absolute`: enable absolute numbers for each entry
- `line_nums 2` or `line_nums relative`: enable numbers relative to selected entry

### `flat`: flattens the directory view up to the specified depth.

- `flat 3`: flatten directory up to 3 directories deep.
  depth of 0 corresponds to the current directory.
  its direct descendents have depth 1, and their descendents have depth 2, and so on.

## Navigation

### `cursor_move_up`: moves the cursor up by x amount

- `cursor_move_up`: moves the cursor up by 1
- `cursor_move_up x`: moves the cursor up by `x` where `x` is a non-negative number

### `cursor_move_down`: moves the cursor down by x amount

- `cursor_move_down`: moves the cursor down by 1
- `cursor_move_down x`: moves the cursor down by `x` where `x` is a non-negative number

### `cursor_move_home`: moves cursor to beginning of directory list

### `cursor_move_end`: moves cursor to end of directory list

### `cursor_move_page_up`: moves the cursor up by `x`

- where `x` is the number of items that can be seen on the screen

### `cursor_move_page_down`: moves the cursor down by `x`

- where `x` is the number of items that can be seen on the screen

### `parent_cursor_move_up`: same as `cursor_move_up` but for parent directory

### `parent_cursor_move_down`: same as `cursor_move_down` but for parent directory

### `preview_cursor_move_up`: moves the preview up

### `preview_cursor_move_down`: moves the preview down

### `cd`: change directory

- `cd ..`: go to parent directory
- `cd ~`: go to home directory
- `cd -`: go to previous directory in history (If it exists)

### `open`: open file or directory

- if joshuto does not know how to open the file format (via extension currently),
  it will prompt `:open_with ` to open with a specific command
- if `xdg_open` is `true` in [joshuto.toml](https://github.com/kamiyaa/joshuto),
  joshuto will try to open it via xdg settings

### `numbered_command`: opens a new mode where user can input numbers and jump to the specified location via hard-coded keybindings

- `numbered_command 3`: initial input is 3

## Tabs

### `new_tab [--current][--cursor][dir]`: opens a new tab

- `new_tab`, without any argument, opens a new tab with the default directory.

(Note: the default directory for new tabs is specified in `joshuto.toml` in the `tab` section.)

- `new_tab some-dir` opens new tab with directory `some-dir`
- `new_tab --current` opens new tab with the same directory as the current tab
- `new_tab --cursor` opens new tab with the directory which is currently marked by the cursor

### `close_tab`: close current tab

### `tab_switch`: switch to next/previous tab by `x`

- where `x` is an integer
- `tab_switch 1`: go to next tab
- `tab_switch -1`: go to previous tab

### `tab_switch_index`: switch to a given tab index

- `tab_switch_index 3`: go to third tab if it exists,
  create one if it does not exist and there is already 3 - 1 = 2 tabs open

## File Operations

### `reload_dirlist`: reloads the current directory listing

### `mkdir`: create a new directory (usually used as `:mkdir `)

### `cut_files`: store selected files (or current file if none were selected) to be moved later

### `copy_files`: store selected files (or current file if none were selected) to be copied later

### `symlink_files`: store selected files (or current file if none were selected) to be symlinked later

- `--relative=true`: relative symlink paths
- `--relative=false`: absolute symlink paths

### `paste_files`: move/copy files stored from a previous `cut_files` or `copy_files` command

### `delete_files`: delete selected files (or current file if none were selected).

- `--foreground=true`: will delete files in the foreground
- `--permanently`: force permanent deletion regardless of `use_trash` value.
- `--noconfirm`: files will be deleted without asking for confirmation
  (can be dangerous when `use_trash` is `false`)
- will **_permanently_** delete files if `use_trash` is `false` in
  [joshuto.toml](https://github.com/kamiyaa/joshuto)/wiki/Configuration#joshutotoml)
- if `use_trash` is `true`, `joshuto` will try to use
  the following command-line tools to try to put the files in the trash can instead
  of permanently deleting them
  - `gio trash`
  - `trash-put`: https://github.com/andreafrancia/trash-cli
  - `trash`

### `rename`: rename the current file the cursor is on

- `:rename new_name`

### `rename_append`: opens the command prompt with the rename command and the current file name filled in.

- cursor will be set right before the extension of the file
  (end of file name if no extension)

### `rename_prepend`: opens the command prompt with the rename command and the current file name filled in.

- cursor will be set to the beginning of the file name

### `rename_keep_ext`: opens the command prompt with the rename command and the extension of the current file filled in.

- cursor will be set right before the extension of the file

### `copy_filename`: copy the file name to clipboard

- clipboard support requires xsel, xclip, or wl-copy

### `copy_filename_without_extension`: copy the file name without the extension to clipboard

### `copy_filepath`: copy the current file path to clipboard

### `copy_dirpath`: copy the current directory path to clipboard

### `set_mode`: Set read, write, execute permissions of current file

### `touch`: create a new file or update the modified date of an existing file

## Search and Selection

### `search`: search the current directory via a string

### `search_glob`: search the current directory via shell globbing (exact match)

- `:search_glob *.png`

### `search_next`: go to next search result in the current directory

### `search_prev`: go to previous search result in the current directory

### `select`: select current file

- `--toggle=true`: toggle the selected state rather than selecting the entry (default)
- `--toggle=false`: select the current file (doesn't change anything if the current file is already selected)
- `--deselect=true`: deselect rather than select the entry
- `--all=true`: select/deselect/toggle all _visible_ files in the current view.
  (Files not visible due to a set filter are not affected.)
- `glob`: select files based on glob (just like `search_glob`)
  - `select *.png`

This example keybinding can be used for *de*selecting all files:

```toml
keymap = [ //..
   {keys = [ "x" ], command = "select --all=true --deselect=true"}
]
```

### `filter`: filter the current directory list.

- `:filter ca`: filter the current directory and show only items with `ca` in the name

### `toggle_visual`: enables or disables “visual mode”.

When disabling, the current “visual mode selection” is turned into normal selection.
(See also [Visual Mode](../misc.md#visual-mode).)

### `escape`: leave visual mode and withdraw the visual-mode-selection.

(See also [Visual Mode](../misc.md#visual-mode).)

### `set_case_sensitivity`: set case sensitivity for different search operations.

- Options
  - `--type=string`: change configurations of operations using substring matching
  - `--type=glob`: change configurations of operations using glob matching
  - `--type=fzf`: change configurations of operations using fzf 
  - when no option is added, type is set to `string` by default
- Value
  - `insensitive`
  - `sensitive`
  - `smart`

An example:

```
:set_case_sensitivity --type=fzf sensitive
```

## Bookmarks

### `add_bookmark`: adds a bookmark to the `bookmarks.toml` file

### `cd_bookmark`: prompts a menu of bookmarks and navigates to selected bookmark

## Integration

### `bulk_rename`: rename all selected files

- this will create a file inside `$TMP_DIR` (or `/tmp` if `$TMP_DIR` is not set) and
  open up your text editor of choice via `$EDITOR` environment variable
- once you've made your changes to the file, saved and quit, it will use the `mv` command to rename everything

### `search_fzf`: search the current directory via `fzf`

### `subdir_fzf`: go to a subdirectory via `fzf`

### `z`: cd via `zoxide`

### `zi`: cd via interactive `zoxide`
