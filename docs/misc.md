## Numbered commands

You can prefix some commands with a number. For this, just start entering
a number and then press any key which is mapped to a command. Currently
supported commands are:

- `cursor_move_up`: moves cursor up by number of lines you prefixed the
  command with
- `cursor_move_down`: moves cursor down by number of lines you prefixed the
  command with
- `g`: this is hard-coded, you cannot remap this command. Just enter a number
  and then press `g` to jump to n'th line

## Visual Mode

“Visual mode” – named after the selection concept in VIM – is basically a way to
select a range of files.
Joshuto allows to select a single file with the `select` command, usually mapped to `SPACE`.
The `select` command conveniently moves the cursor one file down automatically, but
selecting a wider range of files that way can still be tedious.
Here is where visual mode comes to help.

To select a range of files, move the cursor to one end of the range and enter visual mode
with the `toggle_visual` command, by default mapped to `V`.
Now move the cursor to the other end of the desired range.
Of course, any command to position the cursor can be used, like `G`, `gg`, `17j` or `5g`.

While being in visual mode, Joshuto’s footer shows `VIS` on red ground on the left side.

The visual mode selection will follow the cursor, and “visual-mode-selected” files will be
highlighted with the `[visual_mode_selection]` style, which is a light-red foreground and
a bold font by default.
The standard-selection, done with the `select` command, uses a different style (bold, light-yellow by default),
to be able to distinguish the selected and the visual-mode-selected files.

Any command which operates on a file selection, like `copy_files` or `bulk_rename`, will
consider both, files which are selected with `select` and those selected by visual mode.

There are two options to end visual mode. Either by calling `toggle_visual` again, which
will add the currently visual-mode-selected files to the set of standard-selected files.
So the “red” selected files turn into “yellow” selected files.
This way, visual mode can be helpful to build up a selection of multiple ranges

The other option is to issue the `escape` command, by default mapped to the `ESCAPE` key.
When using `escape`, the current visual-mode-selection will be withdrawn.

## Mouse Control

When built with the `mouse` feature, Joshuto supports some mouse control,
which behaves very similar to _ranger_.

⚠ Disclaimer: Mouse control does not work properly in `hsplit` mode.
(See [joshuto.toml docs](configuration/joshuto.toml.md#Different_view_layouts).)

If you click a file or directory with the _left mouse button_,
the cursor in the particular list will move to the clicked entry.
If the file or directory clicked is in the parent or children panel,
that directory level will be moved to the “current” panel in the middle.

If you click a directory in the _parent panel with the right mouse button_,
that directory will be opened, means, its content will appear in the middle panel.

If you click a file or directory in the _middle panel with the right mouse button_,
that file or directory will be opened, just like when pressing the “right arrow”.

The cursor in the parent and in the middle panel can be scrolled by using the _mouse wheel_.

Unlike ranger, it’s currently not possible to scroll the children-panel or a preview
with the mouse wheel.

Unlike ranger, Joshuto allows to set the cursor in the children-panel with a right click.

## Using joshuto with [qutebrowser](https://qutebrowser.org/)

In order to use joshuto as the file picker in qutebrowser, you need to first add a quit
keybind that outputs the selected files. You can do this in `keymap.toml` like so:


```toml
[default_view]

keymap = [
	{ keys = ["o"], command = "quit --output-selected-files" },
	# other keybinds...
]
```

Next, you need to edit qutebrowser's `config.py` to open joshuto in your terminal of choice,
then output the resulting directory, file, or files to a file named `{}`. Here's an example
with [kitty](https://sw.kovidgoyal.net/kitty/index.html):

```python
# Use joshuto as the file selector
c.fileselect.handler = 'external'
c.fileselect.folder.command = ['kitty', '-e', 'joshuto', '--output-file', '{}']
c.fileselect.multiple_files.command = ['kitty', '-e', 'joshuto', '--output-file', '{}']
c.fileselect.single_file.command = ['kitty', '-e', 'joshuto', '--output-file', '{}']
```

Now when the file picker opens in qutebrowser, you should see joshuto in your terminal emulator
of choice. Use the visual selector (default `V`) if you need to select multiple files and press
the key you defined earlier to send the file or files you highlighted back to qutebrowser.
