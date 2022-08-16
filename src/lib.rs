use std::process::Command;
use std::io;
use colored::*;

pub mod argparse;
pub mod db;

pub fn execute(qali_cmd: String, target: Option<String>) -> io::Result<()>{
    let pref_str = db::read(&qali_cmd)?;
    let mut prefs = pref_str.split_whitespace();
    let mut shell_cmd = Command::new(prefs.next().unwrap());
    for arg in prefs{
        shell_cmd.arg(arg);
    }
    let targetval = target.unwrap_or_default();
    if targetval != ""{
        shell_cmd.arg(&targetval);
    }
    println!("Executing command:");
    println!("{} {}", pref_str.blue(), &targetval);
    println!("{}", "-".repeat(10).blue());
    shell_cmd.status().expect("Failed to execute process.");
    Ok(())
}