use std::process;
use std::io::ErrorKind;
use clap::Parser;
use args::Args;
use qali::*;
use db::save;

pub mod args;

fn main() {
    let arg:Args = Args::parse();
    if let Err(err) = try_main(arg){
        outputils::pnt_err(err);
        process::exit(1);
    }
}

fn try_main(args:Args) -> Result<(), String>{
    if args.set{
        match args.target {
            Some(t) => {
                match save(&args.alias, &t){
                    Ok(_) => Ok(()), 
                    Err(e) => {
                        Err(e.to_string())
                    },
                }
            }
            None => Err("Missing target value".to_string())
        }
    }else{
        match execute(args.alias, args.target) {
            Ok(_) => Ok(()),
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => {
                        Err("Command file not found.".to_string())
                    }
                    _ => {
                        Err(e.to_string())
                    }
                }
            }
        }
    }
}