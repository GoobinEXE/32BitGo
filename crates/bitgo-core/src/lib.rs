mod error;
mod identity;
mod plugin;
mod profile;
mod safety;
mod theme;

pub use error::{CoreError, CoreResult};
pub use identity::{DeviceIdentity, DeviceKind, KeyboardEdition, UsbId, EIGHT_BIT_DO_VID};
pub use plugin::{Capabilities, DeviceDriver, DeviceStatus, HidTransport, ProfileSlot};
pub use profile::{KeyMapping, MacroDefinition, Profile, SuperButtonBinding};
pub use safety::{validate_config_payload, EndpointClass, PayloadGate};
pub use theme::{ThemeHint, ThemeId};
