//! Material Design 3 Color Scheme
//!
//! Implements the full MD3 color token system including primary, secondary,
//! tertiary, error, and surface color roles with proper light/dark variants.

/// A complete Material Design 3 color scheme.
///
/// Each role contains base, on-, and container variants following the
/// [MD3 Color System](https://m3.material.io/styles/color/overview).
#[derive(Clone, Debug, PartialEq)]
pub struct ColorScheme {
    // ── Primary ──────────────────────────────────────────────
    pub primary: String,
    pub on_primary: String,
    pub primary_container: String,
    pub on_primary_container: String,

    // ── Secondary ────────────────────────────────────────────
    pub secondary: String,
    pub on_secondary: String,
    pub secondary_container: String,
    pub on_secondary_container: String,

    // ── Tertiary ─────────────────────────────────────────────
    pub tertiary: String,
    pub on_tertiary: String,
    pub tertiary_container: String,
    pub on_tertiary_container: String,

    // ── Error ────────────────────────────────────────────────
    pub error: String,
    pub on_error: String,
    pub error_container: String,
    pub on_error_container: String,

    // ── Surface ──────────────────────────────────────────────
    pub surface: String,
    pub on_surface: String,
    pub surface_variant: String,
    pub on_surface_variant: String,
    pub surface_container_lowest: String,
    pub surface_container_low: String,
    pub surface_container: String,
    pub surface_container_high: String,
    pub surface_container_highest: String,
    pub surface_dim: String,
    pub surface_bright: String,

    // ── Outline ──────────────────────────────────────────────
    pub outline: String,
    pub outline_variant: String,

    // ── Inverse ──────────────────────────────────────────────
    pub inverse_surface: String,
    pub inverse_on_surface: String,
    pub inverse_primary: String,

    // ── Scrim & Shadow ───────────────────────────────────────
    pub scrim: String,
    pub shadow: String,
}

impl Default for ColorScheme {
    fn default() -> Self {
        light_color_scheme()
    }
}

/// MD3 baseline light color scheme (purple seed).
pub fn light_color_scheme() -> ColorScheme {
    ColorScheme {
        primary: "#6750A4".into(),
        on_primary: "#FFFFFF".into(),
        primary_container: "#EADDFF".into(),
        on_primary_container: "#21005D".into(),

        secondary: "#625B71".into(),
        on_secondary: "#FFFFFF".into(),
        secondary_container: "#E8DEF8".into(),
        on_secondary_container: "#1D192B".into(),

        tertiary: "#7D5260".into(),
        on_tertiary: "#FFFFFF".into(),
        tertiary_container: "#FFD8E4".into(),
        on_tertiary_container: "#31111D".into(),

        error: "#B3261E".into(),
        on_error: "#FFFFFF".into(),
        error_container: "#F9DEDC".into(),
        on_error_container: "#410E0B".into(),

        surface: "#FFFBFE".into(),
        on_surface: "#1C1B1F".into(),
        surface_variant: "#E7E0EC".into(),
        on_surface_variant: "#49454F".into(),
        surface_container_lowest: "#FFFFFF".into(),
        surface_container_low: "#F7F2FA".into(),
        surface_container: "#F3EDF7".into(),
        surface_container_high: "#ECE6F0".into(),
        surface_container_highest: "#E6E0E9".into(),
        surface_dim: "#DED8E1".into(),
        surface_bright: "#FFFBFE".into(),

        outline: "#79747E".into(),
        outline_variant: "#CAC4D0".into(),

        inverse_surface: "#313033".into(),
        inverse_on_surface: "#F4EFF4".into(),
        inverse_primary: "#D0BCFF".into(),

        scrim: "#000000".into(),
        shadow: "#000000".into(),
    }
}

/// MD3 baseline dark color scheme (purple seed).
pub fn dark_color_scheme() -> ColorScheme {
    ColorScheme {
        primary: "#D0BCFF".into(),
        on_primary: "#381E72".into(),
        primary_container: "#4F378B".into(),
        on_primary_container: "#EADDFF".into(),

        secondary: "#CCC2DC".into(),
        on_secondary: "#332D41".into(),
        secondary_container: "#4A4458".into(),
        on_secondary_container: "#E8DEF8".into(),

        tertiary: "#EFB8C8".into(),
        on_tertiary: "#492532".into(),
        tertiary_container: "#633B48".into(),
        on_tertiary_container: "#FFD8E4".into(),

        error: "#F2B8B5".into(),
        on_error: "#601410".into(),
        error_container: "#8C1D18".into(),
        on_error_container: "#F9DEDC".into(),

        surface: "#1C1B1F".into(),
        on_surface: "#E6E1E5".into(),
        surface_variant: "#49454F".into(),
        on_surface_variant: "#CAC4D0".into(),
        surface_container_lowest: "#0F0D13".into(),
        surface_container_low: "#1D1B20".into(),
        surface_container: "#211F26".into(),
        surface_container_high: "#2B2930".into(),
        surface_container_highest: "#36343B".into(),
        surface_dim: "#1C1B1F".into(),
        surface_bright: "#3B383E".into(),

        outline: "#938F99".into(),
        outline_variant: "#49454F".into(),

        inverse_surface: "#E6E1E5".into(),
        inverse_on_surface: "#313033".into(),
        inverse_primary: "#6750A4".into(),

        scrim: "#000000".into(),
        shadow: "#000000".into(),
    }
}

/// Convenience: derive on-colors for a given background role.
impl ColorScheme {
    /// Returns the appropriate "on" color for a given surface level.
    pub fn on_for(&self, surface: SurfaceLevel) -> &str {
        match surface {
            SurfaceLevel::Primary => &self.on_primary,
            SurfaceLevel::Secondary => &self.on_secondary,
            SurfaceLevel::Tertiary => &self.on_tertiary,
            SurfaceLevel::Error => &self.on_error,
            SurfaceLevel::Surface => &self.on_surface,
            SurfaceLevel::SurfaceVariant => &self.on_surface_variant,
            SurfaceLevel::PrimaryContainer => &self.on_primary_container,
            SurfaceLevel::SecondaryContainer => &self.on_secondary_container,
            SurfaceLevel::TertiaryContainer => &self.on_tertiary_container,
            SurfaceLevel::ErrorContainer => &self.on_error_container,
        }
    }
}

/// Identifies a surface role for "on" color resolution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceLevel {
    Primary,
    Secondary,
    Tertiary,
    Error,
    Surface,
    SurfaceVariant,
    PrimaryContainer,
    SecondaryContainer,
    TertiaryContainer,
    ErrorContainer,
}