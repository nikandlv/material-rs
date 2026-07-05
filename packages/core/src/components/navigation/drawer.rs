//! Material Design 3 Navigation Drawer
//!
//! Modal, Persistent, and Standard navigation drawers with smooth animations.
//! Supports nested stacking with dynamic z-indexes and maps corner radius design tokens.

use std::sync::atomic::{AtomicU32, Ordering};
use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

static Z_INDEX_COUNTER: AtomicU32 = AtomicU32::new(12000);

/// Drawer variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DrawerVariant {
    /// Standard drawer (always visible, sits in document flow).
    Standard,
    /// Modal drawer (overlays content with a glassmorphic scrim, slides open).
    #[default]
    Modal,
    /// Persistent drawer (can toggle open/closed, sits side-by-side pushing content).
    Persistent,
}

/// A drawer menu item.
#[derive(Clone, PartialEq, Properties)]
pub struct DrawerItem {
    pub key: String,
    pub icon: String,
    pub label: String,
    #[prop_or_default]
    pub section: String,
    #[prop_or(false)]
    pub active: bool,
}

#[derive(Properties, PartialEq)]
pub struct NavigationDrawerProps {
    /// Drawer variant.
    #[prop_or_default]
    pub variant: DrawerVariant,

    /// Whether the drawer is open.
    #[prop_or(true)]
    pub open: bool,

    /// Menu items.
    #[prop_or_default]
    pub items: Vec<DrawerItem>,

    /// Callback when an item is selected.
    #[prop_or_default]
    pub on_select: Callback<String>,

    /// Close callback.
    #[prop_or_default]
    pub on_close: Callback<()>,

    /// Headline (shown above items).
    #[prop_or_default]
    pub headline: String,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn NavigationDrawer(props: &NavigationDrawerProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let local_z_index = use_state(|| 12000u32);

    let is_modal = matches!(props.variant, DrawerVariant::Modal);
    let is_standard = matches!(props.variant, DrawerVariant::Standard);
    let is_open = is_standard || props.open;

    {
        let local_z_index = local_z_index.clone();

        use_effect_with(is_open, move |is_open| {
            if *is_open {
                let next_z = Z_INDEX_COUNTER.fetch_add(10, Ordering::SeqCst);
                local_z_index.set(next_z);
            }
            || ()
        });
    }

    let on_scrim_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let mut sections: Vec<(String, Vec<&DrawerItem>)> = Vec::new();
    for item in &props.items {
        if let Some(last) = sections.last_mut()
            && last.0 == item.section {
                last.1.push(item);
                continue;
            }
        sections.push((item.section.clone(), vec![item]));
    }

    let scrim_opacity = if is_modal && is_open { "1" } else { "0" };
    let scrim_pointer_events = if is_modal && is_open { "auto" } else { "none" };

    let drawer_transform = if is_open {
        "translateX(0)"
    } else {
        "translateX(-100%)"
    };
    let drawer_bg = theme.colors.surface_container_low.clone();

    // ── Styles ──
    let drawer_nav_style = use_style!(
        r#"
        display: flex;
        flex-direction: column;
        width: 320px;
        height: 100%;
        max-height: calc(100vh - 64px);
        border-radius: 0 ${radius} ${radius} 0;
        overflow-y: auto;
        border-inline-end: 1px solid ${border_color};
        -webkit-tap-highlight-color: transparent;
        transform: ${transform};
        transition: transform 300ms cubic-bezier(0.2, 0, 0, 1), background-color 200ms;

        button.md-drawer-item:hover .state-layer {
            opacity: 0.08 !important;
        }
        button.md-drawer-item:active {
            transform: scale(0.98);
        }
        "#,
        radius = theme.shapes.extra_large,
        border_color = theme.colors.outline_variant,
        transform = drawer_transform,
    );

    let headline_style = use_style!(
        r#"
        padding: 28px 28px 16px;
        font-size: 14px;
        font-weight: 600;
        letter-spacing: 0.1px;
        "#,
    );

    let section_label_style = use_style!(
        r#"
        padding: 16px 28px 8px;
        font-size: 12px;
        font-weight: 500;
        letter-spacing: 0.4px;
        "#,
    );

    let item_btn_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        gap: 12px;
        width: calc(100% - 24px);
        padding: 0 16px;
        min-height: 56px;
        height: 56px;
        border: none;
        cursor: pointer;
        text-align: start;
        border-radius: 28px;
        margin: 4px 12px;
        position: relative;
        overflow: hidden;
        transition: background-color 200ms cubic-bezier(0.2, 0, 0, 1), transform 150ms;
        outline: none;
        -webkit-tap-highlight-color: transparent;
        "#,
    );

    let item_state_layer_style = use_style!(
        r#"
        position: absolute;
        inset: 0;
        opacity: 0;
        transition: opacity 150ms;
        pointer-events: none;
        border-radius: inherit;
        "#,
    );

    let item_icon_style = use_style!(
        r#"
        font-size: 24px;
        position: relative;
        z-index: 1;
        "#,
    );

