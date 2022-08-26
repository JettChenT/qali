use std::process;
use clap::Parser;
use args::Args;
use qali::*;
use qali::commands::execute_alias;
use db::save;
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
            Some(t) => save(&args.alias, t),
            None => Err(anyhow!("Missing target value"))
        }
    }else{
        execute_alias(&args.alias, args.target.as_ref())
    }
}