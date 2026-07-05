//! Material Design 3 Surface Component
//!
//! A foundational surface container that maps to MD3 surface tint levels.
//! Provides all 12 surface color roles as variants, automatically resolving the
//! correct background color and recommended "on" content color based on the
//! active theme (light or dark).
//!
//! # Surface Tint Hierarchy (lightest → darkest)
//!
//! - `SurfaceVariant` — base surface, slight tint
//! - `SurfaceContainerLowest` — lightest container tint
//! - `SurfaceContainerLow` — low container tint
//! - `SurfaceContainer` — default container tint
//! - `SurfaceContainerHigh` — high container tint
//! - `SurfaceContainerHighest` — strongest container tint
//! - `Primary` / `Secondary` / `Tertiary` — accent surfaces
//! - `PrimaryContainer` / `SecondaryContainer` / `TertiaryContainer` — accent containers
//! - `InverseSurface` — for high-contrast emphasis
//!
//! # Example
//!
//! ```ignore
//! use material_rs::components::{Surface, SurfaceVariant};
//!
//! html! {
//!     <Surface variant={SurfaceVariant::SurfaceContainerHigh} elevation={2}>
//!         <p>{ "Content on a high-tint surface" }</p>
//!     </Surface>
//! }
//! ```

use stylist::yew::use_style;
use yew::prelude::*;

use crate::theme::Theme;
use crate::theme::elevation::ElevationLevel;
use crate::utils::dynamic_style::dynamic_style;

/// Surface color variant.
///
/// Each variant maps to a specific MD3 surface color token.
/// The component automatically resolves the background color and
/// the recommended "on" color for content rendered inside.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SurfaceVariant {
    /// Base surface color.
    #[default]
    Surface,
    /// Tinted surface variant.
    SurfaceVariant,
    /// Lightest surface container (level 0).
    SurfaceContainerLowest,
    /// Low surface container (level 1).
    SurfaceContainerLow,
    /// Default surface container (level 2).
    SurfaceContainer,
    /// High surface container (level 3).
    SurfaceContainerHigh,
    /// Highest surface container (level 4).
    SurfaceContainerHighest,
    /// Dim surface (subdued, for background regions).
    SurfaceDim,
    /// Bright surface (only differs in dark mode).
    SurfaceBright,
    /// Primary surface (strong brand color).
    Primary,
    /// Secondary surface.
    Secondary,
    /// Tertiary surface.
    Tertiary,
    /// Primary container surface.
    PrimaryContainer,
    /// Secondary container surface.
    SecondaryContainer,
    /// Tertiary container surface.
    TertiaryContainer,
    /// Inverse surface (for high-contrast emphasis, e.g. FABs, badges).
    InverseSurface,
    /// Error surface.
    Error,
    /// Error container surface.
    ErrorContainer,
}

