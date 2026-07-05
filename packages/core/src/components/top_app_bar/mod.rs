//! Material Design 3 Top App Bar
//!
//! Supports Small (default), Medium, and Large top app bars with
//! navigation icon, title, and action icons. Implements smooth height collapse
//! and title sliding transitions on scroll.
//! Supports positioning variants: Fixed (default), Sticky, or Static.

use stylist::css;
use stylist::yew::Global;
use yew::prelude::*;

use crate::components::box_layout::Box;
use crate::components::icon::Icon;
use crate::theme::Theme;
use crate::theme::elevation::ElevationLevel;
use crate::utils::dynamic_style::dynamic_style;

/// Top app bar variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TopAppBarVariant {
    /// Standard 64px height.
    #[default]
    Small,
    /// 112px with expanded title area.
    Medium,
    /// 152px with large title area.
    Large,
}

/// Top app bar positioning mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TopAppBarPosition {
    /// Fixed at the top, overlays content (default).
    #[default]
    Fixed,
    /// Sticky at the top, sticks when scrolled.
    Sticky,
    /// Static placement, scrolls naturally.
    Static,
}

#[derive(Properties, PartialEq)]
pub struct TopAppBarProps {
    /// Title text.
    #[prop_or_default]
    pub title: String,

    /// App bar variant (Small, Medium, Large).
    #[prop_or_default]
    pub variant: TopAppBarVariant,

    /// Positioning mode (Fixed, Sticky, Static).
    #[prop_or_default]
    pub position: TopAppBarPosition,

    /// Navigation icon (text/symbol).
    #[prop_or_default]
    pub nav_icon: String,

    /// Navigation icon click callback.
    #[prop_or_default]
    pub on_nav_click: Callback<MouseEvent>,

    /// Optional logo image URL displayed beside the title.
    #[prop_or_default]
    pub logo: String,

    /// Whether the scroll container is scrolled (collapses Medium/Large variants).
    #[prop_or(false)]
    pub scrolled: bool,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,

    /// Action area content (trailing icons/actions).
    #[prop_or_default]
    pub actions: Children,
}

