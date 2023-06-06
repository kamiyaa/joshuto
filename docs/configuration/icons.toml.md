# icons.toml

This file is used to configure file/directory icons.

> **NOTE**: To display icons correctly you need [Nerd Fonts](https://www.nerdfonts.com/).

The are four sections in this file:

- `defaults` - Used for fallback icons either file or directory.
- `directory_exact` - Used to match the exact directory name.
    e.g. `node_modules` will match exactly `node_modules` directory only.
- `file_exact` - Used to match exact file names.
- `ext` - Used to match file names by their extension.

Each section accepts key/value pairs, key as target and value as the icon.

## `defaults` example
```toml
[defaults]
file = ""
directory = ""
```

## `file_exact` example
```toml
".gitignore" = ""
LICENSE = ""
Makefile = ""
Dockerfile = ""
# ...
```

## `ext` example
```toml
js = ""
docx = ""
rs = ""
# ...
```
