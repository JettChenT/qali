use colored::Colorize;

pub fn pnt_err<U: ToString>(err:U){
    eprintln!("Error: {}", err.to_string().red())
}