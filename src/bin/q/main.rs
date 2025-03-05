use anyhow::{anyhow, Result};
use args::Args;
use clap::Parser;
use colored::Colorize;
use qali::commands;
use qali::commands::execute_alias;
use qali::db::{self, exists_all, StorageMode};
use qali::outputils;
use std::process;

pub mod args;

fn main() {
    let arg: Args = Args::parse();
    if let Err(err) = try_main(&arg) {
        outputils::pnt_err(err, arg.debug);
        process::exit(1);
    }
}

fn try_main(args: &Args) -> Result<()> {
    ctrlc::set_handler(move || {
        let term = console::Term::stdout();
        let _ = term.show_cursor();
    })?;
    let storage_mode = if args.local {
        StorageMode::Local
    } else {
        StorageMode::Global
    };
    if let Some(alias) = &args.alias {
        if args.set {
            match &args.target {
                Some(t) => commands::save_alias(alias, t, &storage_mode),
                None => commands::suggest_save_alias(alias, &storage_mode),
            }
        } else {
            if let Some(t) = &args.target {
                if !exists_all(alias) {
                    eprintln!("Alias {} does not exist, creating one...", alias);
                    return commands::save_alias(alias, t, &storage_mode);
                }
            }
            if db::exists_all(alias) {
                execute_alias(alias, args.target.as_ref())
            } else {
                eprintln!(
                    "Alias {} not found, creating one... (^C to quit)",
                    alias.blue()
                );
                commands::suggest_save_alias(alias, &storage_mode)
            }
        }
    } else {
        commands::select_and_execute_alias()
    }
}
