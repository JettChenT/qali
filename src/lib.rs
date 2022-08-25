use std::process::Command;
use colored::*;
use anyhow::{Result, Context};

pub mod db;
pub mod outputils;
pub mod commands;

pub fn execute(qali_cmd: String, target: Option<String>) -> Result<()>{
    let pref_str = db::read(&qali_cmd)?;
    let mut prefs = pref_str.split_whitespace();
    let mut shell_cmd = Command::new(prefs.next().unwrap());
    for arg in prefs{
        shell_cmd.arg(arg);
    }
    let targetval = target.unwrap_or_default();
    if !targetval.is_empty(){
        shell_cmd.arg(&targetval);
    }
    eprintln!("{} {}", pref_str.blue(), &targetval);
    eprintln!("{}", "-".repeat(10).blue());
    shell_cmd.status().context("Failed to execute process.")?;
    Ok(())
}