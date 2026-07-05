//! Material Design 3 Slider
//!
//! A continuous range slider with track, thumb, tick marks, and labels.

use stylist::yew::use_style;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

#[derive(Properties, PartialEq)]
pub struct SliderProps {
    /// Current value (controlled).
    #[prop_or(0.0)]
    pub value: f64,

    /// Minimum value.
    #[prop_or(0.0)]
    pub min: f64,

    /// Maximum value.
    #[prop_or(100.0)]
    pub max: f64,

    /// Step size (0 for continuous).
    #[prop_or(0.0)]
    pub step: f64,

    /// Whether the slider is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// Label text.
    #[prop_or_default]
    pub label: String,

    /// Show tick marks at each step.
    #[prop_or(false)]
    pub ticks: bool,

    /// Callback when value changes.
    #[prop_or_default]
    pub onchange: Callback<f64>,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn Slider(props: &SliderProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let is_rtl = theme.direction.is_rtl();

    let dragging = use_state(|| false);
    let hovered = use_state(|| false);
    let track_ref = use_node_ref();

    let range = props.max - props.min;
    let percentage = if range > 0.0 {
        ((props.value - props.min) / range).clamp(0.0, 1.0)
    } else {
        0.0
    };

    let active_track_color = if props.disabled {
        with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default()
    } else {
        theme.colors.primary.clone()
    };

    let inactive_track_color = with_alpha(&theme.colors.on_surface, 0.12).unwrap_or_default();

    let thumb_color = if props.disabled {
        with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default()
    } else {
        theme.colors.primary.clone()
    };

    let handle_move_to_x = {
        let track_ref = track_ref.clone();
        let onchange = props.onchange.clone();
        let min = props.min;
        let max = props.max;
        let step = props.step;
        let disabled = props.disabled;

        move |client_x: f64| {
            if disabled { return; }
            if let Some(el) = track_ref.cast::<web_sys::HtmlElement>() {
                let rect = el.get_bounding_client_rect();
                let width = rect.width() - 20.0;
                if width > 0.0 {
                    let relative_x = if is_rtl {
                        // RTL: drag left = increase value, drag right = decrease
                        (rect.right() - client_x - 10.0).clamp(0.0, width)
                    } else {
                        (client_x - rect.left() - 10.0).clamp(0.0, width)
                    };
                    let x = relative_x / width;
                    let mut val = min + x * (max - min);
                    if step > 0.0 {
                        val = (val / step).round() * step;
                    }
                    let val = val.clamp(min, max);
                    onchange.emit(val);
                }
            }
        }
    };

    let on_mouse_down = {
        let dragging = dragging.clone();
        let handle_move_to_x = handle_move_to_x.clone();
        let disabled = props.disabled;

        Callback::from(move |e: MouseEvent| {
            if disabled { return; }
            e.prevent_default();
            dragging.set(true);
            handle_move_to_x(e.client_x() as f64);
        })
    };

    {
        let dragging = dragging.clone();
        let handle_move_to_x = handle_move_to_x.clone();
        let disabled = props.disabled;

        use_effect_with(dragging.clone(), move |dragging_state| {
            let mut cleanup: Option<Box<dyn FnOnce()>> = None;

            if **dragging_state && !disabled {
                let window = web_sys::window().unwrap();

                let on_move = {
                    let handle_move_to_x = handle_move_to_x.clone();
                    wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
                        handle_move_to_x(e.client_x() as f64);
                    })
                };

                let on_up = {
                    let dragging = dragging.clone();
                    wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
                        dragging.set(false);
                    })
                };

                window.add_event_listener_with_callback("mousemove", on_move.as_ref().unchecked_ref()).unwrap();
                window.add_event_listener_with_callback("mouseup", on_up.as_ref().unchecked_ref()).unwrap();

                let on_touch_move = {
                    let handle_move_to_x = handle_move_to_x.clone();
                    wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::TouchEvent)>::new(move |e: web_sys::TouchEvent| {
                        if let Some(touch) = e.touches().get(0) {
                            handle_move_to_x(touch.client_x() as f64);
                        }
                    })
                };

