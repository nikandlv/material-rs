//! Material Design 3 Icon Component
//!
//! A thin wrapper around Material Symbols that provides a consistent API
//! for rendering icons across all components. Replaces raw
//! `<span class="material-symbols-outlined">` with a typed, themed component.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::utils::dynamic_style::dynamic_style;

/// Font variation settings for the icon.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IconWeight {
    #[default]
    Regular,
    /// Filled variant (weight 700).
    Filled,
    /// Light variant (weight 300).
    Light,
    /// Bold variant (weight 700).
    Bold,
}

impl IconWeight {
    pub fn variation_string(&self) -> &'static str {
        match self {
            IconWeight::Regular => "'FILL' 0, 'wght' 400, 'GRAD' 0, 'opsz' 24",
            IconWeight::Filled => "'FILL' 1, 'wght' 400, 'GRAD' 0, 'opsz' 24",
            IconWeight::Light => "'FILL' 0, 'wght' 300, 'GRAD' 0, 'opsz' 24",
            IconWeight::Bold => "'FILL' 0, 'wght' 700, 'GRAD' 0, 'opsz' 24",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct IconProps {
    /// Material icon name (ligature). E.g., "home", "search", "close".
    pub name: String,

    /// Icon size. Defaults to "24px".
    #[prop_or("24px".to_owned())]
    pub size: String,

    /// Icon color. Falls back to `theme.colors.on_surface_variant` when empty.
    #[prop_or_default]
    pub color: String,

    /// Font weight variant.
    #[prop_or_default]
    pub weight: IconWeight,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn Icon(props: &IconProps) -> Html {
    let icon_color = if props.color.is_empty() {
        "inherit".to_string()
    } else {
        props.color.clone()
    };

    let icon_style = use_style!(
        r#"
        display: inline-flex;
        align-items: center;
        justify-content: center;
        vertical-align: middle;
        user-select: none;
        flex-shrink: 0;
        line-height: 1;
        font-variation-settings: ${variation};
        "#,
        variation = props.weight.variation_string(),
    );

    let icon_dynamic_class = dynamic_style(format!("font-size: {}; color: {};", props.size, icon_color));

    html! {
        <span
            class={yew::classes!["material-symbols-outlined", icon_style.get_class_name().to_string(), &props.class, icon_dynamic_class]}
            id={props.id.clone()}
            aria-hidden="true"
        >
            { &props.name }
        </span>
    }
}
