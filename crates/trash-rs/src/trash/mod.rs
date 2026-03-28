#[cfg(target_os = "macos")]
mod macos;
mod path;
#[cfg(target_os = "windows")]
mod windows;

use std::path::PathBuf;

use crate::error::Result;

pub(crate) fn move_to_trash(paths: &[PathBuf]) -> Result<()> {
    let resolved_paths = path::resolve_and_validate_paths(paths)?;

    #[cfg(target_os = "macos")]
    {
        macos::move_files_to_trash(&resolved_paths)
    }

    #[cfg(target_os = "windows")]
    {
        windows::move_files_to_trash(&resolved_paths)
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Err(crate::error::TrashError::UnsupportedPlatform {
            os: std::env::consts::OS,
        })
    }
}
