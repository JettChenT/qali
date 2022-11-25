use clap::{Parser, Subcommand};
use crate::db::StorageMode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,

    /// Debug mode
    #[arg(short, long, action)]
    pub debug: bool,

    /// Storage mode
    #[arg(short='m', long, action, default_value_t=StorageMode::Global)]
    pub storage_mode: StorageMode,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all existing commands
    List,
    /// Remove an alias
    Remove{
        /// The alias
        #[clap(value_parser)]
        alias: String
    },
    /// Set an alias to a command
    Set{
        #[clap(value_parser)]
        alias: String,
        #[clap(value_parser)]
        command: String
    },
    /// Select and execute an alias
    Select,
    /// Set an alias by suggestion
    Add{
        #[clap(value_parser)]
        command: String
    }
}