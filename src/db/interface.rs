use std::vec;
use tabled::{Table, Style, Tabled, Modify, object::Columns, Format};
use colored::Colorize;
use dialoguer::{
    FuzzySelect,
    theme::ColorfulTheme
};

use crate::db::*;

#[derive(Tabled)]
struct QEntry{
    alias: String,
    command: String,
}

impl ToString for QEntry{
    fn to_string(&self) -> String{
        format!("{} : {}", self.alias, self.command)
    }
}

fn get_entries() -> Result<Vec<QEntry>>{
    let dir = get_dir()?;
    let paths = fs::read_dir(dir)?;
    let mut entries: Vec<QEntry> = vec![];
    for p in paths.flatten() {
        let p_path = p.path();
        if p_path.extension() == Some(OsStr::new(FLIE_EXTENSION)){
            if let Some(stem) = p_path.file_stem(){
                let alias = stem.to_string_lossy();
                let cmd = read(&alias.to_string()).unwrap_or_else(|_| "no content".to_string());
                entries.push(QEntry{alias: alias.to_string(), command: cmd});
            }
        }
    }
    // sort entries
    entries.sort_by(|a, b| a.alias.cmp(&b.alias));
    Ok(entries)
}

pub fn ls() -> Result<()>{
    let entries = get_entries()?;
    let table = Table::new(&entries)
        .with(Style::psql())
        .with(Modify::new(Columns::single(0))
        .with(Format::new(|s| s.blue().to_string())))
        .to_string();
    println!("{}", table);
    Ok(())
}

pub fn select() -> Result<String>{
    let entries = get_entries()?;
    let selected = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an alias")
        .items(&entries)
        .default(0)
        .interact()?;
    Ok(entries[selected].alias.clone())
}