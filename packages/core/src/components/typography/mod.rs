//! Material Design 3 Typography component
//!
//! Renders text using the official MD3 typography scale roles (Display, Headline, Title, Body, Label)
//! onto customizable HTML tags (h1-h6, p, span, div, etc.).

use stylist::yew::use_style;
use yew::prelude::*;

use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;

/// MD3 Typography variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TypographyVariant {
    DisplayLarge,
    DisplayMedium,
    DisplaySmall,
    HeadlineLarge,
    HeadlineMedium,
    HeadlineSmall,
    TitleLarge,
    TitleMedium,
    TitleSmall,
    BodyLarge,
    #[default]
    BodyMedium,
    BodySmall,
    LabelLarge,
    LabelMedium,
    LabelSmall,
}

#[derive(Properties, PartialEq)]
pub struct TypographyProps {
    /// HTML tag to render. Defaults to "p".
    #[prop_or("p".to_owned())]
    pub tag: String,

    /// MD3 Type role variant. Defaults to BodyMedium.
    #[prop_or_default]
    pub variant: TypographyVariant,

    /// Custom inline CSS style.
    #[prop_or_default]
    pub style: String,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,

    /// Content children.
    #[prop_or_default]
    pub children: Children,
}

#[component]
pub fn Typography(props: &TypographyProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    // Select the correct TypeRole from theme
    let role = match props.variant {
        TypographyVariant::DisplayLarge => &theme.typography.display_large,
        TypographyVariant::DisplayMedium => &theme.typography.display_medium,
        TypographyVariant::DisplaySmall => &theme.typography.display_small,
        TypographyVariant::HeadlineLarge => &theme.typography.headline_large,
        TypographyVariant::HeadlineMedium => &theme.typography.headline_medium,
        TypographyVariant::HeadlineSmall => &theme.typography.headline_small,
        TypographyVariant::TitleLarge => &theme.typography.title_large,
        TypographyVariant::TitleMedium => &theme.typography.title_medium,
        TypographyVariant::TitleSmall => &theme.typography.title_small,
        TypographyVariant::BodyLarge => &theme.typography.body_large,
        TypographyVariant::BodyMedium => &theme.typography.body_medium,
        TypographyVariant::BodySmall => &theme.typography.body_small,
        TypographyVariant::LabelLarge => &theme.typography.label_large,
        TypographyVariant::LabelMedium => &theme.typography.label_medium,
        TypographyVariant::LabelSmall => &theme.typography.label_small,
    };

    let style = use_style!(
        r#"
        font-family: ${font_family}, sans-serif;
        font-size: ${size}px;
        line-height: ${line_height}px;
        font-weight: ${weight};
        letter-spacing: ${tracking}px;
        text-transform: ${case};
        margin: 0;
        padding: 0;
        "#,
        font_family = role.font_family,
        size = role.font_size,
        line_height = role.line_height,
        weight = role.weight.as_str(),
        tracking = role.tracking,
        case = role.text_case.css_value(),
    );

    let component_override = theme.component_style("Typography").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    let html_tag = props.tag.to_lowercase();
    let class_name = yew::classes![style.get_class_name().to_string(), &props.class, &component_override];
    let custom_style = props.style.clone();
    let children = props.children.clone();
    let id = props.id.clone();

    // Render using correct tag to maintain semantic HTML
    match html_tag.as_str() {
        "h1" => html! { <h1 id={id} class={class_name} style={custom_style}>{ children }</h1> },
        "h2" => html! { <h2 id={id} class={class_name} style={custom_style}>{ children }</h2> },
        "h3" => html! { <h3 id={id} class={class_name} style={custom_style}>{ children }</h3> },
        "h4" => html! { <h4 id={id} class={class_name} style={custom_style}>{ children }</h4> },
        "h5" => html! { <h5 id={id} class={class_name} style={custom_style}>{ children }</h5> },
        "h6" => html! { <h6 id={id} class={class_name} style={custom_style}>{ children }</h6> },
        "span" => html! { <span id={id} class={class_name} style={custom_style}>{ children }</span> },
        "div" => html! { <div id={id} class={class_name} style={custom_style}>{ children }</div> },
        "label" => html! { <label id={id} class={class_name} style={custom_style}>{ children }</label> },
        "a" => html! { <a id={id} class={class_name} style={custom_style}>{ children }</a> },
        _ => html! { <p id={id} class={class_name} style={custom_style}>{ children }</p> },
    }
}
