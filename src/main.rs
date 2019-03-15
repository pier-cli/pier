use std::fs::File;
use std::io::{prelude::*, Error};
use std::env;
use std::process;
use std::collections::HashMap;

use clap::load_yaml;
use clap::App;

use toml;
use toml::Value;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    scripts: Option<HashMap<String, Script>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Script {
    alias: String,
    command: String,
    description: Option<String>,
    reference: Option<String>,
    tags: Option<Vec<String>>,
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = &mut load_config(&matches);

    match matches.subcommand() {
        ("add", Some(sub_matches)) => {
            let script = sub_matches.value_of("INPUT").unwrap();
            let alias = sub_matches.value_of("alias").unwrap();
           
            let appendage = Script {
                alias: alias.to_string(),
                command: script.to_string(), 
                description: None,
                reference: None,
                tags: None
            };

            match &config.scripts {
                Some(_scripts) => {
                    config.scripts.as_mut().unwrap()
                        .entry(alias.to_string()).or_insert(appendage);
                    write_config(&matches, &config)
                        .expect("Failed to save config to file");
                },
                None => {
                    let mut scripts = HashMap::new();
                    scripts.insert(alias.to_string(), appendage);
                    write_config(
                        &matches, 
                        &Config {
                            scripts: Some(scripts)
                        })
                        .expect("Failed to save config to file");
                }
            }

            println!("+ {} / alias {}", script, alias);
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

fn write_config(matches: &clap::ArgMatches, config: &Config) -> Result<(),Error> {
    let config_dir = get_config_dir(matches);
    
    let mut file = File::create(&config_dir)?;
    
    let toml = toml::to_string(config).unwrap();
    file.write_all(toml.as_bytes())
        .expect("Could not write to file!");
    
    Ok(())
}

fn load_config(matches: &clap::ArgMatches) -> Config {
    let mut config_string = String::new();
    let config_dir = get_config_dir(matches);
    
    match File::open(&config_dir) {
        Ok(mut file) => {
            file.read_to_string(&mut config_string)
                .expect("Failed to read config file contents");
        },
        Err(_error) => {
            println!("Config file {} not found", &config_dir);
            process::exit(1);
        },
    };

    return toml::from_str(&config_string).unwrap();
}

fn get_config_dir(matches: &clap::ArgMatches) -> String {
    let mut config_dir;

    if matches.is_present("config") {
        config_dir = format!(
            "{}",
            matches.value_of("config").unwrap()
        );
    } else {
        config_dir = format!(
            "{}/.pier", 
            env::var("HOME").expect("$HOME variable not set")
        );
    }

    return config_dir;
}