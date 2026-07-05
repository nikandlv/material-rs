//! Material Design 3 Toolbar Component
//!
//! A flex container helper for action headers, settings tools, or footer bars.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::box_layout::Box;
use crate::components::typography::{Typography, TypographyVariant};
use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;

#[derive(Properties, PartialEq)]
pub struct ToolbarProps {
    /// Optional leading components.
    #[prop_or_default]
    pub leading: Html,

    /// Title string inside the toolbar.
    #[prop_or_default]
    pub title: String,

    /// Optional trailing action items.
    #[prop_or_default]
    pub actions: Vec<Html>,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,
}

#[component]
pub fn Toolbar(props: &ToolbarProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let style = use_style!(
        r#"
        display: flex;
        align-items: center;
        width: 100%;
        min-height: 56px;
        padding: 0 16px;
        box-sizing: border-box;
        background-color: ${bg};
        color: ${color};
        font-family: ${font_family}, sans-serif;
        gap: 12px;
        "#,
        bg = theme.colors.surface_container_lowest.clone(),
        color = theme.colors.on_surface.clone(),
        font_family = theme.font_family,
    );

    let component_override = theme.component_style("Toolbar").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div
            class={yew::classes![style.get_class_name().to_string(), &props.class, &component_override]}
            id={props.id.clone()}
            role="toolbar"
        >
            // Leading
            if props.leading != Html::default() {
                <Box display="flex" align_items="center" flex_shrink="0">
                    { props.leading.clone() }
                </Box>
            }

            // Title
            if !props.title.is_empty() {
                <Typography variant={TypographyVariant::TitleMedium} tag="div" style="font-size: 18px; font-weight: 500; flex-shrink: 0;">{ html! { { &props.title } } }</Typography>
            }

            // Children/Main content area (takes remaining space)
            <Box display="flex" align_items="center" flex="1" min_width="0" gap="8px">
                { props.children.clone() }
            </Box>

            // Actions
            if !props.actions.is_empty() {
                <Box display="flex" align_items="center" gap="8px" flex_shrink="0">
                    { for props.actions.iter().cloned() }
                </Box>
            }
        </div>
    }
}
