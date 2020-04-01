use std::process;
use structopt::StructOpt;

use pier::{
    cli::{Cli, CliSubcommand},
    open_editor,
    script::Script,
    Pier, Result,
};

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
    let mut pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
    //let interpreter = config.get_interpreter();
    if let Some(subcmd) = cli.cmd {
        match subcmd {
            CliSubcommand::Add {
                command,
                alias,
                tags,
            } => {
                pier.add_script(Script {
                    alias,
                    command: match command {
                        Some(cmd) => cmd,
                        None => open_editor(None)?,
                    },
                    tags,
                    description: None,
                    reference: None,
                })?;
                pier.write()?;
            }

            CliSubcommand::Edit { alias } => {
                pier.edit_script(&alias)?;
                pier.write()?;
            }
            CliSubcommand::Remove { alias } => {
                pier.remove_script(&alias)?;
                pier.write()?;
            }
            CliSubcommand::Show { alias } => {
                let script = pier.fetch_script(&alias)?;
                println!("{}", script.command);
            }
            CliSubcommand::List {
                list_aliases,
                tags,
                cmd_full,
                cmd_width,
            } => {
                if list_aliases {
                    pier.list_aliases(tags)?
                } else {
                    pier.list_scripts(tags, cmd_full, cmd_width)?
                }
            }
            CliSubcommand::Run { alias } => {
                let arg = "";
                pier.run_script(&alias, arg)?;
            }
        };
    } else {
        let arg = "";
        let alias = &cli.alias.expect("Alias is required unless subcommand.");
        pier.run_script(alias, arg)?;
    }

    Ok(())
}
