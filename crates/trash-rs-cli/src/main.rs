mod cli;

use clap::Parser;
use trash::error::TrashError;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", format_cli_error(&err));
        std::process::exit(1);
    }
}

fn run() -> Result<(), TrashError> {
    let cli = cli::Cli::parse();
    trash::trash_all(cli.paths)
}

fn format_cli_error(err: &TrashError) -> String {
    match err {
        TrashError::EmptyInput => "No paths were provided.".to_string(),
        TrashError::PathResolve { input } => format!("Could not resolve path: {input}"),
        TrashError::PathNotFound { path } => format!("Path does not exist: {}", path.display()),
        TrashError::FinderNotRunning => {
            "Finder is not running. Start Finder and try again.".to_string()
        }
        TrashError::DescriptorBuild { detail } => {
            format!("Failed to prepare macOS trash request: {detail}")
        }
        TrashError::AppleEventSend { status } => {
            format!("Failed to send trash request to Finder (status: {status}).")
        }
        TrashError::AppleEventReply { status } => {
            format!("Finder returned an invalid trash response (status: {status}).")
        }
        TrashError::TrashOperation {
            path,
            domain,
            code,
            message,
        } => {
            format!(
                "Failed to move {} to Trash: [{} {}] {}",
                path.display(),
                domain,
                code,
                message
            )
        }
        TrashError::UnsupportedPlatform { os } => {
            format!("This build does not support moving files to Trash on {os}.")
        }
    }
}
