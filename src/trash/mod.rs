#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

pub trait TrashHandler {
    fn move_files_to_trash(paths: Vec<String>) -> crate::error::Result<()>;
}

#[cfg(target_os = "macos")]
pub use macos::Trash;
#[cfg(target_os = "windows")]
pub use windows::Trash;
