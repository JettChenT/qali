use std::process;
use args::Args;
use clap::{Parser};
use qali::{db, proc_iores};
use qali::outputils::pnt_err;
pub mod args;

fn main(){
    let arg = Args::parse();
    if let Err(err) = try_main(arg){
        pnt_err(err);
        process::exit(1);
    }
}

fn try_main(args: Args) -> Result<(), String>{
    use args::Commands::*;
     
    match args.command {
        Ls => proc_iores(db::ls()),
        Rm {alias} => db::remove_alias(&alias),
        Set {alias, command} => proc_iores(db::save(&alias, &command))
    }
}