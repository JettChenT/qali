use super::Execute;
use anyhow::{Context, anyhow};
use std::{path::PathBuf, str::FromStr, process::Command};

pub struct Shell{
    pub filename: String,
}

impl Execute for Shell {
    fn execute(&self, args: Option<&String>) -> anyhow::Result<()> {
        let file = PathBuf::from_str(&self.filename)?;
        if file.exists() {
            return Err(anyhow!("File not found"));
        }
        let mut shell_cmd = Command::new("sh");
        shell_cmd.arg(file);
        if let Some(arg) = args {
            shell_cmd.arg(arg);
        }
        shell_cmd.status().context("Failed to execute process.")?;
        Ok(())
    }
}