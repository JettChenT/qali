use anyhow::Result;
use args::Args;
use clap::Parser;
use qali::commands;
use qali::db;
use qali::outputils::pnt_err;
use std::process;
pub mod args;

fn main() {
    let arg = Args::parse();
    if let Err(err) = try_main(&arg) {
        pnt_err(err, arg.debug);
        process::exit(1);
    }
}

fn try_main(args: &Args) -> Result<()> {
    use args::Commands::*;
    ctrlc::set_handler(move || {
        let term = console::Term::stdout();
        let _ = term.show_cursor();
    })?;

    match &args.command {
        List => db::interface::ls(),
        Remove { alias } => db::remove_alias(alias, &args.storage_mode),
        Set { alias, command } => commands::save_alias(alias, command, &args.storage_mode),
        Select => commands::select_and_execute_alias(),
        Add { command } => commands::suggest_save_alias(command, &args.storage_mode),
    }
}
