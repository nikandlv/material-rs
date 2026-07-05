//! Material Design 3 Spacer Utility Component
//!
//! A flexbox spacing helper that expands to fill empty spaces or provides fixed gap widths/heights.

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SpacerProps {
    /// Optional fixed width (e.g. "16px"). If set, disables automatic flexible expansion.
    #[prop_or_default]
    pub width: String,

    /// Optional fixed height (e.g. "24px"). If set, disables automatic flexible expansion.
    #[prop_or_default]
    pub height: String,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn Spacer(props: &SpacerProps) -> Html {
    let style = if !props.width.is_empty() || !props.height.is_empty() {
        format!(
            "display: inline-block; width: {}; height: {}; flex-shrink: 0; flex-grow: 0;",
            if props.width.is_empty() { "auto" } else { &props.width },
            if props.height.is_empty() { "auto" } else { &props.height }
        )
    } else {
        "flex: 1;".to_owned()
    };

    html! {
        <div
            class={props.class.clone()}
            id={props.id.clone()}
            style={style}
        />
    }
}
