//! Material Design 3 Card
//!
//! Supports Elevated, Filled, and Outlined card variants with
//! proper MD3 surface container colors and elevation.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::ripple::Ripple;
use crate::theme::Theme;
use crate::theme::elevation::ElevationLevel;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

/// Card variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CardVariant {
    #[default]
    Elevated,
    Filled,
    Outlined,
}

#[derive(Properties, PartialEq)]
pub struct CardProps {
    /// Card variant.
    #[prop_or_default]
    pub variant: CardVariant,

    /// Whether the card is interactive (shows ripple on click).
    #[prop_or(false)]
    pub interactive: bool,

    /// Click callback.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    /// Whether the card is outlined in error color.
    #[prop_or(false)]
    pub error: bool,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,

    /// Optional header image URL.
    #[prop_or_default]
    pub image: String,

    /// Image aspect ratio (e.g., "16/9", "4/3", "1/1"). Defaults to "16/9".
    #[prop_or("16/9".to_owned())]
    pub image_ratio: String,

    /// Alt text for the header image.
    #[prop_or_default]
    pub image_alt: String,

    /// Card content.
    #[prop_or_default]
    pub children: Children,
}

#[component]
pub fn Card(props: &CardProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let (bg, border, shadow, ripple_color) = match props.variant {
        CardVariant::Elevated => {
            let elev = theme.elevation(ElevationLevel::Level1);
            (
                theme.surface_container(1).to_owned(),
                "none".into(),
                elev.box_shadow,
                with_alpha(&theme.colors.primary, 0.08).unwrap_or_default(),
            )
        }
        CardVariant::Filled => {
            (
                theme.colors.surface_container_highest.clone(),
                "none".into(),
                "none".into(),
                with_alpha(&theme.colors.primary, 0.08).unwrap_or_default(),
            )
        }
        CardVariant::Outlined => {
            let border_color = if props.error {
                theme.colors.error.clone()
            } else {
                theme.colors.outline_variant.clone()
            };
            (
                theme.surface_container(0).to_owned(),
                format!("1px solid {}", border_color),
                "none".into(),
                with_alpha(&theme.colors.primary, 0.08).unwrap_or_default(),
            )
        }
    };

    let hover_shadow = if props.interactive {
        match props.variant {
            CardVariant::Elevated => theme.elevation(ElevationLevel::Level2).box_shadow.clone(),
            CardVariant::Filled | CardVariant::Outlined => theme.elevation(ElevationLevel::Level1).box_shadow.clone(),
        }
    } else {
        shadow.clone()
    };

    let hover_transform = if props.interactive {
        "translateY(-2px) scale(1.01)"
    } else {
        "none"
    };

    let active_transform = if props.interactive {
        "translateY(0) scale(0.98)"
    } else {
        "none"
    };

    let card_style = use_style!(
        r#"
        position: relative;
        overflow: hidden;
        border-radius: ${radius};
        background-color: ${bg};
        border: ${border};
        box-shadow: ${shadow};
        padding: ${padding};
        cursor: ${cursor};
        transition: transform 200ms cubic-bezier(0.34, 1.56, 0.64, 1), box-shadow 200ms cubic-bezier(0.2, 0, 0, 1), background-color 200ms;
        outline: none;

        &:hover {
            transform: ${hover_transform};
            box-shadow: ${hover_shadow};
        }

        &:active {
            transform: ${active_transform};
        }

        &:focus-visible {
            outline: 2px solid ${focus_ring};
            outline-offset: 2px;
        }
        "#,
        radius = theme.shapes.large,
        bg = bg,
        border = border,
        shadow = shadow,
        cursor = if props.interactive { "pointer" } else { "default" },
        hover_transform = hover_transform,
        hover_shadow = hover_shadow,
        active_transform = active_transform,
        focus_ring = theme.colors.primary,
        padding = if props.image.is_empty() { "16px" } else { "0" },
    );

    let children_padding_style = use_style!(
        r#"
        padding: 16px;
        box-sizing: border-box;
        "#,
    );

    let ripple = if props.interactive {
        html! { <Ripple color={ripple_color} /> }
    } else {
        html! {}
    };

    let component_override = theme.component_style("Card").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div
            class={yew::classes![card_style.get_class_name().to_string(), &props.class, &component_override]}
            id={props.id.clone()}
            onclick={props.onclick.clone()}
            role={if props.interactive { Some("button") } else { None }}
            tabindex={if props.interactive { Some("0") } else { None }}
        >
            { ripple }
            { if !props.image.is_empty() {
                let image_container_class = dynamic_style(format!("width: 100%; aspect-ratio: {}; overflow: hidden; border-radius: {} {} 0 0;", props.image_ratio, theme.shapes.large, theme.shapes.large));
                html! {
                    <div class={image_container_class}>
                        <img src={props.image.clone()} alt={props.image_alt.clone()} style="width: 100%; height: 100%; object-fit: cover; display: block;" loading="lazy" />
                    </div>
                }
            } else {
                html! {}
            } }
            if !props.image.is_empty() {
                <div class={children_padding_style.get_class_name().to_string()}>
                    { props.children.clone() }
                </div>
            } else {
                { props.children.clone() }
            }
        </div>
    }
}