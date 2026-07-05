//! Material Design 3 Ripple Effect
//!
//! A state-layer visual feedback overlay triggered by pointer interaction.
//! Implements the MD3 ripple specification with configurable color and duration.

use stylist::css;
use stylist::yew::{use_style, Global};
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

use crate::utils::dynamic_style::dynamic_style;

#[derive(Clone, PartialEq, Debug)]
struct RippleInstance {
    id: String,
    x: f64,
    y: f64,
    size: f64,
}

#[derive(Properties, PartialEq)]
pub struct RippleProps {
    /// Ripple color in CSS format (e.g., "rgba(255,255,255,0.24)").
    #[prop_or_else(|| "rgba(255, 255, 255, 0.24)".to_owned())]
    pub color: String,

    /// Ripple animation duration in milliseconds.
    #[prop_or(550)]
    pub duration: u32,
}

#[component]
pub fn Ripple(props: &RippleProps) -> Html {
    let container_ref = use_node_ref();
    let ripples = use_mut_ref(Vec::<RippleInstance>::new);
    let trigger_render = use_state(|| 0);

    let duration = props.duration;
    let on_pointerdown = {
        let container_ref = container_ref.clone();
        let ripples = ripples.clone();
        let trigger_render = trigger_render.clone();
        let color = props.color.clone();

        Callback::from(move |e: MouseEvent| {
            if color == "transparent" {
                return;
            }
            if let Some(target) = container_ref.cast::<Element>() {
                let rect = target.get_bounding_client_rect();
                let x = e.client_x() as f64 - rect.left();
                let y = e.client_y() as f64 - rect.top();
                let width = rect.width();
                let height = rect.height();
                let size = (width.powi(2) + height.powi(2)).sqrt() * 2.0;

                let raw_time = web_sys::window()
                    .and_then(|w| w.performance())
                    .map(|p| p.now())
                    .unwrap_or(0.0);
                let id = format!("ripple-{}", raw_time);

                ripples.borrow_mut().push(RippleInstance {
                    id: id.clone(),
                    x,
                    y,
                    size,
                });
                trigger_render.set(*trigger_render + 1);

                let ripples_cleanup = ripples.clone();
                let trigger_cleanup = trigger_render.clone();
                let id_clone = id.clone();
                let cb = wasm_bindgen::closure::Closure::once(move || {
                    ripples_cleanup.borrow_mut().retain(|r| r.id != id_clone);
                    trigger_cleanup.set(*trigger_cleanup + 1);
                });
                let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(), duration as i32
                );
                cb.forget();
            }
        })
    };

    let container_style = use_style!(
        r#"
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        overflow: hidden;
        pointer-events: auto;
        border-radius: inherit;
        z-index: 0;
        "#,
    );

    let ripple_span_style = use_style!(
        r#"
        position: absolute;
        border-radius: 50%;
        pointer-events: none;
        transform: translate(-50%, -50%) scale(0);
        "#,
    );

    let duration_ms = props.duration;

    html! {
        <div
            ref={container_ref}
            class={container_style.get_class_name().to_string()}
            onmousedown={on_pointerdown}
        >
            <Global css={css!(r#"
                @keyframes md-ripple-effect {
                    to {
                        transform: translate(-50%, -50%) scale(1);
                        opacity: 0;
                    }
                }
            "#)} />

            { for ripples.borrow().iter().map(|ripple| {
                let ripple_dynamic_class = dynamic_style(format!(
                    "background: {}; left: {}px; top: {}px; width: {}px; height: {}px; animation: md-ripple-effect {}ms cubic-bezier(0.2, 0, 0, 1) forwards;",
                    props.color, ripple.x, ripple.y, ripple.size, ripple.size, duration_ms
                ));
                html! {
                    <span
                        key={ripple.id.clone()}
                        class={yew::classes![ripple_span_style.get_class_name().to_string(), ripple_dynamic_class]}
                    />
                }
            })}
        </div>
    }
}
