mod cli;
mod error;
mod trash;

use clap::Parser;
use crate::trash::Trash;
use crate::trash::TrashHandler;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:#}", err);
        std::process::exit(1);
    }
}

fn run() -> crate::error::Result<()> {
    let cli = cli::Cli::parse();

    // Attempt to move files to trash and propagate potential errors
    Trash::move_files_to_trash(cli.paths)?;
    Ok(())
}
