use clap::Parser;
/// Qali: Quick Aliaser
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
pub struct InpArg{
    /// The command to execute or set
    #[clap(value_parser)]
    pub cmd: String,

    /// The target value
    #[clap(value_parser)]
    pub target: Option<String>,

    /// Flag set
    #[clap(short, long, action)]
    pub set: bool,
}