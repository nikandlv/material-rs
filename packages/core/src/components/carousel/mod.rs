//! Material Design 3 Carousel
//!
//! A horizontal scrolling container for browsing items (cards, images, etc.)
//! with snap behavior, optional indicators, and dynamic color transitions.
//! Supports small (compact), medium (card), and large (hero) container types.

use stylist::yew::use_style;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

/// Carousel container size variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CarouselSize {
    /// Small: 132×80dp compact items (e.g., small images).
    Small,
    /// Medium: 282×180dp card-sized items (default).
    #[default]
    Medium,
    /// Large: 328×240dp hero-sized items.
    Large,
}

/// A single item in the carousel.
#[derive(Clone, PartialEq)]
pub struct CarouselItem {
    /// Unique key for this item.
    pub key: String,
    /// Content to render inside the carousel cell.
    pub children: Html,
    /// Optional full-bleed background image URL.
    pub image: String,
}

#[derive(Properties, PartialEq)]
pub struct CarouselProps {
    /// Items to display in the carousel.
    pub items: Vec<CarouselItem>,

    /// Size variant for the carousel items.
    #[prop_or_default]
    pub size: CarouselSize,

    /// Show dot indicators below the carousel.
    #[prop_or(true)]
    pub show_indicators: bool,

    /// Auto-scroll to the center of the active item after selection.
    #[prop_or(true)]
    pub auto_center: bool,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,

    /// Callback when an item is clicked.
    #[prop_or_default]
    pub on_select: Callback<String>,
}

