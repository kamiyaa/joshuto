# theme.toml

This file is used to theme Joshuto.

As of now, all theming items are "text style" items where one can define the
background color and the foreground color, and enable some font attributes.


## Style options

Each style allows the following fields:
```toml
# background color
bg = "light_blue"
# foreground color
fg = "blue"
bold = false
underline = false
invert = false
```
Each field is optional. If omitted, it defaults to the terminal default.


## Color

Joshuto supports 16 colors as well as hex colors via `rgb(r,g,b)`.
The 16 default colors are:
* `black`
* `red`
* `blue`
* `green`
* `yellow`
* `magenta`
* `cyan`
* `white`
* `gray`
* `dark_gray`
* `light_red`
* `light_green`
* `light_yellow`
* `light_blue`
* `light_magenta`
* `light_cyan`


## Configuration Items

You can find a complete list of the available configuration items with their default
values and short explanations as comment in 
[`theme.toml`](https://github.com/kamiyaa/joshuto/blob/main/config/theme.toml).

In general, `theme.toml` allows to specify the style of a few UI "widgets",
and the file entries in the file lists.

The file entries can be styled by their basic system file type and by their extension.
The extension-specific style overrides the basic style.

Special file entries (as of now, executable files and invalid symlinks) have
a specific style that overrides the former file-type-styles.

Last but not least, there are styles for _selected_ files which override all the former
styles.

## Theming the Tab-Bar
Theming of the tab-bar is described [here](tabbar/README.md).