    let item_label_style = use_style!(
        r#"
        font-size: 14px;
        font-weight: 600;
        letter-spacing: 0.1px;
        position: relative;
        z-index: 1;
        "#,
    );

    let scrim_style = use_style!(
        r#"
        position: fixed;
        inset: 0;
        background-color: rgba(0,0,0,0.3);
        backdrop-filter: blur(4px);
        -webkit-backdrop-filter: blur(4px);
        transition: opacity 250ms cubic-bezier(0.2, 0, 0, 1);
        "#,
    );

    let persistent_wrapper_style = use_style!(
        r#"
        transition: width 300ms cubic-bezier(0.2, 0, 0, 1);
        height: 100%;
        overflow: hidden;
        position: relative;
        flex-shrink: 0;
        "#,
    );

    let nav_style = dynamic_style(format!(
        "background-color: {}; font-family: {}, sans-serif; z-index: {}; {}",
        drawer_bg,
        theme.font_family,
        if is_modal { *local_z_index } else { 1 },
        if is_modal && is_open { "box-shadow: 0 16px 24px rgba(0,0,0,0.15);" } else { "box-shadow: none;" },
    ));

    let component_override = theme.component_style("NavigationDrawer").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    // Shared drawer content
    let drawer_content = html! {
        <nav
            class={yew::classes![drawer_nav_style.get_class_name().to_string(), nav_style, &props.class, &component_override]}
            id={if !is_modal && !is_standard { String::new() } else { props.id.clone() }}
            role="navigation"
        >
            if !props.headline.is_empty() {
                <div class={yew::classes![headline_style.get_class_name().to_string(), dynamic_style(format!("color: {};", theme.colors.on_surface))]}>
                    { &props.headline }
                </div>
            }

            { for sections.iter().map(|(section_name, items)| {
                html! {
                    <>
                        if !section_name.is_empty() {
                            <div class={yew::classes![section_label_style.get_class_name().to_string(), dynamic_style(format!("color: {};", theme.colors.on_surface_variant))]}>
                                { section_name }
                            </div>
                        }
                        { for items.iter().map(|item| {
                            let key = item.key.clone();
                            let on_click = {
                                let on_select = props.on_select.clone();
                                let on_close = props.on_close.clone();
                                Callback::from(move |_: MouseEvent| {
                                    on_select.emit(key.clone());
                                    if is_modal {
                                        on_close.emit(());
                                    }
                                })
                            };

                            let (bg, icon_color, text_color) = if item.active {
                                (
                                    with_alpha(&theme.colors.secondary_container, 1.0).unwrap_or_default(),
                                    theme.colors.on_secondary_container.clone(),
                                    theme.colors.on_secondary_container.clone(),
                                )
                            } else {
                                (
                                    "transparent".into(),
                                    theme.colors.on_surface_variant.clone(),
                                    theme.colors.on_surface.clone(),
                                )
                            };

                            let item_bg_class = dynamic_style(format!("background: {}; font-family: {}, sans-serif;", bg, theme.font_family));
                            let state_layer_color_class = dynamic_style(format!("background-color: {};", if item.active { theme.colors.on_secondary_container.clone() } else { theme.colors.on_surface.clone() }));
                            let label_color_class = dynamic_style(format!("color: {};", text_color));

                            html! {
                                <button
                                    key={item.key.clone()}
                                    onclick={on_click}
                                    class={yew::classes![item_btn_style.get_class_name().to_string(), item_bg_class, "md-drawer-item"]}
                                >
                                    <div class={yew::classes![item_state_layer_style.get_class_name().to_string(), state_layer_color_class]} />
                                    <Icon
                                        name={item.icon.clone()}
                                        size="24px"
                                        color={icon_color.clone()}
                                        class={item_icon_style.get_class_name().to_string()}
                                    />
                                    <span class={yew::classes![item_label_style.get_class_name().to_string(), label_color_class]}>
                                        { &item.label }
                                    </span>
                                </button>
                            }
                        })}
                    </>
                }
            })}
        </nav>
    };

    let scrim_z_index = if *local_z_index > 0 {
        *local_z_index - 1
    } else {
        11999
    };

    let scrim_class = dynamic_style(format!("z-index: {}; opacity: {}; pointer-events: {};", scrim_z_index, scrim_opacity, scrim_pointer_events));
    let persistent_width_class = dynamic_style(format!("width: {};", if is_open { "320px" } else { "0px" }));

    html! {
        <>
            if is_modal {
                <div
                    class={yew::classes![scrim_style.get_class_name().to_string(), scrim_class]}
                    onclick={on_scrim_click}
                />
                { drawer_content }
            } else if is_standard {
                { drawer_content }
            } else {
                <div
                    id={props.id.clone()}
                    class={yew::classes![persistent_wrapper_style.get_class_name().to_string(), persistent_width_class]}
                >
                    { drawer_content }
                </div>
            }
        </>
    }
}
