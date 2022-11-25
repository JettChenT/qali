use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};
use colored::Colorize;

use python::Python;
use cmd::Cmd;
use shell::Shell;
use uri::Uri;
use crate::db::StorageMode;
use crate::suggest;

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

pub fn save_alias(alias: &String, command: &String, m:&StorageMode) -> Result<()> {
    let action = parse_command(command)?;
    let store = action.export()?;
    db::save(alias, &store, m)
}

pub fn suggest_save_alias(command: &String, m:&StorageMode) -> Result<()>{
    let alias: String = {
        let suggestion = suggest::suggest_alias(command);
        match suggestion{
            Ok(s) => {
                println!("{}", "Suggestion found".blue());
                Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter alias:")
                .default(s)
                .interact()?
            },
            Err(_) => {
                Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter alias:")
                .interact()?
            }
        }
    };
    let action = parse_command(command)?;
    let store = action.export()?;
    db::save(&alias, &store, m)
}

pub fn execute_alias(alias: &String, args: Option<&String>) -> Result<()> {
    let command = db::read_all(alias)?;
    let cmd = parse_command(&command)?;
    cmd.execute(args)
}

pub fn select_and_execute_alias() -> Result<()> {
    let alias = db::interface::select()?;
    let args = {
        let inp: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter arguments(if any)")
            .allow_empty(true)
            .interact_text()?;
        if inp.is_empty() {
            None
        } else {
            Some(inp)
        }
    };
    execute_alias(&alias, args.as_ref())
}