impl SurfaceVariant {
    /// Returns the human-readable label for this variant.
    pub fn label(&self) -> &'static str {
        match self {
            SurfaceVariant::Surface => "Surface",
            SurfaceVariant::SurfaceVariant => "Surface Variant",
            SurfaceVariant::SurfaceContainerLowest => "Surface Container Lowest",
            SurfaceVariant::SurfaceContainerLow => "Surface Container Low",
            SurfaceVariant::SurfaceContainer => "Surface Container",
            SurfaceVariant::SurfaceContainerHigh => "Surface Container High",
            SurfaceVariant::SurfaceContainerHighest => "Surface Container Highest",
            SurfaceVariant::SurfaceDim => "Surface Dim",
            SurfaceVariant::SurfaceBright => "Surface Bright",
            SurfaceVariant::Primary => "Primary",
            SurfaceVariant::Secondary => "Secondary",
            SurfaceVariant::Tertiary => "Tertiary",
            SurfaceVariant::PrimaryContainer => "Primary Container",
            SurfaceVariant::SecondaryContainer => "Secondary Container",
            SurfaceVariant::TertiaryContainer => "Tertiary Container",
            SurfaceVariant::InverseSurface => "Inverse Surface",
            SurfaceVariant::Error => "Error",
            SurfaceVariant::ErrorContainer => "Error Container",
        }
    }

    /// Returns the CSS token key name (e.g. "surface-container-high").
    pub fn token_key(&self) -> &'static str {
        match self {
            SurfaceVariant::Surface => "surface",
            SurfaceVariant::SurfaceVariant => "surface-variant",
            SurfaceVariant::SurfaceContainerLowest => "surface-container-lowest",
            SurfaceVariant::SurfaceContainerLow => "surface-container-low",
            SurfaceVariant::SurfaceContainer => "surface-container",
            SurfaceVariant::SurfaceContainerHigh => "surface-container-high",
            SurfaceVariant::SurfaceContainerHighest => "surface-container-highest",
            SurfaceVariant::SurfaceDim => "surface-dim",
            SurfaceVariant::SurfaceBright => "surface-bright",
            SurfaceVariant::Primary => "primary",
            SurfaceVariant::Secondary => "secondary",
            SurfaceVariant::Tertiary => "tertiary",
            SurfaceVariant::PrimaryContainer => "primary-container",
            SurfaceVariant::SecondaryContainer => "secondary-container",
            SurfaceVariant::TertiaryContainer => "tertiary-container",
            SurfaceVariant::InverseSurface => "inverse-surface",
            SurfaceVariant::Error => "error",
            SurfaceVariant::ErrorContainer => "error-container",
        }
    }

    /// Resolves the background color from a ColorScheme.
    pub fn bg_color(&self, colors: &crate::theme::color::ColorScheme) -> String {
        match self {
            SurfaceVariant::Surface => colors.surface.clone(),
            SurfaceVariant::SurfaceVariant => colors.surface_variant.clone(),
            SurfaceVariant::SurfaceContainerLowest => colors.surface_container_lowest.clone(),
            SurfaceVariant::SurfaceContainerLow => colors.surface_container_low.clone(),
            SurfaceVariant::SurfaceContainer => colors.surface_container.clone(),
            SurfaceVariant::SurfaceContainerHigh => colors.surface_container_high.clone(),
            SurfaceVariant::SurfaceContainerHighest => colors.surface_container_highest.clone(),
            SurfaceVariant::SurfaceDim => colors.surface_dim.clone(),
            SurfaceVariant::SurfaceBright => colors.surface_bright.clone(),
            SurfaceVariant::Primary => colors.primary.clone(),
            SurfaceVariant::Secondary => colors.secondary.clone(),
            SurfaceVariant::Tertiary => colors.tertiary.clone(),
            SurfaceVariant::PrimaryContainer => colors.primary_container.clone(),
            SurfaceVariant::SecondaryContainer => colors.secondary_container.clone(),
            SurfaceVariant::TertiaryContainer => colors.tertiary_container.clone(),
            SurfaceVariant::InverseSurface => colors.inverse_surface.clone(),
            SurfaceVariant::Error => colors.error.clone(),
            SurfaceVariant::ErrorContainer => colors.error_container.clone(),
        }
    }

    /// Resolves the recommended "on" color for content inside this surface.
    pub fn on_color(&self, colors: &crate::theme::color::ColorScheme) -> String {
        match self {
            SurfaceVariant::Surface
            | SurfaceVariant::SurfaceVariant
            | SurfaceVariant::SurfaceContainerLowest
            | SurfaceVariant::SurfaceContainerLow
            | SurfaceVariant::SurfaceContainer
            | SurfaceVariant::SurfaceContainerHigh
            | SurfaceVariant::SurfaceContainerHighest
            | SurfaceVariant::SurfaceDim
            | SurfaceVariant::SurfaceBright => colors.on_surface.clone(),
            SurfaceVariant::Primary => colors.on_primary.clone(),
            SurfaceVariant::Secondary => colors.on_secondary.clone(),
            SurfaceVariant::Tertiary => colors.on_tertiary.clone(),
            SurfaceVariant::PrimaryContainer => colors.on_primary_container.clone(),
            SurfaceVariant::SecondaryContainer => colors.on_secondary_container.clone(),
            SurfaceVariant::TertiaryContainer => colors.on_tertiary_container.clone(),
            SurfaceVariant::InverseSurface => colors.inverse_on_surface.clone(),
            SurfaceVariant::Error => colors.on_error.clone(),
            SurfaceVariant::ErrorContainer => colors.on_error_container.clone(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct SurfaceProps {
    /// Surface color variant.
    #[prop_or_default]
    pub variant: SurfaceVariant,

    /// Elevation level (0–5). Adds a shadow. Defaults to 0 (no shadow).
    #[prop_or(0)]
    pub elevation: u8,

    /// Whether to use the "on" color for the text color of children.
    /// When true, sets `color` to the resolved on-color. Does NOT override
    /// children that set their own color. Defaults to false.
    #[prop_or(false)]
    pub on_color: bool,

    /// HTML element tag. Defaults to "div".
    #[prop_or("div".to_owned())]
    pub tag: String,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,

    /// Inline style override.
    #[prop_or_default]
    pub style: String,

    /// Surface content.
    #[prop_or_default]
    pub children: Children,
}

#[component]
pub fn Surface(props: &SurfaceProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let bg = props.variant.bg_color(&theme.colors);
    let on = props.variant.on_color(&theme.colors);

    // Resolve elevation
    let elev_level = match props.elevation {
        0 => ElevationLevel::Level0,
        1 => ElevationLevel::Level1,
        2 => ElevationLevel::Level2,
        3 => ElevationLevel::Level3,
        4 => ElevationLevel::Level4,
        _ => ElevationLevel::Level5,
    };
    let shadow = theme.elevation(elev_level).box_shadow.clone();

    let base_style = use_style!(
        r#"
        box-sizing: border-box;
        background-color: ${bg};
        box-shadow: ${shadow};
        color: ${color};
        border-radius: ${radius};
        transition: background-color 200ms cubic-bezier(0.2, 0, 0, 1), box-shadow 200ms cubic-bezier(0.2, 0, 0, 1);
        font-family: ${font}, sans-serif;
        "#,
        bg = bg,
        shadow = shadow,
        color = if props.on_color { on.clone() } else { "inherit".into() },
        radius = theme.shapes.large,
        font = theme.font_family,
    );

    let component_override = theme.component_style("Surface").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    let tag = props.tag.to_lowercase();
    let class_name = yew::classes![base_style.get_class_name().to_string(), &props.class, &component_override];
    let custom_style = props.style.clone();
    let children = props.children.clone();
    let id = props.id.clone();

    match tag.as_str() {
        "section" => html! { <section id={id} class={class_name} style={custom_style}>{ children }</section> },
        "article" => html! { <article id={id} class={class_name} style={custom_style}>{ children }</article> },
        "main" => html! { <main id={id} class={class_name} style={custom_style}>{ children }</main> },
        "aside" => html! { <aside id={id} class={class_name} style={custom_style}>{ children }</aside> },
        "header" => html! { <header id={id} class={class_name} style={custom_style}>{ children }</header> },
        "footer" => html! { <footer id={id} class={class_name} style={custom_style}>{ children }</footer> },
        "nav" => html! { <nav id={id} class={class_name} style={custom_style}>{ children }</nav> },
        _ => html! { <div id={id} class={class_name} style={custom_style}>{ children }</div> },
    }
}

/// Convenience: a Paper surface is simply a Surface with
/// `SurfaceContainerLowest` variant and optional elevation.
///
/// "Paper" is the traditional name used by component libraries
/// (MUI, etc.) for a flat elevated surface.
#[derive(Properties, PartialEq)]
pub struct PaperProps {
    /// Elevation level (0–5). Defaults to 1.
    #[prop_or(1)]
    pub elevation: u8,

    /// Override the default surface variant.
    #[prop_or(SurfaceVariant::SurfaceContainerLowest)]
    pub variant: SurfaceVariant,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,

    /// Inline style override.
    #[prop_or_default]
    pub style: String,

    /// Paper content.
    #[prop_or_default]
    pub children: Children,
}

#[component]
pub fn Paper(props: &PaperProps) -> Html {
    html! {
        <Surface
            variant={props.variant}
            elevation={props.elevation}
            id={props.id.clone()}
            class={props.class.clone()}
            style={props.style.clone()}
        >
            { props.children.clone() }
        </Surface>
    }
}