                let on_touch_end = {
                    let dragging = dragging.clone();
                    wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::TouchEvent)>::new(move |_: web_sys::TouchEvent| {
                        dragging.set(false);
                    })
                };

                window.add_event_listener_with_callback("touchmove", on_touch_move.as_ref().unchecked_ref()).unwrap();
                window.add_event_listener_with_callback("touchend", on_touch_end.as_ref().unchecked_ref()).unwrap();

                cleanup = Some(Box::new(move || {
                    let window = web_sys::window().unwrap();
                    window.remove_event_listener_with_callback("mousemove", on_move.as_ref().unchecked_ref()).unwrap();
                    window.remove_event_listener_with_callback("mouseup", on_up.as_ref().unchecked_ref()).unwrap();
                    window.remove_event_listener_with_callback("touchmove", on_touch_move.as_ref().unchecked_ref()).unwrap();
                    window.remove_event_listener_with_callback("touchend", on_touch_end.as_ref().unchecked_ref()).unwrap();
                    let _ = on_move;
                    let _ = on_up;
                    let _ = on_touch_move;
                    let _ = on_touch_end;
                }));
            }

            move || {
                if let Some(c) = cleanup {
                    c();
                }
            }
        });
    }

    let on_touch_start = {
        let dragging = dragging.clone();
        let _handle_move_to_x = handle_move_to_x.clone();
        let disabled = props.disabled;

        Callback::from(move |_: TouchEvent| {
            if disabled { return; }
            dragging.set(true);
        })
    };

    let on_mouseenter = {
        let hovered = hovered.clone();
        Callback::from(move |_| hovered.set(true))
    };

    let on_mouseleave = {
        let hovered = hovered.clone();
        Callback::from(move |_| hovered.set(false))
    };

    let wrapper_style = use_style!(
        r#"
        display: flex;
        flex-direction: column;
        width: 100%;
        font-family: ${font_family}, sans-serif;
        user-select: none;
        -webkit-user-select: none;
        "#,
        font_family = theme.font_family,
    );

    let label_style = use_style!(
        r#"
        font-size: 14px;
        font-weight: 500;
        margin-bottom: 8px;
        letter-spacing: 0.1px;
        "#,
    );

    let track_area_style = use_style!(
        r#"
        position: relative;
        width: 100%;
        height: 44px;
        display: flex;
        align-items: center;
        touch-action: none;
        "#,
    );

    let track_inner_style = use_style!(
        r#"
        position: relative;
        width: 100%;
        height: 16px;
        display: flex;
        align-items: center;
        "#,
    );

    let tick_marks_style = use_style!(
        r#"
        position: absolute;
        inset-inline-start: 10px;
        inset-inline-end: 10px;
        height: 4px;
        pointer-events: none;
        z-index: 1;
        "#,
    );

    let tooltip_style = use_style!(
        r#"
        position: absolute;
        left: 50%;
        top: 0;
        background-color: ${primary};
        color: ${on_primary};
        font-size: 11px;
        font-weight: 500;
        padding: 4px 8px;
        border-radius: 12px;
        pointer-events: none;
        white-space: nowrap;
        z-index: 10;
        "#,
        primary = theme.colors.primary,
        on_primary = theme.colors.on_primary,
    );

    let state_layer_style = use_style!(
        r#"
        position: absolute;
        inset: 0;
        border-radius: 50%;
        pointer-events: none;
        transition: transform 200ms cubic-bezier(0.2, 0, 0, 1), opacity 200ms;
        "#,
    );

    let is_active = *dragging || *hovered;
    let tooltip_opacity = if *dragging { "1" } else { "0" };
    let tooltip_transform = if *dragging { "translate(-50%, -36px) scale(1)" } else { "translate(-50%, -20px) scale(0.8)" };
    let state_layer_opacity = if *dragging { "0.24" } else if *hovered { "0.12" } else { "0" };
    let state_layer_scale = if is_active { "scale(1.8)" } else { "scale(0)" };
    let transition_style = if *dragging { "transition: none;" } else { "transition: inset-inline-start 150ms cubic-bezier(0.2, 0, 0, 1), transform 150ms cubic-bezier(0.2, 0, 0, 1);" };

    let slider_label_color_class = dynamic_style(format!(
        "color: {};",
        if props.disabled {
            with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default()
        } else {
            theme.colors.on_surface.clone()
        }
    ));

    let slider_cursor_class = dynamic_style(format!(
        "cursor: {};",
        if props.disabled { "default" } else { "pointer" }
    ));

    let inactive_track_class = dynamic_style(format!(
        "position: absolute; inset-inline-start: 10px; inset-inline-end: 10px; height: 4px; border-radius: 2px; background-color: {};",
        inactive_track_color
    ));

    let active_track_class = dynamic_style(format!(
        "position: absolute; inset-inline-start: 10px; width: calc({:.5} * (100% - 20px)); height: 4px; border-radius: 2px; background-color: {};",
        percentage, active_track_color
    ));

    let thumb_class = dynamic_style(format!(
        "position: absolute; inset-inline-start: calc({:.5} * (100% - 20px)); width: 20px; height: 20px; border-radius: 50%; background-color: {}; {} box-shadow: {}; z-index: 5;",
        percentage, thumb_color, transition_style,
        if props.disabled { "none" } else { "0 2px 4px rgba(0,0,0,0.2), 0 1px 2px rgba(0,0,0,0.1)" }
    ));

    let tooltip_transform_class = dynamic_style(format!(
        "transform: {}; opacity: {};",
        tooltip_transform, tooltip_opacity
    ));

    let state_layer_class = dynamic_style(format!(
        "background-color: {}; opacity: {}; transform: {};",
        theme.colors.primary, state_layer_opacity, state_layer_scale
    ));

    let component_override = theme.component_style("Slider").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    // Generate tick marks
    let tick_marks = if props.ticks && props.step > 0.0 && range > 0.0 {
        let num_ticks = (range / props.step) as usize + 1;
        let max_ticks = 50;
        let skip = if num_ticks > max_ticks { num_ticks / max_ticks } else { 1 };
        let tick_color = with_alpha(&theme.colors.on_surface, 0.24).unwrap_or_default();
        (0..num_ticks).step_by(skip).map(move |i| {
            let pct = (i as f64 * props.step / range) * 100.0;
            format!("<div style=\"position: absolute; inset-inline-start: calc({:.5}% + 10px); top: 50%; width: 2px; height: 2px; background: {}; border-radius: 50%; transform: translate(-50%, -50%); pointer-events: none;\"></div>", pct, tick_color)
        }).collect::<Vec<_>>().join("")
    } else {
        String::new()
    };

    html! {
        <div class={yew::classes![wrapper_style.get_class_name().to_string(), &props.class, &component_override]} id={props.id.clone()}>
            if !props.label.is_empty() {
                <label class={yew::classes![label_style.get_class_name().to_string(), &slider_label_color_class]}>
                    { &props.label }
                </label>
            }
            <div ref={track_ref}
                class={yew::classes![track_area_style.get_class_name().to_string(), &slider_cursor_class]}
                onmousedown={on_mouse_down}
                ontouchstart={on_touch_start}
                onmouseenter={on_mouseenter}
                onmouseleave={on_mouseleave}
                role="slider"
                aria-valuemin={props.min.to_string()}
                aria-valuemax={props.max.to_string()}
                aria-valuenow={props.value.to_string()}
                aria-label={props.label.clone()}
                tabindex={if props.disabled { None } else { Some("0") }}
            >
                <div class={track_inner_style.get_class_name().to_string()}>
                    <div class={inactive_track_class} />
                    <div class={active_track_class} />
                    <div class={tick_marks_style.get_class_name().to_string()}>
                        { Html::from_html_unchecked(tick_marks.into()) }
                    </div>
                </div>

                <div class={thumb_class}>
                    if !props.disabled {
                        <div class={yew::classes![tooltip_style.get_class_name().to_string(), &tooltip_transform_class]}>
                            { format!("{:.0}", props.value) }
                        </div>
                    }

                    <div class={yew::classes![state_layer_style.get_class_name().to_string(), &state_layer_class]} />
                </div>
            </div>
        </div>
    }
}
