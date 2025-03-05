use crate::db::StorageMode;
use clap::{arg, Parser};

/// QALI: Quick Aliaser
/// q: execute and set commands ergonomically
///
/// # Commands:
/// `q`: interactive mode with fuzzy matching
/// `q <alias> [args]`: execute a command if it alrady exists, otherwise set it
/// `q -s <alias> <command>`: set an alias
///
/// # Examples:
/// `q -s gs "git status"`: alias gs to "git status"
/// `q gs`: execute gs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    /// The alias to execute or set
    #[arg(value_parser)]
    pub alias: Option<String>,

    /// The target value
    #[arg(value_parser)]
    pub target: Option<String>,

    /// Flag set
    #[arg(short, long, action)]
    pub set: bool,

    /// Flag debug
    #[arg(short, long, action)]
    pub debug: bool,

    #[arg(short = 'l', long, action)]
    pub local: bool,
}
