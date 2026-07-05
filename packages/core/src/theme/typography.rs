//! Material Design 3 Typography Scale
//!
//! Defines the complete MD3 type scale with 15 roles. Each role specifies
//! font family, size, line height, weight, tracking, and case transformation.

use std::fmt;

/// MD3 font weight values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontWeight {
    Regular,  // 400
    Medium,   // 500
    SemiBold, // 600
    Bold,     // 700
}

impl FontWeight {
    pub fn as_str(&self) -> &'static str {
        match self {
            FontWeight::Regular => "400",
            FontWeight::Medium => "500",
            FontWeight::SemiBold => "600",
            FontWeight::Bold => "700",
        }
    }
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Case transformation for display text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextCase {
    None,
    Upper,
    Lower,
    Capitalize,
}

impl TextCase {
    pub fn css_value(&self) -> &'static str {
        match self {
            TextCase::None => "none",
            TextCase::Upper => "uppercase",
            TextCase::Lower => "lowercase",
            TextCase::Capitalize => "capitalize",
        }
    }
}

/// A single type-scale role specification.
#[derive(Clone, Debug, PartialEq)]
pub struct TypeRole {
    pub font_family: String,
    pub font_size: f64,
    pub line_height: f64,
    pub weight: FontWeight,
    pub tracking: f64,
    pub text_case: TextCase,
}

impl TypeRole {
    /// Generate a CSS `font` shorthand string.
    pub fn to_css_font(&self) -> String {
        format!(
            "{} {}px/{}px {}, sans-serif",
            self.weight.as_str(),
            self.font_size,
            self.line_height,
            self.font_family,
        )
    }

    /// Generate a CSS string for the role (excluding font-family, which
    /// is typically applied at the root).
    pub fn to_css(&self) -> String {
        format!(
            "font-size: {}px; line-height: {}px; font-weight: {}; letter-spacing: {}px; text-transform: {};",
            self.font_size,
            self.line_height,
            self.weight.as_str(),
            self.tracking,
            self.text_case.css_value(),
        )
    }
}

/// The complete MD3 type scale.
///
/// Reference: <https://m3.material.io/styles/typography/overview>
#[derive(Clone, Debug, PartialEq)]
pub struct TypeScale {
    pub display_large: TypeRole,
    pub display_medium: TypeRole,
    pub display_small: TypeRole,

    pub headline_large: TypeRole,
    pub headline_medium: TypeRole,
    pub headline_small: TypeRole,

    pub title_large: TypeRole,
    pub title_medium: TypeRole,
    pub title_small: TypeRole,

    pub body_large: TypeRole,
    pub body_medium: TypeRole,
    pub body_small: TypeRole,

    pub label_large: TypeRole,
    pub label_medium: TypeRole,
    pub label_small: TypeRole,
}

/// Creates the default MD3 type scale with the "Roboto" font family.
pub fn default_type_scale() -> TypeScale {
    let ff = "Roboto".to_owned();

    TypeScale {
        display_large: TypeRole { font_family: ff.clone(), font_size: 57.0, line_height: 64.0, weight: FontWeight::Regular, tracking: -0.25, text_case: TextCase::None },
        display_medium: TypeRole { font_family: ff.clone(), font_size: 45.0, line_height: 52.0, weight: FontWeight::Regular, tracking: 0.0, text_case: TextCase::None },
        display_small: TypeRole { font_family: ff.clone(), font_size: 36.0, line_height: 44.0, weight: FontWeight::Regular, tracking: 0.0, text_case: TextCase::None },

        headline_large: TypeRole { font_family: ff.clone(), font_size: 32.0, line_height: 40.0, weight: FontWeight::Regular, tracking: 0.0, text_case: TextCase::None },
        headline_medium: TypeRole { font_family: ff.clone(), font_size: 28.0, line_height: 36.0, weight: FontWeight::Regular, tracking: 0.0, text_case: TextCase::None },
        headline_small: TypeRole { font_family: ff.clone(), font_size: 24.0, line_height: 32.0, weight: FontWeight::Regular, tracking: 0.0, text_case: TextCase::None },

        title_large: TypeRole { font_family: ff.clone(), font_size: 22.0, line_height: 28.0, weight: FontWeight::Regular, tracking: 0.0, text_case: TextCase::None },
        title_medium: TypeRole { font_family: ff.clone(), font_size: 16.0, line_height: 24.0, weight: FontWeight::Medium, tracking: 0.15, text_case: TextCase::None },
        title_small: TypeRole { font_family: ff.clone(), font_size: 14.0, line_height: 20.0, weight: FontWeight::Medium, tracking: 0.1, text_case: TextCase::None },

        body_large: TypeRole { font_family: ff.clone(), font_size: 16.0, line_height: 24.0, weight: FontWeight::Regular, tracking: 0.5, text_case: TextCase::None },
        body_medium: TypeRole { font_family: ff.clone(), font_size: 14.0, line_height: 20.0, weight: FontWeight::Regular, tracking: 0.25, text_case: TextCase::None },
        body_small: TypeRole { font_family: ff.clone(), font_size: 12.0, line_height: 16.0, weight: FontWeight::Regular, tracking: 0.4, text_case: TextCase::None },

        label_large: TypeRole { font_family: ff.clone(), font_size: 14.0, line_height: 20.0, weight: FontWeight::Medium, tracking: 0.1, text_case: TextCase::None },
        label_medium: TypeRole { font_family: ff.clone(), font_size: 12.0, line_height: 16.0, weight: FontWeight::Medium, tracking: 0.5, text_case: TextCase::None },
        label_small: TypeRole { font_family: ff.clone(), font_size: 11.0, line_height: 16.0, weight: FontWeight::Medium, tracking: 0.5, text_case: TextCase::None },
    }
}