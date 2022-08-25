use super::Execute;
use anyhow::Context;
use std::process::Command;

pub struct Cmd {
    pub command: String
}

impl Execute for Cmd {
    fn execute(&self, args: Option<&String>) -> anyhow::Result<()> {
        let mut prefs = self.command.split_whitespace();
        let cmd = prefs.next().context("failed to get command")?;
        let mut shell_cmd = Command::new(cmd);
        for arg in prefs {
            shell_cmd.arg(arg);
        }
        shell_cmd.status().context("Failed to execute process.")?;
        Ok(())
    }
}