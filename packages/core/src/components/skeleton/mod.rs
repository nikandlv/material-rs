//! Material Design 3 Skeleton
//!
//! A loading placeholder component with shimmer animation.
//! Supports text, circular, and rectangular shapes with configurable dimensions.

use stylist::css;
use stylist::yew::{use_style, Global};
use yew::prelude::*;

use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

/// Shape variant for the skeleton.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SkeletonShape {
    #[default]
    Text,
    Circle,
    Rounded,
}

#[derive(Properties, PartialEq)]
pub struct SkeletonProps {
    #[prop_or_default]
    pub shape: SkeletonShape,

    #[prop_or("100%".into())]
    pub width: String,

    #[prop_or("16px".into())]
    pub height: String,

    #[prop_or_default]
    pub border_radius: String,

    #[prop_or(true)]
    pub animated: bool,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn Skeleton(props: &SkeletonProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let bg_base = with_alpha(&theme.colors.on_surface, 0.08).unwrap_or_default();
    let shimmer_highlight = with_alpha(&theme.colors.on_surface, 0.12).unwrap_or_default();

    let radius = if !props.border_radius.is_empty() {
        props.border_radius.clone()
    } else {
        match props.shape {
            SkeletonShape::Text => "4px".into(),
            SkeletonShape::Circle => "50%".into(),
            SkeletonShape::Rounded => theme.shapes.medium.clone(),
        }
    };

    let skeleton_box_style = use_style!(
        r#"
        display: inline-block;
        position: relative;
        overflow: hidden;
        vertical-align: middle;
        "#,
    );

    let shimmer_class = if props.animated { "md-skeleton-shimmer" } else { "" };

    let bg_value = if props.animated {
        format!("background-image: linear-gradient(90deg, transparent 0%, {} 50%, transparent 100%); background-size: 200% 100%;", shimmer_highlight)
    } else {
        "background: transparent;".into()
    };

    let skeleton_dynamic_class = dynamic_style(format!(
        "width: {}; height: {}; background-color: {}; border-radius: {}; {}",
        props.width, props.height, bg_base, radius, bg_value
    ));

    let component_override = theme.component_style("Skeleton").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <>
            if props.animated {
                <Global css={css!(r#"
                    @keyframes md-skeleton-shimmer {
                        0% { background-position: 200% 0; }
                        100% { background-position: -200% 0; }
                    }
                    .md-skeleton-shimmer {
                        animation: md-skeleton-shimmer 2.5s ease-in-out infinite;
                    }
                "#)} />
            }
            <div
                class={yew::classes![skeleton_box_style.get_class_name().to_string(), shimmer_class, &props.class, skeleton_dynamic_class, &component_override]}
                id={props.id.clone()}
                aria-hidden="true"
            />
        </>
    }
}

/// A skeleton loader for a multi-line text block.
#[derive(Properties, PartialEq)]
pub struct SkeletonTextProps {
    #[prop_or(3)]
    pub lines: u32,

    #[prop_or("100%".into())]
    pub first_width: String,

    #[prop_or_default]
    pub line_width: String,

    #[prop_or("14px".into())]
    pub line_height: String,

    #[prop_or("8px".into())]
    pub gap: String,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn SkeletonText(props: &SkeletonTextProps) -> Html {
    let fallback_widths = ["100%", "85%", "70%", "90%", "60%", "75%", "95%", "50%"];

    let lines: Vec<Html> = (0..props.lines)
        .map(|i| {
            let w = if !props.line_width.is_empty() {
                props.line_width.clone()
            } else if i == 0 {
                props.first_width.clone()
            } else {
                fallback_widths[(i as usize) % fallback_widths.len()].to_string()
            };

            html! {
                <Skeleton
                    shape={SkeletonShape::Text}
                    width={w}
                    height={props.line_height.clone()}
                />
            }
        })
        .collect();

    let wrapper_style = use_style!(
        r#"
        display: flex;
        flex-direction: column;
        "#,
    );

    let gap_class = dynamic_style(format!("gap: {};", props.gap));

    html! {
        <div
            id={props.id.clone()}
            class={yew::classes![wrapper_style.get_class_name().to_string(), &props.class, gap_class]}
        >
            { for lines }
        </div>
    }
}

/// A skeleton loader for a card-like content block.
#[derive(Properties, PartialEq)]
pub struct SkeletonCardProps {
    #[prop_or(true)]
    pub show_media: bool,

    #[prop_or("200px".into())]
    pub media_height: String,

    #[prop_or(2)]
    pub text_lines: u32,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn SkeletonCard(props: &SkeletonCardProps) -> Html {
    let wrapper_style = use_style!(
        r#"
        display: flex;
        flex-direction: column;
        gap: 12px;
        width: 100%;
        "#,
    );

    html! {
        <div
            id={props.id.clone()}
            class={yew::classes![wrapper_style.get_class_name().to_string(), &props.class]}
        >
            if props.show_media {
                <Skeleton
                    shape={SkeletonShape::Rounded}
                    width="100%"
                    height={props.media_height.clone()}
                />
            }
            <SkeletonText
                lines={props.text_lines}
                first_width="100%"
                line_width="80%"
                line_height="14px"
                gap="8px"
            />
        </div>
    }
}
