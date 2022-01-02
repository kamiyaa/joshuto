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

**Please copy these configs and use it as a base, then modify them accordingly.**

Joshuto's behavior is:
- If there exists a config file, use that config. (No default or inherited values from a default config)
- If there is no config file, a default config will be used (found under `config/`)

This means Joshuto will have no themes or no mimetype entries if you have an empty `theme.toml` or `mimetype.toml` file