#[component]
pub fn Carousel(props: &CarouselProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let scroll_ref = use_node_ref();
    let active_index = use_state(|| 0usize);

    let (item_width, item_height, gap) = match props.size {
        CarouselSize::Small => (132.0, 80.0, 8.0),
        CarouselSize::Medium => (282.0, 180.0, 12.0),
        CarouselSize::Large => (328.0, 240.0, 16.0),
    };

    // Handle scroll to update active indicator
    {
        let scroll_ref = scroll_ref.clone();
        let active_index = active_index.clone();
        use_effect_with((), move |_| {
            let active_index2 = active_index.clone();
            let scroll_ref2 = scroll_ref.clone();
            let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                if let Some(el) = scroll_ref2.cast::<HtmlElement>() {
                    let scroll_left = el.scroll_left() as f64;
                    let step = item_width + gap;
                    let idx = (scroll_left / step).round() as usize;
                    active_index2.set(idx);
                }
            }) as Box<dyn FnMut()>);
            if let Some(el) = scroll_ref.cast::<HtmlElement>() {
                let cb_js: wasm_bindgen::JsValue = cb.as_ref().into();
                let et: web_sys::EventTarget = el.clone().into();
                let _ = et.add_event_listener_with_callback("scroll", cb_js.unchecked_ref());
            }
            cb.forget();
            || ()
        });
    }

    let container_style = use_style!(
        r#"
        display: flex;
        overflow-x: auto;
        scroll-snap-type: x mandatory;
        scroll-behavior: smooth;
        -webkit-overflow-scrolling: touch;
        scrollbar-width: none;
        padding: 0 24px;

        &::-webkit-scrollbar {
            display: none;
        }
        "#,
    );

    let item_style = use_style!(
        r#"
        flex-shrink: 0;
        scroll-snap-align: center;
        border-radius: ${radius};
        overflow: hidden;
        position: relative;
        cursor: pointer;
        transition: transform 200ms cubic-bezier(0.2, 0, 0, 1),
                    box-shadow 200ms cubic-bezier(0.2, 0, 0, 1);

        &:hover {
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        }

        &:active {
            transform: scale(0.97);
        }
        "#,
        radius = theme.shapes.large,
    );

    let indicator_container_style = use_style!(
        r#"
        display: flex;
        justify-content: center;
        gap: 8px;
        padding-top: 12px;
        "#,
    );

    let component_override = theme.component_style("Carousel").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    let arrow_style = use_style!(
        r#"
        position: absolute;
        top: 50%;
        transform: translateY(-50%);
        width: 40px;
        height: 40px;
        border-radius: 50%;
        border: none;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 3;
        opacity: 0;
        transition: opacity 200ms, background-color 200ms;
        pointer-events: none;
        background-color: ${surface};
        color: ${on_surface};
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);

        &:hover {
            background-color: ${surface_container_highest};
        }
        "#,
        surface = theme.colors.surface,
        on_surface = theme.colors.on_surface,
        surface_container_highest = theme.colors.surface_container_highest,
    );

    let scroll_by = {
        let scroll_ref = scroll_ref.clone();
        move |direction: i32| {
            if let Some(el) = scroll_ref.cast::<HtmlElement>() {
                let step = (item_width + gap) * direction as f64;
                let new_left = (el.scroll_left() as f64 + step).max(0.0);
                el.set_scroll_left(new_left as i32);
            }
        }
    };

    let on_prev = {
        let scroll_by = scroll_by.clone();
        Callback::from(move |_: MouseEvent| scroll_by(-1))
    };
    let on_next = {
        let scroll_by = scroll_by.clone();
        Callback::from(move |_: MouseEvent| scroll_by(1))
    };

    let items_html = props.items.iter().enumerate().map(|(idx, item)| {
        let key = item.key.clone();
        let on_click = {
            let on_select = props.on_select.clone();
            let key = key.clone();
            Callback::from(move |_: MouseEvent| {
                on_select.emit(key.clone());
            })
        };

        let is_active = idx == *active_index;
        let scale = if is_active { "scale(1)" } else { "scale(0.92)" };
        let shadow = if is_active {
            "0 8px 24px rgba(0, 0, 0, 0.15)"
        } else {
            "0 2px 8px rgba(0, 0, 0, 0.08)"
        };

        let bg = if item.image.is_empty() {
            with_alpha(&theme.colors.surface_container_high, 0.6).unwrap_or_default()
        } else {
            "transparent".to_string()
        };

        html! {
            <div
                key={key.clone()}
                class={yew::classes![item_style.get_class_name().to_string(), dynamic_style(format!(
                    "width: {}px; height: {}px; margin: 0 {}px; background-color: {}; transform: {}; box-shadow: {}; transition: transform 300ms cubic-bezier(0.34, 1.56, 0.64, 1), box-shadow 300ms;",
                    item_width, item_height, gap / 2.0, bg, scale, shadow
                ))]}
                onclick={on_click}
            >
                if !item.image.is_empty() {
                    <img
                        src={item.image.clone()}
                        style="width: 100%; height: 100%; object-fit: cover; display: block;"
                        loading="lazy"
                    />
                }
                <div style="position: relative; z-index: 1; width: 100%; height: 100%; display: flex; flex-direction: column; justify-content: flex-end; padding: 16px; box-sizing: border-box;">
                    { item.children.clone() }
                </div>
            </div>
        }
    }).collect::<Html>();

    let indicators = if props.show_indicators && !props.items.is_empty() {
        html! {
            <div class={indicator_container_style.get_class_name().to_string()}>
                { for props.items.iter().enumerate().map(|(idx, item)| {
                    let is_active = idx == *active_index;
                    let dot_color = if is_active { &theme.colors.primary } else { &theme.colors.outline_variant };
                    let dot_width = if is_active { "24px" } else { "8px" };
                    html! {
                        <div
                            key={item.key.clone()}
                            class={dynamic_style(format!(
                                "width: {}; height: 8px; border-radius: 4px; background-color: {}; transition: all 300ms cubic-bezier(0.2, 0, 0, 1);",
                                dot_width, dot_color
                            ))}
                        />
                    }
                })}
            </div>
        }
    } else {
        html! {}
    };

    let wrapper_style = use_style!(
        r#"
        position: relative;

        &:hover .carousel-arrow {
            opacity: 1;
            pointer-events: auto;
        }
        "#,
    );

    let arrow_left_style = format!("{} left: 4px;", arrow_style.get_class_name());
    let arrow_right_style = format!("{} right: 4px;", arrow_style.get_class_name());

    html! {
        <div
            id={props.id.clone()}
            class={yew::classes![&props.class, &component_override, wrapper_style.get_class_name().to_string()]}
        >
            <button class={arrow_left_style} aria-label="Previous" onclick={on_prev}>
                <Icon name="chevron_left" size="24px" />
            </button>
            <div
                ref={scroll_ref}
                class={yew::classes![container_style.get_class_name().to_string(), dynamic_style("margin: 0 -24px; padding: 24px;".to_string())]}
            >
                { items_html }
            </div>
            <button class={arrow_right_style} aria-label="Next" onclick={on_next}>
                <Icon name="chevron_right" size="24px" />
            </button>
            { indicators }
        </div>
    }
}
