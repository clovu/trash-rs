use path_absolutize::Absolutize;
use std::path::PathBuf;

use crate::error::Result;
use crate::error::TrashError;

pub(crate) fn resolve_and_validate_paths(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut resolved_paths = Vec::with_capacity(paths.len());

    for file_path in paths {
        let input = file_path.to_string_lossy().to_string();
        let absolute_path = file_path
            .absolutize()
            .map_err(|_| TrashError::PathResolve {
                input: input.clone(),
            })?
            .into_owned();

        if !absolute_path.exists() {
            return Err(TrashError::PathNotFound {
                path: absolute_path,
            });
        }

        resolved_paths.push(absolute_path);
    }

    Ok(resolved_paths)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::resolve_and_validate_paths;
    use crate::error::TrashError;

    #[test]
    fn resolve_and_validate_paths_returns_empty_for_empty_input() {
        let resolved = resolve_and_validate_paths(&[]).expect("should resolve empty input");
        assert!(resolved.is_empty());
    }

    #[test]
    fn resolve_and_validate_paths_returns_error_for_missing_path() {
        let missing_path = PathBuf::from(format!(
            "/tmp/trash-rs-missing-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock should be monotonic")
                .as_nanos()
        ));

        let result = resolve_and_validate_paths(&[missing_path]);
        assert!(matches!(result, Err(TrashError::PathNotFound { .. })));
    }
}
