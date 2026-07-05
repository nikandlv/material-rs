//! Material Design 3 Button
//!
//! Supports Filled, Filled Tonal, Filled Tertiary, Outlined, Text, Elevated, Icon,
//! and FAB (Small, Standard, Large, Extended; Primary or Tertiary) variants with full state layers.

pub mod split;
pub use split::SplitButton;

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::components::ripple::Ripple;
use crate::theme::Theme;
use crate::theme::elevation::ElevationLevel;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

/// Button variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Filled,
    FilledTonal,
    FilledTertiary,
    Outlined,
    Text,
    Elevated,
    Icon,
    Fab,
    FabTertiary,
    SmallFab,
    SmallFabTertiary,
    LargeFab,
    LargeFabTertiary,
    ExtendedFab,
    ExtendedFabTertiary,
}

/// Button size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    /// Button variant.
    #[prop_or_default]
    pub variant: ButtonVariant,

    /// Button size (for standard buttons).
    #[prop_or_default]
    pub size: ButtonSize,

    /// Text label.
    #[prop_or_default]
    pub label: String,

    /// Leading icon.
    #[prop_or_default]
    pub icon: String,

    /// Trailing icon.
    #[prop_or_default]
    pub trailing_icon: String,

    /// Click callback.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    /// Disabled state.
    #[prop_or(false)]
    pub disabled: bool,

    /// Optional HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,

    /// Optional inline CSS style.
    #[prop_or_default]
    pub style: String,

    /// Whether the button should take the full width of its container.
    #[prop_or(false)]
    pub full_width: bool,

    /// Child content (overrides label rendering if provided).
    #[prop_or_default]
    pub children: Children,
}

