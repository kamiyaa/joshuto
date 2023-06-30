# mimetype.toml

This file tells joshuto what programs to use when opening files.

There are currently 2 ways to configure opening files:

- via extension (1st priority)
- via mimetype (2nd priority)
  - must have `file` command available
  - joshuto will use `file --mime-type -Lb` to determine the file's mimetype

## Class and inherit

To alleviate the lack of variables and programmability in TOML,
there is a section for users to specify "classes" called `[class]`.
Here, users can specify a list of commands to open a file and inherit these
for a specific file format.

## Silent and Fork

The `silent` option will redirect any stdout and stderr output to `/dev/null`.
This is ideal for launching GUI applications because many output debug messages to
the terminal, disrupting Joshuto's interface.

The `fork` option will launch the application in a new thread, allowing users to
continue using Joshuto.

As a rule of thumb:
- TUI applications should almost always run without `silent` or `fork`
- GUI applications should almost always run with the `silent` flag enabled
  - If the user wishes to continue using joshuto while the
  GUI application is running, enable the `fork` flag

## Example

```toml
[class]
image_default	= [
	{ command = "qimgv", fork = true, silent = true },
	{ command = "krita", fork = true, silent = true } ]

[extension]
# inherit from image_default class
png.inherit	= "image_default"

# inherit from image_default class
jpg.inherit	= "image_default"
# in addition, also add gimp for .jpg files only
jpg.app_list	= [
	{ command = "gimp", fork = true, silent = true } ]

mkv.app_list	= [
	{ command = "mpv", args = [ "--" ] , fork = true, silent = true },
	{ command = "mediainfo", confirm_exit = true },
	{ command = "mpv", args = [ "--mute", "on", "--" ], fork = true, silent = true } ]
rs.app_list	= [
	{ command = "micro" },
	{ command = "gedit", fork = true, silent = true },
	{ command = "bat", confirm_exit = true } ]

[mimetype]

# text/*
[mimetype.text]
inherit = "text_default"

# application/octet-stream
[mimetype.application.subtype.octet-stream]
inherit = "video_default"
```

each extension has the following fields:

- `inherit`: string indicating the class to inherit from, if any
- `app_list`: list of commands

each command has the following fields:

- `command`: the command to run
- `args`: (optional) list of arguments for the command
- `fork`: tells joshuto to run the program in foreground or background
  - foreground will pause joshuto
- `silent`: tells joshuto to discard all output of the program
  - useful when the program outputs debug messages into the terminal,
    potentially ruining joshuto's UI
- `confirm_exit`: requires the user's input before going back to joshuto
  - useful when you want to read the output of the command

## Explanation of the configuration above

For files with `.png` extension, joshuto opens them with `qimgv`.
Joshuto suppresses all terminal output from `qimgv` to prevent UI disturbance.
Joshuto forks `qimgv` so we can continue using joshuto will viewing the image.
Alternatively, we can open it with `krita` or `mediainfo`
via `:open_with 1` and `:open_with 2`.
With `mediainfo`, we want to see the output of the command before going back to joshuto,
so we have `confirm_exit = true`

For files with `.rs` extension, joshuto will open it with `micro`, a command line text editor.
In order for joshuto and `micro` to not conflict, joshuto will wait for `micro` to exit, before
redrawing. Joshuto will also not suppress `micro`'s output.
