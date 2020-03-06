# **pier** ~ A Linux script management tool
[![Build Status](https://travis-ci.com/BenSchZA/pier.svg?branch=master)](https://travis-ci.com/BenSchZA/pier)
![crates.io](https://img.shields.io/crates/v/pier.svg)

**THANKS:** To @Plommonsorbet for the great improvements to the shell

**NEW:** Nix derivation

**COMING SOON:** Syntax highlighting

A central repository to manage all your one-liners, scripts, tools, and CLIs. Add, remove, list, and run scripts - storing metadata to easily find them later. No more digging through your `bin` folder...

![Boat pier](https://raw.githubusercontent.com/BenSchZA/pier/master/.media/boat-dock.png)

## Description

If you've spent any amount of time in the terminal you no doubt have built up a lovely collection of one-liners, scripts, useful tools, and CLIs. Whenever you want to use them you dig through your `bin` folder trying to remember what you called the script... Linux users love hard-to-remember naming conventions.

Scripts should be first-class citizens. In a GUI world we can find our programs using a menu of sorts. In the terminal scripts get lost.

The idea behind `pier` is to create a central repository for all your scripts, and provide a way to attach metadata about these scripts. Using `pier` you can add, remove, list, and run scripts. These can be managed by `pier` in a human-readable TOML config, or you can use it to catalog existing scripts that you may have lying around - you'd then simply add the metadata for the specific script, and attach it to the name in the `PATH`.

## Installation

From **Crates.io**: `cargo install pier`

From **GitHub release**: simply download the release binary

Using **Nix** package manager:
1. From GitHub release: `make install` or `nix-env -if derivation.nix`
2. From source: update `src` in derivation to `./.`

## Operation

See `src/cli.yml` for a more detailed spec.

```
pier 0.1.4
Benjamin Scholtz, Isak Johansson
A simple script management CLI

USAGE:
    pier [FLAGS] [OPTIONS] <alias>
    pier [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information

    -v, --verbose    
            The level of verbosity


OPTIONS:
    -c, --config-file <path>    
            Sets a custom config file.
            
            DEFAULT PATH is otherwise determined in this order:
            
            - $PIER_CONFIG_PATH (environment variable if set)
            
            - pier.toml (in the current directory)
            
            - $XDG_CONFIG_HOME/pier/config.toml
            
            - $XDG_CONFIG_HOME/pier/config
            
            - $XDG_CONFIG_HOME/pier.toml
            
            - $HOME/.pier.toml
            
            - $HOME/.pier
            
             [env: PIER_CONFIG_PATH=]

ARGS:
    <alias>    
            The alias or name for the script.


SUBCOMMANDS:
    add       Add a new script to config.
    edit      Edit a script matching alias.
    help      Prints this message or the help of the given subcommand(s)
    list      alias: ls - List scripts
    remove    alias: rm - Remove a script matching alias.
    run       Run a script matching alias.
    show      Show a script matching alias.
```

`pier list`, `pier add "ip link set wlp58s0 down && sleep 5 && ip link set wlp58s0 up" --alias refresh-wifi`, `pier refresh-wifi`

### Execute pier scripts in any interpreted languages
Scripts starting with a shebang `#!` will be run with the specified interpeter just like it would in a normal script. Pier does this by creating a temp file from your script, executing it and then finally cleaning the file up. This allows you to write your pier script in python, node.js etc. even compiled languages can be run if using something like scriptisto.

#### Shebang example config
```

[scripts.run_rust_script]
alias = "run_rust_script"
command = '''
#!/usr/bin/env scriptisto

// scriptisto-begin
// script_src: src/main.rs
// build_cmd: cargo build --release && strip ./target/release/script
// target_bin: ./target/release/script
// files:
//  - path: Cargo.toml
//    content: |
//     package = { name = "script", version = "0.1.0", edition = "2018"}
//     [dependencies]
// scriptisto-end


fn main() {
    println!("This is a rust script!");
}

'''

[scripts.run_python]
alias = "run_python"
command = '''
#!/usr/bin/env python3
import sys

print("Running python with version {}".format(sys.version))

'''
```

### Setting the default shell / interpreter
By default if no shebang is specified pier will try to use your default shell to execute the script inline. This can be overwritten with the variable default_interpreter. It needs to be a list with the first item being the binary and the rest is any flags necessary.

#### Default interpreter example config
```
# Sets the default interpreter, the first item in the list should be the binary and the rest are the arguments for the interpreter cli option.
default_interpreter = ["node", "-e"]

# Runs as the fallback interpreter nodejs as it's lacking a shebang
[scripts.hello_world_nodejs]
alias = "hello_world_nodejs"
command = '''
console.log("Hello world!")

'''

# This will be run as a posix sh script as it has a shebang
[scripts.a_shell_script]
alias = "a_shell_script"
command = '''
#!/bin/sh

nohup st > /dev/null 2>&1&

'''
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

## Example `pier list` output

```
▶ pier list           
 Alias             | Command 
-------------------+----------------------------------------------------------------------------------------------------------------
 fromscratch       | appimage-run ~/AppImage/FromScratch.1.4.3.AppImage 
 nspawn-bionic     | sudo systemd-nspawn --bind=/tmp/.X11-unix -D /var/lib/machines/bionic --bind /home/bscholtz:/home/bscholtz 
 bspwm-refresh     | .config/bspwm/bspwmrc 
 flush-docker      | docker container stop $(docker container ls -a -q) && docker system prune -a -f --volumes 
 zfs-compression   | sudo zfs get all | grep compressratio 
 mongo-docker      | docker run --name mongodb -d mongo:latest 
 forward-mongo     | kubectl port-forward mongo-molecule-set-0 27018:27017 
 refresh-wifi      | ip link set wlp58s0 down && sleep 5 && ip link set wlp58s0 up 
 lepton            | appimage-run ~/AppImage/Lepton-1.8.0-x86_64.AppImage 
 ledger            | appimage-run AppImage/ledger-live-desktop-1.6.0-linux-x86_64.AppImage 
 reload-urxvt      | xrdb ~/.Xresources 
 kill-docker       | rm -rf /var/lib/docker 
 reload-xresources | xrdb ~/.Xresources 
 graphiql          | appimage-run ~/AppImage/graphiql-app-0.7.2-x86_64.AppImage 
 enabled-services  | systemctl list-unit-files --state=enabled 
 ports             | netstat -tulpn 
 chmod-copy        | chmod --reference= 
 zfs-drop-caches   | sync; echo 2 | sudo tee /proc/sys/vm/drop_caches 
 update            | sudo nix-channel --update && sudo nixos-rebuild switch 
 flush-untagged    | docker images -q --filter dangling=true | xargs -r docker rmi 
 twa-analyze       | docker run --rm -t trailofbits/twa -vw 
 parity-ubuntu     | docker image pull yodascholtz/parity-ubuntu:latest && docker run -p 8545:8545 yodascholtz/parity-ubuntu:latest
```

## Origin

Originally intended as a way to manage Docker one-liners, the name `pier` continues along the same maritime theme. I realized Pier can manage a lot more than just Docker scripts.

## Roadmap

* Accept script arguments using Rust Command `.arg()` parameter
