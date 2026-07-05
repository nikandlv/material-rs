//! Material Design 3 Theme System
//!
//! Provides a comprehensive `Theme` context containing all MD3 design tokens:
//! color scheme, typography scale, elevation, shape, and motion.
//!
//! # Usage
//!
//! ```ignore
//! use material_rs::theme::{Theme, MaterialThemeProvider, default_light_theme};
//!
//! #[component]
//! fn App() -> Html {
//!     html! {
//!         <MaterialThemeProvider theme={default_light_theme()}>
//!             <YourContent />
//!         </MaterialThemeProvider>
//!     }
//! }
//! ```

pub mod color;
pub mod components;
pub mod elevation;
pub mod motion;
pub mod scheme;
pub mod shape;
pub mod typography;

use color::ColorScheme;
use components::ComponentStyles;
use elevation::ElevationLevel;
use motion::MotionTokens;
use shape::ShapeTokens;
use typography::TypeScale;

use yew::prelude::*;

/// Whether the theme is light or dark.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

/// Text direction for RTL/LTR support.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Ltr,
    Rtl,
}

impl Direction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::Ltr => "ltr",
            Direction::Rtl => "rtl",
        }
    }

    pub fn is_rtl(&self) -> bool {
        matches!(self, Direction::Rtl)
    }
}

/// The complete Material Design 3 theme context.
///
/// All components access tokens through this struct via `use_context::<Theme>()`.
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub mode: ThemeMode,
    pub direction: Direction,
    pub colors: ColorScheme,
    pub typography: TypeScale,
    pub shapes: ShapeTokens,
    pub motion: MotionTokens,
    pub font_family: String,
    /// Per-component style overrides keyed by `"ComponentName.slot"`.
    pub component_styles: ComponentStyles,
}

// ── Theme Context Provider ──────────────────────────────────────

#[derive(Properties, PartialEq)]
pub struct ThemeProviderProps {
    pub children: Html,
    pub theme: Theme,
}

#[component]
pub fn MaterialThemeProvider(props: &ThemeProviderProps) -> Html {
    html! {
        <ContextProvider<Theme> context={props.theme.clone()}>
            <div dir={props.theme.direction.as_str()}>
                {props.children.clone()}
            </div>
        </ContextProvider<Theme>>
    }
}

// ── Theme Builders / Defaults ───────────────────────────────────

/// Creates the default MD3 light theme (purple seed).
pub fn default_light_theme() -> Theme {
    Theme {
        mode: ThemeMode::Light,
        direction: Direction::Ltr,
        colors: color::light_color_scheme(),
        typography: typography::default_type_scale(),
        shapes: ShapeTokens::rounded(),
        motion: MotionTokens::standard(),
        font_family: "Roboto, Outfit, system-ui".into(),
        component_styles: ComponentStyles::new(),
    }
}

pub fn default_dark_theme() -> Theme {
    Theme {
        mode: ThemeMode::Dark,
        direction: Direction::Ltr,
        colors: color::dark_color_scheme(),
        typography: typography::default_type_scale(),
        shapes: ShapeTokens::rounded(),
        motion: MotionTokens::standard(),
        font_family: "Roboto, Outfit, system-ui".into(),
        component_styles: ComponentStyles::new(),
    }
}

/// Backwards-compatible alias.
pub fn default_theme() -> Theme {
    default_light_theme()
}

// ── Theme Builder ───────────────────────────────────────────────

/// Fluent builder for constructing a custom `Theme`.
///
/// # Example
///
/// ```ignore
/// let theme = ThemeBuilder::new()
///     .mode(ThemeMode::Dark)
///     .font_family("Inter, sans-serif")
///     .build();
/// ```
#[derive(Default)]
pub struct ThemeBuilder {
    mode: Option<ThemeMode>,
    direction: Option<Direction>,
    colors: Option<ColorScheme>,
    seed: Option<String>,
    typography: Option<TypeScale>,
    shapes: Option<ShapeTokens>,
    motion: Option<MotionTokens>,
    font_family: Option<String>,
    component_styles: ComponentStyles,
}

