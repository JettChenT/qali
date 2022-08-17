use std::process::Command;
use std::io;
use std::io::ErrorKind;
use colored::*;
use argparse::InpArg;
use db::save;

pub mod argparse;
pub mod db;

pub fn run(args:InpArg) -> Result<(), String>{
    if args.set{
        match args.target {
            Some(t) => {
                match save(&args.cmd, &t){
                    Ok(_) => Ok(()), 
                    Err(e) => {
                        Err(e.to_string())
                    },
                }
            }
            None => Err("Missing target value".to_string())
        }
    }else{
        match execute(args.cmd, args.target) {
            Ok(_) => Ok(()),
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => {
                        Err("Command file not found.".to_string())
                    }
                    _ => {
                        Err(e.to_string())
                    }
                }
            }
        }
    }
}


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
    eprintln!("Executing command:");
    eprintln!("{} {}", pref_str.blue(), &targetval);
    eprintln!("{}", "-".repeat(10).blue());
    shell_cmd.status().expect("Failed to execute process.");
    Ok(())
}