use std::{collections::BTreeMap, fmt, fs, path::PathBuf};
use std::collections::btree_map::Iter;
use std::marker::PhantomData;

use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{MapAccess, Visitor};
use snafu::ResultExt;

use super::error::*;
use super::script::Script;
use super::PierResult;

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
    #[serde(default)]
    pub scripts: Scripts,

    #[serde(default)]
    pub default: ConfigDefaultOpts,
}

#[derive(Serialize, Debug, Default)]
pub struct Scripts(BTreeMap<String, Script>);

impl Config {
    /// Helper function to read file.
    pub fn read(path: &PathBuf) -> PierResult<String> {
        let file_content = fs::read_to_string(path).context(ConfigRead { path })?;

        Ok(file_content)
    }

    /// Writes the current Config to file.
    pub fn write(&self, path: &PathBuf) -> PierResult<()> {
        let config_string = toml::to_string_pretty(&self).context(TomlSerialize)?;

        fs::write(path, config_string).context(ConfigWrite { path })?;

        Ok(())
    }

    pub fn from(path: &PathBuf) -> PierResult<Self> {
        let config_str = Config::read(path)?;

        let config = toml::from_str(&config_str).context(TomlParse { path })?;

        Ok(config)
    }
}

impl Scripts {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&Script> {
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Script> {
        self.0.get_mut(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn insert(&mut self, key: String, script: Script) -> Option<Script> {
        self.0.insert(key, script)
    }

    pub fn remove(&mut self, key: &str) -> Option<Script> {
        self.0.remove(key)
    }

    pub fn iter(&self) -> Iter<'_, String, Script> {
        self.0.iter()
    }
}

struct ScriptsVisitor {
    marker: PhantomData<fn() -> Scripts>
}

impl ScriptsVisitor {
    fn new() -> Self {
        ScriptsVisitor {
            marker: PhantomData
        }
    }
}
// Gives us an iterating over map entries during deserialization process where we can access entry
// key and modify the script to populate alias.
impl<'de> Visitor<'de> for ScriptsVisitor
{
    type Value = Scripts;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("map is expected")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
    {
        let mut map = Scripts(BTreeMap::new());

        while let Some((key, mut value)) = access.next_entry::<String, Script>()? {
            value.alias = key.clone();
            map.0.insert(key, value);
        }

        Ok(map)
    }
}
// Custom serde deserialization
impl<'de> Deserialize<'de> for Scripts
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ScriptsVisitor::new())
    }
}
