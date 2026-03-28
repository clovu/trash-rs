#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Files or directories to move to Trash
    #[arg(required = true, num_args = 1..)]
    pub paths: Vec<String>,
}
