use enum_kinds::EnumKind;
use snafu::Snafu;
use std::path::PathBuf;
use scrawl;

#[derive(Snafu, Debug, EnumKind)]
#[enum_kind(PierErrorKind)]
#[snafu(visibility = "pub(crate)")]
pub enum PierError {
    #[snafu(display("error: Unable to read config from file {}: {} ", path.display(), source))]
    ConfigRead {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("error: Unable to parse toml config from file {}: {}", path.display(), source))]
    TomlParse {
        source: toml::de::Error,
        path: PathBuf,
    },
    #[snafu(display(
        "error: Unable to serialize config: {}. Probably a bug in the code.",
        source
    ))]
    TomlSerialize { source: toml::ser::Error },

    #[snafu(display("error: Unable to write config to {}: {}", path.display(), source))]
    ConfigWrite {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("error: AliasNotFound: No script found by alias {}", alias))]
    AliasNotFound { alias: String },

    #[snafu(display("error: No scripts exist. Would you like to add a new script?"))]
    NoScriptsExists,

    #[snafu(display("error: No $SHELL environment variable: {}", source))]
    NoDefaultShell { source: std::env::VarError },

    #[snafu(display("error: Command execution failed with: {}", source))]
    CommandExec { source: std::io::Error },

    #[snafu(display("error: No default config file found. See help for more info."))]
    NoConfigFile,

    #[snafu(display("error: EditorError: Failed when trying to get input from editor {}", source))]
    EditorError { source: scrawl::error::ScrawlError },

    #[snafu(display("error: Failed when trying to create executable tempfile. {}", source))]
    ExecutableTempFileCreate { source: std::io::Error },
}
