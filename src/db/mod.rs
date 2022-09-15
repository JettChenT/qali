use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::ffi::OsStr;

use directories::ProjectDirs;
use colored::*;
use anyhow::{Result, Context};
use dialoguer::{Confirm, theme::ColorfulTheme, Input};

use crate::suggest;

mod tst;
pub mod interface;

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
        if !Confirm::with_theme(&ColorfulTheme::default()).with_prompt("Do you want to overwrite it?").interact()? {
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


pub fn remove_alias(alias: &String) -> Result<()>{
    match fs::remove_file(get_path(alias)) {
        Ok(_) => {
            println!("🗑 Removed command {}", alias.blue());
            Ok(())
        },
        Err(err) => Err(err).with_context(|| format!("Failed to remove {}", alias))
    }
} 
 