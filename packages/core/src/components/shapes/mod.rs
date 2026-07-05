//! Material Design 3 Shape Components
//!
//! Individual shape components for Beer CSS expressive mask shapes,
//! plus a base `Shape` component for CSS-based shapes (Circle, Pill, Rounded*, etc.).
//!
//! CSS-based shapes use `use_style!` for zero-inline-style rendering.
//! Mask-based shapes use minimal inline style for the dynamic mask-image data URI.

pub mod data;

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::box_layout::{Box, BoxTag};
use crate::theme::Theme;

// ── ShapeType enum (for the base Shape component) ──

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ShapeType {
    Circle,
    Pill,
    #[default]
    RoundedSmall,
    RoundedMedium,
    RoundedLarge,
    RoundedExtraLarge,
    Beveled,
    Diamond,
    Square,
    TopRound,
    BottomRound,
    LeftRound,
    RightRound,
}

#[derive(Properties, PartialEq)]
pub struct ShapeProps {
    #[prop_or_default]
    pub shape: ShapeType,

    #[prop_or_default]
    pub tag: BoxTag,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,
}

/// Base Shape component for CSS-based shapes (Circle, Pill, Rounded*, Beveled, Diamond, Square).
#[component]
pub fn Shape(props: &ShapeProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let (border_radius, clip_path): (String, String) = match props.shape {
        ShapeType::Circle => ("50%".to_string(), "none".to_string()),
        ShapeType::Pill => ("9999px".to_string(), "none".to_string()),
        ShapeType::Square => ("0".to_string(), "none".to_string()),
        ShapeType::RoundedSmall => (theme.shapes.small.clone(), "none".to_string()),
        ShapeType::RoundedMedium => (theme.shapes.medium.clone(), "none".to_string()),
        ShapeType::RoundedLarge => (theme.shapes.large.clone(), "none".to_string()),
        ShapeType::RoundedExtraLarge => (theme.shapes.extra_large.clone(), "none".to_string()),
        ShapeType::TopRound => (format!("{} {} 0 0", theme.shapes.medium, theme.shapes.medium), "none".to_string()),
        ShapeType::BottomRound => (format!("0 0 {} {}", theme.shapes.medium, theme.shapes.medium), "none".to_string()),
        ShapeType::LeftRound => (format!("{} 0 0 {}", theme.shapes.medium, theme.shapes.medium), "none".to_string()),
        ShapeType::RightRound => (format!("0 {} {} 0", theme.shapes.medium, theme.shapes.medium), "none".to_string()),
        ShapeType::Beveled => (
            "0".to_string(),
            "polygon(12px 0%, calc(100% - 12px) 0%, 100% 12px, 100% calc(100% - 12px), calc(100% - 12px) 100%, 12px 100%, 0% calc(100% - 12px), 0% 12px)".to_string(),
        ),
        ShapeType::Diamond => (
            "0".to_string(),
            "polygon(50% 0%, 100% 50%, 50% 100%, 0% 50%)".to_string(),
        ),
    };

    let style = use_style!(
        r#"
        display: inline-flex;
        align-items: center;
        justify-content: center;
        overflow: hidden;
        box-sizing: border-box;
        "#,
    );

    html! {
        <Box
            id={props.id.clone()}
            class={format!("{} {}", style.get_class_name(), props.class)}
            tag={props.tag}
            border_radius={border_radius}
            style={if clip_path != "none" { format!("clip-path: {}; -webkit-clip-path: {};", clip_path, clip_path) } else { String::new() }}
        >
            { props.children.clone() }
        </Box>
    }
}

// ── Macro for generating mask-based shape components ──

macro_rules! mask_shape_component {
    ($name:ident, $props_name:ident, $doc:expr, $svg_const:expr) => {
        #[doc = $doc]
        #[derive(Properties, PartialEq)]
        pub struct $props_name {
            #[prop_or("3.5rem".into())]
            pub size: String,

            #[prop_or_default]
            pub color: String,

            #[prop_or_default]
            pub class: String,

            #[prop_or_default]
            pub children: Children,
        }

        #[component]
        pub fn $name(props: &$props_name) -> Html {
            let theme = use_context::<Theme>().expect("Theme context required");
            let color = if props.color.is_empty() {
                &theme.colors.primary
            } else {
                &props.color
            };

            let mask_style = "display: inline-flex; align-items: center; justify-content: center; \
                 border-radius: 0; background-color: ".to_string()
                + color
                + "; mask-image: url(\"" + $svg_const + "\"); -webkit-mask-image: url(\"" + $svg_const + "\"); \
                 mask-repeat: no-repeat; mask-position: center; mask-size: contain; \
                 -webkit-mask-repeat: no-repeat; -webkit-mask-position: center; -webkit-mask-size: contain; \
                 box-sizing: border-box; block-size: "
                + &props.size
                + "; inline-size: "
                + &props.size
                + ";";

            html! {
                <Box class={props.class.clone()} style={mask_style}>
                    { props.children.clone() }
                </Box>
            }
        }
    };
}

