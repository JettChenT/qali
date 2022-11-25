use std::process;
use clap::Parser;
use args::Args;
use qali::db::exists_all;
use qali::{*, db::exists};
use qali::commands::execute_alias;
use qali::commands;
use anyhow::{Result, anyhow};
use colored::Colorize;

pub mod args;

fn main() {
    let arg:Args = Args::parse();
    if let Err(err) = try_main(&arg){
        outputils::pnt_err(err, arg.debug);
        process::exit(1);
    }
}

fn try_main(args:&Args) -> Result<()>{
    if let Some(alias) = &args.alias{
        if args.set{
            match &args.target {
                Some(t) => commands::save_alias(alias, t, &args.storage_mode),
                None => commands::suggest_save_alias(alias, &args.storage_mode)
            }
        }else{
            if let Some(t) = &args.target{
                if !exists_all(alias){
                    eprintln!("Alias {} does not exist, creating one...", alias);
                    return commands::save_alias(alias, t, &args.storage_mode);
                }
            }
            if db::exists_all(alias){
                execute_alias(alias, args.target.as_ref())
            }else{
                eprintln!("Alias {} not found, creating one... (^C to quit)", alias.blue());
                commands::suggest_save_alias(alias, &args.storage_mode)
            }
        }
    }else{
        commands::select_and_execute_alias()
    }
}