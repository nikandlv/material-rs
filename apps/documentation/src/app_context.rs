//! Shared application context for palette and theme state.

use material_rs::theme::{Theme, ThemeBuilder, ThemeMode};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorPalette { Purple, Blue, Green, Red, Orange }

impl ColorPalette {
    pub fn label(&self) -> &'static str {
        match self { Self::Purple=>"Purple", Self::Blue=>"Blue", Self::Green=>"Green", Self::Red=>"Red", Self::Orange=>"Orange" }
    }

    /// Return the MD3 seed hex color for this palette.
    pub fn seed(&self) -> &'static str {
        match self {
            Self::Purple => "#6750A4",
            Self::Blue => "#0061A4",
            Self::Green => "#386A20",
            Self::Red => "#BA1A1A",
            Self::Orange => "#904D00",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct AppState {
    pub dark_mode: bool,
    pub rtl: bool,
    pub palette: ColorPalette,
    pub palette_menu_open: bool,
    pub drawer_open: bool,
    pub is_mobile: bool,
    pub snackbar_visible: bool,
    pub snackbar_message: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self { dark_mode: false, rtl: false, palette: ColorPalette::Purple, palette_menu_open: false, drawer_open: true, is_mobile: false, snackbar_visible: false, snackbar_message: String::new() }
    }
}

pub fn theme_for_palette(palette: ColorPalette, dark: bool) -> Theme {
    ThemeBuilder::new()
        .mode(if dark { ThemeMode::Dark } else { ThemeMode::Light })
        .seed(palette.seed())
        .build()
}
