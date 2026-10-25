use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThemeId {
    Nes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThemeHint {
    Nes,
}

impl From<ThemeHint> for ThemeId {
    fn from(hint: ThemeHint) -> Self {
        match hint {
            ThemeHint::Nes => ThemeId::Nes,
        }
    }
}
