use clap::Parser;
/// Qali: Quick Aliaser
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
pub struct InpArg{
    /// The command to execute or set
    #[clap(short, long, value_parser)]
    pub cmd: String,

    /// The target value
    #[clap(short, long, value_parser)]
    pub target: Option<String>,
}