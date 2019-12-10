use clap::load_yaml;
use clap::App;
use std::process;

use pier::{CliOptions, Config, Result, Script, editor};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Err(err) = handle_subcommands(&matches) {
        eprintln!("{}", err);
        // Only exits the process once the used memory has been cleaned up.
        process::exit(1);
    }
}

/// Handles the commandline subcommands
fn handle_subcommands(matches: &clap::ArgMatches) -> Result<()> {
    let path = matches.value_of("config");
    let mut config = Config::from_input(path)?;

    config.opts = CliOptions {
        verbose: matches.is_present("verbose"),
    };

    if config.opts.verbose { println!("Config file used: {}", config.path.display()) };

    match matches.subcommand() {
        ("add", Some(sub_matches)) => {
            let command = match sub_matches.value_of("INPUT") {
                Some(cmd) => cmd.to_string(),
                None => editor("")?
            };
            let alias = sub_matches.value_of("alias").unwrap().to_string();
            let tags: Option<Vec<String>> = match sub_matches.values_of("tags") {
                Some(values) => Some(values.map(|tag| tag.to_string()).collect()),
                None => None 
            };
            let appendage = Script {
                alias,
                command,
                description: None,
                reference: None,
                tags,
            };

            config.add_script(appendage)?;
            config.write()?;
        }
        ("edit", Some(sub_matches)) => {
            let alias = sub_matches.value_of("INPUT").unwrap();
            config.edit_script(&alias)?;
            config.write()?;
            //let alias = sub_matches.value_of("alias").unwrap().to_string();
            //let tags: Option<Vec<String>> = match sub_matches.values_of("tags") {
            //    Some(values) => Some(values.map(|tag| tag.to_string()).collect()),
            //    None => None 
            //};
            //let appendage = Script {
            //    alias,
            //    command,
            //    description: None,
            //    reference: None,
            //    tags,
            //};

            //config.add_script(appendage)?;
            //config.write()?;
        }
        ("remove", Some(sub_matches)) => {
            let alias = sub_matches.value_of("INPUT").unwrap();
            config.remove_script(&alias)?;
            config.write()?;
        }
        ("run", Some(sub_matches)) => {
            let arg = "";
            let alias = sub_matches.value_of("INPUT").unwrap();
            let script = config.fetch_script(&alias)?;

            script.run(arg)?;
        }
        ("show", Some(sub_matches)) => {
            let alias = sub_matches.value_of("INPUT").unwrap();
            let script = config.fetch_script(&alias)?;
            
            println!("{}", script.command);
        }
        ("list", Some(sub_matches)) => {
            let tags: Option<Vec<String>> = match sub_matches.values_of("tags") {
                Some(values) => Some(values.map(|tag| tag.to_string()).collect()),
                None => None 
            };
            config.list_scripts(tags)?;
        }
        _ => {
            let arg = "";
            let alias = matches.value_of("INPUT").unwrap();
            let script = config.fetch_script(&alias)?;
            script.run(arg)?;
        }
    };
    Ok(())
}
