use thiserror::Error;

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("device disconnected")]
    Disconnected,
    #[error("hid I/O timeout")]
    Timeout,
    #[error("protocol error: {0}")]
    Protocol(String),
    #[error("payload validation rejected: {0}")]
    ValidationRejected(String),
    #[error("endpoint not allowed for configuration I/O")]
    UnsupportedEndpoint,
    #[error("firmware / DFU / OTA endpoints are blocked in this release")]
    FirmwarePathBlocked,
    #[error("permission denied opening HID device")]
    PermissionDenied,
    #[error("device or edition not supported yet")]
    UnsupportedDevice,
    #[error("transport error: {0}")]
    Transport(String),
}
