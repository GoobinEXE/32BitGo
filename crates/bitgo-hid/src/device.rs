use std::ffi::CString;

use bitgo_core::{CoreError, CoreResult, HidTransport};

use crate::error::{HidError, HidResult};

pub struct HidDevice {
    device: hidapi::HidDevice,
}

impl HidDevice {
    pub fn open(vid: u16, pid: u16) -> HidResult<Self> {
        let api = hidapi::HidApi::new().map_err(HidError::from)?;
        let device = api.open(vid, pid).map_err(HidError::from)?;
        Ok(Self { device })
    }

    pub fn open_path(path: &str) -> HidResult<Self> {
        let api = hidapi::HidApi::new().map_err(HidError::from)?;
        let c_path = CString::new(path).map_err(|e| HidError::Io(e.to_string()))?;
        let device = api.open_path(&c_path).map_err(HidError::from)?;
        Ok(Self { device })
    }
}

impl HidTransport for HidDevice {
    fn write(&mut self, data: &[u8]) -> CoreResult<usize> {
        self.device
            .write(data)
            .map_err(|e| CoreError::from(HidError::from(e)))
    }

    fn read_timeout(&mut self, buf: &mut [u8], timeout_ms: i32) -> CoreResult<usize> {
        match self.device.read_timeout(buf, timeout_ms) {
            Ok(0) => Err(CoreError::Timeout),
            Ok(n) => Ok(n),
            Err(e) => Err(CoreError::from(HidError::from(e))),
        }
    }

    fn send_feature_report(&mut self, data: &[u8]) -> CoreResult<()> {
        self.device
            .send_feature_report(data)
            .map(|_| ())
            .map_err(|e| CoreError::from(HidError::from(e)))
    }

    fn get_feature_report(&mut self, buf: &mut [u8]) -> CoreResult<usize> {
        self.device
            .get_feature_report(buf)
            .map_err(|e| CoreError::from(HidError::from(e)))
    }
}
