//! Material Design 3 Grid Layout Component
//!
//! Renders responsive grid layouts matching 12-column Grid containers and span cells.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;

#[derive(Properties, PartialEq)]
pub struct GridProps {
    /// Renders as a grid container. Defaults to false.
    #[prop_or(false)]
    pub container: bool,

    /// Renders as a grid item cell. Defaults to false.
    #[prop_or(false)]
    pub item: bool,

    /// Total grid columns for container. Defaults to 12.
    #[prop_or(12)]
    pub columns: u32,

    /// Row/column spacing gap in pixels or CSS string (e.g. "8", "16px"). Defaults to "16px".
    #[prop_or("16px".to_owned())]
    pub spacing: String,

    // Column spans (1 to 12) for item cells
    #[prop_or_default] pub xs: Option<u32>,
    #[prop_or_default] pub sm: Option<u32>,
    #[prop_or_default] pub md: Option<u32>,
    #[prop_or_default] pub lg: Option<u32>,
    #[prop_or_default] pub xl: Option<u32>,

    // Alignment
    #[prop_or_default] pub justify_content: Option<String>,
    #[prop_or_default] pub align_items: Option<String>,

    /// HTML element tag. Defaults to "div".
    #[prop_or("div".to_owned())]
    pub tag: String,

    #[prop_or_default] pub style: String,
    #[prop_or_default] pub id: String,
    #[prop_or_default] pub class: String,
    #[prop_or_default] pub children: Children,
}

#[component]
pub fn Grid(props: &GridProps) -> Html {
    let _theme = use_context::<Theme>().expect("Theme context required");

    let spacing_css = if props.spacing.chars().all(char::is_numeric) {
        format!("{}px", props.spacing)
    } else {
        props.spacing.clone()
    };

    // Responsive media styles for item cell spans using Stylist
    let style = use_style!(
        r#"
        box-sizing: border-box;

        /* Container styles */
        display: ${display};
        grid-template-columns: repeat(${cols}, 1fr);
        gap: ${gap};
        justify-content: ${justify};
        align-items: ${align};

        /* Default item styles */
        grid-column: span ${xs_span};

        /* Breakpoints spans */
        @media (min-width: 600px) {
            grid-column: span ${sm_span};
        }
        @media (min-width: 905px) {
            grid-column: span ${md_span};
        }
        @media (min-width: 1240px) {
            grid-column: span ${lg_span};
        }
        @media (min-width: 1440px) {
            grid-column: span ${xl_span};
        }
        "#,
        display = if props.container { "grid" } else { "block" },
        cols = props.columns,
        gap = spacing_css,
        justify = props.justify_content.clone().unwrap_or_else(|| "stretch".into()),
        align = props.align_items.clone().unwrap_or_else(|| "stretch".into()),
        
        // Dynamic spans
        xs_span = props.xs.unwrap_or(12),
        sm_span = props.sm.or(props.xs).unwrap_or(12),
        md_span = props.md.or(props.sm).or(props.xs).unwrap_or(12),
        lg_span = props.lg.or(props.md).or(props.sm).or(props.xs).unwrap_or(12),
        xl_span = props.xl.or(props.lg).or(props.md).or(props.sm).or(props.xs).unwrap_or(12),
    );

    let component_override = _theme.component_style("Grid").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    let html_tag = props.tag.to_lowercase();
    let class_name = yew::classes![style.get_class_name().to_string(), &props.class, &component_override];
    let custom_style = props.style.clone();
    let children = props.children.clone();
    let id = props.id.clone();

    match html_tag.as_str() {
        "section" => html! { <section id={id} class={class_name} style={custom_style}>{ children }</section> },
        "main" => html! { <main id={id} class={class_name} style={custom_style}>{ children }</main> },
        "span" => html! { <span id={id} class={class_name} style={custom_style}>{ children }</span> },
        _ => html! { <div id={id} class={class_name} style={custom_style}>{ children }</div> },
    }
}
