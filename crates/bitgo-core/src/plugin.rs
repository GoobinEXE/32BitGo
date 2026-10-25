use serde::{Deserialize, Serialize};

use crate::error::CoreResult;
use crate::identity::DeviceIdentity;
use crate::profile::Profile;
use crate::theme::ThemeHint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSlot(pub u8);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceStatus {
    pub battery_percent: Option<u8>,
    pub charging: Option<bool>,
    pub active_profile: Option<ProfileSlot>,
    pub firmware_version_hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capabilities {
    pub remapping: bool,
    pub macros: bool,
    pub super_buttons: bool,
    pub profiles: bool,
    pub battery: bool,
    pub theme_hint: ThemeHint,
}

pub trait HidTransport: Send {
    fn write(&mut self, data: &[u8]) -> CoreResult<usize>;
    fn read_timeout(&mut self, buf: &mut [u8], timeout_ms: i32) -> CoreResult<usize>;
    fn send_feature_report(&mut self, data: &[u8]) -> CoreResult<()>;
    fn get_feature_report(&mut self, buf: &mut [u8]) -> CoreResult<usize>;
}

pub trait DeviceDriver: Send + Sync {
    fn identity(&self) -> &DeviceIdentity;
    fn capabilities(&self) -> Capabilities;
    fn read_status(&mut self, transport: &mut dyn HidTransport) -> CoreResult<DeviceStatus>;
    fn pull_profile(
        &mut self,
        transport: &mut dyn HidTransport,
        slot: ProfileSlot,
    ) -> CoreResult<Profile>;
    fn push_profile(
        &mut self,
        transport: &mut dyn HidTransport,
        profile: &Profile,
    ) -> CoreResult<()>;
}
