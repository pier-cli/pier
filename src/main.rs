use std::fs::File;
use std::io::{prelude::*, Error};
use std::env;
use std::process;
use std::process::Command;
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
            let command = sub_matches.value_of("INPUT").unwrap();
            let alias = sub_matches.value_of("alias").unwrap();
           
            let appendage = Script {
                alias: alias.to_string(),
                command: command.to_string(), 
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

            println!("+ {} / alias {}", command, alias);
        },
        ("remove", Some(sub_matches)) => {
            let alias = sub_matches.value_of("INPUT").unwrap();
            let script: Script;

            match &config.scripts {
                Some(scripts) => {
                    if scripts.contains_key(&alias.to_string()) {
                        script = config.scripts.as_mut().unwrap()
                            .remove(&alias.to_string())
                            .expect("Failed to remove script");
                        write_config(&matches, &config)
                            .expect("Failed to save config to file");
                    } else {
                        println!("Invalid alias");
                        process::exit(1);
                    } 
                },
                None => {
                    println!("Invalid alias");
                    process::exit(1);
                }
            }

            println!("- {:?} / alias {}", script, alias);
        },
        ("run", Some(sub_matches)) => {
            let alias = sub_matches.value_of("INPUT").unwrap();
            let mut args: Vec<_> = match &sub_matches.values_of("args") {
                Some(ref unwrapped) => unwrapped.clone().map(String::from).collect::<Vec<_>>(),
                None => vec![]
            };

            println!("Starting script {}", alias);
            println!("-------------------------");

            match &config.scripts {
                Some(_scripts) => {
                    match &config.scripts.as_mut().unwrap().get(&alias.to_string()) {
                        Some(script) => {
                            let mut command = Command::new("sh");
                            let mut script_with_args = vec![String::from(r#"-c"#), String::from(r#"'"#), script.command.clone()];
                            script_with_args.append(&mut args);
                            script_with_args.append(&mut vec![String::from(r#"'"#)]);
                            println!("{:?}", script_with_args);
                            let output = command
                                .args(script_with_args)
                                .output().expect("Failed to execute process");
                            println!("{:?}", String::from_utf8_lossy(&output.stdout));
                            
                            assert!(output.status.success());
                            println!("-------------------------");
                            println!("Script complete");
                        },
                        None => println!("Invalid alias, would you like to create a new script?")
                    }
                },
                None => {}
            }
        },
        ("list", Some(sub_matches)) => {
            match &config.scripts {
                Some(scripts) => {
                    for (alias, script) in scripts {
                        println!("{}: \"{:?}\"", alias, script);
                    }
                },
                None => println!("No scripts exist. Would you like to add a new script?")
            }
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