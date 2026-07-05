//! Material Design 3 seed-based color scheme generation.
//!
//! Uses the `material-color-utils` crate (official Rust port of
//! `@material/material-color-utilities`) to generate the full MD3 color scheme
//! from a single seed color via the HCT color space.
//!
//! # Example
//!
//! ```ignore
//! use material_rs::theme::scheme::generate_scheme;
//!
//! let light = generate_scheme("#6750A4", false);
//! let dark = generate_scheme("#6750A4", true);
//! ```

use material_color_utils::theme_from_color;
use material_color_utils::utils::color_utils::Argb;

use super::color::ColorScheme;

/// Generate a full MD3 color scheme from a seed hex color.
///
/// Uses the official Material Color Utilities library to produce
/// perceptually correct colors via the HCT color space.
pub fn generate_scheme(seed: &str, dark: bool) -> ColorScheme {
    let source = match Argb::from_hex(seed) {
        Ok(c) => c,
        Err(_) => Argb(0xFF6750A4), // fallback to purple
    };

    let theme = theme_from_color(source).call();

    let scheme = if dark { &theme.schemes.dark } else { &theme.schemes.light };

    ColorScheme {
        primary: scheme.primary.to_hex(),
        on_primary: scheme.on_primary.to_hex(),
        primary_container: scheme.primary_container.to_hex(),
        on_primary_container: scheme.on_primary_container.to_hex(),

        secondary: scheme.secondary.to_hex(),
        on_secondary: scheme.on_secondary.to_hex(),
        secondary_container: scheme.secondary_container.to_hex(),
        on_secondary_container: scheme.on_secondary_container.to_hex(),

        tertiary: scheme.tertiary.to_hex(),
        on_tertiary: scheme.on_tertiary.to_hex(),
        tertiary_container: scheme.tertiary_container.to_hex(),
        on_tertiary_container: scheme.on_tertiary_container.to_hex(),

        error: scheme.error.to_hex(),
        on_error: scheme.on_error.to_hex(),
        error_container: scheme.error_container.to_hex(),
        on_error_container: scheme.on_error_container.to_hex(),

        surface: scheme.surface.to_hex(),
        on_surface: scheme.on_surface.to_hex(),
        surface_variant: scheme.surface_variant.to_hex(),
        on_surface_variant: scheme.on_surface_variant.to_hex(),
        surface_container_lowest: scheme.surface_container_lowest.to_hex(),
        surface_container_low: scheme.surface_container_low.to_hex(),
        surface_container: scheme.surface_container.to_hex(),
        surface_container_high: scheme.surface_container_high.to_hex(),
        surface_container_highest: scheme.surface_container_highest.to_hex(),
        surface_dim: scheme.surface_dim.to_hex(),
        surface_bright: scheme.surface_bright.to_hex(),

        outline: scheme.outline.to_hex(),
        outline_variant: scheme.outline_variant.to_hex(),

        inverse_surface: scheme.inverse_surface.to_hex(),
        inverse_on_surface: scheme.inverse_on_surface.to_hex(),
        inverse_primary: scheme.inverse_primary.to_hex(),

        scrim: scheme.scrim.to_hex(),
        shadow: scheme.shadow.to_hex(),
    }
}
