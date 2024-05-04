# joshuto.toml

This file is for general configurations.

All options available and their default values:

```toml
# Enables mouse support (true by default)
mouse_support = true
# This is for configuring how many items to reach before 'scrolling' the view
scroll_offset = 6

# If joshuto does not know how to open the file, it can resort to opening it via xdg settings
xdg_open = false

# Fork xdg_open so you can continue using joshuto with application open
xdg_open_fork = false

# If true, all file extensions checks will be case sensitive.
# Applies to `[extension]` in `mimetype.toml` and `[ext]` in `theme.toml` and `icons.toml`
case_insensitive_ext = false

# Use system trash can instead of permanently removing files
use_trash = true

# Watch for filesystem changes and update directory listings accordingly
watch_files = true

# If true the cursor will focus newly created files or directories with `:touch` or `:mkdir`
# Even if true, the behavior can be avoided prefixing the new file/dir with "./"
# E.g.:
# - `:mkdir a` moves the cursor to the new directory `a`
# - `:mkdir ./b` keeps the cursor where it was
focus_on_create = true

# The maximum file size to show a preview for
max_preview_size = 2097152 # 2MB

# Update the zoxide database with every navigation type instead of only with the z command
zoxide_update = false

# Define custom commands (using shell) with parameters like %text, %s etc.
custom_commands = [
   { name = "rgfzf", command = "/home/<USER>/.config/joshuto/rgfzf '%text' %s" },
   { name = "rg", command = "/home/<USER>/.config/joshuto/rg '%text' %s" }
]

# Configurations related to the display
[display]
# Different view layouts
# Options include
# - default
# - hsplit
mode = "default"

# Collapse the preview window when there is no preview available
collapse_preview = true

# Ratios for parent view, current view and preview. You can specify 0 for
# parent view or omit it (So there are only 2 nums) and it won't be displayed
column_ratio = [1, 3, 4]

# Show borders around different views
show_borders = true

# Show hidden files
show_hidden = false

# Show file icons (requires a supporting font)
show_icons = true

# Shorten /home/$USER to ~
tilde_in_titlebar = true

# Options include
# - none
# - absolute
# - relative
line_number_style = "none"

# Options include
# - size
# - mtime
# - user
# - group
# - perm
# - none (can't be combined with other options)
# - all (can't be combined with other options)
linemode = "size"

# Configurations related to file sorting
[display.sort]
# Options include
# - lexical  (10.txt comes before 2.txt)
# - natural  (2.txt comes before 10.txt)
# - mtime
# - size
# - ext
sort_method = "natural"

# case sensitive sorting
case_sensitive = false

# show directories first
directories_first = true

# sort in reverse
reverse = false

# Configurations related to preview
[preview]

# Maximum preview file size in bytes
max_preview_size = 2097152

# Executable script for previews
preview_script = "~/.config/joshuto/preview_file.sh"

# Use thumbnail images according to the freedesktop.org (XDG) standard.
# (https://specifications.freedesktop.org/thumbnail-spec/thumbnail-spec-latest.html)
# This only affects Joshuto's internal image-thumbnail feature.
# It does not affect the hook-script based previews.
use_xdg_thumbs = true

# The XDG thumb size used for the preview.
# Allowed values are 'normal', 'large', 'xlarge', and 'xxlarge' with maximum edge lengths
# of 128 px, 256 px, 512 px, and 1024 px respectively.
xdg_thumb_size = "xlarge"

# Configurations related to searching and selecting files
[search]
# Different case sensitivities for operations using substring matching
# - insensitive
# - sensitive
# - smart: when the pattern contains at least one uppercase letter, joshuto can search
#   files case-sensitively, otherwise it will ignore the difference between lowercase
#   and uppercase
# Note that to apply changes after changing configurations at runtime, re-executing
# the search command is required.
# For substring matching
string_case_sensitivity = "insensitive"
# For glob matching
glob_case_sensitivity = "sensitive"
# For regex matching
regex_case_sensitivity = "sensitive"
# For matching with fzf
fzf_case_sensitivity = "insensitive"

# Optional list of command aliases (empty by default)
[cmd_aliases]
# q = "quit"
# ...

[tab]
# inherit, home, root
home_page = "home"

```
