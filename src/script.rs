use super::error::*;
use super::Result;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Output, Stdio};
use tempfile;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
    pub alias: String,
    pub command: String,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl Script {
    pub fn has_shebang(&self) -> bool {
        match self.command.lines().nth(0) {
            Some(line) => line.starts_with("#!"),
            None => false,
        }
    }
    pub fn display_command(&self, display_full: bool, width: usize) -> &str {
        match display_full {
            true => &self.command,
            false => match &self.command.lines().nth(0) {
                Some(line) => match line.chars().count() {
                    c if c < width => line,
                    c if c > width => &line[0..width],
                    _ => "",
                },
                None => &self.command,
            },
        }
    }
    /// Runs the script inline using something like sh -c "<script>" or python -c "<script."...
    pub fn run_with_cli_interpreter(&self, interpreter: &Vec<String>, args: Vec<String>) -> Result<Output> {
        // First item in interpreter is the binary
        let cmd = Command::new(&interpreter[0])
            // The following items after the binary is any commandline args that are necessary.
            .args(&interpreter[1..])
            .arg(&self.command)
	    .arg(&self.alias)
	    .args(&args)
            .stderr(Stdio::piped())
            .spawn()
            .context(CommandExec)?
            .wait_with_output()
            .context(CommandExec)?;

        Ok(cmd)
    }

    /// First creates a temporary file and then executes the file before removing it.
    pub fn run_with_shebang(&self, args: Vec<String>) -> Result<Output> {
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
	    .args(&args)
            .spawn()
            .context(CommandExec)?
            .wait_with_output()
            .context(CommandExec)?;

        Ok(cmd)
    }
}
