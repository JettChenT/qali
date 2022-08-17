use std::process;
use colored::*;
use clap::Parser;
use qali::{run, argparse::InpArg};

pub mod argparse;
pub mod db;


fn main() {
    let args:InpArg = InpArg::parse();
    run(args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err.to_string().red());
        process::exit(1);
    })   
}
