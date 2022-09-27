use std::process;
use clap::Parser;
use args::Args;
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
                Some(t) => commands::save_alias(alias, t),
                None => commands::suggest_save_alias(alias)
            }
        }else{
            if let Some(t) = &args.target{
                if !exists(alias){
                    eprintln!("Alias {} does not exist, creating one...", alias);
                    return commands::save_alias(alias, t);
                }
            }
            if db::exists(alias){
                execute_alias(alias, args.target.as_ref())
            }else{
                eprintln!("Alias {} not found, creating one... (^C to quit)", alias.blue());
                commands::suggest_save_alias(alias)
            }
        }
    }else{
        commands::select_and_execute_alias()
    }
}