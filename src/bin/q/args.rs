use clap::Parser;
/// Qali: Quick Aliaser
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
pub struct Args{
    /// The alias to execute or set
    #[clap(value_parser)]
    pub alias: String,

    /// The target value
    #[clap(value_parser)]
    pub target: Option<String>,

    /// Flag set
    #[clap(short, long, action)]
    pub set: bool,

    /// Flag debug
    #[clap(short, long, action)]
    pub debug: bool,
}
