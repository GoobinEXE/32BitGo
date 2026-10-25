use crate::error::{CoreError, CoreResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointClass {
    Config,
    Status,
    FirmwareUpdate,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct PayloadGate {
    pub max_len: usize,
    pub allowed_report_ids: &'static [u8],
}

impl Default for PayloadGate {
    fn default() -> Self {
        Self {
            max_len: 64,
            allowed_report_ids: &[],
        }
    }
}

pub fn validate_config_payload(class: EndpointClass, data: &[u8], gate: &PayloadGate) -> CoreResult<()> {
    match class {
        EndpointClass::FirmwareUpdate => return Err(CoreError::FirmwarePathBlocked),
        EndpointClass::Unknown => return Err(CoreError::UnsupportedEndpoint),
        EndpointClass::Config | EndpointClass::Status => {}
    }

    if data.is_empty() {
        return Err(CoreError::ValidationRejected(
            "empty HID payload".to_string(),
        ));
    }

    if data.len() > gate.max_len {
        return Err(CoreError::ValidationRejected(format!(
            "payload length {} exceeds max {}",
            data.len(),
            gate.max_len
        )));
    }

    if !gate.allowed_report_ids.is_empty() {
        let report_id = data[0];
        if !gate.allowed_report_ids.contains(&report_id) {
            return Err(CoreError::ValidationRejected(format!(
                "report id 0x{report_id:02x} is not on the configuration allowlist"
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_firmware_class() {
        let gate = PayloadGate::default();
        let err = validate_config_payload(EndpointClass::FirmwareUpdate, &[0x01], &gate).unwrap_err();
        assert!(matches!(err, CoreError::FirmwarePathBlocked));
    }

    #[test]
    fn rejects_unknown_report_id() {
        let gate = PayloadGate {
            max_len: 64,
            allowed_report_ids: &[0x5A],
        };
        let err = validate_config_payload(EndpointClass::Config, &[0x01, 0x00], &gate).unwrap_err();
        assert!(matches!(err, CoreError::ValidationRejected(_)));
    }

    #[test]
    fn accepts_allowlisted_report() {
        let gate = PayloadGate {
            max_len: 64,
            allowed_report_ids: &[0x5A],
        };
        validate_config_payload(EndpointClass::Config, &[0x5A, 0x00], &gate).unwrap();
    }
}
