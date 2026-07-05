//! Material Design 3 Elevation System
//!
//! MD3 uses both tonal elevation (surface tint overlay) and shadow elevation
//! to convey depth. This module provides pre-computed shadow values and
//! tonal surface tint opacities for each elevation level (0–5).

/// Shadow token set for a single elevation level.
#[derive(Clone, Debug, PartialEq)]
pub struct ElevationShadow {
    /// CSS `box-shadow` value for this level.
    pub box_shadow: String,
    /// Recommended surface tint overlay opacity (0.0–1.0).
    pub surface_tint_opacity: f64,
}

/// Elevation level indices (0–5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ElevationLevel {
    Level0 = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
    Level4 = 4,
    Level5 = 5,
}

impl ElevationLevel {
    /// Returns the numeric level value.
    pub fn value(&self) -> u8 {
        *self as u8
    }
}

/// Returns the MD3 shadow specification for the given elevation level (light theme).
pub fn light_elevation(level: ElevationLevel) -> ElevationShadow {
    let (box_shadow, tint) = match level {
        ElevationLevel::Level0 => (
            "none".into(),
            0.0,
        ),
        ElevationLevel::Level1 => (
            "0 1px 2px 0 rgba(0,0,0,0.3), 0 1px 3px 1px rgba(0,0,0,0.15)".into(),
            0.05,
        ),
        ElevationLevel::Level2 => (
            "0 1px 2px 0 rgba(0,0,0,0.3), 0 2px 6px 2px rgba(0,0,0,0.15)".into(),
            0.08,
        ),
        ElevationLevel::Level3 => (
            "0 4px 8px 3px rgba(0,0,0,0.15), 0 1px 3px 0 rgba(0,0,0,0.3)".into(),
            0.11,
        ),
        ElevationLevel::Level4 => (
            "0 6px 10px 4px rgba(0,0,0,0.15), 0 2px 3px 0 rgba(0,0,0,0.3)".into(),
            0.12,
        ),
        ElevationLevel::Level5 => (
            "0 8px 12px 6px rgba(0,0,0,0.15), 0 4px 4px 0 rgba(0,0,0,0.3)".into(),
            0.14,
        ),
    };
    ElevationShadow { box_shadow, surface_tint_opacity: tint }
}

/// Returns the MD3 shadow specification for the given elevation level (dark theme).
pub fn dark_elevation(level: ElevationLevel) -> ElevationShadow {
    let (box_shadow, tint) = match level {
        ElevationLevel::Level0 => (
            "none".into(),
            0.0,
        ),
        ElevationLevel::Level1 => (
            "0 1px 3px 1px rgba(0,0,0,0.15), 0 1px 2px 0 rgba(0,0,0,0.3)".into(),
            0.05,
        ),
        ElevationLevel::Level2 => (
            "0 2px 6px 2px rgba(0,0,0,0.15), 0 1px 2px 0 rgba(0,0,0,0.3)".into(),
            0.08,
        ),
        ElevationLevel::Level3 => (
            "0 4px 8px 3px rgba(0,0,0,0.15), 0 1px 3px 0 rgba(0,0,0,0.3)".into(),
            0.11,
        ),
        ElevationLevel::Level4 => (
            "0 6px 10px 4px rgba(0,0,0,0.15), 0 2px 3px 0 rgba(0,0,0,0.3)".into(),
            0.12,
        ),
        ElevationLevel::Level5 => (
            "0 8px 12px 6px rgba(0,0,0,0.15), 0 4px 4px 0 rgba(0,0,0,0.3)".into(),
            0.14,
        ),
    };
    ElevationShadow { box_shadow, surface_tint_opacity: tint }
}