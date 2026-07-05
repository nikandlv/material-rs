//! Material Design 3 List Items
//!
//! One-line, two-line, and three-line list items with leading/trailing
//! content areas.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;


/// List item variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ListItemVariant {
    /// Single line of text.
    #[default]
    OneLine,
    /// Two lines (headline + supporting text).
    TwoLine,
    /// Three lines (headline + overline + supporting text).
    ThreeLine,
}

#[derive(Properties, PartialEq)]
pub struct ListItemProps {
    /// Headline text.
    pub headline: String,

    /// Supporting text (line 2).
    #[prop_or_default]
    pub supporting_text: String,

    /// Overline text (shown above headline for three-line variant).
    #[prop_or_default]
    pub overline: String,

    /// Leading content (icon, avatar, etc.).
    #[prop_or_default]
    pub leading: Html,

    /// Trailing content (icon, checkbox, etc.).
    #[prop_or_default]
    pub trailing: Html,

    /// Whether this item is interactive.
    #[prop_or(true)]
    pub interactive: bool,

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
pub fn ListItem(props: &ListItemProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let variant = if !props.overline.is_empty() {
        ListItemVariant::ThreeLine
    } else if !props.supporting_text.is_empty() {
        ListItemVariant::TwoLine
    } else {
        ListItemVariant::OneLine
    };

    let height = match variant {
        ListItemVariant::OneLine => 56,
        ListItemVariant::TwoLine => 72,
        ListItemVariant::ThreeLine => 88,
    };

    let on_click = {
        let onclick = props.onclick.clone();
        let interactive = props.interactive;
        Callback::from(move |e: MouseEvent| {
            if interactive {
                onclick.emit(e);
            }
        })
    };

    let item_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        width: 100%;
        padding: 0 16px;
        cursor: pointer;
        position: relative;
        box-sizing: border-box;
        flex-shrink: 0;
        border-radius: inherit;
        margin: 0;
        background-color: transparent;
        overflow: hidden;
        transition: background-color 200ms, transform 150ms cubic-bezier(0.2, 0, 0, 1);
        -webkit-tap-highlight-color: transparent;
        "#,
    );

    let hover_style = use_style!(
        r#"
        .md-list-item:hover .state-layer {
            opacity: 0.08 !important;
        }
        .md-list-item:active {
            transform: scale(0.99);
        }
        "#,
    );

    let state_layer_style = use_style!(
        r#"
        position: absolute;
        inset: 0;
        border-radius: inherit;
        opacity: 0;
        transition: opacity 150ms;
        pointer-events: none;
        "#,
    );

    let leading_style = use_style!(
        r#"
        margin-inline-end: 16px;
        display: flex;
        align-items: center;
        flex-shrink: 0;
        font-size: 24px;
        position: relative;
        z-index: 1;
        "#,
    );

    let content_style = use_style!(
        r#"
        flex: 1;
        min-width: 0;
        overflow: hidden;
        position: relative;
        z-index: 1;
        "#,
    );

    let overline_style = use_style!(
        r#"
        font-size: 12px;
        font-weight: 500;
        letter-spacing: 0.4px;
        margin-bottom: 2px;
        "#,
    );

    let headline_style = use_style!(
        r#"
        font-size: 16px;
        font-weight: 500;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        "#,
    );

    let supporting_style = use_style!(
        r#"
        font-size: 14px;
        font-weight: 400;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        margin-top: 2px;
        "#,
    );

    let trailing_style = use_style!(
        r#"
        margin-inline-start: 16px;
        display: flex;
        align-items: center;
        flex-shrink: 0;
        position: relative;
        z-index: 1;
        "#,
    );

    let item_height_font_class = dynamic_style(format!("height: {}px; font-family: {}, sans-serif;", height, theme.font_family));
    let state_layer_bg_class = dynamic_style(format!("background: {};", theme.colors.on_surface));
    let leading_color_class = dynamic_style(format!("color: {};", theme.colors.on_surface_variant));
    let overline_color_class = dynamic_style(format!("color: {};", theme.colors.on_surface_variant));
    let headline_color_class = dynamic_style(format!("color: {};", theme.colors.on_surface));
    let supporting_color_class = dynamic_style(format!("color: {};", theme.colors.on_surface_variant));

    let component_override = theme.component_style("ListItem").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div
            class={yew::classes![item_style.get_class_name().to_string(), hover_style.get_class_name().to_string(), &props.class, "md-list-item", item_height_font_class, &component_override]}
            id={props.id.clone()}
            onclick={on_click}
            role={if props.interactive { Some("listitem") } else { None }}
        >
            // State layer for hover feedback
            if props.interactive {
                <div class={yew::classes![state_layer_style.get_class_name().to_string(), state_layer_bg_class]} />
            }

            // Leading area
            if props.leading != Html::default() {
                <div class={yew::classes![leading_style.get_class_name().to_string(), leading_color_class]}>
                    { props.leading.clone() }
                </div>
            }

            // Content area
            <div class={content_style.get_class_name().to_string()}>
                if !props.overline.is_empty() {
                    <div class={yew::classes![overline_style.get_class_name().to_string(), overline_color_class]}>
                        { &props.overline }
                    </div>
                }
                <div class={yew::classes![headline_style.get_class_name().to_string(), headline_color_class]}>
                    { &props.headline }
                </div>
                if !props.supporting_text.is_empty() {
                    <div class={yew::classes![supporting_style.get_class_name().to_string(), supporting_color_class]}>
                        { &props.supporting_text }
                    </div>
                }
            </div>

            // Trailing area
            if props.trailing != Html::default() {
                <div class={trailing_style.get_class_name().to_string()}>
                    { props.trailing.clone() }
                </div>
            }
        </div>
    }
}
