use prettytable::{cell, format, row, Table};
use snafu::{ensure, OptionExt, ResultExt};
use std::{path::PathBuf};
pub mod cli;
mod config;
pub mod error;
use config::Config;
mod macros;
mod defaults;
use defaults::*;
pub mod script;
use script::Script;
use error::*;
use scrawl;

// Creates a Result type that return PierError by default
pub type Result<T, E = PierError> = ::std::result::Result<T, E>;

/// Main library interface
#[derive(Debug, Default)]
pub struct Pier {
    config: Config,
    path: PathBuf,
    verbose: bool,
}

impl Pier {
    /// Wrapper to write the configuration to path.
    pub fn write(&self) -> Result<()> {
        self.config.write(&self.path)?;

        Ok(())
    }

    pub fn new() -> Self {
        Pier::default()
    }

    /// Create new pier directly from path.
    pub fn from_file(path: PathBuf, verbose: bool) -> Result<Self> {
        let pier = Self {
            config: Config::from(&path)?,
            verbose,
            path,
        };
        Ok(pier)
    }
    /// Create new pier from what might be a path, otherwise use the first existing default path.
    pub fn from(input_path: Option<PathBuf>, verbose: bool) -> Result<Self> {
        let path = match input_path {
            Some(path) => path,
            None => fallback_path()?,
        };

        let pier = Pier::from_file(path, verbose)?;

        Ok(pier)
    }

    /// Fetches a script that matches the alias
    pub fn fetch_script(&self, alias: &str) -> Result<&Script> {
        ensure!(!self.config.scripts.is_empty(), NoScriptsExists);

        let script = self
            .config
            .scripts
            .get(&alias.to_string())
            .context(AliasNotFound {
                alias: &alias.to_string(),
            })?;

        Ok(script)
    }

    /// Edits a script that matches the alias
    pub fn edit_script(&mut self, alias: &str) -> Result<&Script> {
        ensure!(!self.config.scripts.is_empty(), NoScriptsExists);

        let mut script =
            self.config
                .scripts
                .get_mut(&alias.to_string())
                .context(AliasNotFound {
                    alias: &alias.to_string(),
                })?;

        script.command = open_editor(Some(&script.command))?;

        println!("Edited {}", &alias);

        Ok(script)
    }

    /// Removes a script that matches the alias
    pub fn remove_script(&mut self, alias: &str) -> Result<()> {
        ensure!(!self.config.scripts.is_empty(), NoScriptsExists);

        self.config
            .scripts
            .remove(&alias.to_string())
            .context(AliasNotFound {
                alias: &alias.to_string(),
            })?;

        println!("Removed {}", &alias);

        Ok(())
    }

    /// Adds a script that matches the alias
    pub fn add_script(&mut self, script: Script) -> Result<()> {
	ensure!( ! &self.config.scripts.contains_key(&script.alias),
		AliasAlreadyExists { alias: script.alias });
        println!("Added {}", &script.alias);

        self.config.scripts.insert(script.alias.to_string(), script);

        Ok(())
    }

    /// Prints only the aliases in current config file that matches tags.
    pub fn list_aliases(&self, tags: Option<Vec<String>>) -> Result<()> {
        ensure!(!self.config.scripts.is_empty(), NoScriptsExists);

        for (alias, script) in &self.config.scripts {
            match (&tags, &script.tags) {
                (Some(list_tags), Some(script_tags)) => {
                    for tag in list_tags {
                        if script_tags.contains(tag) {
                            println!("{}", alias);

                            continue;
                        }
                    }
                }
                (None, _) => {
                    println!("{}", alias);

                    continue;
                }
                _ => (),
            };
        }

        Ok(())
    }

    /// Prints a terminal table of the scripts in current config file that matches tags.
    pub fn list_scripts(
        &self,
        tags: Option<Vec<String>>,
        cmd_full: bool,
        cmd_width: Option<usize>,
    ) -> Result<()> {
        let width = match (cmd_width, self.config.default.command_width) {
            (Some(width), _) => width,
            (None, Some(width)) => width,
            (None, None) => FALLBACK_COMMAND_DISPLAY_WIDTH,
        };
        ensure!(!self.config.scripts.is_empty(), NoScriptsExists);

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["Alias", "tags", "Command"]);

        for (alias, script) in &self.config.scripts {
            match (&tags, &script.tags) {
                (Some(list_tags), Some(script_tags)) => {
                    for tag in list_tags {
                        if script_tags.contains(tag) {
                            table.add_row(row![
                                &alias,
                                script_tags.join(","),
                                script.display_command(cmd_full, width)
                            ]);

                            continue;
                        }
                    }
                }
                (None, Some(script_tags)) => {
                    table.add_row(row![
                        &alias,
                        script_tags.join(","),
                        script.display_command(cmd_full, width)
                    ]);

                    continue;
                }
                (None, None) => {
                    table.add_row(row![&alias, "", script.display_command(cmd_full, width)]);

                    continue;
                }
                _ => (),
            };
        }

        table.printstd();

        Ok(())
    }

    /// Runs a script and print stdout and stderr of the command.
    pub fn run_script(&self, alias: &str, _arg: &str) -> Result<()> {
        let script = self.fetch_script(alias)?;
        let interpreter = match self.config.default.interpreter {
            Some(ref interpreter) => interpreter.clone(),
            None => fallback_shell(),
        };

        if self.verbose {
            println!("Starting script \"{}\"", alias);
            println!("-------------------------");
        };

        let cmd = match script.has_shebang() {
            true => script.run_with_shebang()?,
            false => script.run_with_cli_interpreter(&interpreter)?,
        };

        let stdout = String::from_utf8_lossy(&cmd.stdout);
        let stderr = String::from_utf8_lossy(&cmd.stderr);

        if stdout.len() > 0 {
            println!("{}", stdout);
        };
        if stderr.len() > 0 {
            eprintln!("{}", stderr);
        };

        if self.verbose {
            println!("-------------------------");
            println!("Script complete");
        };

        Ok(())
    }
}

pub fn open_editor(content: Option<&str>) -> Result<String> {
    let edited_text = scrawl::editor::new()
        .contents(match content {
            Some(txt) => txt,
            None => "",
        })
        .open()
        .context(EditorError)?;

    Ok(edited_text)
}
