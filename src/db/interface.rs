use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::vec;
use tabled::{
    settings::{object::Columns, Format, Modify, Style},
    Table, Tabled,
};

use crate::db::*;
use anyhow::{anyhow, Result};

#[derive(Tabled)]
pub struct QEntry {
    pub alias: String,
    pub command: String,
    pub mode: StorageMode,
}

impl ToString for QEntry {
    fn to_string(&self) -> String {
        format!("{}({}) : {}", self.alias, self.mode, self.command)
    }
}

fn get_mode_entries(m: &StorageMode) -> Result<Vec<QEntry>> {
    let dir = get_dir(m)?;
    let paths = fs::read_dir(dir)?;
    let mut entries: Vec<QEntry> = vec![];
    for p in paths.flatten() {
        let p_path = p.path();
        if p_path.extension() == Some(OsStr::new(FLIE_EXTENSION)) {
            if let Some(stem) = p_path.file_stem() {
                let alias = stem.to_string_lossy();
                let cmd = read(&alias.to_string(), m).unwrap_or_else(|_| "no content".to_string());
                entries.push(QEntry {
                    alias: alias.to_string(),
                    command: cmd,
                    mode: m.clone(),
                });
            }
        }
    }
    // sort entries
    entries.sort_by(|a, b| a.alias.cmp(&b.alias));
    Ok(entries)
}

pub fn get_entries() -> Result<Vec<QEntry>> {
    // combined
    let mut entries: Vec<QEntry> = vec![];
    if let Ok(g) = get_mode_entries(&StorageMode::Global) {
        entries.extend(g);
    }
    if let Ok(l) = get_mode_entries(&StorageMode::Local) {
        entries.extend(l);
    }
    Ok(entries)
}

pub fn ls() -> Result<()> {
    let entries = get_entries()?;
    let table = Table::new(&entries)
        .with(Style::psql())
        .with(Modify::new(Columns::single(0)).with(Format::content(|s| s.blue().to_string())))
        .to_string();
    // println!("Alias directory: {}\n", get_dir()?.to_string_lossy());
    println!("{}", table);
    Ok(())
}

pub fn select() -> Result<String> {
    let entries = get_entries()?;
    let selected = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an alias (press esc to exit)")
        .items(&entries)
        .default(0)
        .interact_opt()?;
    if let Some(i) = selected {
        Ok(entries[i].alias.clone())
    } else {
        Err(anyhow!("No alias selected."))
    }
}