impl ThemeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mode(mut self, mode: ThemeMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn colors(mut self, colors: ColorScheme) -> Self {
        self.colors = Some(colors);
        self
    }

    /// Generate the color scheme from a seed hex color (e.g. `"#6750A4"`).
    /// Takes precedence over `.colors()` if both are set.
    pub fn seed(mut self, hex: impl Into<String>) -> Self {
        self.seed = Some(hex.into());
        self
    }

    pub fn typography(mut self, typography: TypeScale) -> Self {
        self.typography = Some(typography);
        self
    }

    pub fn shapes(mut self, shapes: ShapeTokens) -> Self {
        self.shapes = Some(shapes);
        self
    }

    pub fn motion(mut self, motion: MotionTokens) -> Self {
        self.motion = Some(motion);
        self
    }

    pub fn font_family(mut self, font_family: impl Into<String>) -> Self {
        self.font_family = Some(font_family.into());
        self
    }

    /// Set a custom CSS override for a component slot.
    ///
    /// # Arguments
    /// * `key` - Component slot key, e.g. `"Button.root"`, `"Card.content"`, `"TextField.filled"`
    /// * `css` - CSS string to apply to the component slot
    ///
    /// # Example
    ///
    /// ```ignore
    /// ThemeBuilder::new()
    ///     .component("Button.root", "border-radius: 24px; font-weight: 700;".into())
    ///     .component("Card.root", "box-shadow: 0 4px 12px rgba(0,0,0,0.1);".into())
    ///     .build()
    /// ```
    pub fn component(mut self, key: impl Into<String>, css: impl Into<String>) -> Self {
        self.component_styles.insert(key.into(), css.into());
        self
    }

    pub fn build(self) -> Theme {
        let mode = self.mode.unwrap_or(ThemeMode::Light);
        let colors = if let Some(seed) = self.seed {
            scheme::generate_scheme(&seed, mode == ThemeMode::Dark)
        } else {
            self.colors.unwrap_or_else(|| match mode {
                ThemeMode::Light => color::light_color_scheme(),
                ThemeMode::Dark => color::dark_color_scheme(),
            })
        };

        Theme {
            mode,
            direction: self.direction.unwrap_or_default(),
            colors,
            typography: self
                .typography
                .unwrap_or_else(typography::default_type_scale),
            shapes: self.shapes.unwrap_or_else(ShapeTokens::rounded),
            motion: self.motion.unwrap_or_else(MotionTokens::standard),
            font_family: self
                .font_family
                .unwrap_or_else(|| "Roboto, system-ui".into()),
            component_styles: self.component_styles,
        }
    }
}

// ── Convenience Helpers on Theme ────────────────────────────────

impl Theme {
    pub fn elevation(&self, level: ElevationLevel) -> elevation::ElevationShadow {
        match self.mode {
            ThemeMode::Light => elevation::light_elevation(level),
            ThemeMode::Dark => elevation::dark_elevation(level),
        }
    }

    pub fn surface_container(&self, level: u8) -> &str {
        match level {
            0 => &self.colors.surface_container_lowest,
            1 => &self.colors.surface_container_low,
            2 => &self.colors.surface_container,
            3 => &self.colors.surface_container_high,
            _ => &self.colors.surface_container_highest,
        }
    }

    pub fn is_dark(&self) -> bool {
        self.mode == ThemeMode::Dark
    }

    pub fn is_rtl(&self) -> bool {
        self.direction.is_rtl()
    }

    /// Get the CSS override for a specific component slot.
    ///
    /// # Example
    /// ```ignore
    /// let css = theme.component_style("Button.root");
    /// ```
    pub fn component_style(&self, key: &str) -> Option<&str> {
        self.component_styles.get(key).map(|s| s.as_str())
    }

    /// Get all CSS overrides for a component (all its slots).
    ///
    /// Returns a concatenated string of all matching overrides.
    /// For example, `component_styles_for("Button")` returns the CSS
    /// from "Button.root", "Button.label", "Button.icon", etc.
    pub fn component_styles_for(&self, component: &str) -> String {
        let prefix = format!("{}.", component);
        let mut combined = String::new();
        for (key, css) in &self.component_styles {
            if key == component || key.starts_with(&prefix) {
                combined.push_str(css);
                combined.push(' ');
            }
        }
        combined
    }
}
