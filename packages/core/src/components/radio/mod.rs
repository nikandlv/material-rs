//! Material Design 3 Radio Button
//!
//! Implements the MD3 radio button with dot indicator and state layers.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

#[derive(Properties, PartialEq)]
pub struct RadioButtonProps {
    #[prop_or(false)]
    pub checked: bool,

    #[prop_or_default]
    pub onchange: Callback<bool>,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub name: String,

    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn RadioButton(props: &RadioButtonProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let on_click = {
        let onchange = props.onchange.clone();
        let disabled = props.disabled;
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                onchange.emit(true);
            }
        })
    };

    let on_keydown = {
        let onchange = props.onchange.clone();
        let disabled = props.disabled;
        Callback::from(move |e: KeyboardEvent| {
            if !disabled && (e.key() == " " || e.key() == "Enter") {
                e.prevent_default();
                onchange.emit(true);
            }
        })
    };

    let (outer_color, inner_color) = if props.disabled {
        let on_surface_38 = with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default();
        (on_surface_38.clone(), on_surface_38)
    } else if props.checked {
        (theme.colors.primary.clone(), theme.colors.primary.clone())
    } else {
        (theme.colors.on_surface_variant.clone(), theme.colors.primary.clone())
    };

    let state_layer = with_alpha(&theme.colors.primary, 0.08).unwrap_or_default();

    let radio_opacity_class = dynamic_style(format!(
        "opacity: {};",
        if props.disabled { "0.38" } else { "1" }
    ));

    let radio_border_color_class = dynamic_style(format!(
        "border-color: {};",
        outer_color
    ));

    let radio_dot_class = dynamic_style(format!(
        "background-color: {}; transform: {};",
        inner_color, if props.checked { "scale(1)" } else { "scale(0)" }
    ));

    let radio_state_layer_class = dynamic_style(format!(
        "background: {};",
        state_layer
    ));

    let radio_label_color_class = dynamic_style(format!(
        "color: {};",
        if props.disabled {
            with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default()
        } else {
            theme.colors.on_surface.clone()
        }
    ));

    let component_override = theme.component_style("RadioButton").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    let wrapper_style = use_style!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: 12px;
        font-family: ${font_family}, sans-serif;
        -webkit-tap-highlight-color: transparent;
        transition: opacity 200ms;
        user-select: none;
        -webkit-user-select: none;

        &:hover .state-layer {
            opacity: 1;
        }
        "#,
        font_family = theme.font_family,
    );

    let outer_ring_style = use_style!(
        r#"
        position: relative;
        width: 20px;
        height: 20px;
        border-radius: 50%;
        border: 2px solid;
        display: flex;
        align-items: center;
        justify-content: center;
        box-sizing: border-box;
        transition: border-color 200ms cubic-bezier(0.2, 0, 0, 1);
        "#,
    );

    let inner_dot_style = use_style!(
        r#"
        width: 10px;
        height: 10px;
        border-radius: 50%;
        box-sizing: border-box;
        transition: transform 250ms cubic-bezier(0.34, 1.56, 0.64, 1);
        "#,
    );

    let state_layer_style = use_style!(
        r#"
        position: absolute;
        inset: -10px;
        border-radius: 50%;
        opacity: 0;
        transition: opacity 150ms cubic-bezier(0.2, 0, 0, 1);
        pointer-events: none;
        "#,
    );

    let label_style = use_style!(
        r#"
        font-size: 14px;
        font-weight: 500;
        "#,
    );

    html! {
        <div
            class={yew::classes![wrapper_style.get_class_name().to_string(), &props.class, &radio_opacity_class, &component_override]}
            id={props.id.clone()}
            onclick={on_click}
            onkeydown={on_keydown}
            role="radio"
            aria-checked={if props.checked { "true" } else { "false" }}
            aria-label={if props.label.is_empty() { None } else { Some(props.label.clone()) }}
            tabindex={if props.disabled { None } else { Some("0") }}
        >
            <div class={yew::classes![outer_ring_style.get_class_name().to_string(), &radio_border_color_class]}>
                <div class={yew::classes![inner_dot_style.get_class_name().to_string(), &radio_dot_class]} />
                <div class={yew::classes![state_layer_style.get_class_name().to_string(), &radio_state_layer_class]} />
            </div>
            if !props.label.is_empty() {
                <span class={yew::classes![label_style.get_class_name().to_string(), &radio_label_color_class]}>
                    { &props.label }
                </span>
            }
        </div>
    }
}
