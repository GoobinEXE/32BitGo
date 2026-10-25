use bitgo_core::{ThemeHint, ThemeId};

use crate::nes::NES;
use crate::tokens::ThemeTokens;

pub struct ThemeEngine;

impl ThemeEngine {
    pub fn resolve(id: ThemeId) -> &'static ThemeTokens {
        match id {
            ThemeId::Nes => &NES,
        }
    }

    pub fn resolve_hint(hint: ThemeHint) -> &'static ThemeTokens {
        Self::resolve(ThemeId::from(hint))
    }
}

pub fn resolve(id: ThemeId) -> &'static ThemeTokens {
    ThemeEngine::resolve(id)
}

pub fn resolve_from_hint(hint: ThemeHint) -> &'static ThemeTokens {
    ThemeEngine::resolve_hint(hint)
}
