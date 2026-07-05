//! Material Design 3 Bottom Sheet
//!
//! A surface that slides up from the bottom edge of the screen.
//! Supports standard, expanding, and modal variants.
//! Uses portal rendering for proper stacking above page content.

use std::sync::atomic::{AtomicU32, Ordering};
use stylist::yew::use_style;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;
use crate::utils::portal::Portal;

static Z_INDEX_COUNTER: AtomicU32 = AtomicU32::new(14000);

/// Bottom sheet variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BottomSheetVariant {
    /// Standard bottom sheet — content only, no drag handle.
    Standard,
    /// Modal bottom sheet — with scrim, drag handle, and dismiss on scrim click.
    #[default]
    Modal,
    /// Expanding bottom sheet — can be dragged up to fill more of the screen.
    Expanding,
}

#[derive(Properties, PartialEq)]
pub struct BottomSheetProps {
    /// Whether the bottom sheet is open.
    #[prop_or(false)]
    pub open: bool,

    /// Bottom sheet variant.
    #[prop_or_default]
    pub variant: BottomSheetVariant,

    /// Title shown above the content (modal/expanding only).
    #[prop_or_default]
    pub title: String,

    /// Close callback (scrim click or drag down).
    #[prop_or_default]
    pub on_close: Callback<()>,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,

    /// Sheet content.
    #[prop_or_default]
    pub children: Children,
}

#[component]
pub fn BottomSheet(props: &BottomSheetProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let local_z_index = use_state(|| 14000u32);
    let touch_start_y = use_state(|| 0.0f64);

    let is_modal = matches!(props.variant, BottomSheetVariant::Modal);
    let show_handle = is_modal || matches!(props.variant, BottomSheetVariant::Expanding);

    {
        let local_z_index = local_z_index.clone();
        let open = props.open;
        use_effect_with(open, move |open| {
            if *open {
                let next_z = Z_INDEX_COUNTER.fetch_add(10, Ordering::SeqCst);
                local_z_index.set(next_z);
            }
            || ()
        });
    }

    let on_scrim_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |e: MouseEvent| {
            // Close when clicking directly on the scrim (not on the sheet content)
            let target = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok());
            if let Some(el) = target
                && el.get_attribute("data-scrim").is_some() {
                    on_close.emit(());
                }
        })
    };

    let transform = if props.open { "translateY(0)" } else { "translateY(100%)" };

    // ── Styles ──

    let scrim_style = use_style!(
        r#"
        position: fixed;
        inset: 0;
        background-color: rgba(0, 0, 0, 0.4);
        backdrop-filter: blur(4px);
        -webkit-backdrop-filter: blur(4px);
        transition: opacity 300ms cubic-bezier(0.2, 0, 0, 1);
        pointer-events: auto;
        "#,
    );

    let sheet_style = use_style!(
        r#"
        position: fixed;
        bottom: 0;
        left: 0;
        right: 0;
        max-height: 80vh;
        background-color: ${bg};
        border-radius: ${radius} ${radius} 0 0;
        box-shadow: 0 -2px 16px rgba(0, 0, 0, 0.15);
        transition: transform 350ms cubic-bezier(0.2, 0, 0, 1);
        overflow-y: auto;
        overscroll-behavior: contain;
        touch-action: pan-y;
        "#,
        bg = theme.colors.surface_container_low,
        radius = theme.shapes.extra_large,
    );

    let handle_style = use_style!(
        r#"
        display: flex;
        justify-content: center;
        padding: 12px 0 8px;
        cursor: grab;
        "#,
    );

    let handle_bar_style = use_style!(
        r#"
        width: 32px;
        height: 4px;
        border-radius: 2px;
        background-color: ${color};
        "#,
        color = theme.colors.on_surface_variant,
    );

    let title_style = use_style!(
        r#"
        padding: 0 24px 12px;
        font-size: 16px;
        font-weight: 600;
        letter-spacing: 0.15px;
        font-family: ${font_family}, sans-serif;
        "#,
        font_family = theme.font_family,
    );

    let content_style = use_style!(
        r#"
        padding: 0 24px 24px;
        font-family: ${font_family}, sans-serif;
        "#,
        font_family = theme.font_family,
    );

    let component_override = theme.component_style("BottomSheet").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    // Touch drag-to-dismiss
    let on_touch_start = {
        let touch_start_y = touch_start_y.clone();
        Callback::from(move |e: TouchEvent| {
            if let Some(touch) = e.touches().get(0) {
                touch_start_y.set(touch.client_y() as f64);
            }
        })
    };

    let on_touch_end = {
        let on_close = props.on_close.clone();
        let touch_start_y = touch_start_y.clone();
        Callback::from(move |e: TouchEvent| {
            if let Some(touch) = e.changed_touches().get(0) {
                let dy = touch.client_y() as f64 - *touch_start_y;
                if dy > 100.0 {
                    on_close.emit(());
                }
            }
        })
    };

    let portal_content = html! {
        <>
            if is_modal {
                <div
                    class={yew::classes![scrim_style.get_class_name().to_string(), dynamic_style(format!("z-index: {}; opacity: {}; pointer-events: {};", *local_z_index, if props.open { "1" } else { "0" }, if props.open { "auto" } else { "none" }))]}
                    data-scrim="true"
                    onclick={on_scrim_click.clone()}
                />
            }
            <div
                class={yew::classes![sheet_style.get_class_name().to_string(), &props.class, dynamic_style(format!("z-index: {}; transform: {};", *local_z_index, transform)), &component_override]}
                id={props.id.clone()}
                role="dialog"
                aria-modal={if is_modal { "true" } else { "false" }}
                aria-label={props.title.clone()}
                ontouchstart={on_touch_start}
                ontouchend={on_touch_end}
            >
                if show_handle {
                    <div class={handle_style.get_class_name().to_string()}>
                        <div class={handle_bar_style.get_class_name().to_string()} />
                    </div>
                }
                if !props.title.is_empty() {
                    <div class={yew::classes![title_style.get_class_name().to_string(), dynamic_style(format!("color: {};", theme.colors.on_surface))]}>
                        { &props.title }
                    </div>
                }
                <div class={content_style.get_class_name().to_string()}>
                    { props.children.clone() }
                </div>
            </div>
        </>
    };

    html! {
        <Portal id={format!("bottom-sheet-{}", props.id)}>
            { portal_content }
        </Portal>
    }
}
