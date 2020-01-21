use prettytable::{cell, format, row, Table};
use serde::{Deserialize, Serialize};
use snafu::{ensure, OptionExt, ResultExt};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;
use structopt::{clap::AppSettings, StructOpt};
use tempfile;
use toml;
pub mod error;
mod macros;
use dirs;
use error::*;
use scrawl;

// Creates a Result type that return PierError by default
pub type Result<T, E = PierError> = ::std::result::Result<T, E>;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    // Don't write anything to config if map is empty
    #[serde(default = "BTreeMap::new", skip_serializing_if = "BTreeMap::is_empty")]
    scripts: BTreeMap<String, Script>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_interpreter: Option<Vec<String>>,

    #[serde(skip)]
    pub verose: bool,

    #[serde(skip)]
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
    pub alias: String,
    pub command: String,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, StructOpt)]
pub enum CliSubcommand {
    /// Add a new script to config.
    Add {
        /// The command/script content to be executed.
        /// If this argument is not found it will open your $EDITOR for you to enter the script into.
        command: Option<String>,

        /// The alias or name for the script.
        #[structopt(short = "a", long = "alias")]
        alias: String,

        /// Set which tags the script belongs to.
        #[structopt(short = "t", long = "tag")]
        tags: Option<Vec<String>>,
    },
    /// alias: rm - Remove a script matching alias.
    #[structopt(alias = "rm")]
    Remove {
        /// The alias or name for the script.
        alias: String,
    },
    /// Edit a script matching alias.
    Edit {
        /// The alias or name for the script.
        alias: String,
    },
    /// Show a script matching alias.
    Show {
        /// The alias or name for the script.
        alias: String,
    },
    /// Run a script matching alias.
    Run {
        /// The alias or name for the script.
        alias: String,
        //#[structopt(short = "a", long = "arg")]
        //arg: String,
    },
    /// alias: ls - List scripts
    #[structopt(alias = "ls")]
    List {
        /// Only displays aliases of the scripts.
        #[structopt(short = "q", long = "list_aliases")]
        list_aliases: bool,

        /// Filter based on tags.
        #[structopt(short = "t", long = "tag")]
        tags: Option<Vec<String>>,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(setting = AppSettings::SubcommandsNegateReqs, author)]
/// A simple script management CLI
pub struct Cli {
    /// The level of verbosity
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    /// Sets a custom config file.
    ///
    /// DEFAULT PATH is otherwise determined in this order:
    ///
    ///   - $PIER_CONFIG_PATH (environment variable if set)
    ///
    ///   - pier.toml (in the current directory)
    ///
    ///   - $XDG_CONFIG_HOME/pier/config.toml
    ///
    ///   - $XDG_CONFIG_HOME/pier/config
    ///
    ///   - $XDG_CONFIG_HOME/pier.toml
    ///
    ///   - $HOME/.pier.toml
    ///
    ///   - $HOME/.pier
    ///
    #[structopt(short = "c", long = "config-file", env = "PIER_CONFIG_PATH")]
    pub path: Option<PathBuf>,

    /// The alias or name for the script.
    #[structopt(required_unless = "cmd")]
    pub alias: Option<String>,

    /// Pier subcommands
    #[structopt(subcommand)]
    pub cmd: Option<CliSubcommand>,
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

impl Config {
    // Generates a new empty config
    pub fn new() -> Config {
        Config::default()
    }
    pub fn get_interpreter(&self) -> Vec<String> {
        match self.default_interpreter.clone() {
            Some(interpreter) => interpreter,
            None => {
                let shell = match env::var("SHELL") {
                    Ok(default_shell) => default_shell,
                    Err(_error) => String::from("/bin/sh")
                };
                vec![shell, String::from("-c")]
            }
        }
    }

    /// Helper function to read file.
    fn read(path: &Path) -> Result<String> {
        let file_content = fs::read_to_string(path).context(ConfigRead { path })?;
        Ok(file_content)
    }

    /// Writes the current Config to file.
    pub fn write(&self) -> Result<()> {
        let config_string = toml::to_string_pretty(&self).context(TomlSerialize)?;
        fs::write(&self.path, config_string).context(ConfigWrite { path: &self.path })?;
        Ok(())
    }

    /// Generate a new Config based on a file path.
    pub fn from_file(path: PathBuf) -> Result<Config> {
        let config_string = Config::read(&path)?;
        let mut config: Config =
            toml::from_str(&config_string).context(TomlParse { path: &path })?;
        config.path = path;
        Ok(config)
    }

    /// Generate a new Config from path specified specified with cli flag or environment variable
    /// if no path is specified try to look for any config in a default location.
    pub fn from_input(selected_path: Option<PathBuf>) -> Result<Config> {
        if let Some(sel_path) = selected_path {
            Ok(Config::from_file(sel_path)?)
        } else {
            let default_config_paths: Vec<Option<PathBuf>> = vec![
                Some(PathBuf::from("pier.toml")),
                xdg_config_home!("pier/config.toml"),
                xdg_config_home!("pier/config"),
                xdg_config_home!("pier.toml"),
                home!(".pier.toml"),
                home!(".pier"), // Kept the .pier path for backwards compatibility
            ];

            // Loops for a vector of possible paths and tries to generate config from the first
            // default path that exists.
            for config_path in default_config_paths {
                if let Some(path) = config_path {
                    if path.exists() {
                        return Ok(Config::from_file(path)?);
                    }
                }
            }
            pier_err!(PierError::NoConfigFile)
        }
    }