#[component]
pub fn Button(props: &ButtonProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let is_fab = matches!(
        props.variant,
        ButtonVariant::Fab | ButtonVariant::SmallFab | ButtonVariant::LargeFab | ButtonVariant::ExtendedFab |
        ButtonVariant::FabTertiary | ButtonVariant::SmallFabTertiary | ButtonVariant::LargeFabTertiary | ButtonVariant::ExtendedFabTertiary
    );

    let is_square_fab = matches!(
        props.variant,
        ButtonVariant::Fab | ButtonVariant::SmallFab | ButtonVariant::LargeFab |
        ButtonVariant::FabTertiary | ButtonVariant::SmallFabTertiary | ButtonVariant::LargeFabTertiary
    );

    let is_icon_only = matches!(props.variant, ButtonVariant::Icon) || is_square_fab;

    // Sizing attributes
    let (height, base_horizontal_padding, font_size, icon_size) = match props.size {
        ButtonSize::Small => (32, 16, 12, 18),
        ButtonSize::Medium => (40, 24, 14, 18),
        ButtonSize::Large => (48, 32, 16, 24),
    };

    // Horizontal padding per variant
    let horizontal_padding = if is_icon_only {
        0
    } else if matches!(props.variant, ButtonVariant::ExtendedFab | ButtonVariant::ExtendedFabTertiary) {
        20
    } else {
        base_horizontal_padding
    };

    let (fab_size, fab_icon_size) = match props.variant {
        ButtonVariant::Fab | ButtonVariant::FabTertiary => (56, 24),
        ButtonVariant::SmallFab | ButtonVariant::SmallFabTertiary => (40, 18),
        ButtonVariant::LargeFab | ButtonVariant::LargeFabTertiary => (96, 36),
        ButtonVariant::ExtendedFab | ButtonVariant::ExtendedFabTertiary => (56, 24),
        _ => (0, 0),
    };

    let actual_height = if fab_size > 0 { fab_size } else { height };
    let actual_icon_size = if fab_icon_size > 0 { fab_icon_size } else { icon_size };

    // ── Color resolution per variant ──
    let (bg, text_color, border_color, ripple_color) = if props.disabled {
        let on_surface_disabled = with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default();
        let on_surface_12 = with_alpha(&theme.colors.on_surface, 0.12).unwrap_or_default();
        (
            if matches!(props.variant, ButtonVariant::Text | ButtonVariant::Icon) { "transparent".into() } else { on_surface_12.clone() },
            on_surface_disabled,
            if matches!(props.variant, ButtonVariant::Outlined) { with_alpha(&theme.colors.on_surface, 0.12).unwrap_or_default() } else { "transparent".into() },
            on_surface_12,
        )
    } else {
        match props.variant {
            ButtonVariant::Filled => {
                let rc = with_alpha(&theme.colors.on_primary, 0.24).unwrap_or_default();
                (theme.colors.primary.clone(), theme.colors.on_primary.clone(), "transparent".into(), rc)
            }
            ButtonVariant::FilledTonal => {
                let rc = with_alpha(&theme.colors.on_primary_container, 0.24).unwrap_or_default();
                (theme.colors.primary_container.clone(), theme.colors.on_primary_container.clone(), "transparent".into(), rc)
            }
            ButtonVariant::FilledTertiary => {
                let rc = with_alpha(&theme.colors.on_tertiary, 0.24).unwrap_or_default();
                (theme.colors.tertiary.clone(), theme.colors.on_tertiary.clone(), "transparent".into(), rc)
            }
            ButtonVariant::Outlined => {
                let rc = with_alpha(&theme.colors.primary, 0.12).unwrap_or_default();
                ("transparent".into(), theme.colors.primary.clone(), theme.colors.outline.clone(), rc)
            }
            ButtonVariant::Text => {
                let rc = with_alpha(&theme.colors.primary, 0.12).unwrap_or_default();
                ("transparent".into(), theme.colors.primary.clone(), "transparent".into(), rc)
            }
            ButtonVariant::Elevated => {
                let rc = with_alpha(&theme.colors.primary, 0.12).unwrap_or_default();
                (theme.colors.surface_container_low.clone(), theme.colors.primary.clone(), "transparent".into(), rc)
            }
            ButtonVariant::Icon => {
                let rc = with_alpha(&theme.colors.primary, 0.12).unwrap_or_default();
                ("transparent".into(), theme.colors.on_surface_variant.clone(), "transparent".into(), rc)
            }
            ButtonVariant::Fab | ButtonVariant::SmallFab | ButtonVariant::LargeFab | ButtonVariant::ExtendedFab => {
                let rc = with_alpha(&theme.colors.on_primary_container, 0.12).unwrap_or_default();
                (theme.colors.primary_container.clone(), theme.colors.on_primary_container.clone(), "transparent".into(), rc)
            }
            ButtonVariant::FabTertiary | ButtonVariant::SmallFabTertiary | ButtonVariant::LargeFabTertiary | ButtonVariant::ExtendedFabTertiary => {
                let rc = with_alpha(&theme.colors.on_tertiary_container, 0.12).unwrap_or_default();
                (theme.colors.tertiary_container.clone(), theme.colors.on_tertiary_container.clone(), "transparent".into(), rc)
            }
        }
    };

    let border_width = if matches!(props.variant, ButtonVariant::Outlined) {
        "1px"
    } else {
        "0"
    };

    let elevation_shadow = if !props.disabled {
        match props.variant {
            ButtonVariant::Elevated => theme.elevation(ElevationLevel::Level1).box_shadow.clone(),
            ButtonVariant::Fab | ButtonVariant::SmallFab | ButtonVariant::LargeFab | ButtonVariant::ExtendedFab |
            ButtonVariant::FabTertiary | ButtonVariant::SmallFabTertiary | ButtonVariant::LargeFabTertiary | ButtonVariant::ExtendedFabTertiary => {
                theme.elevation(ElevationLevel::Level3).box_shadow.clone()
            }
            _ => "none".into()
        }
    } else {
        "none".into()
    };

    let hover_shadow = if !props.disabled {
        match props.variant {
            ButtonVariant::Elevated => theme.elevation(ElevationLevel::Level2).box_shadow.clone(),
            ButtonVariant::Fab | ButtonVariant::SmallFab | ButtonVariant::LargeFab | ButtonVariant::ExtendedFab |
            ButtonVariant::FabTertiary | ButtonVariant::SmallFabTertiary | ButtonVariant::LargeFabTertiary | ButtonVariant::ExtendedFabTertiary => {
                theme.elevation(ElevationLevel::Level4).box_shadow.clone()
            }
            _ => "none".into()
        }
    } else {
        "none".into()
    };

    let state_layer_hover = if is_fab {
        if matches!(props.variant, ButtonVariant::FabTertiary | ButtonVariant::SmallFabTertiary | ButtonVariant::LargeFabTertiary | ButtonVariant::ExtendedFabTertiary) {
            with_alpha(&theme.colors.on_tertiary_container, 0.08).unwrap_or_default()
        } else {
            with_alpha(&theme.colors.on_primary_container, 0.08).unwrap_or_default()
        }
    } else {
        if matches!(props.variant, ButtonVariant::FilledTertiary) {
            with_alpha(&theme.colors.on_tertiary, 0.08).unwrap_or_default()
        } else {
            with_alpha(&theme.colors.primary, 0.08).unwrap_or_default()
        }
    };

    let state_layer_active = if is_fab {
        if matches!(props.variant, ButtonVariant::FabTertiary | ButtonVariant::SmallFabTertiary | ButtonVariant::LargeFabTertiary | ButtonVariant::ExtendedFabTertiary) {
            with_alpha(&theme.colors.on_tertiary_container, 0.12).unwrap_or_default()
        } else {
            with_alpha(&theme.colors.on_primary_container, 0.12).unwrap_or_default()
        }
    } else {
        if matches!(props.variant, ButtonVariant::FilledTertiary) {
            with_alpha(&theme.colors.on_tertiary, 0.12).unwrap_or_default()
        } else {
            with_alpha(&theme.colors.primary, 0.12).unwrap_or_default()
        }
    };

    let border_radius = match props.variant {
        ButtonVariant::Icon => "50%".into(),
        ButtonVariant::Fab | ButtonVariant::FabTertiary => theme.shapes.extra_large.clone(),
        ButtonVariant::SmallFab | ButtonVariant::SmallFabTertiary => theme.shapes.medium.clone(),
        ButtonVariant::LargeFab | ButtonVariant::LargeFabTertiary => theme.shapes.extra_large.clone(),
        ButtonVariant::ExtendedFab | ButtonVariant::ExtendedFabTertiary => "28px".into(),
        _ => "20px".into(), // Standard MD3 rounded pill buttons
    };

    // Min-width for standard buttons
    let min_width = if is_square_fab || matches!(props.variant, ButtonVariant::Icon) {
        actual_height
    } else if matches!(props.variant, ButtonVariant::ExtendedFab | ButtonVariant::ExtendedFabTertiary) {
        0
    } else {
        64
    };

    let display_mode = "inline-flex";

    let button_style = use_style!(
        r#"
        display: ${display_mode};
        align-items: center;
        justify-content: center;
        height: ${height}px;
        min-height: ${height}px;
        min-width: ${min_width}px;
        box-sizing: border-box;
        padding: 0 ${h_padding}px;
        border: ${border_width} solid ${border_color};
        border-radius: ${border_radius};
        background-color: ${bg};
        color: ${text_color};
        font-family: ${font_family}, sans-serif;
        font-size: ${font_size}px;
        font-weight: 500;
        letter-spacing: 0.1px;
        cursor: ${cursor};
        box-shadow: ${elevation_shadow};
        position: relative;
        overflow: hidden;
        user-select: none;
        -webkit-user-select: none;
        outline: none;
        -webkit-tap-highlight-color: transparent;
        transition: box-shadow 280ms cubic-bezier(0.4, 0, 0.2, 1), \
                    background-color 280ms cubic-bezier(0.4, 0, 0.2, 1), \
                    transform 150ms;

        button.disabled {
            box-shadow: none !important;
            transform: none !important;
        }

        &:not(:disabled):hover {
            box-shadow: ${hover_shadow};
        }

        &:not(:disabled):hover .state-layer-btn {
            opacity: 0.08;
        }

        &:not(:disabled):active {
            transform: scale(0.96);
        }

        &:not(:disabled):active .state-layer-btn {
            background-color: ${state_layer_active};
            opacity: 0.12;
        }
        "#,
        display_mode = display_mode,
        height = actual_height,
        h_padding = horizontal_padding,
        min_width = min_width,
        border_width = border_width,
        border_color = border_color,
        border_radius = border_radius,
        bg = bg,
        text_color = text_color,
        font_family = theme.font_family,
        font_size = font_size,
        cursor = if props.disabled { "default" } else { "pointer" },
        elevation_shadow = elevation_shadow,
        hover_shadow = hover_shadow,
        state_layer_active = state_layer_active,
    );

    // Inline style for FAB width (square FABs need explicit width to stay square)
    let fab_width_style = if is_square_fab {
        format!("width: {}px;", actual_height)
    } else {
        String::new()
    };
    // Full width mode
    let full_width_style = if props.full_width && !is_square_fab {
        "width: 100%".to_string()
    } else {
        String::new()
    };
    let combined_style = {
        let mut parts: Vec<String> = Vec::new();
        if !fab_width_style.is_empty() { parts.push(fab_width_style); }
        if !full_width_style.is_empty() { parts.push(full_width_style); }
        if !props.style.is_empty() { parts.push(props.style.clone()); }
        parts.join("; ")
    };

    let state_layer_style = use_style!(
        r#"
        position: absolute;
        inset: 0;
        opacity: 0;
        transition: opacity 150ms;
        pointer-events: none;
        "#,
    );

    let icon_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        justify-content: center;
        "#,
    );

    let content_span_style = use_style!(
        r#"
        position: relative;
        z-index: 1;
        pointer-events: none;
        display: flex;
        align-items: center;
        "#,
    );

    let button_override = theme.component_style("Button.root").map(|css| {
        dynamic_style(css.to_string())
    }).unwrap_or_default();

    let content_classes = yew::classes![
        "md-button",
        if props.disabled { "disabled" } else { "" },
        button_style.get_class_name().to_string(),
        button_override,
        &props.class
    ];

    html! {
        <button
            id={props.id.clone()}
            class={content_classes}
            disabled={props.disabled}
            onclick={props.onclick.clone()}
            style={combined_style}
        >
            // Ripple Component
            if !props.disabled {
                <Ripple color={ripple_color} />
            }

            // Hover state overlay layer
            <div class={yew::classes!["state-layer-btn", state_layer_style.get_class_name().to_string(), dynamic_style(format!("background-color: {};", state_layer_hover))]} />

            // Leading Icon
            if !props.icon.is_empty() {
                <span
                    class={yew::classes![icon_style.get_class_name().to_string(), dynamic_style(format!(
                        "margin-inline-end: {}; margin-inline-start: {}; width: {}px; height: {}px;",
                        if is_icon_only { "0" } else { "8px" },
                        if is_icon_only { "0" } else { "-4px" },
                        actual_icon_size,
                        actual_icon_size
                    ))]}
                >
                    <Icon name={props.icon.clone()} size={format!("{}px", actual_icon_size)} />
                </span>
            }

            // Inner content rendering
            if !is_icon_only {
                <span class={content_span_style.get_class_name().to_string()}>
                    if props.children.is_empty() {
                        { &props.label }
                    } else {
                        { props.children.clone() }
                    }
                </span>
            }

            // Trailing Icon
            if !props.trailing_icon.is_empty() && !is_icon_only {
                <span class={yew::classes![icon_style.get_class_name().to_string(), dynamic_style("margin-inline-start: 8px; margin-inline-end: -4px;".to_string())]}>
                    <Icon name={props.trailing_icon.clone()} size={format!("{}px", actual_icon_size)} />
                </span>
            }
        </button>
    }
}