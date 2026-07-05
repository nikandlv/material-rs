//! RTL (Right-to-Left) support utilities.
//!
//! Provides helpers for generating direction-aware CSS properties
//! using CSS logical properties.

/// Returns the appropriate CSS logical property for inline-start padding/margin.
/// In LTR: `padding-left` / `margin-left`. In RTL: `padding-right` / `margin-right`.
pub fn inline_start(is_rtl: bool) -> &'static str {
    if is_rtl { "padding-right" } else { "padding-left" }
}

/// Returns the appropriate CSS logical property for inline-end padding/margin.
/// In LTR: `padding-right` / `margin-right`. In RTL: `padding-left` / `margin-left`.
pub fn inline_end(is_rtl: bool) -> &'static str {
    if is_rtl { "padding-left" } else { "padding-right" }
}

/// Returns left/right for inline start based on direction.
pub fn inset_inline_start(is_rtl: bool) -> &'static str {
    if is_rtl { "right" } else { "left" }
}

/// Returns left/right for inline end based on direction.
pub fn inset_inline_end(is_rtl: bool) -> &'static str {
    if is_rtl { "left" } else { "right" }
}

/// Returns transform for mirroring content in RTL mode.
pub fn rtl_transform(is_rtl: bool) -> &'static str {
    if is_rtl { "scaleX(-1)" } else { "none" }
}

/// Generates a full inline style string for directional padding.
pub fn directional_padding(is_rtl: bool, start: &str, end: &str) -> String {
    format!(
        "{}: {}; {}: {};",
        inset_inline_start(is_rtl), start,
        inset_inline_end(is_rtl), end,
    )
}
