use std::fs::File;
use std::fs;
use std::io::{self, Write};

pub fn get_path(command: &String) -> String{
    format!("{}.qali", command)
}

pub fn save(command: &String, value: &String) -> io::Result<()>{
    let mut file = File::create(get_path(command))?;
    file.write_all(value.as_bytes())?;
    Ok(())
}

pub fn read(command: &String) -> io::Result<String>{
    fs::read_to_string({
        let p = get_path(command);
        println!("Reading from path{}...", p);
        p
    })
}