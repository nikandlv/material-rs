//! Material Design 3 Avatar Component
//!
//! Renders user profile pictures, fallback text/initials, or icons inside circular/rounded bounds.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::box_layout::Box;
use crate::components::icon::Icon;
use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AvatarShape {
    #[default]
    Circle,
    Squircle,
    Square,
}

#[derive(Properties, PartialEq)]
pub struct AvatarProps {
    #[prop_or_default]
    pub src: String,

    #[prop_or_default]
    pub initials: String,

    #[prop_or_default]
    pub icon: String,

    #[prop_or_default]
    pub shape: AvatarShape,

    #[prop_or("md".to_owned())]
    pub size: String,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn Avatar(props: &AvatarProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let size_px = match props.size.as_str() {
        "xs" => 24,
        "sm" => 32,
        "md" => 40,
        "lg" => 56,
        "xl" => 72,
        other => other.replace("px", "").parse::<u32>().unwrap_or(40),
    };

    let border_radius = match props.shape {
        AvatarShape::Circle => "50%".to_string(),
        AvatarShape::Squircle => format!("{}px", size_px / 4),
        AvatarShape::Square => "4px".to_string(),
    };

    let font_size = format!("{}px", size_px * 4 / 10);

    let container_style = use_style!(
        r#"
        display: inline-flex;
        align-items: center;
        justify-content: center;
        box-sizing: border-box;
        width: ${size}px;
        height: ${size}px;
        border-radius: ${radius};
        background-color: ${bg};
        color: ${color};
        font-family: ${font_family}, sans-serif;
        font-size: ${font_size};
        font-weight: 600;
        overflow: hidden;
        user-select: none;
        -webkit-user-select: none;
        vertical-align: middle;
        "#,
        size = size_px,
        radius = border_radius,
        bg = theme.colors.primary_container.clone(),
        color = theme.colors.on_primary_container.clone(),
        font_family = theme.font_family,
        font_size = font_size,
    );

    let component_override = theme.component_style("Avatar").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <Box
            id={props.id.clone()}
            class={format!("{} {} {}", container_style.get_class_name(), props.class, component_override)}
        >
            if !props.src.is_empty() {
                <img
                    src={props.src.clone()}
                    alt={props.initials.clone()}
                    style="width: 100%; height: 100%; object-fit: cover;"
                />
            } else if !props.initials.is_empty() {
                { &props.initials }
            } else if !props.icon.is_empty() {
                <Icon name={props.icon.clone()} size={format!("{}px", size_px * 6 / 10)} />
            } else {
                <Icon name="person" size={format!("{}px", size_px * 6 / 10)} />
            }
        </Box>
    }
}