    /// Fetches a script that matches the alias
    pub fn fetch_script(&self, alias: &str) -> Result<&Script> {
        ensure!(!self.scripts.is_empty(), NoScriptsExists);
        let script = self
            .scripts
            .get(&alias.to_string())
            .context(AliasNotFound {
                alias: &alias.to_string(),
            })?;
        Ok(script)
    }

    /// Edits a script that matches the alias
    pub fn edit_script(&mut self, alias: &str) -> Result<&Script> {
        ensure!(!self.scripts.is_empty(), NoScriptsExists);
        let mut script = self
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
        ensure!(!self.scripts.is_empty(), NoScriptsExists);
        self.scripts
            .remove(&alias.to_string())
            .context(AliasNotFound {
                alias: &alias.to_string(),
            })?;
        println!("Removed {}", &alias);
        Ok(())
    }

    /// Adds a script that matches the alias
    pub fn add_script(&mut self, script: Script) -> Result<()> {
        println!("Added {}", &script.alias);
        self.scripts.insert(script.alias.to_string(), script);
        Ok(())
    }

    /// Prints only the aliases in current config file that matches tags.
    pub fn list_aliases(&self, tags: Option<Vec<String>>) -> Result<()> {
        ensure!(!self.scripts.is_empty(), NoScriptsExists);
        for (alias, script) in &self.scripts {
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
    pub fn list_scripts(&self, tags: Option<Vec<String>>) -> Result<()> {
        ensure!(!self.scripts.is_empty(), NoScriptsExists);
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["Alias", "tags", "Command"]);
        for (alias, script) in &self.scripts {
            match (&tags, &script.tags) {
                (Some(list_tags), Some(script_tags)) => {
                    for tag in list_tags {
                        if script_tags.contains(tag) {
                            table.add_row(row![&alias, script_tags.join(","), &script.command]);
                            continue;
                        }
                    }
                }
                (None, Some(script_tags)) => {
                    table.add_row(row![&alias, script_tags.join(","), &script.command]);
                    continue;
                }
                (None, None) => {
                    table.add_row(row![&alias, "", &script.command]);
                    continue;
                }
                _ => (),
            };
        }

        table.printstd();
        Ok(())
    }
}


impl Script {
    /// Runs a script and print stdout and stderr of the command.
    pub fn run(&self, default_interpreter: Vec<String>, verbose: bool, _arg: &str) -> Result<()> {
        if verbose {
            println!("Starting script \"{}\"", &self.alias);
            println!("-------------------------");
        };

        // Check that the script is not empty
        if let Some(first_line) = self.command.lines().nth(0) {
            // Check whether the script starts with a shebang.
            let cmd = match first_line.starts_with("#!") {
                true => self.run_with_shebang()?,
                false => self.run_with_cli_interpreter(default_interpreter)?,
            };

            let stdout = String::from_utf8_lossy(&cmd.stdout);
            let stderr = String::from_utf8_lossy(&cmd.stderr);

            if stdout.len() > 0 {
                println!("{}", stdout);
            };
            if stderr.len() > 0 {
                eprintln!("{}", stderr);
            };

        };

        if verbose {
            println!("-------------------------");
            println!("Script complete");
        };

        Ok(())
    }

    /// Runs the script inline using something like sh -c "<script>" or python -c "<script."...
    fn run_with_cli_interpreter(&self, interpreter: Vec<String>) -> Result<Output> {
        // First item in interpreter is the binary
        let cmd = Command::new(&interpreter[0])
            // The following items after the binary is any commandline args that are necessary.
            .args(&interpreter[1..])
            .arg(&self.command)
            .stderr(Stdio::piped())
            .spawn()
            .context(CommandExec)?
            .wait_with_output()
            .context(CommandExec)?;

        Ok(cmd)
    }

    /// First creates a temporary file and then executes the file before removing it.
    fn run_with_shebang(&self) -> Result<Output> {
        // Creates a temp directory to place our tempfile inside.
        let tmpdir = tempfile::Builder::new()
            .prefix("pier")
            .tempdir()
            .context(ExecutableTempFileCreate)?;

        let exec_file_path = tmpdir.path().join(&self.alias);

        // Creating the file inside a closure is convenient because rust will automatically handle
        // closing the file for us so we can go ahead and execute it after writing to it and setting the file permissions.
        {
            let mut exec_file = File::create(&exec_file_path).context(ExecutableTempFileCreate)?;

            exec_file
                .write(self.command.as_bytes())
                .context(ExecutableTempFileCreate)?;

            let mut permissions = exec_file
                .metadata()
                .context(ExecutableTempFileCreate)?
                .permissions();

            // Set the file permissions to allow read and execute for the current user.
            permissions.set_mode(0o500);

            exec_file
                .set_permissions(permissions)
                .context(ExecutableTempFileCreate)?;
        }

        let cmd = Command::new(exec_file_path)
            .stderr(Stdio::piped())
            .spawn()
            .context(CommandExec)?
            .wait_with_output()
            .context(CommandExec)?;

        Ok(cmd)
    }
}