#[component]
pub fn TopAppBar(props: &TopAppBarProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let is_small = matches!(props.variant, TopAppBarVariant::Small);

    // Scrolled height collapses down to 64px standard size.
    let target_height = if props.scrolled {
        64
    } else {
        match props.variant {
            TopAppBarVariant::Small => 64,
            TopAppBarVariant::Medium => 112,
            TopAppBarVariant::Large => 152,
        }
    };

    let bg_color = if props.scrolled {
        theme.colors.surface_container.clone()
    } else {
        theme.colors.surface.clone()
    };

    let shadow = if props.scrolled {
        theme.elevation(ElevationLevel::Level1).box_shadow
    } else {
        "none".into()
    };

    let position_style = match props.position {
        TopAppBarPosition::Fixed => "position: fixed; top: 0; inset-inline-start: 0; inset-inline-end: 0;".to_owned(),
        TopAppBarPosition::Sticky => "position: sticky; top: 0;".to_owned(),
        TopAppBarPosition::Static => "position: static;".to_owned(),
    };

    let nav_btn_style =
        "background: none; border: none; cursor: pointer; padding: 12px; border-radius: 9999px; \
         display: flex; align-items: center; justify-content: center; width: 48px; height: 48px; \
         color: inherit; font-size: 24px; position: relative; overflow: hidden; \
         transition: transform 150ms cubic-bezier(0.2, 0, 0, 1); outline: none; -webkit-tap-highlight-color: transparent;".to_string();

    let header_class = dynamic_style(format!(
        "{} z-index: 100; max-width: 100dvw; overflow:visible; box-sizing: border-box; \
         height: {}px; background-color: {}; color: {}; \
         box-shadow: {}; \
         font-family: {}, sans-serif; \
         display: flex; flex-direction: column; justify-content: flex-start; \
         transition: height 280ms cubic-bezier(0.2, 0, 0, 1), \
                     background-color 280ms cubic-bezier(0.2, 0, 0, 1), \
                     box-shadow 280ms cubic-bezier(0.2, 0, 0, 1);",
        position_style,
        target_height,
        bg_color,
        theme.colors.on_surface,
        shadow,
        theme.font_family,
    ));
    let state_layer_bg_class = dynamic_style(format!("position: absolute; inset: 0; background-color: {}; opacity: 0; transition: opacity 150ms;", theme.colors.on_surface));
    let title_class = dynamic_style(format!(
        "font-size: 22px; font-weight: 400; line-height: 28px; margin: 0; padding-inline-start: 8px; \
         color: {}; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; \
         transition: opacity 200ms cubic-bezier(0.2, 0, 0, 1), transform 200ms cubic-bezier(0.2, 0, 0, 1); \
         opacity: {}; transform: {}; pointer-events: {}; min-width: 0;",
        theme.colors.on_surface,
        if props.scrolled || is_small { "1" } else { "0" },
        if props.scrolled || is_small { "translateY(0)" } else { "translateY(8px)" },
        if props.scrolled || is_small { "auto" } else { "none" },
    ));
    let expanded_area_class = dynamic_style(format!(
        "position: relative; flex: 1; width: 100%; box-sizing: border-box; \
         overflow: hidden; \
         opacity: {}; transition: opacity 150ms cubic-bezier(0.2, 0, 0, 1); \
         pointer-events: {};",
        if props.scrolled { "0" } else { "1" },
        if props.scrolled { "none" } else { "auto" },
    ));
    let expanded_title_class = dynamic_style(format!(
        "position: absolute; inset-inline-start: 24px; bottom: {}; margin: 0; \
         font-weight: 400; color: {}; \
         font-size: {}px; line-height: {}px; \
         transition: transform 250ms cubic-bezier(0.2, 0, 0, 1); \
         transform: {};",
        if matches!(props.variant, TopAppBarVariant::Large) { "28px" } else { "20px" },
        theme.colors.on_surface,
        if matches!(props.variant, TopAppBarVariant::Large) { 32 } else { 24 },
        if matches!(props.variant, TopAppBarVariant::Large) { 40 } else { 32 },
        if props.scrolled { "translateY(16px)" } else { "translateY(0)" }
    ));

    let component_override = theme.component_style("TopAppBar").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <header
            class={yew::classes![&props.class, header_class, &component_override]}
            id={props.id.clone()}
        >
            <Global css={css!(r#"
                button.md-nav-btn:hover .state-layer {
                    opacity: 0.08 !important;
                }
                button.md-nav-btn:active {
                    transform: scale(0.92) !important;
                }
            "#)} />

            // Top Row (Always 64px height)
            <Box display="flex" align_items="center" height="64px" min_height="64px" padding="0 4px" gap="4px" width="100%" position="relative" z_index="2" style="box-sizing: border-box;">
                if !props.nav_icon.is_empty() {
                    <button onclick={props.on_nav_click.clone()} style={nav_btn_style} class="md-nav-btn">
                    <div class={yew::classes!["state-layer", state_layer_bg_class]} />
                    <div style="position: relative; z-index: 1; display: flex; align-items: center; justify-content: center;">
                        <Icon
                            name={props.nav_icon.clone()}
                            size="24px"
                        />
                    </div>
                    </button>
                }

                // Title copy that fades in on the top row when collapsed
                <h1 class={title_class}>
                    if !props.logo.is_empty() {
                        <img src={props.logo.clone()} alt="" style="height: 24px; width: 24px; margin-right: 8px; vertical-align: middle;" />
                    }
                    { &props.title }
                </h1>

                <Box flex="1" />

                <Box display="flex" align_items="center" gap="4px" padding_right="4px">
                    { props.actions.clone() }
                </Box>
            </Box>

            // Bottom Expanded Title Area (for Medium & Large variants when unscrolled)
            if !is_small {
                <div class={expanded_area_class}>
                    <h1 class={expanded_title_class}>
                        if !props.logo.is_empty() {
                            <img src={props.logo.clone()} alt="" style="height: 32px; width: 32px; margin-right: 12px; vertical-align: middle;" />
                        }
                        { &props.title }
                    </h1>
                </div>
            }
        </header>
    }
}
