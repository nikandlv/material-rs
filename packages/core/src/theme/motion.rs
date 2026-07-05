//! Material Design 3 Motion Tokens
//!
//! Defines standard duration and easing curves used across MD3 components.

/// Standard MD3 motion durations in milliseconds.
#[derive(Clone, Debug, PartialEq)]
pub struct MotionDurations {
    pub short1: u32,  // 50ms
    pub short2: u32,  // 100ms
    pub short3: u32,  // 150ms
    pub short4: u32,  // 200ms
    pub medium1: u32, // 250ms
    pub medium2: u32, // 300ms
    pub medium3: u32, // 350ms
    pub medium4: u32, // 400ms
    pub long1: u32,   // 450ms
    pub long2: u32,   // 500ms
    pub long3: u32,   // 550ms
    pub long4: u32,   // 700ms
    pub extra_long1: u32, // 900ms
    pub extra_long2: u32, // 1400ms
}

impl Default for MotionDurations {
    fn default() -> Self {
        Self::standard()
    }
}

impl MotionDurations {
    pub fn standard() -> Self {
        Self {
            short1: 50,
            short2: 100,
            short3: 150,
            short4: 200,
            medium1: 250,
            medium2: 300,
            medium3: 350,
            medium4: 400,
            long1: 450,
            long2: 500,
            long3: 550,
            long4: 700,
            extra_long1: 900,
            extra_long2: 1400,
        }
    }
}

/// Standard MD3 easing curves.
#[derive(Clone, Debug, PartialEq)]
pub struct MotionEasings {
    pub standard: String,       // cubic-bezier(0.2, 0, 0, 1)
    pub standard_accelerate: String, // cubic-bezier(0.3, 0, 1, 1)
    pub standard_decelerate: String, // cubic-bezier(0, 0, 0, 1)
    pub emphasized: String,     // cubic-bezier(0.2, 0, 0, 1)
    pub emphasized_accelerate: String, // cubic-bezier(0.2, 0, 1, 1)
    pub emphasized_decelerate: String, // cubic-bezier(0.05, 0.7, 0.1, 1)
    pub legacy: String,         // cubic-bezier(0.4, 0, 0.2, 1)
    pub legacy_accelerate: String, // cubic-bezier(0.4, 0, 1, 1)
    pub legacy_decelerate: String,  // cubic-bezier(0, 0, 0.2, 1)
    pub linear: String,         // cubic-bezier(0, 0, 1, 1) / linear
}

impl Default for MotionEasings {
    fn default() -> Self {
        Self::standard()
    }
}

impl MotionEasings {
    pub fn standard() -> Self {
        Self {
            standard: "cubic-bezier(0.2, 0, 0, 1)".into(),
            standard_accelerate: "cubic-bezier(0.3, 0, 1, 1)".into(),
            standard_decelerate: "cubic-bezier(0, 0, 0, 1)".into(),
            emphasized: "cubic-bezier(0.2, 0, 0, 1)".into(),
            emphasized_accelerate: "cubic-bezier(0.2, 0, 1, 1)".into(),
            emphasized_decelerate: "cubic-bezier(0.05, 0.7, 0.1, 1)".into(),
            legacy: "cubic-bezier(0.4, 0, 0.2, 1)".into(),
            legacy_accelerate: "cubic-bezier(0.4, 0, 1, 1)".into(),
            legacy_decelerate: "cubic-bezier(0, 0, 0.2, 1)".into(),
            linear: "linear".into(),
        }
    }
}

/// Combined motion configuration.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct MotionTokens {
    pub duration: MotionDurations,
    pub easing: MotionEasings,
}

impl MotionTokens {
    pub fn standard() -> Self {
        Self::default()
    }

    /// Helper to produce a CSS transition string.
    pub fn transition(&self, property: &str, duration: u32, easing: &str) -> String {
        format!("transition: {} {}ms {};", property, duration, easing)
    }
}