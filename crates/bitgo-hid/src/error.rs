use bitgo_core::CoreError;
use thiserror::Error;

pub type HidResult<T> = Result<T, HidError>;

#[derive(Debug, Error)]
pub enum HidError {
    #[error("hidapi error: {0}")]
    Api(String),
    #[error("device not found")]
    NotFound,
    #[error("permission denied")]
    PermissionDenied,
    #[error("disconnected")]
    Disconnected,
    #[error("timeout")]
    Timeout,
    #[error("I/O: {0}")]
    Io(String),
}

impl From<HidError> for CoreError {
    fn from(value: HidError) -> Self {
        match value {
            HidError::Disconnected => CoreError::Disconnected,
            HidError::Timeout => CoreError::Timeout,
            HidError::PermissionDenied => CoreError::PermissionDenied,
            other => CoreError::Transport(other.to_string()),
        }
    }
}

impl From<hidapi::HidError> for HidError {
    fn from(value: hidapi::HidError) -> Self {
        let msg = value.to_string();
        let lower = msg.to_ascii_lowercase();
        if lower.contains("permission") || lower.contains("access") {
            HidError::PermissionDenied
        } else if lower.contains("timeout") {
            HidError::Timeout
        } else {
            HidError::Api(msg)
        }
    }
}
