//! Material Design 3 Tab Bar
//!
//! Scrollable and fixed tab bars following MD3 specs.

use stylist::yew::use_style;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::components::typography::{Typography, TypographyVariant};
use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;

/// A single tab item.
#[derive(Clone, PartialEq, Properties)]
pub struct TabItem {
    /// Unique key.
    pub key: String,
    /// Tab label.
    pub label: String,
    /// Icon (optional).
    #[prop_or_default]
    pub icon: String,
    /// Whether this tab is active.
    #[prop_or(false)]
    pub active: bool,
}

#[derive(Properties, PartialEq)]
pub struct TabBarProps {
    /// Tab items.
    pub tabs: Vec<TabItem>,

    /// Callback when a tab is selected.
    #[prop_or_default]
    pub on_select: Callback<String>,

    /// Whether the tab bar is scrollable.
    #[prop_or(false)]
    pub scrollable: bool,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn TabBar(props: &TabBarProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let container_ref = use_node_ref();
    let indicator_state = use_state(|| (0.0f64, 0.0f64)); // (left_px, width_px)

    let num_tabs = props.tabs.len();
    let active_index = props.tabs.iter().position(|t| t.active).unwrap_or(0);

    // For scrollable mode: measure the active tab's actual DOM position
    {
        let indicator_state = indicator_state.clone();
        let container_ref = container_ref.clone();
        let scrollable = props.scrollable;
        let tabs_snapshot = props.tabs.iter().map(|t| t.key.clone()).collect::<Vec<_>>();
        use_effect_with((active_index, scrollable, tabs_snapshot), move |(active_index, scrollable, _)| {
            if *scrollable {
                let indicator_state = indicator_state.clone();
                let container_ref = container_ref.clone();
                let ai = *active_index;
                let cb = wasm_bindgen::closure::Closure::once(move || {
                    if let Some(container) = container_ref.cast::<web_sys::HtmlElement>()
                        && let Some(tab_button) = container.children().item(ai as u32) {
                            let el = tab_button.unchecked_into::<web_sys::HtmlElement>();
                            indicator_state.set((el.offset_left() as f64, el.offset_width() as f64));
                        }
                });
                let _ = web_sys::window().unwrap().request_animation_frame(cb.as_ref().unchecked_ref());
                cb.forget();
            }
            || ()
        });
    }

    // For scrollable mode: scroll active tab into view
    {
        let container_ref = container_ref.clone();
        let scrollable = props.scrollable;
        let tabs_snapshot = props.tabs.iter().map(|t| t.key.clone()).collect::<Vec<_>>();
        use_effect_with((active_index, scrollable, tabs_snapshot), move |(active_index, scrollable, _)| {
            if *scrollable {
                let container_ref = container_ref.clone();
                let ai = *active_index;
                let cb = wasm_bindgen::closure::Closure::once(move || {
                    if let Some(container) = container_ref.cast::<web_sys::HtmlElement>()
                        && let Some(tab_button) = container.children().item(ai as u32) {
                            let el = tab_button.unchecked_into::<web_sys::HtmlElement>();
                            let tab_left = el.offset_left() as f64;
                            let tab_width = el.offset_width() as f64;
                            let container_width = container.offset_width() as f64;
                            let scroll_left = tab_left - (container_width - tab_width) / 2.0;
                            container.set_scroll_left(scroll_left.max(0.0) as i32);
                        }
                });
                let _ = web_sys::window().unwrap().request_animation_frame(cb.as_ref().unchecked_ref());
                cb.forget();
            }
            || ()
        });
    }

    let tab_style = use_style!(
        r#"
        flex: ${flex};
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 48px;
        padding: 0 16px;
        border: none;
        background: none;
        cursor: pointer;
        position: relative;
        font-family: ${font_family}, sans-serif;
        white-space: nowrap;
        min-width: 90px;
        transition: background-color 150ms;
        outline: none;
        -webkit-tap-highlight-color: transparent;

        &:hover {
            background-color: ${hover_bg};
        }
        "#,
        flex = if props.scrollable { "0 1 auto" } else { "1 1 0" },
        font_family = theme.font_family,
        hover_bg = crate::utils::color::with_alpha(&theme.colors.on_surface, 0.04).unwrap_or_default(),
    );

    let sliding_indicator = if num_tabs > 0 {
        if props.scrollable {
            let (left, width) = *indicator_state;
            let indicator_outer_class = dynamic_style(format!(
                "position: absolute; bottom: 0; inset-inline-start: {}px; width: {}px; height: 3px; \
                 transition: inset-inline-start 250ms cubic-bezier(0.2, 0, 0, 1), width 250ms cubic-bezier(0.2, 0, 0, 1); \
                 z-index: 2; display: flex; justify-content: center; pointer-events: none;",
                left, width
            ));
            let indicator_inner_class = dynamic_style(format!(
                "width: 100%; height: 100%; border-radius: 3px 3px 0 0; background-color: {};",
                theme.colors.primary
            ));
            html! {
                <div class={indicator_outer_class}>
                    <div class={indicator_inner_class} />
                </div>
            }
        } else {
            let indicator_width = 100.0 / num_tabs as f64;
            let indicator_outer_class = dynamic_style(format!(
                "position: absolute; bottom: 0; inset-inline-start: 0; width: {}%; height: 3px; \
                 transform: translateX({}%); \
                 transition: transform 250ms cubic-bezier(0.2, 0, 0, 1); \
                 z-index: 2; display: flex; justify-content: center; pointer-events: none;",
                indicator_width,
                active_index as f64 * 100.0
            ));
            let indicator_inner_class = dynamic_style(format!(
                "width: 100%; height: 100%; border-radius: 3px 3px 0 0; background-color: {};",
                theme.colors.primary
            ));
            html! {
                <div class={indicator_outer_class}>
                    <div class={indicator_inner_class} />
                </div>
            }
        }
    } else {
        html! {}
    };

    let container_class = dynamic_style(format!(
        "display: flex; width: 100%; position: relative; \
         font-family: {}, sans-serif; \
         overflow-x: {};",
        theme.font_family,
        if props.scrollable { "auto" } else { "hidden" },
    ));

    let component_override = theme.component_style("TabBar").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div
            class={yew::classes![&props.class, container_class, &component_override]}
            id={props.id.clone()}
            ref={container_ref}
            role="tablist"
        >
            { for props.tabs.iter().map(|tab| {
                let key = tab.key.clone();
                let on_click = {
                    let on_select = props.on_select.clone();
                    Callback::from(move |_: MouseEvent| {
                        on_select.emit(key.clone());
                    })
                };

                let label_color = if tab.active {
                    theme.colors.primary.clone()
                } else {
                    theme.colors.on_surface_variant.clone()
                };

                html! {
                    <button
                        key={tab.key.clone()}
                        class={yew::classes![tab_style.get_class_name().to_string()]}
                        onclick={on_click}
                        role="tab"
                        aria-selected={if tab.active { "true" } else { "false" }}
                    >
                        if !tab.icon.is_empty() {
                            <div style="margin-bottom: 2px;">
                                <Icon
                                    name={tab.icon.clone()}
                                    size="24px"
                                    color={label_color.clone()}
                                />
                            </div>
                        }
                        <Typography variant={TypographyVariant::BodyMedium} tag="span" class={dynamic_style(format!(
                            "font-weight: 600; letter-spacing: 0.1px; color: {};",
                            label_color
                        ))}>{ html! { { &tab.label } } }</Typography>
                    </button>
                }
            })}

            { sliding_indicator }

            // Bottom divider
            <div class={dynamic_style(format!(
                "position: absolute; bottom: 0; inset-inline-start: 0; inset-inline-end: 0; height: 1px; \
                 background-color: {}; z-index: 1;",
                theme.colors.outline_variant
            ))} />
        </div>
    }
}
