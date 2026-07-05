//! Material Design 3 Shape System
//!
//! MD3 defines shape using corner radii categorized by size classes.
//! This module provides named shape tokens and helper functions.

/// Named shape tokens mapping to MD3 size categories.
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeTokens {
    pub none: String,          // 0px
    pub extra_small: String,   // 4px
    pub small: String,         // 8px
    pub medium: String,        // 12px
    pub large: String,         // 16px
    pub extra_large: String,   // 28px
    pub full: String,          // 50% / 9999px (pill/circle)
}

impl Default for ShapeTokens {
    fn default() -> Self {
        Self::rounded()
    }
}

impl ShapeTokens {
    /// MD3 default rounded shape scale.
    pub fn rounded() -> Self {
        Self {
            none: "0px".into(),
            extra_small: "4px".into(),
            small: "8px".into(),
            medium: "12px".into(),
            large: "16px".into(),
            extra_large: "28px".into(),
            full: "9999px".into(),
        }
    }

    /// More squared shape scale (reduced rounding).
    pub fn squared() -> Self {
        Self {
            none: "0px".into(),
            extra_small: "2px".into(),
            small: "4px".into(),
            medium: "8px".into(),
            large: "12px".into(),
            extra_large: "16px".into(),
            full: "9999px".into(),
        }
    }

    /// Fully rounded shape scale (more aggressive rounding).
    pub fn more_rounded() -> Self {
        Self {
            none: "0px".into(),
            extra_small: "8px".into(),
            small: "12px".into(),
            medium: "16px".into(),
            large: "20px".into(),
            extra_large: "32px".into(),
            full: "9999px".into(),
        }
    }

    /// Generate a `border-radius` CSS property string for a given shape name.
    pub fn radius_css(&self, value: &str) -> String {
        format!("border-radius: {};", value)
    }

    /// Generate uniform `border-radius` shorthand.
    pub fn uniform_radius_css(&self, value: &str) -> String {
        format!("border-radius: {};", value)
    }
}

/// Pre-defined MD3 shape mappings for common component categories.
pub struct ComponentShapes;

impl ComponentShapes {
    /// Returns the default border-radius for a given component type.
    pub fn for_component(component: &str) -> &'static str {
        match component {
            // Full-pill shapes
            "button" | "fab" | "icon_button" | "chip" | "snackbar" => "9999px",
            "segmented_button" => "9999px",

            // Extra-large rounded
            "card" | "dialog" | "navigation_drawer" => "28px",

            // Large rounded
            "navigation_bar" | "search_bar" => "16px",

            // Medium rounded
            "text_field" | "menu" | "tooltip" => "12px",

            // Small rounded
            "switch" | "checkbox" => "8px",

            // Extra-small rounded
            "divider" | "progress" => "4px",

            // None
            "top_app_bar" | "tab_bar" | "list_item" => "0px",

            _ => "12px",
        }
    }
}