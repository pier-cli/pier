# **pier** ~ A Linux script management tool

A central repository to manage all your one-liners, scripts, tools, and CLIs. Add, remove, list, and run scripts - storing metadata to easily find them later. No more digging through your `bin` folder...

## Description

If you've spent any amount of time in the terminal you no doubt have built up a lovely collection of one-liners, scripts, useful tools, and CLIs. Whenever you want to use them you dig through your `bin` folder trying to remember what you called the script... Linux users love hard-to-remember naming conventions.

Scripts should be first-class citizens. In a GUI world we can find our programs using a menu of sorts. In the terminal scripts get lost.

The idea behind `pier` is to create a central repository for all your scripts, and provide a way to attach metadata about these scripts. Using `pier` you can add, remove, list, and run scripts. These can be managed by `pier` in a human-readable TOML config, or you can use it to catalog existing scripts that you may have lying around - you'd then simply add the metadata for the specific script, and attach it to the name in the `PATH`.

## Operation

See `src/cli.yml` for a more detailed spec.

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

## Example `pier` TOML config

```
[scripts.refresh-wifi]
alias = "refresh-wifi"
command = "ip link set wlp58s0 down && sleep 5 && ip link set wlp58s0 up"

[scripts.twa-analyze]
alias = "twa-analyze"
command = "docker run --rm -t trailofbits/twa -vw"
tags = [ "infosec" ]

[scripts.enabled-services]
alias = "enabled-services"
command = "systemctl list-unit-files --state=enabled"

[scripts.flush-docker]
alias = "flush-docker"
command = "docker container stop $(docker container ls -a -q) && docker system prune -a -f --volumes"
description = "A script to clear out old Docker containers and images"
tags = [ "docker", "flush" ]
```

## Origin

Originally intended as a way to manage Docker one-liners, the name `pier` continues along the same maritime theme. I realized Pier can manage a lot more than just Docker scripts.

## Roadmap

* Accept script arguments using Rust Command `.arg()` parameter
* Allow listing using tags
