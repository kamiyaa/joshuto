# Command Line Arguments

```
usage: joshuto [options] [path]
```
 - `[path]`: tells joshuto to start in a specific directory

Joshuto supports the following options from the command line:

 - `-v` `--version`: outputs version of joshuto

 - `-h` `--help`: shows help menu

 - `--change-directory`: sets the quit behavior to change directory instead of
   noop when closing the last tab

 - `--output-file <output-file>`: tells joshuto to output data to `<output-file>`.
   - This is usually used so programs can know how to behave after joshuto exits.
   - For example, cd into joshuto's current directory on quit

