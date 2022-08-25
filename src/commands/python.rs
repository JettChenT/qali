use super::Execute;
use anyhow::{Result, Context, anyhow};
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

pub struct Python{
    pub env: String,
    pub filename: String,
}

impl Execute for Python{
    fn execute(&self, args: Option<&String>) -> Result<()>{
        let file = PathBuf::from_str(&self.filename)?;
        if file.exists(){
            return Err(anyhow!("File not found"));
        }
        let mut shell_cmd = Command::new("python");
        shell_cmd.arg(file);
        if let Some(arg) = args{
            shell_cmd.arg(arg);
        }
        shell_cmd.status().context("Failed to execute process.")?;
        Ok(())
    }
}