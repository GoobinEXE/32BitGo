use bitgo_core::EIGHT_BIT_DO_VID;

use crate::error::{HidError, HidResult};

#[derive(Debug, Clone)]
pub struct HidDeviceInfo {
    pub vid: u16,
    pub pid: u16,
    pub manufacturer: String,
    pub product: String,
    pub path: String,
    pub usage_page: u16,
    pub usage: u16,
    pub interface_number: i32,
}

pub fn enumerate_all() -> HidResult<Vec<HidDeviceInfo>> {
    let api = hidapi::HidApi::new().map_err(HidError::from)?;
    let mut out = Vec::new();
    for info in api.device_list() {
        out.push(HidDeviceInfo {
            vid: info.vendor_id(),
            pid: info.product_id(),
            manufacturer: info.manufacturer_string().unwrap_or("").to_string(),
            product: info.product_string().unwrap_or("").to_string(),
            path: info.path().to_string_lossy().into_owned(),
            usage_page: info.usage_page(),
            usage: info.usage(),
            interface_number: info.interface_number(),
        });
    }
    Ok(out)
}

pub fn enumerate_eight_bit_do() -> HidResult<Vec<HidDeviceInfo>> {
    Ok(enumerate_all()?
        .into_iter()
        .filter(|d| d.vid == EIGHT_BIT_DO_VID)
        .collect())
}
