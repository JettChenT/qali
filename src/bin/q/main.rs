use std::process;
use clap::Parser;
use args::Args;
use qali::*;
use db::save;
use anyhow::{Result, anyhow};

pub mod args;

fn main() {
    let arg:Args = Args::parse();
    if let Err(err) = try_main(arg){
        outputils::pnt_err(err);
        process::exit(1);
    }
}

fn try_main(args:Args) -> Result<()>{
    if args.set{
        match args.target {
            Some(t) => save(&args.alias, &t),
            None => Err(anyhow!("Missing target value"))
        }
    }else{
        execute(args.alias, args.target) 
    }
}