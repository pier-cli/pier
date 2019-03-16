# **pier** ~ A simple Docker script management CLI

If you've spent any amount of time in the terminal you no doubt have built up a lovely collection of one-liners, scripts, useful tools, and CLIs. Whenever you want to use them you dig through your `bin` folder trying to remember what you called the script... Linux users love hard-to-remember naming conventions.

Scripts should be first-class citizens. In a GUI world we can find our programs using a menu of sorts. In the terminal scripts get lost.

The idea behind `pier` is to create a central repository for all your scripts, and provide a way to attach metadata about these scripts. Using `pier` you can add, remove, list, and run scripts. These can either be managed by `pier` in a TOML config, or you can use it to catalog existing scripts that you may have lying around - you'd then simply add the metadata for the specific script, and attach it to the name in the `PATH`.

See `src/cli.yml` for the spec.

```
pier 0.1.0
Benjamin Scholtz <bscholtz.bds@gmail.com>
A simple Docker script management CLI

USAGE:
    pier [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -y, --accept     answer yes to all questions
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    sets a custom config file (default "$HOME/.pier")

SUBCOMMANDS:
    add       Add a script using alias
    help      Prints this message or the help of the given subcommand(s)
    list      List all scripts with optional filters
    remove    Remove a script using alias
    run       Run script
```