// ── Generate all mask shape components ──

mask_shape_component!(ShapeArch, ShapeArchProps, "Arch shape mask component.", data::ARCH);
mask_shape_component!(ShapeArrow, ShapeArrowProps, "Arrow shape mask component.", data::ARROW);
mask_shape_component!(ShapeBoom, ShapeBoomProps, "Boom/explosion shape mask component.", data::BOOM);
mask_shape_component!(ShapeBun, ShapeBunProps, "Bun shape mask component.", data::BUN);
mask_shape_component!(ShapeBurst, ShapeBurstProps, "Burst/starburst shape mask component.", data::BURST);
mask_shape_component!(ShapeClamshell, ShapeClamshellProps, "Clamshell shape mask component.", data::CLAMSHELL);
mask_shape_component!(ShapeFan, ShapeFanProps, "Fan shape mask component.", data::FAN);
mask_shape_component!(ShapeFlower, ShapeFlowerProps, "Flower shape mask component.", data::FLOWER);
mask_shape_component!(ShapeGem, ShapeGemProps, "Gem shape mask component.", data::GEM);
mask_shape_component!(ShapeGhostIsh, ShapeGhostIshProps, "Ghost-ish shape mask component.", data::GHOST_ISH);
mask_shape_component!(ShapeHeart, ShapeHeartProps, "Heart shape mask component.", data::HEART);
mask_shape_component!(ShapeLeafClover4, ShapeLeafClover4Props, "4-leaf clover shape mask component.", data::LEAF_CLOVER4);
mask_shape_component!(ShapeLeafClover8, ShapeLeafClover8Props, "8-leaf clover shape mask component.", data::LEAF_CLOVER8);
mask_shape_component!(ShapeOval, ShapeOvalProps, "Oval shape mask component.", data::OVAL);
mask_shape_component!(ShapePentagon, ShapePentagonProps, "Pentagon shape mask component.", data::PENTAGON);
mask_shape_component!(ShapePixelCircle, ShapePixelCircleProps, "Pixel circle shape mask component.", data::PIXEL_CIRCLE);
mask_shape_component!(ShapePixelTriangle, ShapePixelTriangleProps, "Pixel triangle shape mask component.", data::PIXEL_TRIANGLE);
mask_shape_component!(ShapePuffy, ShapePuffyProps, "Puffy shape mask component.", data::PUFFY);
mask_shape_component!(ShapePuffyDiamond, ShapePuffyDiamondProps, "Puffy diamond shape mask component.", data::PUFFY_DIAMOND);
mask_shape_component!(ShapeSemicircle, ShapeSemicircleProps, "Semicircle shape mask component.", data::SEMICIRCLE);
mask_shape_component!(ShapeSidedCookie4, ShapeSidedCookie4Props, "4-sided cookie shape mask component.", data::SIDED_COOKIE4);
mask_shape_component!(ShapeSidedCookie6, ShapeSidedCookie6Props, "6-sided cookie shape mask component.", data::SIDED_COOKIE6);
mask_shape_component!(ShapeSidedCookie7, ShapeSidedCookie7Props, "7-sided cookie shape mask component.", data::SIDED_COOKIE7);
mask_shape_component!(ShapeSidedCookie9, ShapeSidedCookie9Props, "9-sided cookie shape mask component.", data::SIDED_COOKIE9);
mask_shape_component!(ShapeSidedCookie12, ShapeSidedCookie12Props, "12-sided cookie shape mask component.", data::SIDED_COOKIE12);
mask_shape_component!(ShapeSlanted, ShapeSlantedProps, "Slanted shape mask component.", data::SLANTED);
mask_shape_component!(ShapeSoftBoom, ShapeSoftBoomProps, "Soft boom shape mask component.", data::SOFT_BOOM);
mask_shape_component!(ShapeSoftBurst, ShapeSoftBurstProps, "Soft burst shape mask component.", data::SOFT_BURST);
mask_shape_component!(ShapeSunny, ShapeSunnyProps, "Sunny/sunburst shape mask component.", data::SUNNY);
mask_shape_component!(ShapeTriangle, ShapeTriangleProps, "Triangle shape mask component.", data::TRIANGLE);
mask_shape_component!(ShapeVerySunny, ShapeVerySunnyProps, "Very sunny shape mask component.", data::VERY_SUNNY);
