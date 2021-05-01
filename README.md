![Rust](https://github.com/kamiyaa/joshuto/workflows/Rust/badge.svg)

# joshuto

[ranger](https://github.com/ranger/ranger)-like terminal file manager written in Rust.

![Alt text](screenshot.png?raw=true "joshuto")

## Dependencies

- [cargo](https://github.com/rust-lang/cargo/)
- [rustc](https://www.rust-lang.org/)
- xsel/xclip/wl-clipboard (optional, for clipboard support)

Also see [Cargo.toml](Cargo.toml)

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

#### Packaging status

##### Fedora ([COPR](https://copr.fedorainfracloud.org/coprs/atim/joshuto/))

```
sudo dnf copr enable atim/joshuto -y
sudo dnf install joshuto
```

## Usage

```
~ $ joshuto
```

## Configuration

Place config files inside `$XDG_CONFIG_HOME/joshuto` (usually `$HOME/.config/joshuto/` for GNU/Linux).

Joshuto can currently be configured using the following files:

#### [joshuto.toml](config/joshuto.toml)

- general configurations

#### [keymap.toml](/config/keymap.toml)

- for keybindings, please take a look at [src/util/key_mapping.rs](/src/util/key_mapping.rs#L18) for non-printable keys
- for commands, please take a look at [src/commands/commands.rs](/src/commands/commands.rs#L139)

#### [mimetype.toml](/config/mimetype.toml)

- for opening files with applications

#### [theme.toml](/config/theme.toml)

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
