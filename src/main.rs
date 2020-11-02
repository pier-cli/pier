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

    match handle_subcommands(opt) {
        Ok(status) => {
            if let Some(status) = status {
                let code = status.code().unwrap_or(0);
                process::exit(code)
            } else {
                process::exit(0)
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };
}

/// Handles the commandline subcommands
fn handle_subcommands(cli: Cli) -> Result<Option<process::ExitStatus>> {
    if let Some(subcmd) = cli.cmd {
        match subcmd {
            CliSubcommand::Add {
                command,
                alias,
                description,
                tags,
            } => {
                let mut pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
                pier.add_script(Script {
                    alias,
                    description,
                    command: match command {
                        Some(cmd) => cmd,
                        None => open_editor(None)?,
                    },
                    tags,
                    reference: None,
                })?;
                pier.write()?;
            }

            CliSubcommand::Edit { alias } => {
                let mut pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
                pier.edit_script(&alias)?;
                pier.write()?;
            }
            CliSubcommand::Remove { alias } => {
                let mut pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
                pier.remove_script(&alias)?;
                pier.write()?;
            }
            CliSubcommand::ConfigInit => {
                let mut pier = Pier::new();
                pier.config_init(cli.opts.path)?;
            }
            CliSubcommand::Show { alias } => {
                let pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
                let script = pier.fetch_script(&alias)?;
                println!("{}", script.command);
            }
            CliSubcommand::List {
                list_aliases,
                tags,
                cmd_full,
                cmd_width,
            } => {
                let pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
                if list_aliases {
                    pier.list_aliases(tags)?
                } else {
                    pier.list_scripts(tags, cmd_full, cmd_width)?
                }
            }
            CliSubcommand::Run { alias, args } => {
                let pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
                let exit_code = pier.run_script(&alias, args)?;
                return Ok(Some(exit_code));
            }
            CliSubcommand::Copy {
                from_alias,
                to_alias,
            } => {
                let mut pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
                pier.copy_script(&from_alias, &to_alias)?;
                pier.write()?;
            }
        };
    } else {
        let alias = &cli.alias.expect("Alias is required unless subcommand.");
        let pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
        let exit_code = pier.run_script(alias, cli.args)?;
        return Ok(Some(exit_code));
    }

    Ok(None)
}
