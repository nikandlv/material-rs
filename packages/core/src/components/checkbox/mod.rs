//! Material Design 3 Checkbox
//!
//! Implements the MD3 checkbox with checkmark animation and state layers.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

#[derive(Properties, PartialEq)]
pub struct CheckboxProps {
    #[prop_or(false)]
    pub checked: bool,

    /// Indeterminate state (shows a dash).
    #[prop_or(false)]
    pub indeterminate: bool,

    #[prop_or_default]
    pub onchange: Callback<bool>,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn Checkbox(props: &CheckboxProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

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

    let is_active = props.checked || props.indeterminate;

    let (container_color, check_color, border_color) = if props.disabled {
        let on_surface_38 = with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default();
        ("transparent".into(), on_surface_38.clone(), on_surface_38)
    } else if is_active {
        (
            theme.colors.primary.clone(),
            theme.colors.on_primary.clone(),
            theme.colors.primary.clone(),
        )
    } else {
        ("transparent".into(), theme.colors.primary.clone(), theme.colors.on_surface_variant.clone())
    };

    let state_layer = with_alpha(&theme.colors.primary, 0.08).unwrap_or_default();

    let checkbox_box_class = dynamic_style(format!(
        "background-color: {}; border: 2px solid {};",
        container_color, border_color
    ));

    let checkbox_state_layer_class = dynamic_style(format!(
        "background: {};",
        state_layer
    ));

    let checkbox_label_color_class = dynamic_style(format!(
        "color: {};",
        if props.disabled {
            with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default()
        } else {
            theme.colors.on_surface.clone()
        }
    ));

    let wrapper_style = use_style!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: 12px;
        cursor: pointer;
        font-family: ${font_family}, sans-serif;
        -webkit-tap-highlight-color: transparent;

        &.disabled {
            cursor: default;
        }

        &:hover:not(.disabled) .checkbox-state-layer {
            opacity: 1;
        }
        "#,
        font_family = theme.font_family,
    );

    let box_style = use_style!(
        r#"
        position: relative;
        width: 18px;
        height: 18px;
        border-radius: 2px;
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
        transition: background-color 200ms cubic-bezier(0.2, 0, 0, 1), border-color 200ms cubic-bezier(0.2, 0, 0, 1);
        "#,
    );

    let state_layer_style = use_style!(
        r#"
        position: absolute;
        inset: -8px;
        border-radius: 16px;
        background: transparent;
        opacity: 0;
        transition: opacity 200ms cubic-bezier(0.2, 0, 0, 1);
        pointer-events: none;
        "#,
    );

    let label_style = use_style!(
        r#"
        font-size: 14px;
        "#,
    );

    let component_override = theme.component_style("Checkbox").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div
            class={yew::classes![
                wrapper_style.get_class_name().to_string(),
                if props.disabled { "disabled" } else { "" },
                &props.class,
                &component_override
            ]}
            id={props.id.clone()}
            onclick={on_click}
            onkeydown={on_keydown}
            role="checkbox"
            aria-checked={if props.indeterminate { "mixed" } else if props.checked { "true" } else { "false" }}
            aria-label={if props.label.is_empty() { None } else { Some(props.label.clone()) }}
            tabindex={if props.disabled { None } else { Some("0") }}
            style={if props.disabled { "opacity: 0.38;" } else { "" }}
        >
            <div class={yew::classes![box_style.get_class_name().to_string(), &checkbox_box_class]}>
                if is_active {
                    if props.indeterminate {
                        <svg width="18" height="18" viewBox="0 0 18 18" style="position: absolute;">
                            <rect x="4" y="8.25" width="10" height="1.5" rx="0.75" fill={check_color.clone()} />
                        </svg>
                    } else {
                        <svg width="18" height="18" viewBox="0 0 18 18" style="position: absolute;">
                            <path d="M4.5 9.5L7.5 12.5L13.5 6" fill="none" stroke={check_color.clone()} stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                        </svg>
                    }
                }
                <div class={yew::classes![state_layer_style.get_class_name().to_string(), &checkbox_state_layer_class, "checkbox-state-layer"]} />
            </div>
            if !props.label.is_empty() {
                <span class={yew::classes![label_style.get_class_name().to_string(), &checkbox_label_color_class]}>
                    { &props.label }
                </span>
            }
        </div>
    }
}
