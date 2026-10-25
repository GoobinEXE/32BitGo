use bitgo_core::ThemeId;

use crate::tokens::{ColorTokens, RadiusTokens, SpacingTokens, ThemeTokens, TypeTokens};

pub const NES: ThemeTokens = ThemeTokens {
    id: ThemeId::Nes,
    display_name: "NES Edition",
    colors: ColorTokens {
        background: 0xFF1A1A1A,
        surface: 0xFF2B2B2B,
        surface_alt: 0xFF3A3A3A,
        accent: 0xFFE6392E,
        accent_dim: 0xFFA32820,
        text: 0xFFF5F0E8,
        text_muted: 0xFFA8A29A,
        danger: 0xFFD64545,
        success: 0xFF5AAF6A,
        border: 0xFF4A4A4A,
    },
    typography: TypeTokens {
        display_px: 28,
        title_px: 20,
        body_px: 14,
        mono_px: 13,
    },
    spacing: SpacingTokens {
        xs: 4,
        sm: 8,
        md: 16,
        lg: 24,
        xl: 40,
    },
    radii: RadiusTokens {
        sm: 2,
        md: 4,
        lg: 6,
    },
};
