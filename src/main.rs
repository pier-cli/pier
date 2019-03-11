use std::fs::File;
use std::io::prelude::*;
use std::env;

use clap::load_yaml;
use clap::App;

use toml;
use toml::Value;

use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct Config {
    scripts: Option<Vec<Script>>,
}

#[derive(Deserialize, Debug)]
struct Script {
    alias: Option<String>,
    command: Option<String>,
    description: Option<String>,
    reference: Option<String>,
    tags: Option<Vec<String>>,
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config: Config = load_config(&matches);

    println!("{:#?}", config);

    match matches.subcommand() {
        ("add", Some(sub_matches)) => {
           println!("add subcommand was used");
        },
        ("remove", Some(sub_matches)) => {
            println!("remove subcommand was used");
        },
        ("run", Some(sub_matches)) => {
            println!("run subcommand was used");
        },
        ("list", Some(sub_matches)) => {
            println!("list subcommand was used");
        },
        ("", None) => println!("No subcommand was used"),
        _          => unreachable!(),
    }
}

fn load_config(matches: &clap::ArgMatches) -> Config {
    let mut config_string = String::new();
    let mut config_dir = String::new();

    if matches.is_present("config") {
        config_dir = format!("{}", matches.value_of("config").unwrap());
    } else {
        config_dir = format!("{}/.pier", env::var("HOME").unwrap());
    }
    
    let config_file = File::open(&config_dir);
    let config_file = match config_file {
        Ok(mut file) => {
            file.read_to_string(&mut config_string);
        },
        Err(error) => {
            println!("Config file {} not found", &config_dir);
        },
    };

    //return config_string.parse::<Value>().unwrap();
    return toml::from_str(&config_string).unwrap();
}