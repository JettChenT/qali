use std::process;
use clap::Parser;
use args::Args;
use qali::{*, db::exists};
use qali::commands::execute_alias;
use qali::commands;
use anyhow::{Result, anyhow};

pub mod args;

fn main() {
    let arg:Args = Args::parse();
    if let Err(err) = try_main(&arg){
        outputils::pnt_err(err, arg.debug);
        process::exit(1);
    }
}

fn try_main(args:&Args) -> Result<()>{
    if args.set{
        match &args.target {
            Some(t) => commands::save_alias(&args.alias, t),
            None => Err(anyhow!("Missing target value"))
        }
    }else{
        if let Some(t) = &args.target{
            if !exists(&args.alias){
                eprintln!("Alias {} does not exist, creating one...", args.alias);
                return commands::save_alias(&args.alias, t);
            }
        }
        execute_alias(&args.alias, args.target.as_ref())
    }
}