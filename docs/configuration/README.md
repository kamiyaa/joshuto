# Configuration
Joshuto reads configurations from the following directories using environment variables (in order of precedence):
 - `$JOSHUTO_CONFIG_HOME`
 - `$XDG_CONFIG_HOME/joshuto`
 - `$HOME/.config/joshuto`

Joshuto can currently be configured using the following files:

- [joshuto.toml](/docs/configuration/joshuto.toml.md): basic/general configurations
- [keymap.toml](/docs/configuration/keymap.toml.md): keymapping configurations
- [mimetype.toml](/docs/configuration/mimetype.toml.md): mimetype configurations
- [theme.toml](/docs/configuration/theme.toml.md): theming configurations
