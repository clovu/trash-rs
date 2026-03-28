use std::path::PathBuf;

use crate::error::Result;
use crate::error::TrashError;

pub(crate) fn move_files_to_trash(_paths: &[PathBuf]) -> Result<()> {
    Err(TrashError::UnsupportedPlatform {
        os: std::env::consts::OS,
    })
}
