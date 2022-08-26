use anyhow::Result;
use python::Python;
use cmd::Cmd;
use shell::Shell;
use uri::Uri;

use crate::db;

pub mod python;
pub mod cmd;
pub mod shell;
pub mod uri;

pub trait QaliCommand {
    fn execute(&self, args: Option<&String>) -> Result<()>;
    fn new(command: &String) -> Result<Self> where Self: Sized;
    fn is_valid(command: &String) -> bool where Self: Sized;
    fn export(&self) -> Result<String>;
}

pub fn parse_command(command: &String) -> Result<Box<dyn QaliCommand>> {
    if Python::is_valid(command) {
        Ok(Box::new(Python::new(command)?))
    } else if Shell::is_valid(command) {
        Ok(Box::new(Shell::new(command)?))
    } else if Uri::is_valid(command) {
        Ok(Box::new(Uri::new(command)?))
    } else if Cmd::is_valid(command) {
        Ok(Box::new(Cmd::new(command)?))
    } else {
        Err(anyhow::anyhow!("Command {} is not valid", command))
    }
}

pub fn save_alias(alias: &String, command: &String) -> Result<()> {
    let action = parse_command(command)?;
    let store = action.export()?;
    db::save(alias, &store)
}

pub fn execute_alias(alias: &String, args: Option<&String>) -> Result<()> {
    let command = db::read(alias)?;
    let cmd = parse_command(&command)?;
    cmd.execute(args)
}
