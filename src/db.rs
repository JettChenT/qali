use std::fs::File;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use directories::ProjectDirs;
use colored::*;

pub fn get_dir() -> io::Result<PathBuf>{
    let cur_dir = ProjectDirs::from("", "", "qali").unwrap();
    let d_dir = cur_dir.data_dir();
    fs::create_dir_all(d_dir)?;
    return Ok(d_dir.to_path_buf());
}

pub fn exists(command: &String) -> bool{
    get_path(command).exists()
}

pub fn get_path(command: &String) -> PathBuf{
    let p = get_dir().unwrap();
    let filname =format!("{}.qali", command);
    p.join(filname)
}

pub fn save(command: &String, value: &String) -> io::Result<()>{
    let path = get_path(command);
    let mut file = File::create(&path)?;
    file.write_all(value.as_bytes())?;
    Ok(())
}

pub fn read(command: &String) -> io::Result<String>{
    fs::read_to_string({
        let p = get_path(command);
        p
    })
}