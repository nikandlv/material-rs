//! Material Design 3 Switch
//!
//! A toggle switch with track and thumb, following MD3 specifications.
//! Supports optional checked and unchecked icons inside the switch handle.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

#[derive(Properties, PartialEq)]
pub struct SwitchProps {
    #[prop_or(false)]
    pub checked: bool,

    #[prop_or_default]
    pub onchange: Callback<bool>,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub checked_icon: String,

    #[prop_or_default]
    pub unchecked_icon: String,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn Switch(props: &SwitchProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let hovered = use_state(|| false);
    let active = use_state(|| false);

    let on_click = {
        let onchange = props.onchange.clone();
        let checked = props.checked;
        let disabled = props.disabled;
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                onchange.emit(!checked);
            }
        })
    };

    let on_keydown = {
        let onchange = props.onchange.clone();
        let checked = props.checked;
        let disabled = props.disabled;
        Callback::from(move |e: KeyboardEvent| {
            if !disabled && (e.key() == " " || e.key() == "Enter") {
                e.prevent_default();
                onchange.emit(!checked);
            }
        })
    };

    let on_mouseenter = {
        let hovered = hovered.clone();
        Callback::from(move |_| hovered.set(true))
    };

    let on_mouseleave = {
        let hovered = hovered.clone();
        let active = active.clone();
        Callback::from(move |_| {
            hovered.set(false);
            active.set(false);
        })
    };

    let on_mousedown = {
        let active = active.clone();
        Callback::from(move |_| active.set(true))
    };

    let on_mouseup = {
        let active = active.clone();
        Callback::from(move |_| active.set(false))
    };

    let (track_color, thumb_color, border_color) = if props.disabled {
        let track_bg = if props.checked {
            with_alpha(&theme.colors.on_surface, 0.12).unwrap_or_default()
        } else {
            with_alpha(&theme.colors.on_surface, 0.04).unwrap_or_default()
        };
        let thumb_bg = with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default();
        let border_c = if props.checked {
            "transparent".into()
        } else {
            with_alpha(&theme.colors.on_surface, 0.12).unwrap_or_default()
        };
        (track_bg, thumb_bg, border_c)
    } else if props.checked {
        (
            theme.colors.primary.clone(),
            theme.colors.on_primary.clone(),
            theme.colors.primary.clone(),
        )
    } else {
        (
            theme.colors.surface_container_highest.clone(),
            theme.colors.outline.clone(),
            theme.colors.outline.clone(),
        )
    };

    let has_unchecked_icon = !props.unchecked_icon.is_empty();
    let thumb_size = if props.checked || has_unchecked_icon { "24px" } else { "16px" };
    let thumb_left = if props.checked { "22px" } else { if has_unchecked_icon { "2px" } else { "6px" } };
    let thumb_top = if props.checked || has_unchecked_icon { "2px" } else { "6px" };

    let wrapper_style = use_style!(
        r#"
        display: inline-flex;
        align-items: center;
        height: 48px;
        box-sizing: border-box;
        gap: 16px;
        font-family: ${font_family}, sans-serif;
        user-select: none;
        -webkit-user-select: none;
        outline: none;
        "#,
        font_family = theme.font_family,
    );

    let track_style = use_style!(
        r#"
        position: relative;
        width: 52px;
        height: 32px;
        border-radius: 16px;
        box-sizing: border-box;
        transition: background-color 250ms cubic-bezier(0.2, 0, 0, 1), border-color 250ms cubic-bezier(0.2, 0, 0, 1);
        display: flex;
        align-items: center;
        "#,
    );

    let thumb_style = use_style!(
        r#"
        position: absolute;
        border-radius: 50%;
        box-sizing: border-box;
        transition: inset-inline-start 250ms cubic-bezier(0.34, 1.56, 0.64, 1),
                    top 250ms cubic-bezier(0.34, 1.56, 0.64, 1),
                    width 250ms cubic-bezier(0.34, 1.56, 0.64, 1),
                    height 250ms cubic-bezier(0.34, 1.56, 0.64, 1),
                    background-color 250ms cubic-bezier(0.2, 0, 0, 1);
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: 0 1px 3px rgba(0,0,0,0.2);
        "#,
    );

    let state_layer_style = use_style!(
        r#"
        position: absolute;
        width: 24px;
        height: 24px;
        border-radius: 50%;
        transition: transform 200ms cubic-bezier(0.2, 0, 0, 1), opacity 200ms;
        pointer-events: none;
        "#,
    );

    let icon_style = use_style!(
        r#"
        position: relative;
        z-index: 2;
        transition: font-size 250ms;
        display: flex;
        align-items: center;
        justify-content: center;
        "#,
    );

    let label_style = use_style!(
        r#"
        font-size: 14px;
        font-weight: 500;
        "#,
    );

    let state_layer_color = if props.checked {
        with_alpha(&theme.colors.primary, 0.12).unwrap_or_default()
    } else {
        with_alpha(&theme.colors.on_surface, 0.12).unwrap_or_default()
    };

    let state_layer_visible = if !props.disabled && (*active || *hovered) { "1" } else { "0" };
    let state_layer_scale = if *active { "scale(1.8)" } else if *hovered { "scale(1.5)" } else { "scale(0)" };

    let current_icon = if props.checked && !props.checked_icon.is_empty() {
        Some(&props.checked_icon)
    } else if !props.checked && !props.unchecked_icon.is_empty() {
        Some(&props.unchecked_icon)
    } else {
        None
    };

    let icon_color = if props.disabled {
        with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default()
    } else if props.checked {
        theme.colors.on_primary_container.clone()
    } else {
        theme.colors.surface.clone()
    };

    let icon_size = if props.checked { "16px" } else { "12px" };

    let switch_cursor_class = dynamic_style(format!(
        "cursor: {};",
        if props.disabled { "default" } else { "pointer" }
    ));

    let switch_track_class = dynamic_style(format!(
        "background-color: {}; border: 2px solid {};",
        track_color, border_color
    ));

    let switch_thumb_class = dynamic_style(format!(
        "inset-inline-start: {}; top: {}; width: {}; height: {}; background-color: {};",
        thumb_left, thumb_top, thumb_size, thumb_size, thumb_color
    ));

    let switch_state_layer_class = dynamic_style(format!(
        "background-color: {}; opacity: {}; transform: {};",
        state_layer_color, state_layer_visible, state_layer_scale
    ));

    let switch_label_color_class = dynamic_style(format!(
        "color: {};",
        if props.disabled {
            with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default()
        } else {
            theme.colors.on_surface.clone()
        }
    ));

    let component_override = theme.component_style("Switch").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div
            class={yew::classes![wrapper_style.get_class_name().to_string(), &props.class, &switch_cursor_class, &component_override]}
            id={props.id.clone()}
            onclick={on_click}
            onkeydown={on_keydown}
            onmouseenter={on_mouseenter}
            onmouseleave={on_mouseleave}
            onmousedown={on_mousedown}
            onmouseup={on_mouseup}
            role="switch"
            aria-checked={if props.checked { "true" } else { "false" }}
            aria-label={if props.label.is_empty() { None } else { Some(props.label.clone()) }}
            tabindex={if props.disabled { None } else { Some("0") }}
        >
            <div class={yew::classes![track_style.get_class_name().to_string(), &switch_track_class]}>
                <div class={yew::classes![thumb_style.get_class_name().to_string(), &switch_thumb_class]}>
                    <div class={yew::classes![state_layer_style.get_class_name().to_string(), &switch_state_layer_class]} />

                    if let Some(icon) = current_icon {
                        <span class={icon_style.get_class_name().to_string()}>
                            <Icon name={(*icon).clone()} size={icon_size.to_string()} color={icon_color.clone()} />
                        </span>
                    }
                </div>
            </div>
            if !props.label.is_empty() {
                <span class={yew::classes![label_style.get_class_name().to_string(), &switch_label_color_class]}>
                    { &props.label }
                </span>
            }
        </div>
    }
}
