use yew::prelude::*;
use material_rs::components::{
    Shape, ShapeType, ShapeArch, ShapeArrow, ShapeBoom, ShapeBun, ShapeBurst, ShapeClamshell,
    ShapeFan, ShapeFlower, ShapeGem, ShapeGhostIsh, ShapeHeart, ShapeLeafClover4, ShapeLeafClover8,
    ShapeOval, ShapePentagon, ShapePixelCircle, ShapePixelTriangle, ShapePuffy, ShapePuffyDiamond,
    ShapeSemicircle, ShapeSidedCookie4, ShapeSidedCookie6, ShapeSidedCookie7, ShapeSidedCookie9,
    ShapeSidedCookie12, ShapeSlanted, ShapeSoftBoom, ShapeSoftBurst, ShapeSunny, ShapeTriangle,
    ShapeVerySunny,
};
use material_rs::theme::Theme;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn ShapeGalleryPage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let base_shape_props = vec![
        PropRow { name: "shape".into(), r#type: "ShapeType".into(), default_value: "ShapeType::RoundedSmall".into(), description: "The CSS-based shape variant to apply.".into() },
        PropRow { name: "tag".into(), r#type: "String".into(), default_value: "\"div\"".into(), description: "HTML tag to use as the container element.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Content to render inside the shaped container.".into() },
    ];

    let mask_shape_props = vec![
        PropRow { name: "size".into(), r#type: "String".into(), default_value: "\"3.5rem\"".into(), description: "Width and height of the shape.".into() },
        PropRow { name: "color".into(), r#type: "String".into(), default_value: "\"\" (theme primary)".into(), description: "Background color. Falls back to theme primary when empty.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Content to render inside the shape.".into() },
    ];

    let css_shapes = vec![
        ("Circle", ShapeType::Circle),
        ("Pill", ShapeType::Pill),
        ("Rounded Small", ShapeType::RoundedSmall),
        ("Rounded Medium", ShapeType::RoundedMedium),
        ("Rounded Large", ShapeType::RoundedLarge),
        ("Rounded XL", ShapeType::RoundedExtraLarge),
        ("Beveled", ShapeType::Beveled),
        ("Diamond", ShapeType::Diamond),
        ("Square", ShapeType::Square),
        ("Top Round", ShapeType::TopRound),
        ("Bottom Round", ShapeType::BottomRound),
        ("Left Round", ShapeType::LeftRound),
        ("Right Round", ShapeType::RightRound),
    ];

    html! {
        <>
            <Section title="CSS Shapes">
                <p style="font-size: 14px; line-height: 1.6; color: {}; margin-bottom: 24px;">
                    {"CSS-based shapes use border-radius and clip-path to create shapes. \
                      They wrap children content and are useful for clipping or decorative backgrounds."}
                </p>

                <Demo title="All CSS Shape Variants">
                    <div style="display: flex; gap: 16px; align-items: center; flex-wrap: wrap;">
                        { for css_shapes.iter().map(|(name, shape_type)| {
                            html! {
                                <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
                                    <Shape shape={*shape_type}>
                                        <div style={format!("width: 64px; height: 64px; background-color: {};", theme.colors.primary_container)} />
                                    </Shape>
                                    <span style="font-size: 10px; color: {};">{ name }</span>
                                </div>
                            }
                        })}
                    </div>
                </Demo>

                <CodeBlock code={r#"<Shape shape={ShapeType::Circle}>
    <div style="width: 64px; height: 64px; background-color: primary_container;" />
</Shape>
<Shape shape={ShapeType::Pill}>
    <div style="width: 120px; height: 40px; background-color: secondary_container;" />
</Shape>"#.to_string()} language={"rust".to_string()} />

                <PropTable rows={base_shape_props} />
            </Section>

            <Section title="Mask Shapes (SVG)">
                <p style="font-size: 14px; line-height: 1.6; color: {}; margin-bottom: 24px;">
                    {"Expressive mask shapes use SVG mask-image to create organic, decorative shapes. \
                      They come from Material Design 3 Expressive guidelines and are perfect for \
                      badges, avatars, decorative elements, and creative layouts."}
                </p>

                <Demo title="Default Colors (Primary)">
                    <div style="display: flex; gap: 16px; align-items: center; flex-wrap: wrap;">
                        <ShapeArch size="4rem" />
                        <ShapeArrow size="4rem" />
                        <ShapeBoom size="4rem" />
                        <ShapeBun size="4rem" />
                        <ShapeBurst size="4rem" />
                        <ShapeClamshell size="4rem" />
                        <ShapeFan size="4rem" />
                        <ShapeFlower size="4rem" />
                        <ShapeGem size="4rem" />
                        <ShapeGhostIsh size="4rem" />
                        <ShapeHeart size="4rem" />
                        <ShapeLeafClover4 size="4rem" />
                        <ShapeLeafClover8 size="4rem" />
                        <ShapeOval size="4rem" />
                        <ShapePentagon size="4rem" />
                        <ShapePixelCircle size="4rem" />
                        <ShapePixelTriangle size="4rem" />
                        <ShapePuffy size="4rem" />
                        <ShapePuffyDiamond size="4rem" />
                        <ShapeSemicircle size="4rem" />
                        <ShapeSidedCookie4 size="4rem" />
                        <ShapeSidedCookie6 size="4rem" />
                        <ShapeSidedCookie7 size="4rem" />
                        <ShapeSidedCookie9 size="4rem" />
                        <ShapeSidedCookie12 size="4rem" />
                        <ShapeSlanted size="4rem" />
                        <ShapeSoftBoom size="4rem" />
                        <ShapeSoftBurst size="4rem" />
                        <ShapeSunny size="4rem" />
                        <ShapeTriangle size="4rem" />
                        <ShapeVerySunny size="4rem" />
                    </div>
                </Demo>

                <CodeBlock code={r#"<ShapeHeart size="4rem" />
<ShapeFlower size="3rem" color={theme.colors.secondary.clone()} />
<ShapeGem size="5rem" color={theme.colors.error.clone()} />"#.to_string()} language={"rust".to_string()} />

                <Demo title="Custom Colors and Sizes">
                    <div style="display: flex; gap: 16px; align-items: center; flex-wrap: wrap;">
                        <ShapeHeart size="3rem" color={theme.colors.tertiary.clone()} />
                        <ShapeFlower size="3rem" color={theme.colors.secondary.clone()} />
                        <ShapeGem size="3rem" color={theme.colors.error.clone()} />
                        <ShapeArch size="5rem" color={theme.colors.primary_container.clone()} />
                        <ShapeBurst size="5rem" color={theme.colors.tertiary_container.clone()} />
                    </div>
                </Demo>

                <PropTable rows={mask_shape_props} />
            </Section>
        </>
    }
}
