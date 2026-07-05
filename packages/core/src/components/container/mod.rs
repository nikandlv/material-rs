//! Material Design 3 Container Component
//!
//! A center-aligned layout wrapper that limits page content maximum widths, heights, and paddings.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    /// Maximum width breakpoint. Value is one of "xs", "sm", "md", "lg", "xl", or custom values like "800px". Defaults to "lg".
    #[prop_or("lg".to_owned())]
    pub max_width: String,

    /// Removes side margin gutter paddings. Defaults to false.
    #[prop_or(false)]
    pub disable_gutters: bool,

    /// Layout padding sizing config: "none", "small" (8px), "medium" (16px), "large" (24px). Defaults to "medium".
    #[prop_or("medium".to_owned())]
    pub padding: String,

    /// Height constraint: "auto", "min" (min-content), "max" (max-content), or exact css like "100vh". Defaults to "auto".
    #[prop_or("auto".to_owned())]
    pub height: String,

    /// HTML element tag. Defaults to "div".
    #[prop_or("div".to_owned())]
    pub tag: String,

    #[prop_or_default] pub style: String,
    #[prop_or_default] pub id: String,
    #[prop_or_default] pub class: String,
    #[prop_or_default] pub children: Children,
}

#[component]
pub fn Container(props: &ContainerProps) -> Html {
    let _theme = use_context::<Theme>().expect("Theme context required");

    let max_w_val = match props.max_width.as_str() {
        "xs" => "444px",
        "sm" => "600px",
        "md" => "905px",
        "lg" => "1240px",
        "xl" => "1440px",
        "none" => "100%",
        other => other,
    };

    let padding_val = if props.disable_gutters {
        "0"
    } else {
        match props.padding.as_str() {
            "none" => "0",
            "small" => "8px",
            "medium" => "16px",
            "large" => "24px",
            other => other,
        }
    };

    let height_val = match props.height.as_str() {
        "auto" => "auto",
        "min" => "min-content",
        "max" => "max-content",
        other => other,
    };

    let style = use_style!(
        r#"
        box-sizing: border-box;
        width: 100%;
        margin-inline-start: auto;
        margin-inline-end: auto;
        max-width: ${max_width};
        padding-inline-start: ${padding};
        padding-inline-end: ${padding};
        height: ${height};
        display: block;
        "#,
        max_width = max_w_val,
        padding = padding_val,
        height = height_val,
    );

    let component_override = _theme.component_style("Container").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    let html_tag = props.tag.to_lowercase();
    let class_name = yew::classes![style.get_class_name().to_string(), &props.class, &component_override];
    let custom_style = props.style.clone();
    let children = props.children.clone();
    let id = props.id.clone();

    match html_tag.as_str() {
        "main" => html! { <main id={id} class={class_name} style={custom_style}>{ children }</main> },
        "section" => html! { <section id={id} class={class_name} style={custom_style}>{ children }</section> },
        _ => html! { <div id={id} class={class_name} style={custom_style}>{ children }</div> },
    }
}
