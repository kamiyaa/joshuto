# joshuto.toml

This file is for general configurations.

All options available and their default values:

```toml
# This is for configuring how many items to reach before 'scrolling' the view
scroll_offset = 6

# If joshuto does not know how to open the file, it can resort to opening it via xdg settings
xdg_open = false

# Fork xdg_open so you can continue using joshuto with application open
xdg_open_fork = false

# Use system trash can instead of permanently removing files
use_trash = true

# Watch for filesystem changes and update directory listings accordingly
watch_files = true

# The maximum file size to show a preview for
max_preview_size = 2097152 # 2MB

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

# Optional list of command aliases (empty by default)
[cmd_aliases]
# q = "quit"
# ...
```
