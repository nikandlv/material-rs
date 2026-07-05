//! Shared style utilities for MD3 components.
//!
//! Provides reusable CSS fragments for common patterns: state layers,
//! focus rings, transitions, and disabled states.

use crate::theme::Theme;
use crate::utils::color::with_alpha;

/// Generate a state layer overlay CSS block.
///
/// The state layer is an absolutely-positioned overlay that shows on hover (0.08 opacity)
/// and active (0.12 opacity) states.
pub fn state_layer_css(hover_color: &str, _active_color: &str) -> String {
    format!(
        "position: absolute; inset: 0; border-radius: inherit; pointer-events: none; \
         background: {}; opacity: 0; transition: opacity 150ms;",
        hover_color
    )
}

/// Generate CSS for the `::before` pseudo-element state layer (used with `use_style!`).
pub fn state_layer_pseudo_css(hover_color: &str) -> String {
    format!(
        "&::before {{ content: ''; position: absolute; inset: 0; border-radius: inherit; \
          background: {}; opacity: 0; transition: opacity 150ms; pointer-events: none; }} \
         &:hover::before {{ opacity: 1; }}",
        hover_color
    )
}

/// Generate MD3 focus-visible ring CSS fragment.
pub fn focus_ring_css(ring_color: &str) -> String {
    format!(
        "&:focus-visible {{ outline: 2px solid {}; outline-offset: 2px; }}",
        ring_color
    )
}

/// Get disabled button colors: (bg, text_color) for the given theme.
pub fn disabled_button_colors(theme: &Theme, variant: &str) -> (String, String) {
    let on_surface_disabled = with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default();
    let on_surface_12 = with_alpha(&theme.colors.on_surface, 0.12).unwrap_or_default();

    let bg = match variant {
        "text" | "icon" => "transparent".into(),
        _ => on_surface_12,
    };

    (bg, on_surface_disabled)
}

/// Get disabled text input colors: (container_bg, label_color, text_color).
pub fn disabled_text_colors(theme: &Theme) -> (String, String, String) {
    let on_surf_38 = with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default();
    let on_surf_12 = with_alpha(&theme.colors.on_surface, 0.12).unwrap_or_default();
    (on_surf_12, on_surf_38.clone(), on_surf_38)
}

/// Build a CSS transition value from property/duration/easing triples.
pub fn transition(props: &[(&str, u32, &str)]) -> String {
    props
        .iter()
        .map(|(prop, dur, ease)| format!("{} {}ms {}", prop, dur, ease))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Standard MD3 standard easing curve.
pub const STANDARD_EASING: &str = "cubic-bezier(0.2, 0, 0, 1)";

/// Standard MD3 emphasized easing curve.
pub const EMPHASIZED_EASING: &str = "cubic-bezier(0.34, 1.56, 0.64, 1)";

/// Generate a standard MD3 transition for a single property.
pub fn md_transition(prop: &str, duration_ms: u32) -> String {
    format!("{} {}ms {}", prop, duration_ms, STANDARD_EASING)
}

/// Generate a standard MD3 transition for a single property with custom easing.
pub fn md_transition_eased(prop: &str, duration_ms: u32, easing: &str) -> String {
    format!("{} {}ms {}", prop, duration_ms, easing)
}
