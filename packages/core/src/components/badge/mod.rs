//! Material Design 3 Badge
//!
//! Small overlay indicator showing count or status.

use yew::prelude::*;

use crate::components::box_layout::Box;
use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;

/// Badge size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BadgeSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    /// Numeric count (0 = dot badge).
    #[prop_or(0)]
    pub count: usize,

    /// Whether to show as a dot (no number).
    #[prop_or(false)]
    pub dot: bool,

    /// Whether the badge is hidden (zero count and not dot).
    #[prop_or(false)]
    pub is_hidden: bool,

    /// Badge size.
    #[prop_or_default]
    pub size: BadgeSize,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,

    /// Child content (the element the badge is anchored to).
    #[prop_or_default]
    pub children: Children,
}

#[component]
pub fn Badge(props: &BadgeProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let show_badge = !props.is_hidden && (props.dot || props.count > 0);

    let (badge_size, font_size) = match props.size {
        BadgeSize::Small => ("6px", "0px"),
        BadgeSize::Medium => ("16px", "11px"),
        BadgeSize::Large => ("20px", "12px"),
    };

    let badge_html = if show_badge {
        let bg = &theme.colors.error;
        let text = &theme.colors.on_error;

        if props.dot {
            html! {
                <Box
                    position="absolute"
                    top="-2px"
                    right="-2px"
                    width="6px"
                    height="6px"
                    border_radius="50%"
                    bg={bg.clone()}
                    style="min-width: 6px; z-index: 1;"
                />
            }
        } else {
            let badge_class = dynamic_style(format!("min-width: {}; font-size: {}; font-weight: 500; display: flex; align-items: center; justify-content: center; padding: 0 4px; z-index: 1; font-family: {}, sans-serif; line-height: 1; transition: min-width 200ms cubic-bezier(0.2, 0, 0, 1), width 200ms cubic-bezier(0.2, 0, 0, 1), transform 200ms cubic-bezier(0.34, 1.56, 0.64, 1); transform-origin: top right;", badge_size, font_size, theme.font_family));
            html! {
                <Box
                    position="absolute"
                    top="-6px"
                    right="-6px"
                    width={badge_size.to_string()}
                    height={badge_size.to_string()}
                    border_radius="9999px"
                    bg={bg.clone()}
                    color={text.clone()}
                    class={badge_class}
                >
                    if props.count > 999 {
                        { "999+" }
                    } else {
                        { props.count.to_string() }
                    }
                </Box>
            }
        }
    } else {
        html! {}
    };

    let component_override = theme.component_style("Badge").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <Box
            display="inline-flex"
            position="relative"
            id={props.id.clone()}
            class={format!("{} {}", props.class, component_override)}
        >
            { props.children.clone() }
            { badge_html }
        </Box>
    }
}
