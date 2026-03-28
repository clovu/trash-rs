use std::path::PathBuf;

use crate::error::Result;
use crate::error::TrashError;

pub(crate) fn move_files_to_trash(paths: &[PathBuf]) -> Result<()> {
    use objc2_foundation::NSFileManager;
    use objc2_foundation::NSString;
    use objc2_foundation::NSURL;

    let file_manager = NSFileManager::defaultManager();

    for path in paths {
        let ns_path = NSString::from_str(&path.to_string_lossy());
        let file_url = NSURL::fileURLWithPath(&ns_path);

        if let Err(error) = file_manager.trashItemAtURL_resultingItemURL_error(&file_url, None) {
            return Err(TrashError::TrashOperation {
                path: path.clone(),
                domain: error.domain().to_string(),
                code: error.code() as isize,
                message: error.localizedDescription().to_string(),
            });
        }
    }

    Ok(())
}
