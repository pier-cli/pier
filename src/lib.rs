use prettytable::{cell, format, row, Table};
use serde::{Deserialize, Serialize};
use snafu::{ensure, OptionExt, ResultExt};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use toml;
pub mod error;
mod macros;
use dirs;
use error::*;
use scrawl;

// Creates a Result type that return PierError by default
pub type Result<T, E = PierError> = ::std::result::Result<T, E>;

// Struct containing Extra Options.
#[derive(Debug, Default)]
pub struct CliOptions {
    pub verbose: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // Don't write anything to config if map is empty
    #[serde(default = "BTreeMap::new", skip_serializing_if = "BTreeMap::is_empty")]
    scripts: BTreeMap<String, Script>,
    #[serde(skip)]
    pub path: PathBuf,
    #[serde(skip)]
    pub opts: CliOptions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
    pub alias: String,
    pub command: String,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub fn editor(content: &str) -> Result<String> {
    Ok(scrawl::editor::new().contents(content).open().context(EditorError)?)
}

impl Config {
    // Generates a new empty config
    pub fn new() -> Config {
        Config {
            scripts: BTreeMap::new(),
            path: PathBuf::from(""),
            opts: CliOptions { verbose: false },
        }
    }

    /// Helper function to read file
    fn read(path: &Path) -> Result<String> {
        let file_content = fs::read_to_string(path).context(ConfigRead { path })?;
        Ok(file_content)
    }

    /// Writes the current Config to file
    pub fn write(&self) -> Result<()> {
        let config_string = toml::to_string_pretty(&self).context(TomlSerialize)?;
        fs::write(&self.path, config_string).context(ConfigWrite { path: &self.path })?;
        Ok(())
    }

    /// Generate a new Config based on a file path
    pub fn from_file(path: &Path) -> Result<Config> {
        let config_string = Config::read(path)?;
        let mut config: Config = toml::from_str(&config_string).context(TomlParse { path })?;
        config.path = path.to_path_buf();
        Ok(config)
    }

    /// Generate a new Config from path specified specified with cli flag or environment variable
    /// if no path is specified try to look for any config in a default location.
    pub fn from_input(selected_path: Option<&str>) -> Result<Config> {
        if let Some(path_str) = selected_path {
            Ok(Config::from_file(Path::new(path_str))?)
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
                        return Ok(Config::from_file(&path)?);
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
        script.command = editor(&script.command)?;
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
        Ok(())
    }

    /// Adds a script that matches the alias
    pub fn add_script(&mut self, script: Script) -> Result<()> {
        println!("+ {} / alias {}", &script.command, &script.alias);
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
                _ => ()
                
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
                _ => ()
                
            };
        }

        table.printstd();
        Ok(())
    }

}

impl Script {
    /// Runs a script and print stdout and stderr of the command.
    pub fn run(&self, opts: &CliOptions, arg: &str) -> Result<()> {
        if opts.verbose { 
            println!("Starting script \"{}\"", &self.alias);
            println!("-------------------------");
        };

        let default_shell = env::var("SHELL").context(NoDefaultShell)?;

        let cmd = Command::new(default_shell)
            .args(&["-c", &self.command])
            .stderr(Stdio::piped())
            .spawn()
            .context(CommandExec)?
            .wait_with_output()
            .context(CommandExec)?;

        let stdout = String::from_utf8_lossy(&cmd.stdout);
        let stderr = String::from_utf8_lossy(&cmd.stderr);

        if stdout.len() > 0 {
            println!("{}", stdout);

        };
        if stdout.len() > 0 {
            eprintln!("{}", stderr);

        };

        if opts.verbose { 
            println!("Starting script \"{}\"", &self.alias);
            println!("-------------------------");
        };

        Ok(())
    }
}
