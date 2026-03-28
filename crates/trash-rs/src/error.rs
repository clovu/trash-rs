use std::fmt;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, TrashError>;

#[derive(Debug, PartialEq, Eq)]
pub enum TrashError {
    EmptyInput,
    PathResolve {
        input: String,
    },
    PathNotFound {
        path: PathBuf,
    },
    FinderNotRunning,
    DescriptorBuild {
        detail: String,
    },
    AppleEventSend {
        status: i32,
    },
    AppleEventReply {
        status: i32,
    },
    TrashOperation {
        path: PathBuf,
        domain: String,
        code: isize,
        message: String,
    },
    UnsupportedPlatform {
        os: &'static str,
    },
}

impl fmt::Display for TrashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrashError::EmptyInput => write!(f, "no paths were provided"),
            TrashError::PathResolve { input } => write!(f, "failed to resolve path: {input}"),
            TrashError::PathNotFound { path } => {
                write!(f, "path does not exist: {}", path.display())
            }
            TrashError::FinderNotRunning => write!(f, "Finder is not running"),
            TrashError::DescriptorBuild { detail } => {
                write!(f, "failed to build Apple Event descriptor: {detail}")
            }
            TrashError::AppleEventSend { status } => {
                write!(f, "failed to send Apple Event to Finder: status {status}")
            }
            TrashError::AppleEventReply { status } => {
                write!(f, "failed to parse Apple Event reply: status {status}")
            }
            TrashError::TrashOperation {
                path,
                domain,
                code,
                message,
            } => {
                write!(
                    f,
                    "failed to move {} to Trash: [{} {}] {}",
                    path.display(),
                    domain,
                    code,
                    message
                )
            }
            TrashError::UnsupportedPlatform { os } => {
                write!(f, "moving files to Trash is not supported on {os}")
            }
        }
    }
}

impl std::error::Error for TrashError {}
