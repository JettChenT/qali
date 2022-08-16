use std::fs::File;
use std::{fs, path};
use std::io::{self, Write};
use std::env;
use std::path::{Path, PathBuf};
use directories::ProjectDirs;

pub fn get_dir() -> io::Result<PathBuf>{
    let cur_dir = ProjectDirs::from("", "", "qali").unwrap();
    let d_dir = cur_dir.data_dir();
    fs::create_dir_all(d_dir)?;
    return Ok(d_dir.to_path_buf());
}

pub fn get_path(command: &String) -> PathBuf{
    let p = get_dir().unwrap();
    let filname =format!("{}.qali", command);
    p.join(filname)
}

pub fn save(command: &String, value: &String) -> io::Result<()>{
    let mut file = File::create(get_path(command))?;
    file.write_all(value.as_bytes())?;
    Ok(())
}

pub fn read(command: &String) -> io::Result<String>{
    fs::read_to_string({
        let p = get_path(command);
        println!("Reading from path{}...", p.display());
        p
    })
}