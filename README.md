# **pier** ~ A simple Docker script management CLI

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
