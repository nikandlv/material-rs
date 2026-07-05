//! Material Design 3 Icon Button
//!
//! Standalone icon button component with MD3 styling.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::components::ripple::Ripple;
use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

/// Icon button variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IconButtonVariant {
    /// Standard icon button (transparent background).
    #[default]
    Standard,
    /// Filled background (primary container).
    Filled,
    /// Tonal background.
    Tonal,
    /// Outlined with border.
    Outlined,
}

#[derive(Properties, PartialEq)]
pub struct IconButtonProps {
    /// Icon text/symbol.
    pub icon: String,

    /// Button variant.
    #[prop_or_default]
    pub variant: IconButtonVariant,

    /// Whether the button is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// Accessible label.
    #[prop_or_default]
    pub label: String,

    /// Click callback.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn IconButton(props: &IconButtonProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let (bg, text_color, border) = if props.disabled {
        let on_surface_04 = with_alpha(&theme.colors.on_surface, 0.04).unwrap_or_default();
        let on_surface_38 = with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default();
        (on_surface_04, on_surface_38, "none".into())
    } else {
        match props.variant {
            IconButtonVariant::Standard => (
                "transparent".into(),
                theme.colors.on_surface_variant.clone(),
                "none".into(),
            ),
            IconButtonVariant::Filled => (
                theme.colors.primary.clone(),
                theme.colors.on_primary.clone(),
                "none".into(),
            ),
            IconButtonVariant::Tonal => (
                theme.colors.secondary_container.clone(),
                theme.colors.on_secondary_container.clone(),
                "none".into(),
            ),
            IconButtonVariant::Outlined => (
                "transparent".into(),
                theme.colors.on_surface_variant.clone(),
                format!("1px solid {}", theme.colors.outline),
            ),
        }
    };

    let ripple_color = if props.disabled {
        "transparent".into()
    } else {
        with_alpha(&theme.colors.primary, 0.12).unwrap_or_default()
    };

    let btn_style = use_style!(r#"
        position: relative;
        overflow: hidden;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 40px;
        height: 40px;
        border-radius: 9999px;
        cursor: pointer;
        outline: none;
        font-size: 24px;
        padding: 0;
        transition: background-color 200ms cubic-bezier(0.2, 0, 0, 1), transform 150ms cubic-bezier(0.34, 1.56, 0.64, 1);
        -webkit-tap-highlight-color: transparent;

        &:hover {
            transform: scale(1.08);
        }

        &:hover::before {
            opacity: 1;
        }

        &:active {
            transform: scale(0.92);
        }

        &:focus-visible {
            outline: 2px solid currentColor;
            outline-offset: 2px;
        }

        &:disabled {
            cursor: default;
        }

        &:disabled:hover {
            transform: none;
        }

        &:disabled:active {
            transform: none;
        }
    "#);

    let icon_span_style = use_style!(
        r#"
        position: relative;
        z-index: 1;
        pointer-events: none;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 24px;
        height: 24px;
        font-size: 24px;
        line-height: 1;
        "#,
    );

    let btn_dynamic_class = dynamic_style(format!(
        "border: {}; background-color: {}; color: {};",
        border, bg, text_color
    ));

    let component_override = theme.component_style("IconButton").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <button
            class={yew::classes![btn_style.get_class_name().to_string(), &props.class, btn_dynamic_class, &component_override]}
            id={props.id.clone()}
            disabled={props.disabled}
            onclick={props.onclick.clone()}
            aria-label={if !props.label.is_empty() { Some(props.label.clone()) } else { None }}
        >
            <Ripple color={ripple_color} />
            <span class={icon_span_style.get_class_name().to_string()}>
                <Icon name={props.icon.clone()} />
            </span>
        </button>
    }
}
