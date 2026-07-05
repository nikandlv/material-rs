//! Material Design 3 Tooltip
//!
//! A lightweight label that appears on hover/focus near an anchor element.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;

/// Tooltip position relative to the anchor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    /// Tooltip text.
    pub text: String,

    /// Position relative to child.
    #[prop_or_default]
    pub position: TooltipPosition,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Child content (anchor element).
    #[prop_or_default]
    pub children: Children,
}

#[component]
pub fn Tooltip(props: &TooltipProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let visible = use_state(|| false);

    let on_mouse_enter = {
        let visible = visible.clone();
        Callback::from(move |_: MouseEvent| visible.set(true))
    };
    let on_mouse_leave = {
        let visible = visible.clone();
        Callback::from(move |_: MouseEvent| visible.set(false))
    };
    let on_focus = {
        let visible = visible.clone();
        Callback::from(move |_: FocusEvent| visible.set(true))
    };
    let on_blur = {
        let visible = visible.clone();
        Callback::from(move |_: FocusEvent| visible.set(false))
    };

    let (position_css, active_transform, inactive_transform) = match props.position {
        TooltipPosition::Top => (
            "inset-block-end: 100%; inset-inline-start: 50%; margin-block-end: 8px;",
            "translateX(-50%) translateY(0) scale(1)",
            "translateX(-50%) translateY(6px) scale(0.85)",
        ),
        TooltipPosition::Bottom => (
            "inset-block-start: 100%; inset-inline-start: 50%; margin-block-start: 8px;",
            "translateX(-50%) translateY(0) scale(1)",
            "translateX(-50%) translateY(-6px) scale(0.85)",
        ),
        TooltipPosition::Left => (
            "inset-inline-end: 100%; top: 50%; margin-inline-end: 8px;",
            "translateY(-50%) translateX(0) scale(1)",
            "translateY(-50%) translateX(6px) scale(0.85)",
        ),
        TooltipPosition::Right => (
            "inset-inline-start: 100%; top: 50%; margin-inline-start: 8px;",
            "translateY(-50%) translateX(0) scale(1)",
            "translateY(-50%) translateX(-6px) scale(0.85)",
        ),
        TooltipPosition::TopLeft => (
            "inset-block-end: 100%; inset-inline-end: 0; margin-block-end: 8px;",
            "translateY(0) scale(1)",
            "translateY(6px) scale(0.85)",
        ),
        TooltipPosition::TopRight => (
            "inset-block-end: 100%; inset-inline-start: 0; margin-block-end: 8px;",
            "translateY(0) scale(1)",
            "translateY(6px) scale(0.85)",
        ),
        TooltipPosition::BottomLeft => (
            "inset-block-start: 100%; inset-inline-end: 0; margin-block-start: 8px;",
            "translateY(0) scale(1)",
            "translateY(-6px) scale(0.85)",
        ),
        TooltipPosition::BottomRight => (
            "inset-block-start: 100%; inset-inline-start: 0; margin-block-start: 8px;",
            "translateY(0) scale(1)",
            "translateY(-6px) scale(0.85)",
        ),
    };

    let anchor_style = use_style!(
        r#"
        position: relative;
        display: inline-flex;
        "#,
    );

    let tooltip_surface_style = use_style!(
        r#"
        position: absolute;
        z-index: 1000;
        padding: 6px 10px;
        border-radius: 8px;
        backdrop-filter: blur(8px);
        -webkit-backdrop-filter: blur(8px);
        font-size: 11px;
        letter-spacing: 0.4px;
        line-height: 14px;
        font-weight: 500;
        white-space: nowrap;
        pointer-events: none;
        transform-origin: center;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15), 0 1px 4px rgba(0, 0, 0, 0.1);
        transition: transform 180ms cubic-bezier(0.34, 1.56, 0.64, 1), opacity 150ms;
        "#,
    );

    let opacity = if *visible { "1" } else { "0" };
    let transform = if *visible { active_transform } else { inactive_transform };
    let pointer_events = if *visible { "auto" } else { "none" };

    let tooltip_dynamic_class = dynamic_style(format!(
        "background-color: {}; color: {}; font-family: {}, sans-serif; pointer-events: {}; opacity: {}; transform: {}; {}",
        theme.colors.inverse_surface,
        theme.colors.inverse_on_surface,
        theme.font_family,
        pointer_events,
        opacity,
        transform,
        position_css,
    ));

    let component_override = theme.component_style("Tooltip").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div
            class={yew::classes![anchor_style.get_class_name().to_string(), &component_override]}
            onmouseenter={on_mouse_enter}
            onmouseleave={on_mouse_leave}
            onfocus={on_focus}
            onblur={on_blur}
            id={props.id.clone()}
        >
            { props.children.clone() }
            <div
                role="tooltip"
                class={yew::classes![tooltip_surface_style.get_class_name().to_string(), tooltip_dynamic_class]}
            >
                { &props.text }
            </div>
        </div>
    }
}
