use std::process;
use args::Args;
use clap::{Parser};
use anyhow::Result;
use qali::db;
use qali::outputils::pnt_err;
pub mod args;

fn main(){
    let arg = Args::parse();
    if let Err(err) = try_main(arg){
        pnt_err(err);
        process::exit(1);
    }
}

fn try_main(args: Args) -> Result<()>{
    use args::Commands::*;
     
    match args.command {
        Ls => db::ls(),
        Rm {alias} => db::remove_alias(&alias),
        Set {alias, command} => db::save(&alias, &command)
    }
}