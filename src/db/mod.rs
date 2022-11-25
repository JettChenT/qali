use std::fmt::Display;
use std::str::FromStr;
use std::{fs::File, env};
use std::{fs, clone};
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

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq)]
pub enum StorageMode {
    Global,
    Local
}

impl Display for StorageMode{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageMode::Global => write!(f, "global"),
            StorageMode::Local => write!(f, "local"),
        }
    }
}


pub fn get_dir(m:&StorageMode) -> Result<PathBuf>{
    match m{
        StorageMode::Global => {
            let dirs = ProjectDirs::from("", "", "qali")
                .context("Could not find project directories")?;
            Ok(dirs.data_dir().to_path_buf())
        }
        StorageMode::Local => {
            let mut path = env::current_dir()?;
            path.push(".qali");
            Ok(path)
        }
    }
}

pub fn exists(alias: &String, m:&StorageMode) -> bool{
    get_path(alias, m).exists()
}

pub fn exists_all(alias: &String) -> bool {
    exists(alias, &StorageMode::Local) || exists(alias, &StorageMode::Global)
}

pub fn get_path(alias: &String, m:&StorageMode) -> PathBuf{
    let p = get_dir(m).unwrap();
    let filename =format!("{}.{}", alias, FLIE_EXTENSION);
    p.join(filename)
}

pub fn save(alias: &String, command: &String, m:&StorageMode) -> Result<()>{
    if exists(alias, m) {
        println!("Alias named {} already exists!", alias.blue());
        if !Confirm::with_theme(&ColorfulTheme::default()).with_prompt("Do you want to overwrite it?").interact()? {
            return Ok(());
        }
    }
    let path = get_path(alias, m);
    if let Some(pth) = path.parent(){
        fs::create_dir_all(pth)?;
    }
    let mut file = File::create(&path)?;
    file.write_all(command.as_bytes())?;
    eprintln!("✅ Aliased {} to {}", &alias.blue(), &command.blue());
    Ok(())
}

pub fn read(alias: &String, m:&StorageMode) -> Result<String>{
    fs::read_to_string(get_path(alias, m)).
        with_context(|| format!("Alias {} not found", alias))
}

pub fn read_all(alias: &String) -> Result<String>{
    match read(alias, &StorageMode::Local){
        Ok(loc) => Ok(loc),
        Err(_) => read(alias, &StorageMode::Global)
    }
}


pub fn remove_alias(alias: &String, m:&StorageMode) -> Result<()>{
    match fs::remove_file(get_path(alias, m)) {
        Ok(_) => {
            println!("🗑 Removed command {}", alias.blue());
            Ok(())
        },
        Err(err) => Err(err).with_context(|| format!("Failed to remove {}", alias))
    }
} 
 