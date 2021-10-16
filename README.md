[![Linux build](https://github.com/kamiyaa/joshuto/actions/workflows/rust-linux-main.yml/badge.svg)](https://github.com/kamiyaa/joshuto/actions/workflows/rust-linux-main.yml)

[![MacOS build](https://github.com/kamiyaa/joshuto/actions/workflows/rust-macos-main.yml/badge.svg)](https://github.com/kamiyaa/joshuto/actions/workflows/rust-macos-main.yml)

# joshuto

[ranger](https://github.com/ranger/ranger)-like terminal file manager written in Rust.

![Alt text](screenshot.png?raw=true "joshuto")

## Dependencies

- [cargo](https://github.com/rust-lang/cargo/)
- [rustc](https://www.rust-lang.org/)
- xsel/xclip/wl-clipboard (optional, for clipboard support)
- fzf (optional)

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

##### Arch ([AUR](https://aur.archlinux.org))

* [release](https://aur.archlinux.org/packages/joshuto)
```
[yay/paru] -S joshuto
```

* [build from source](https://aur.archlinux.org/packages/joshuto-git)
```
[yay/paru] -S joshuto-git
```

##### MacOS ([MacPorts](https://ports.macports.org/port/joshuto/details/))

```
sudo port install joshuto
```

## Usage
```
~ $ joshuto
```

## Configuration

Check out [wiki/Configuration](https://github.com/kamiyaa/joshuto/wiki/Configuration) for details
and [config/](config/) for examples

#### [joshuto.toml](config/joshuto.toml)
- general configurations

#### [keymap.toml](/config/keymap.toml)
- for keybindings

#### [mimetype.toml](/config/mimetype.toml)
- for opening files with applications

#### [theme.toml](/config/theme.toml)
- color customizations

## Contributing
Feel free to contribute code (see [wiki/Contributing](https://github.com/kamiyaa/joshuto/wiki/Contributing))
or buy me a cup of tea by donating :)

## Features/Bugs

Please create an issue :)

## TODOs

- [x] Migrate to [tui-rs](https://github.com/fdehau/tui-rs)
- [x] Tab support
- [x] Ctrl/Shift/Alt support
- [x] Asynch File IO (cut/copy/paste)
- [ ] Built-in command line (in progress)
  - Currently implementation is kind of janky
- [ ] File previews (in progress)
  - Waiting for tui-rs to support parsing ANSI color codes
- [ ] Tab autocomplete (in progress)
- [x] Bulk rename
