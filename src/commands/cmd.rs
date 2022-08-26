use super::QaliCommand;
use anyhow::Context;
use std::process::Command;

#[derive(Debug)]
pub struct Cmd {
    pub command: String
}

impl QaliCommand for Cmd {
    fn execute(&self, args: Option<&String>) -> anyhow::Result<()> {
        println!("Executing command");
        let mut prefs = self.command.split_whitespace();
        let cmd = prefs.next().context("failed to get command")?;
        let mut shell_cmd = Command::new(cmd);
        for arg in prefs {
            shell_cmd.arg(arg);
        }
        if let Some(arg) = args {
            shell_cmd.arg(arg);
        }
        shell_cmd.status().context("Failed to execute process.")?;
        Ok(())
    }

    fn is_valid(_command: &String) -> bool where Self: Sized {
        true
    }

    fn new(command: &String) -> anyhow::Result<Self> where Self: Sized {
        Ok(Cmd{
            command: command.to_string(),
        })
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn run_cmd() -> anyhow::Result<()>{
        let cmd = Cmd{
            command: "echo Hello World".to_string(),
        };
        cmd.execute(None)
    }
}