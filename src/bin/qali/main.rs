use std::process;
use args::Args;
use clap::{Parser};
use anyhow::Result;
use qali::db;
use qali::outputils::pnt_err;
use qali::commands;
pub mod args;

fn main(){
    let arg = Args::parse();
    if let Err(err) = try_main(&arg){
        pnt_err(err, arg.debug);
        process::exit(1);
    }
}

fn try_main(args: &Args) -> Result<()>{
    use args::Commands::*;
     
    match &args.command {
        List => db::interface::ls(),
        Remove {alias} => db::remove_alias(alias),
        Set {alias, command} => commands::save_alias(alias, command),
        Select => commands::select_and_execute_alias()
    }
}