//! Material Design 3 Navigation Bar
//!
//! A bottom navigation bar with 3–5 destinations.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

/// A single navigation bar destination.
#[derive(Clone, PartialEq, Properties)]
pub struct NavDestination {
    /// Unique key.
    pub key: String,
    /// Icon text/symbol.
    pub icon: String,
    /// Active icon (shown when selected).
    #[prop_or_default]
    pub active_icon: String,
    /// Label text.
    pub label: String,
    /// Whether this destination is currently selected.
    #[prop_or(false)]
    pub active: bool,
}

#[derive(Properties, PartialEq)]
pub struct NavigationBarProps {
    /// Navigation destinations (3–5 items recommended).
    pub destinations: Vec<NavDestination>,

    /// Callback when a destination is selected.
    #[prop_or_default]
    pub on_select: Callback<String>,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn NavigationBar(props: &NavigationBarProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let nav_style = use_style!(
        r#"
        display: flex;
        justify-content: space-around;
        align-items: center;
        height: 80px;
        padding-bottom: 16px;
        box-sizing: border-box;
        position: relative;
        "#,
    );

    let dest_btn_style = use_style!(
        r#"
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 4px;
        border: none;
        background: none;
        cursor: pointer;
        min-width: 64px;
        height: 64px;
        padding: 12px 0 16px;
        border-radius: 16px;
        outline: none;
        position: relative;
        -webkit-tap-highlight-color: transparent;
        transition: background-color 200ms;
        "#,
    );

    let indicator_container_style = use_style!(
        r#"
        position: relative;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 64px;
        height: 32px;
        "#,
    );

    let indicator_style = use_style!(
        r#"
        position: absolute;
        inset: 0;
        border-radius: 16px;
        transition: transform 250ms cubic-bezier(0.34, 1.56, 0.64, 1),
                    opacity 250ms;
        pointer-events: none;
        "#,
    );

    let icon_span_style = use_style!(
        r#"
        position: relative;
        z-index: 1;
        transition: color 200ms;
        line-height: 24px;
        "#,
    );

    let label_style = use_style!(
        r#"
        position: relative;
        z-index: 1;
        font-size: 12px;
        font-weight: 500;
        transition: color 200ms;
        letter-spacing: 0.4px;
        "#,
    );

    let component_override = theme.component_style("NavigationBar").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <nav
            class={yew::classes![nav_style.get_class_name().to_string(), &props.class, &component_override]}
            id={props.id.clone()}
            role="navigation"
            style={dynamic_style(format!(
                "background-color: {}; font-family: {}, sans-serif; box-shadow: 0 -1px 3px rgba(0,0,0,0.05), 0 -1px 0 0 {};",
                theme.colors.surface_container,
                theme.font_family,
                theme.colors.outline_variant,
            ))}
        >
            { for props.destinations.iter().map(|dest| {
                let key = dest.key.clone();
                let on_click = {
                    let on_select = props.on_select.clone();
                    Callback::from(move |_: MouseEvent| {
                        on_select.emit(key.clone());
                    })
                };

                let icon_color = if dest.active {
                    theme.colors.on_secondary_container.clone()
                } else {
                    theme.colors.on_surface_variant.clone()
                };

                let label_color = if dest.active {
                    theme.colors.on_surface.clone()
                } else {
                    with_alpha(&theme.colors.on_surface, 0.74).unwrap_or_default()
                };

                let shown_icon = if dest.active && !dest.active_icon.is_empty() {
                    &dest.active_icon
                } else {
                    &dest.icon
                };

                let indicator_opacity = if dest.active { "1" } else { "0" };
                let indicator_transform = if dest.active { "scaleX(1)" } else { "scaleX(0.4)" };

                let btn_active_class = dynamic_style(format!(
                    "background-color: {};",
                    if dest.active {
                        with_alpha(&theme.colors.secondary_container, 0.3).unwrap_or_default()
                    } else {
                        "transparent".to_string()
                    }
                ));

                html! {
                    <button
                        key={dest.key.clone()}
                        onclick={on_click}
                        class={yew::classes![dest_btn_style.get_class_name().to_string(), btn_active_class]}
                        aria-selected={if dest.active { "true" } else { "false" }}
                        role="tab"
                    >
                        <div class={indicator_container_style.get_class_name().to_string()}>
                            <div class={indicator_style.get_class_name().to_string()}
                                 style={dynamic_style(format!(
                                     "background-color: {}; opacity: {}; transform: {};",
                                     theme.colors.secondary_container, indicator_opacity, indicator_transform
                                 ))} />

                            <Icon
                                name={shown_icon.to_string()}
                                size="24px"
                                color={icon_color.clone()}
                                class={icon_span_style.get_class_name().to_string()}
                            />
                        </div>

                        <span class={yew::classes![label_style.get_class_name().to_string(), dynamic_style(format!(
                            "font-weight: {}; color: {};",
                            if dest.active { "600" } else { "500" },
                            label_color
                        ))]}>
                            { &dest.label }
                        </span>
                    </button>
                }
            })}
        </nav>
    }
}
