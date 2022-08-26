use colored::Colorize;
use std::fmt::Debug;

pub fn pnt_err<U: ToString+Debug>(err:U, debug: bool){
    if !debug {
        eprintln!("Error: {}", err.to_string().red())
    }else{
        eprintln!("Error: {:?}", err)
    }
}