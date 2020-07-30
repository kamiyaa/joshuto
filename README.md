![Rust](https://github.com/kamiyaa/joshuto/workflows/Rust/badge.svg)

# joshuto

[ranger](https://github.com/ranger/ranger)-like terminal file manager written in Rust.

![Alt text](screenshot.png?raw=true "joshuto")

## Dependencies

- [cargo](https://github.com/rust-lang/cargo/) >= 0.41.0
- [rustc](https://www.rust-lang.org/) >= 1.41.0

Also see [Cargo.toml](https://github.com/kamiyaa/joshuto/blob/master/Cargo.toml)

## Building

```
~$ cargo build
```

## Installation

#### For single user

```
~$ cargo install --path=. --force
```

#### System wide

```
~# cargo install --path=. --force --root=/usr/local     # /usr also works
```

## Usage

```
~ $ joshuto
```

## Configuration

Place config files inside `$XDG_CONFIG_HOME/joshuto` (usually `$HOME/.config/joshuto/` for GNU/Linux).

Joshuto can currently be configured using the following files:

#### [joshuto.toml](https://github.com/kamiyaa/joshuto/blob/master/config/joshuto.toml)

- general configurations

#### [keymap.toml](https://github.com/kamiyaa/joshuto/blob/master/config/keymap.toml)

- for keybindings, please take a look at [keymap.rs](https://github.com/kamiyaa/joshuto/blob/master/src/config/keymap.rs#L102) for non-printable keys
- for commands, please take a look at commands/[mod.rs](https://github.com/kamiyaa/joshuto/blob/master/src/commands/mod.rs#L73) for available commands

#### [mimetype.toml](https://github.com/kamiyaa/joshuto/blob/master/config/mimetype.toml)

- for opening files with applications

#### [theme.toml](https://github.com/kamiyaa/joshuto/blob/master/config/theme.toml)

- color customizations

## Contributing

Please create a pull request :)

## Features/Bugs

Please create an issue :)

## TODOs

- [x] Migrate to [tui-rs](https://github.com/fdehau/tui-rs)
- [x] Tab support
- [x] Ctrl/Shift/Alt support
- [x] Asynch File IO (cut/copy/paste/delete/rename) (in progress)
- [ ] Built-in command line (in progress)
- [ ] File previews (in progress)
- [ ] Tab autocomplete (in progress)
- [x] Bulk rename
