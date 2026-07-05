//! Material Design 3 Snackbar
//!
//! A brief message that appears at the bottom of the screen, with optional action.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::typography::{Typography, TypographyVariant};
use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

#[derive(Properties, PartialEq)]
pub struct SnackbarProps {
    /// Whether the snackbar is visible.
    #[prop_or(false)]
    pub visible: bool,

    /// Message text.
    pub message: String,

    /// Optional action button label.
    #[prop_or_default]
    pub action_label: String,

    /// Action callback.
    #[prop_or_default]
    pub on_action: Callback<()>,

    /// Dismiss callback.
    #[prop_or_default]
    pub on_dismiss: Callback<()>,

    /// Duration in ms before auto-dismiss (0 = no auto-dismiss).
    #[prop_or(5000)]
    pub duration: u32,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn Snackbar(props: &SnackbarProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let visible = props.visible;
    let on_dismiss = props.on_dismiss.clone();
    let duration = props.duration;

    use_effect_with((visible, duration), move |(visible, duration)| {
        if *visible && *duration > 0 {
            let on_dismiss = on_dismiss.clone();
            let handle = gloo_timers::callback::Timeout::new(*duration, move || {
                on_dismiss.emit(());
            });
            handle.forget();
        }
        || ()
    });

    let on_action_click = {
        let on_action = props.on_action.clone();
        Callback::from(move |_: MouseEvent| on_action.emit(()))
    };

    let on_dismiss_click = {
        let on_dismiss = props.on_dismiss.clone();
        Callback::from(move |_: MouseEvent| on_dismiss.emit(()))
    };

    let wrapper_style = use_style!(
        r#"
        position: fixed;
        bottom: 24px;
        left: 50%;
        z-index: 9999;
        pointer-events: none;
        transition: transform 300ms cubic-bezier(0.34, 1.56, 0.64, 1), opacity 300ms;
        "#,
    );

    let surface_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 14px 16px;
        border-radius: 8px;
        backdrop-filter: blur(16px);
        -webkit-backdrop-filter: blur(16px);
        font-size: 14px;
        font-weight: 500;
        box-shadow: 0 10px 30px rgba(0,0,0,0.2), 0 1px 8px rgba(0,0,0,0.1);
        min-width: 288px;
        max-width: 560px;
        "#,
    );

    let action_style = use_style!(
        r#"
        background: none;
        border: none;
        font-size: 14px;
        font-weight: 600;
        letter-spacing: 0.4px;
        padding: 8px 12px;
        border-radius: 4px;
        cursor: pointer;
        white-space: nowrap;
        position: relative;
        overflow: hidden;
        transition: background-color 200ms;
        outline: none;
        -webkit-tap-highlight-color: transparent;
        "#,
    );

    let close_style = use_style!(
        r#"
        background: none;
        border: none;
        color: inherit;
        cursor: pointer;
        padding: 6px;
        opacity: 0.7;
        font-size: 20px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 50%;
        width: 28px;
        height: 28px;
        transition: opacity 150ms, background-color 150ms;
        outline: none;
        "#,
    );

    let state_layer_style = use_style!(
        r#"
        position: absolute;
        inset: 0;
        opacity: 0;
        transition: opacity 150ms;
        "#,
    );

    let hover_style = use_style!(
        r#"
        button:hover .state-layer {
            opacity: 0.12 !important;
        }
        button:active {
            transform: scale(0.96);
        }
        "#,
    );

    let wrapper_transform = if props.visible {
        "translateX(-50%) translateY(0) scale(1)"
    } else {
        "translateX(-50%) translateY(24px) scale(0.92)"
    };
    let wrapper_opacity = if props.visible { "1" } else { "0" };
    let pointer_events = if props.visible { "auto" } else { "none" };

    let wrapper_class = dynamic_style(format!("transform: {}; opacity: {}; pointer-events: {};", wrapper_transform, wrapper_opacity, pointer_events));
    let surface_class = dynamic_style(format!(
        "background-color: {}; color: {}; font-family: {}, sans-serif;",
        with_alpha(&theme.colors.inverse_surface, 0.88).unwrap_or_default(),
        theme.colors.inverse_on_surface,
        theme.font_family,
    ));
    let action_class = dynamic_style(format!("color: {}; font-family: {}, sans-serif;", theme.colors.inverse_primary, theme.font_family));
    let state_layer_bg_class = dynamic_style(format!("background-color: {};", theme.colors.inverse_primary));

    let component_override = theme.component_style("Snackbar").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div
            class={yew::classes![wrapper_style.get_class_name().to_string(), hover_style.get_class_name().to_string(), &props.class, wrapper_class, &component_override]}
            id={props.id.clone()}
            role="status"
            aria-live="polite"
        >
            <div
                class={yew::classes![surface_style.get_class_name().to_string(), surface_class]}
            >
                <Typography variant={TypographyVariant::BodyMedium} style="flex: 1; letter-spacing: 0.2px;">
                    { html! { { &props.message } } }
                </Typography>
                if !props.action_label.is_empty() {
                    <button
                        onclick={on_action_click}
                        class={yew::classes![action_style.get_class_name().to_string(), action_class]}
                    >
                        <div class={yew::classes!["state-layer", state_layer_style.get_class_name().to_string(), state_layer_bg_class]} />
                        <Typography variant={TypographyVariant::TitleSmall} style="position: relative; z-index: 1;">{ html! { { &props.action_label } } }</Typography>
                    </button>
                }
                <button
                    onclick={on_dismiss_click}
                    class={close_style.get_class_name().to_string()}
                    aria-label="Dismiss"
                >
                    <Typography variant={TypographyVariant::BodyMedium} style="line-height: 1;">{ "\u{00D7}" }</Typography>
                </button>
            </div>
        </div>
    }
}
