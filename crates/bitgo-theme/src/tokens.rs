use bitgo_core::ThemeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ColorTokens {
    pub background: u32,
    pub surface: u32,
    pub surface_alt: u32,
    pub accent: u32,
    pub accent_dim: u32,
    pub text: u32,
    pub text_muted: u32,
    pub danger: u32,
    pub success: u32,
    pub border: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TypeTokens {
    pub display_px: u16,
    pub title_px: u16,
    pub body_px: u16,
    pub mono_px: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SpacingTokens {
    pub xs: u16,
    pub sm: u16,
    pub md: u16,
    pub lg: u16,
    pub xl: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RadiusTokens {
    pub sm: u16,
    pub md: u16,
    pub lg: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ThemeTokens {
    pub id: ThemeId,
    pub display_name: &'static str,
    pub colors: ColorTokens,
    pub typography: TypeTokens,
    pub spacing: SpacingTokens,
    pub radii: RadiusTokens,
}
