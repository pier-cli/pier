use std::fs::File;
use std::io::{prelude::*, Error};
use std::env;
use std::process;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

use clap::load_yaml;
use clap::App;

#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, format};

use toml;
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

    match matches.value_of("INPUT") {
        Some(alias) => {
            // let arg = match sub_matches.value_of("arg") {
            //     Some(arg) => String::from(arg),
            //     None => String::from("")
            // };
            let arg = String::from("");

            match fetch_script(alias, config) {
                Some(script) => run_command(alias, &script.command, &arg),
                None => println!("Invalid alias, would you like to create a new script?"),
            }
        },
        None => handle_subcommands(&matches, config).expect("No input or subcommands"),
    }    
}

fn handle_subcommands(matches: &clap::ArgMatches, config: & mut Config) -> Result<(),Error> {
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
            let arg = match sub_matches.value_of("arg") {
                Some(arg) => String::from(arg),
                None => String::from("")
            };

            match fetch_script(alias, config) {
                Some(script) => run_command(alias, &script.command, &arg),
                None => println!("Invalid alias, would you like to create a new script?"),
            }
        },
        ("list", Some(_sub_matches)) => {
            match &config.scripts {
                Some(scripts) => {
                    let mut table = Table::new();
                    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
                    table.set_titles(row!["Alias", "Command"]);
                    for (alias, script) in scripts {
                        table.add_row(row![alias, script.command]);
                    }
                    // Print the table to stdout
                    table.printstd();
                },
                None => println!("No scripts exist. Would you like to add a new script?")
            }
        },
        ("", None) => println!("No subcommand was used"),
        _          => unreachable!(),
    }

    Ok(())
}

fn fetch_script<'a>(alias: &str, config: &'a Config) -> Option<&'a Script> {
    return match &config.scripts {
        Some(scripts) => {
            scripts.get(&alias.to_string())
        },
        None => None
    }
}

fn run_command(alias: &str, command: &str, arg: &str) {
    println!("Starting script \"{}\"", alias);
    println!("-------------------------");
    
    let default_shell = env::var("SHELL").expect("No default shell set!");
    let cmd = Command::new(default_shell)
        .args(&["-c", command])
        .spawn()
        .expect("Failed to create process.")
        .wait_with_output();
    match cmd {
        Ok(output) => {
            println!("{:?}", String::from_utf8_lossy(&output.stdout));
        }
        Err(why) => panic!("error when running command: {}", why)
    };

    println!("-------------------------");
    println!("Script complete");
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

    toml::from_str(&config_string).unwrap()
}

fn get_config_dir(matches: &clap::ArgMatches) -> String {
    if matches.is_present("config") {
        matches.value_of("config").unwrap().to_string()
    } else if let Ok(pier_config_path_env) = env::var("PIER_CONFIG_PATH") {
        // Adds possibility of user defined config path.
        pier_config_path_env
    } else {
        let home_dir = env::var("HOME").expect("$HOME variable not set");

        let xdg_config_home_path = match env::var("XDG_CONFIG_HOME") {
            Ok(config_dir) => format!("{}/pier/config", config_dir),
            Err(_) => format!("{}/.config/pier/config", home_dir)
        };

        let alternate_config_path = format!("{}/.pier", home_dir);

        if Path::new(&xdg_config_home_path).exists() {
            // Try using the XDG Base Directory standard if config path exists.
            xdg_config_home_path
        } else if Path::new(&alternate_config_path).exists() {
            // Use the ~/.pier as a second choice of config path if it exists.
            alternate_config_path
        } else {
            // If no valid config file can be found use xdg_config_home_path as a last resort even if it doesn't exist 
            xdg_config_home_path
        }
    }
}
