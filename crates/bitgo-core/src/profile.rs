use serde::{Deserialize, Serialize};

use crate::plugin::ProfileSlot;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyMapping {
    pub source_usage: u16,
    pub target_usage: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroDefinition {
    pub id: u8,
    pub name: String,
    pub steps: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuperButtonBinding {
    pub button_index: u8,
    pub action_id: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    pub slot: ProfileSlot,
    pub name: String,
    pub keys: Vec<KeyMapping>,
    pub macros: Vec<MacroDefinition>,
    pub super_buttons: Vec<SuperButtonBinding>,
}
