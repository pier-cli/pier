use serde::{Deserialize, Serialize};
use super::Result;
use super::error::*;
use super::script::Script;
use snafu::{ResultExt};
use std::{collections::BTreeMap, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConfigDefaultOpts {
    // Default interpreter to use if script doesn't have a shebang.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpreter: Option<Vec<String>>,

    // Default width of the command when listing the scripts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_width: Option<usize>,
}
    

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    // Don't write anything to config if map is empty
    #[serde(default = "BTreeMap::new", skip_serializing_if = "BTreeMap::is_empty")]
    pub scripts: BTreeMap<String, Script>,

    #[serde(default)]
    pub default: ConfigDefaultOpts
}

impl Config {
    /// Helper function to read file.
    pub fn read(path: &PathBuf) -> Result<String> {
        let file_content = fs::read_to_string(path).context(ConfigRead { path })?;

        Ok(file_content)
    }

    /// Writes the current Config to file.
    pub fn write(&self, path: &PathBuf) -> Result<()> {
        let config_string = toml::to_string_pretty(&self).context(TomlSerialize)?;

        fs::write(path, config_string).context(ConfigWrite { path })?;

        Ok(())
    }

    pub fn from(path: &PathBuf) -> Result<Self> {
        let config_str = Config::read(path)?;

        let config = toml::from_str(&config_str).context(TomlParse { path })?;

        Ok(config)
    }
}
