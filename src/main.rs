use std::process;
use structopt::StructOpt;

use pier::{open_editor, Cli, CliSubcommand, Config, Result, Script};

fn main() {
    let opt = Cli::from_args();

    if let Err(err) = handle_subcommands(opt) {
        eprintln!("{}", err);
        // Only exits the process once the used memory has been cleaned up.
        process::exit(1);
    }
}

/// Handles the commandline subcommands
fn handle_subcommands(cli: Cli) -> Result<()> {
    let mut config = Config::from_input(cli.path)?;
    let interpreter = config.get_interpreter();
    if let Some(subcmd) = cli.cmd {
        match subcmd {
            CliSubcommand::Add {
                command,
                alias,
                tags,
            } => {
                config.add_script(Script {
                    alias,
                    command: match command {
                        Some(cmd) => cmd,
                        None => open_editor(None)?,
                    },
                    tags,
                    description: None,
                    reference: None,
                })?;
                config.write()?;
            }

            CliSubcommand::Edit { alias } => {
                config.edit_script(&alias)?;
                config.write()?;
            }
            CliSubcommand::Remove { alias } => {
                config.remove_script(&alias)?;
                config.write()?;
            }
            CliSubcommand::Show { alias } => {
                let script = config.fetch_script(&alias)?;
                println!("{}", script.command);
            }
            CliSubcommand::List { list_aliases, tags } => match list_aliases {
                true => config.list_aliases(tags)?,
                false => config.list_scripts(tags)?,
            },
            CliSubcommand::Run { alias } => {
                let arg = "";
                let script = config.fetch_script(&alias)?;
                script.run(interpreter, cli.verbose, arg)?;
            }
        };
    } else {
        let arg = "";
        let alias = &cli.alias.expect("Alias is required unless subcommand.");
        let script = config.fetch_script(alias)?;
        script.run(interpreter, cli.verbose, arg)?;
    }

    Ok(())
}
