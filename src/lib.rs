pub mod error;
mod trash;

use std::path::Path;
use std::path::PathBuf;

pub use error::Result;
pub use error::TrashError;

pub fn trash<P: AsRef<Path>>(path: P) -> Result<()> {
    trash_all(std::iter::once(path))
}

pub fn trash_all<I, P>(paths: I) -> Result<()>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let path_list: Vec<PathBuf> = paths
        .into_iter()
        .map(|path| path.as_ref().to_path_buf())
        .collect();

    if path_list.is_empty() {
        return Err(TrashError::EmptyInput);
    }

    trash::move_to_trash(&path_list)
}

#[cfg(test)]
mod tests {
    use super::trash_all;
    use crate::error::TrashError;

    #[test]
    fn trash_all_returns_empty_input_error_for_empty_iterable() {
        let result = trash_all::<Vec<String>, String>(Vec::new());
        assert!(matches!(result, Err(TrashError::EmptyInput)));
    }
}
