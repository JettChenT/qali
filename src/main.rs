use std::io::ErrorKind;
use colored::*;
use clap::Parser;
use argparse::InpArg;
use qali::{execute, db::{exists, save}};

pub mod argparse;
pub mod db;

fn run(args:InpArg){
    match execute(args.cmd, args.target) {
        Ok(()) => {},
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    println!("{}", "Command not found!".red());
                }
                other => {
                    panic!("Error: {:?}", other);
                }
            }
        }
    }
}

fn main() {
    let args:InpArg = InpArg::parse();
    if exists(&args.cmd){
        run(args);
    }else{
        match args.target {
            Some(t) => {save(&args.cmd, &t).unwrap();}
            None => {println!("{}", "File does not exist!".red())}
        }
    }
}
