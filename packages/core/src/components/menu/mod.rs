//! Material Design 3 Dropdown Menu
//!
//! A positioned menu with selectable items that automatically opens to the left
//! if there is insufficient space on the right (smart positioning).

use std::sync::atomic::{AtomicU32, Ordering};
use stylist::css;
use stylist::yew::use_style;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::theme::Theme;
use crate::theme::elevation::ElevationLevel;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;
use crate::utils::portal::Portal;

static Z_INDEX_COUNTER: AtomicU32 = AtomicU32::new(10000);

/// A single menu item.
#[derive(Clone, PartialEq, Properties)]
pub struct MenuItem {
    /// Unique key.
    pub key: String,
    /// Item label.
    pub label: String,
    /// Leading icon/symbol.
    #[prop_or_default]
    pub icon: String,
    /// Trailing text (e.g., shortcut hint).
    #[prop_or_default]
    pub trailing_text: String,
    /// Whether this item is disabled.
    #[prop_or(false)]
    pub disabled: bool,
}

#[derive(Properties, PartialEq)]
pub struct MenuProps {
    /// Whether the menu is open.
    #[prop_or(false)]
    pub open: bool,

    /// Menu items.
    pub items: Vec<MenuItem>,

    /// Callback when an item is selected.
    #[prop_or_default]
    pub on_select: Callback<String>,

    /// Close callback.
    #[prop_or_default]
    pub on_close: Callback<()>,

    /// Anchor element id (for positioning).
    #[prop_or_default]
    pub anchor_id: String,

