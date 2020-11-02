# **pier** ~ A Linux script management tool
[![Build Status](https://travis-ci.com/BenSchZA/pier.svg?branch=master)](https://travis-ci.com/BenSchZA/pier)
![crates.io](https://img.shields.io/crates/v/pier.svg)

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


## Recent Breaking Changes

#### Version `0.1.4`:
The configuration variable `default_interpreter` has been _**removed**_:
```toml
default_interpreter = ["foorunner", "-c"]
```
So when upgrading from to `0.1.4` from an earlier version you will need to instead specify the variable in this format:
```toml
[default]
interpreter = ["foorunner", "-c"]
```


## Origin

Originally intended as a way to manage Docker one-liners, the name `pier` continues along the same maritime theme. I realized Pier can manage a lot more than just Docker scripts.

## Roadmap to v1.0.0

* Fuzzy search + autocomplete e.g. prompts for alias
* Complete base features: mv, tag
* Rework testing
* Update documentation e.g. contributor guidelines, templates
