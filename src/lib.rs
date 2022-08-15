use std::process::Command;
use std::io;

pub mod argparse;
pub mod db;

pub fn execute(qali_cmd: String, target: Option<String>) -> io::Result<()>{
    let pref_str = db::read(&qali_cmd)?;
    let mut prefs = pref_str.split_whitespace();
    let curcmd = prefs.next().unwrap();
    let mut shell_cmd = Command::new(curcmd);
    for arg in prefs{
        shell_cmd.arg(arg);
    }
    println!("Executing command: {:?}", shell_cmd.get_program());
    shell_cmd.status().expect("Failed to execute process.");
    Ok(())
}