//! Material Design 3 Divider
//!
//! Full-width and inset dividers with MD3 color tokens.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;

/// Divider variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DividerVariant {
    #[default]
    FullWidth,
    /// Indented from the left edge.
    Inset,
    /// Indented from the left edge, thicker for list subheaders.
    Middle,
}

#[derive(Properties, PartialEq)]
pub struct DividerProps {
    /// Divider variant.
    #[prop_or_default]
    pub variant: DividerVariant,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn Divider(props: &DividerProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let margin = match props.variant {
        DividerVariant::FullWidth => "0",
        DividerVariant::Inset => "0 0 0 16px",
        DividerVariant::Middle => "0 0 0 16px",
    };

    let style = use_style!(
        r#"
        border: none;
        border-top: 1px solid ${color};
        margin: ${margin};
        "#,
        color = theme.colors.outline_variant,
        margin = margin,
    );

    let component_override = theme.component_style("Divider").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <hr class={yew::classes![style.get_class_name().to_string(), &props.class, &component_override]} id={props.id.clone()} role="separator" />
    }
}