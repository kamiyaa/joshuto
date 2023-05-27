# theme.toml

This file is used to theme joshuto

## Style

Each style has the following fields:

```toml
# background color
bg = "light_blue"
# foreground color
fg = "blue"
bold = false
underline = false
invert = false
```

## Color

Joshuto supports 16 colors as well as hex colors via `rgb(r,g,b)`

```
black
red
blue
green
yellow
magenta
cyan
white
gray
dark_gray
light_red
light_green
light_yellow
light_blue
light_magenta
light_cyan
_
```

## Theme

joshuto supports theming via system file types as well as extensions

System file types include:

```toml
# for selected files
[selection]
fg = "light_yellow"
bold = true

# for executable files
[executable]
fg = "light_green"
bold = true

# default theme
[regular]
fg = "white"

# for directories
[directory]
fg = "light_blue"
bold = true

# for symlinks
[link]
fg = "cyan"
bold = true

# for invalid symlinks
[link_invalid]
fg = "red"
bold = true

# for sockets
[socket]
fg = "light_magenta"
bold = true
```

Via extensions

```toml
[ext]
[ext.jpg]
fg = "yellow"
[ext.jpeg]
fg = "yellow"
[ext.png]
fg = "yellow"
```
