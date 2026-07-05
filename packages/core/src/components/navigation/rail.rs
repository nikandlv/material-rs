//! Material Design 3 Navigation Rail
//!
//! A side navigation component for wider layouts.

use yew::prelude::*;

use crate::components::box_layout::Box;
use crate::components::icon::Icon;
use crate::components::typography::{Typography, TypographyVariant};
use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

/// A navigation rail destination.
#[derive(Clone, PartialEq, Properties)]
pub struct RailDestination {
    pub key: String,
    pub icon: String,
    #[prop_or_default]
    pub active_icon: String,
    pub label: String,
    #[prop_or(false)]
    pub active: bool,
}

#[derive(Properties, PartialEq)]
pub struct NavigationRailProps {
    pub destinations: Vec<RailDestination>,

    #[prop_or_default]
    pub on_select: Callback<String>,

    /// Header content (e.g., a FAB).
    #[prop_or_default]
    pub header: Children,

    /// Footer content.
    #[prop_or_default]
    pub footer: Children,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn NavigationRail(props: &NavigationRailProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let nav_rail_class = dynamic_style(format!(
        "display: flex; flex-direction: column; align-items: center; \
         width: 80px; height: 100dvh; background-color: {}; \
         font-family: {}, sans-serif; \
         padding: 24px 0; border-inline-end: 1px solid {};",
        theme.colors.surface,
        theme.font_family,
        theme.colors.outline_variant,
    ));

    let component_override = theme.component_style("NavigationRail").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <nav
            class={yew::classes![&props.class, nav_rail_class, &component_override]}
            id={props.id.clone()}
            role="navigation"
        >
            // Header
            if !props.header.is_empty() {
                <Box padding_bottom="24px">
                    { props.header.clone() }
                </Box>
            }

            // Destinations
            <Box display="flex" flex_direction="column" align_items="center" gap="16px" flex="1" width="100%">
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

                    let shown_icon = if dest.active && !dest.active_icon.is_empty() {
                        &dest.active_icon
                    } else {
                        &dest.icon
                    };

                    let indicator_opacity = if dest.active { "1" } else { "0" };
                    let indicator_transform = if dest.active { "translateX(-50%) scaleX(1)" } else { "translateX(-50%) scaleX(0.4)" };

                    let btn_class = dynamic_style(
                        "display: flex; flex-direction: column; align-items: center; \
                         gap: 4px; padding: 4px 0 8px; border: none; background: none; \
                         cursor: pointer; position: relative; width: 100%; outline: none; \
                         -webkit-tap-highlight-color: transparent;".to_string()
                    );
                    let indicator_class = dynamic_style(format!(
                        "position: absolute; left: 50%; top: 4px; width: 56px; height: 32px; \
                         border-radius: 16px; background-color: {}; \
                         opacity: {}; transform: {}; \
                         transition: transform 250ms cubic-bezier(0.34, 1.56, 0.64, 1), opacity 250ms; \
                         pointer-events: none;",
                        theme.colors.secondary_container, indicator_opacity, indicator_transform
                    ));
                    let icon_wrap_class = dynamic_style("position: relative; z-index: 1; transition: color 200ms; line-height: 32px; height: 32px; display: flex; align-items: center; justify-content: center;".to_string());
                    let label_class = dynamic_style(format!(
                        "font-weight: {}; \
                         color: {}; position: relative; z-index: 1; transition: color 200ms; \
                         margin-top: 4px;",
                        if dest.active { "600" } else { "500" },
                        if dest.active { theme.colors.on_surface.clone() } else { with_alpha(&theme.colors.on_surface, 0.74).unwrap_or_default() }
                    ));

                    html! {
                        <button
                            key={dest.key.clone()}
                            onclick={on_click}
                            aria-label={dest.label.clone()}
                            aria-selected={if dest.active { "true" } else { "false" }}
                            class={btn_class}
                        >
                            // Indicator pill
                            <div class={indicator_class} />

                            <div class={icon_wrap_class}>
                                <Icon
                                    name={shown_icon.to_string()}
                                    size="24px"
                                    color={icon_color.clone()}
                                />
                            </div>

                            <Typography variant={TypographyVariant::LabelMedium} tag="span" class={label_class}>{ html! { { &dest.label } } }</Typography>
                        </button>
                    }
                })}
            </Box>

            // Footer
            if !props.footer.is_empty() {
                <Box padding_top="24px">
                    { props.footer.clone() }
                </Box>
            }
        </nav>
    }
}