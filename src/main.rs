use clap::Parser;
use argparse::InpArg;
use qali::execute;

pub mod argparse;
pub mod db;

fn main() {
    let args:InpArg = InpArg::parse();
    println!("Current command: {}", args.cmd);
    match args.target {
        Some(target) => {
            db::save(&args.cmd,&target).unwrap();
        },
        None => {
            execute(args.cmd, args.target).unwrap();
        }
    }
}
