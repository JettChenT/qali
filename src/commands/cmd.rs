use super::QaliCommand;
use anyhow::Context;
use colored::Colorize;
use std::process::Command;

#[derive(Debug)]
pub struct Cmd {
    pub command: String
}

impl QaliCommand for Cmd {
    fn execute(&self, args: Option<&String>) -> anyhow::Result<()> {
        println!("$ {} {}", &self.command.blue(), args.unwrap_or(&"".to_string()));
        let cmd = match args {
            Some(args) => format!("{} {}", &self.command, args),
            None => self.command.clone()
        };
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", cmd.as_str()])
                .status()
                .context("Failed to execute process.")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(cmd.as_str())
                .status()
                .context("Failed to execute process.")
        }?;
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

    fn export(&self) -> anyhow::Result<String> {
        Ok(self.command.clone())
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