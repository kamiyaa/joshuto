# WORK IN PROGRESS

# joshuto

ranger-like terminal file manager written in Rust

## Dependencies
 - ncurses

Also see [Cargo.toml](https://github.com/kamiyaa/joshuto/blob/master/Cargo.toml)

## Installation
```
$ cargo build
# cargo install --root=/usr/local --force
```

## Usage
```
~ $ joshuto
```

## Configuration
Joshuto can currently be configured using the following files:

[joshuto.toml](https://github.com/kamiyaa/joshuto/blob/master/config/joshuto.toml)
 - general configurations

[keymap.toml](https://github.com/kamiyaa/joshuto/blob/master/config/keymap.toml)
 - for keybindings
   - **currently causes program to have no bindings when this config is missing**
   - please take a look at [keymapll.rs](https://github.com/kamiyaa/joshuto/blob/master/src/joshuto/keymapll.rs) for keycodes

[mimetype.toml](https://github.com/kamiyaa/joshuto/blob/master/config/mimetype.toml)
 - for opening files with applications
   - **currently causes program to be unable to open any files when this config is missing**

Place these config files in your `XDG_CONFIG_DIR/joshuto` (usually `$HOME/.config/joshuto/`)

## Features/Bugs
Please create an issue :)
