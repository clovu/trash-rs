use super::TrashHandler;

pub struct Trash;

impl TrashHandler for Trash {
    fn move_files_to_trash(_paths: Vec<String>) -> crate::error::Result<()> {
        anyhow::bail!("move to trash is not implemented on Windows in this build");
    }
}