    /// Whether the menu width should match the anchor's width.
    #[prop_or(false)]
    pub match_anchor_width: bool,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn Menu(props: &MenuProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let position = use_state(|| (0.0, 0.0, false, 0.0, 0.0)); // (left, top, opens_left, right_offset, width)
    let has_anchor = use_state(|| false);
    let local_z_index = use_state(|| 10000u32);

    let display = if props.open { "block" } else { "none" };

    let item_hover_style = use_style!(
        r#"
        button.md-menu-item:not(:disabled):hover {
            background-color: ${hover_bg};
        }
        button.md-menu-item:not(:disabled):active {
            background-color: ${active_bg};
        }
        "#,
        hover_bg = with_alpha(&theme.colors.primary, 0.08).unwrap_or_default(),
        active_bg = with_alpha(&theme.colors.primary, 0.12).unwrap_or_default(),
    );

    {
        let position = position.clone();
        let has_anchor = has_anchor.clone();
        let local_z_index = local_z_index.clone();
        let open = props.open;
        let anchor_id = props.anchor_id.clone();

        use_effect_with((open, anchor_id), move |(open, anchor_id)| {
            let scroll_guard = if *open {
                let window = web_sys::window().unwrap();
                let doc = window.document().unwrap();

                // Lock body scroll
                let guard = doc.body().map(|body| {
                    let prev = body
                        .style()
                        .get_property_value("overflow")
                        .unwrap_or_default();
                    let _ = body.style().set_property("overflow", "hidden");
                    (body, prev)
                });

                // Increment Z-index sequence for overlapping overlay stacks
                let next_z = Z_INDEX_COUNTER.fetch_add(10, Ordering::SeqCst);
                local_z_index.set(next_z);

                if !anchor_id.is_empty() {
                    if let Some(anchor) = doc.get_element_by_id(anchor_id) {
                        let rect = anchor.get_bounding_client_rect();
                        let viewport_width =
                            window.inner_width().unwrap().as_f64().unwrap_or(1200.0);

                        let opens_left = rect.left() > viewport_width / 2.0;
                        let right_offset = viewport_width - rect.right();
                        let width = rect.width();

                        position.set((rect.left(), rect.bottom(), opens_left, right_offset, width));
                        has_anchor.set(true);
                    } else {
                        has_anchor.set(false);
                    }
                } else {
                    has_anchor.set(false);
                }

                guard
            } else {
                has_anchor.set(false);
                None
            };

            move || {
                if let Some((body, prev_overflow)) = scroll_guard {
                    let _ = body.style().set_property("overflow", &prev_overflow);
                }
            }
        });
    }

    let on_click_outside = {
        let on_close = props.on_close.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(target) = e
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                && (target.class_name().contains("md-menu-surface")
                    || target.closest(".md-menu-surface").ok().flatten().is_some())
                {
                    return;
                }
            on_close.emit(());
        })
    };

    // Position styling: absolute next to anchor (left/right aligned), or fallback to fixed top-right
    let positioning_style = if *has_anchor {
        let pos = &*position;
        if pos.2 {
            format!(
                "position: absolute; inset-inline-end: {}px; top: {}px; transform-origin: top right;",
                pos.3, pos.1
            )
        } else {
            format!(
                "position: absolute; inset-inline-start: {}px; top: {}px; transform-origin: top left;",
                pos.0, pos.1
            )
        }
    } else {
        "position: fixed; inset-inline-end: 16px; top: 64px; transform-origin: top right;".to_owned()
    };

    let width_style = if props.match_anchor_width && *has_anchor {
        let anchor_w = position.4;
        format!("width: {}px; min-width: 112px;", anchor_w)
    } else {
        "min-width: 112px; max-width: 280px;".to_owned()
    };

    let portal_backdrop_class = dynamic_style(format!(
        "position: fixed; inset: 0; z-index: {}; display: {}; background-color: transparent; pointer-events: auto;",
        *local_z_index,
        display,
    ));
    let menu_surface_class = dynamic_style(format!(
        "{} z-index: 1; {} \
         border-radius: {}; background-color: {}; \
         backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px); \
         box-shadow: {}; \
         padding: 8px 0; max-height: 320px; overflow-y: auto; \
         font-family: {}, sans-serif; pointer-events: auto; \
         transform: {}; opacity: {}; transition: transform 200ms cubic-bezier(0.34, 1.56, 0.64, 1), opacity 200ms;",
        positioning_style,
        width_style,
        theme.shapes.medium,
        with_alpha(&theme.colors.surface_container_low, 0.88).unwrap_or_default(),
        theme.elevation(ElevationLevel::Level2).box_shadow,
        theme.font_family,
        if props.open { "scale(1)" } else { "scale(0.9)" },
        if props.open { "1" } else { "0" },
    ));

    let component_override = theme.component_style("Menu").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <Portal id={format!("menu-{}", props.id)}>
            <div
                class={yew::classes![&props.class, portal_backdrop_class, &component_override]}
                id={props.id.clone()}
                onclick={on_click_outside}
                role="menu"
            >
            // Menu surface
            <div
                class={yew::classes!["md-menu-surface", item_hover_style.get_class_name().to_string(), menu_surface_class]}
            >
                { for props.items.iter().map(|item| {
                    let key = item.key.clone();
                    let on_click = {
                        let on_select = props.on_select.clone();
                        let on_close = props.on_close.clone();
                        Callback::from(move |_: MouseEvent| {
                            on_select.emit(key.clone());
                            on_close.emit(());
                        })
                    };

                    let (text_color, cursor) = if item.disabled {
                        (
                            with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default(),
                            "default",
                        )
                    } else {
                        (theme.colors.on_surface.clone(), "pointer")
                    };

                    let icon_color = if item.disabled {
                        with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default()
                    } else {
                        theme.colors.on_surface_variant.clone()
                    };
                    let trailing_color = icon_color.clone();
                    let item_btn_class = dynamic_style(format!(
                        "display: flex; align-items: center; width: 100%; \
                         padding: 0 16px; height: 48px; gap: 12px; \
                         border: none; background: none; cursor: {}; \
                         font-family: {}, sans-serif; text-align: start; \
                         font-size: 14px; color: {}; position: relative; overflow: hidden; \
                         outline: none; -webkit-tap-highlight-color: transparent; \
                         transition: background-color 150ms;",
                        cursor, theme.font_family, text_color
                    ));

                    html! {
                        <button
                            key={item.key.clone()}
                            onclick={on_click}
                            disabled={item.disabled}
                            role="menuitem"
                            class={yew::classes![item_btn_class, "md-menu-item"]}
                        >
                            // State Layer (Hover effect)
                            <div class={css!(
                                r#"position: absolute; inset: 0; background-color: black; opacity: 0;
                                 transition: opacity 150ms; z-index:1; width:100%; height:100%;
                                  &:hover {
            opacity: 0.1;
        }
                                 "#
                            )} />

                            if !item.icon.is_empty() {
                                <div style="pointer-events: none; flex-shrink: 0;">
                                    <Icon
                                        name={item.icon.clone()}
                                        size="24px"
                                        color={icon_color.clone()}
                                    />
                                </div>
                            }

                            <span style="flex: 1; pointer-events: none;">{ &item.label }</span>

                            if !item.trailing_text.is_empty() {
                                <span class={dynamic_style(format!(
                                    "font-size: 12px; color: {}; pointer-events: none;",
                                    trailing_color
                                ))}>
                                    { &item.trailing_text }
                                </span>
                            }
                        </button>
                    }
                })}
            </div>
            </div>
        </Portal>
    }
}
