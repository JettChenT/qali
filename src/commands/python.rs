use super::QaliCommand;
use anyhow::{Result, Context, anyhow};
use colored::Colorize;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::fs;
use regex::Regex;

#[derive(Debug)]
pub struct Python{
    pub env: String,
    pub filename: String,
}



impl QaliCommand for Python{
    fn execute(&self, args: Option<&String>) -> Result<()>{
        eprintln!("{}","Running Python script".dimmed());
        let file = PathBuf::from_str(&self.filename)?;
        if !file.exists(){
            return Err(anyhow!("File {} not found", self.filename));
        }
        let prog = if cfg!(target_os = "windows"){
            "py"
        } else {
            "python3"
        };
        let mut shell_cmd = Command::new(prog);
        shell_cmd.arg(file);
        if let Some(arg) = args{
            shell_cmd.arg(arg);
        }
        shell_cmd.status().context("Failed to execute process.")?;
        Ok(())
    }

    fn is_valid(command: &String) -> bool {
        let re = Regex::new(r"^.*\.py$").unwrap();
        re.is_match(command)
    }

    fn new(command: &String) -> Result<Self> {
        let file = PathBuf::from_str(command)?;
        if !file.exists(){
            return Err(anyhow!("Python file {} not found", command));
        } 
        Ok(Python{
            env: "python".to_string(),
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
mod tests{
    use super::*;
    use std::env;

    #[test]
    fn run_file() -> Result<()>{
        let mut dir = env::current_exe()?;
        dir.pop();
        dir.push("test.py");

        let program = "for i in range(3):print('Hello World')";
        // write program to dir
        std::fs::write(dir.to_str().unwrap(), program)?;

        let python = Python{
            env: "".to_string(),
            filename: dir.to_string_lossy().to_string(),
        };
        python.execute(None)
    }
}