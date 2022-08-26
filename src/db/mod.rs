use std::fs::File;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::ffi::OsStr;

use directories::ProjectDirs;
use colored::*;
use anyhow::{Result, Context};

mod tst;

const FLIE_EXTENSION: &str = "qali";

pub fn get_dir() -> Result<PathBuf>{
    let cur_dir = ProjectDirs::from("", "", "qali").unwrap();
    let d_dir = cur_dir.data_dir();
    fs::create_dir_all(d_dir)?;
    Ok(d_dir.to_path_buf())
}

pub fn exists(alias: &String) -> bool{
    get_path(alias).exists()
}

pub fn get_path(alias: &String) -> PathBuf{
    let p = get_dir().unwrap();
    let filename =format!("{}.{}", alias, FLIE_EXTENSION);
    p.join(filename)
}

pub fn save(alias: &String, command: &String) -> Result<()>{
    if exists(alias) {
        println!("Alias named {} already exists!", alias.blue());
        print!("Do you want to override existing command? y/n: ");
        io::stdout().flush()?;
        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm)?;
        if !confirm.starts_with('y') {
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

pub fn read(alias: &String) -> Result<String>{
    fs::read_to_string(get_path(alias)).
        with_context(|| format!("Alias {} not found", alias))
}

pub fn ls() -> Result<()>{
    let dir = get_dir()?;
    println!("Directory: {}", &dir.to_string_lossy());

    let paths = fs::read_dir(dir)?;


    for p in paths.flatten() {
        let p_path = p.path();
        if p_path.extension() == Some(OsStr::new(FLIE_EXTENSION)){
            if let Some(stem) = p_path.file_stem(){
                let alias = stem.to_string_lossy();
                let cmd = read(&alias.to_string()).unwrap_or_else(|_| "no content".to_string());
                println!("{} = {}", alias, cmd.blue());
            }
        }
    }
    Ok(())
}

pub fn remove_alias(alias: &String) -> Result<()>{
    match fs::remove_file(get_path(alias)) {
        Ok(_) => {
            println!("🗑 Removed command {}", alias.blue());
            Ok(())
        },
        Err(err) => Err(err).with_context(|| format!("Failed to remove {}", alias))
    }
} 
 