use std::fs::File;
use std::fs;
use std::io::{self, Write, ErrorKind};
use std::path::PathBuf;

use directories::ProjectDirs;
use colored::*;
use std::ffi::OsStr;

pub fn get_dir() -> io::Result<PathBuf>{
    let cur_dir = ProjectDirs::from("", "", "qali").unwrap();
    let d_dir = cur_dir.data_dir();
    fs::create_dir_all(d_dir)?;
    return Ok(d_dir.to_path_buf());
}

pub fn exists(alias: &String) -> bool{
    get_path(alias).exists()
}

pub fn get_path(alias: &String) -> PathBuf{
    let p = get_dir().unwrap();
    let filename =format!("{}.qali", alias);
    p.join(filename)
}

pub fn save(alias: &String, command: &String) -> io::Result<()>{
    if exists(alias) {
        println!("Command named {} already exists!", alias.blue());
        print!("Do you want to override existing command? y/n: ");
        io::stdout().flush()?;
        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm)?;
        if !confirm.starts_with("y") {
            eprintln!("exiting program...");
            return Ok(());
        }
    }
    let path = get_path(alias);
    let mut file = File::create(&path)?;
    file.write_all(command.as_bytes())?;
    eprintln!("✅ Aliased {} to {}", &alias.blue(), &command.blue());
    Ok(())
}

pub fn read(alias: &String) -> io::Result<String>{
    fs::read_to_string(get_path(alias))
}

pub fn ls() -> io::Result<()>{
    let dir = get_dir().unwrap();
    let paths = fs::read_dir(dir)?;

    for path in paths {
        if let Ok(p) = path {
            let p_path = p.path();
            if p_path.extension() == Some(OsStr::new("qali")){
                if let Some(stem) = p_path.file_stem(){
                    let alias = stem.to_string_lossy();
                    let cmd = read(&alias.to_string()).unwrap_or("no content".to_string());
                    println!("{} = {}", alias, cmd.blue());
                }
            }
        }
    }
    Ok(())
}

pub fn remove_alias(alias: &String) -> Result<(), String>{
    match fs::remove_file(get_path(alias)) {
        Ok(_) => {
            println!("🗑 Removed command {}", alias.blue());
            Ok(())
        },
        Err(err) => match err.kind(){
            ErrorKind::NotFound => Err("No such command".to_string()),
            _ => Err(err.to_string())
        }
    }
}