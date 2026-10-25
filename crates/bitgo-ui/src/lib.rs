use bitgo_core::ThemeHint;
use bitgo_theme::{resolve_from_hint, ThemeTokens};

pub fn default_theme() -> &'static ThemeTokens {
    resolve_from_hint(ThemeHint::Nes)
}

pub fn phase_status() -> &'static str {
    "bitgo-ui: Phase 2 stub — Iced shell arrives with remapping GUI"
}
