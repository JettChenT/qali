use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all existing commands
    Ls,
    /// Remove an alias
    Rm{
        #[clap(value_parser)]
        alias: String
    },
    /// Set an alias to a command
    Set{
        #[clap(value_parser)]
        alias: String,
        #[clap(value_parser)]
        command: String
    }
}