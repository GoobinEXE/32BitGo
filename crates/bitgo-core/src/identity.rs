use serde::{Deserialize, Serialize};

pub const EIGHT_BIT_DO_VID: u16 = 0x2DC8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UsbId {
    pub vid: u16,
    pub pid: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceKind {
    RetroKeyboard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyboardEdition {
    Nes,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceIdentity {
    pub usb: UsbId,
    pub kind: DeviceKind,
    pub edition: Option<KeyboardEdition>,
    pub product_name: String,
    pub manufacturer: String,
    pub path: String,
}

impl DeviceIdentity {
    pub fn is_eight_bit_do(&self) -> bool {
        self.usb.vid == EIGHT_BIT_DO_VID
    }
}
