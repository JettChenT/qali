use super::QaliCommand;
use anyhow::{Context, anyhow, Result};
use std::{fs, path::PathBuf, str::FromStr, process::Command};
use regex::Regex;

#[derive(Debug)]
pub struct Shell{
    pub filename: String,
}

impl QaliCommand for Shell {
    fn execute(&self, args: Option<&String>) -> Result<()> {
        eprintln!("Running Shell script");
        let file = PathBuf::from_str(&self.filename)?;
        if !file.exists() {
            return Err(anyhow!("File {} not found", self.filename));
        }
        let mut shell_cmd = Command::new("sh");
        shell_cmd.arg(file);
        if let Some(arg) = args {
            shell_cmd.arg(arg);
        }
        shell_cmd.status().context("Failed to execute process.")?;
        Ok(())
    }

    fn is_valid(command: &String) -> bool {
        let re = Regex::new(r"^.*.sh$").unwrap();
        re.is_match(command)
    }

    fn new(command: &String) -> Result<Self> where Self: Sized {
        let file = PathBuf::from_str(command)?;
        if !file.exists() {
            return Err(anyhow!("File {} not found", command));
        }
        Ok(Shell{
            filename: command.to_string(),
        })
    }

    fn export(&self) -> Result<String> {
        let file = PathBuf::from_str(&self.filename)?;
        let fp = fs::canonicalize(file)?;
        Ok(fp.to_string_lossy().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    #[test]
    fn run_shell() -> Result<()> {
        let mut dir = env::current_exe()?;
        dir.pop();
        dir.push("test.sh");
        let program = "echo Hello World";
        // write program to dir
        std::fs::write(dir.to_str().unwrap(), program)?;
        let shell = Shell{
            filename: dir.to_str().unwrap().to_string(),
        };
        shell.execute(None)
    }
}