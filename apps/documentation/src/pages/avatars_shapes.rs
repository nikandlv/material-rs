use yew::prelude::*;
use material_rs::components::{Avatar, AvatarShape, Shape, ShapeType};
use material_rs::theme::Theme;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn AvatarsShapesPage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let avatar_props = vec![
        PropRow { name: "src".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "URL of the avatar image. When provided, displays an <img> instead of text/icon.".into() },
        PropRow { name: "initials".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Fallback text/initials displayed when no image is provided. Typically 1-2 characters.".into() },
        PropRow { name: "icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Material icon name displayed when no image or initials are provided.".into() },
        PropRow { name: "shape".into(), r#type: "AvatarShape".into(), default_value: "AvatarShape::Circle".into(), description: "Shape of the avatar: Circle (fully round), Squircle (super-ellipse), or Square (4px radius).".into() },
        PropRow { name: "size".into(), r#type: "String".into(), default_value: "\"md\"".into(), description: "Size preset: \"xs\" (24px), \"sm\" (32px), \"md\" (40px), \"lg\" (56px), \"xl\" (72px), or a custom pixel value like \"48px\".".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    let shape_props = vec![
        PropRow { name: "shape".into(), r#type: "ShapeType".into(), default_value: "ShapeType::RoundedSmall".into(), description: "The shape variant to apply to the children.".into() },
        PropRow { name: "tag".into(), r#type: "String".into(), default_value: "\"div\"".into(), description: "HTML tag to use as the container element.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Content to render inside the shaped container.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    html! {
        <>
            <Section title="Avatars">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Avatars represent a person or entity with an image, initials, or icon. \
                      They come in three shape variants (Circle, Squircle, Square) and five \
                      size presets. Avatars are commonly used in lists, profiles, and comments."}
                </p>

                // ── All Sizes ──
                <Demo title="Sizes">
                    <div style="display: flex; gap: 16px; align-items: center; flex-wrap: wrap;">
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
                            <Avatar size="xs" initials="XS" />
                            <span style={format!("font-size: 11px; color: {};", theme.colors.on_surface_variant)}>{"XS (24px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
                            <Avatar size="sm" initials="SM" />
                            <span style={format!("font-size: 11px; color: {};", theme.colors.on_surface_variant)}>{"SM (32px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
                            <Avatar size="md" initials="MD" />
                            <span style={format!("font-size: 11px; color: {};", theme.colors.on_surface_variant)}>{"MD (40px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
                            <Avatar size="lg" initials="LG" />
                            <span style={format!("font-size: 11px; color: {};", theme.colors.on_surface_variant)}>{"LG (56px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
                            <Avatar size="xl" initials="XL" />
                            <span style={format!("font-size: 11px; color: {};", theme.colors.on_surface_variant)}>{"XL (72px)"}</span>
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={"<Avatar size=\"xs\" initials=\"XS\" />   // 24px\n<Avatar size=\"sm\" initials=\"SM\" />   // 32px\n<Avatar size=\"md\" initials=\"MD\" />   // 40px (default)\n<Avatar size=\"lg\" initials=\"LG\" />   // 56px\n<Avatar size=\"xl\" initials=\"XL\" />   // 72px\n\n// Or use a custom pixel value:\n<Avatar size=\"48px\" initials=\"CU\" /> // 48px custom".to_string()} language={"rust".to_string()} />

                // ── All Variants (Shapes) ──
                <Demo title="Shape Variants">
                    <div style="display: flex; gap: 16px; align-items: flex-end; flex-wrap: wrap;">
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
                            <Avatar size="lg" initials="CI" shape={AvatarShape::Circle} />
                            <span style={format!("font-size: 11px; color: {};", theme.colors.on_surface_variant)}>{"Circle"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
                            <Avatar size="lg" initials="SQ" shape={AvatarShape::Squircle} />
                            <span style={format!("font-size: 11px; color: {};", theme.colors.on_surface_variant)}>{"Squircle"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
                            <Avatar size="lg" initials="RE" shape={AvatarShape::Square} />
                            <span style={format!("font-size: 11px; color: {};", theme.colors.on_surface_variant)}>{"Square"}</span>
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={"<Avatar size=\"lg\" initials=\"CI\" shape={AvatarShape::Circle} />    // default\n<Avatar size=\"lg\" initials=\"SQ\" shape={AvatarShape::Squircle} /> // super-ellipse\n<Avatar size=\"lg\" initials=\"RE\" shape={AvatarShape::Square} />    // 4px radius".to_string()} language={"rust".to_string()} />

                // ── Icon Avatar ──
                <Demo title="Icon Avatar">
                    <div style="display: flex; gap: 16px; align-items: center;">
                        <Avatar size="md" icon="person" />
                        <Avatar size="lg" icon="group" />
                        <Avatar size="xl" icon="settings" />
                        <Avatar size="lg" icon="star" shape={AvatarShape::Squircle} />
                        <Avatar size="md" icon="palette" shape={AvatarShape::Square} />
                    </div>
                </Demo>
                <CodeBlock code={"<Avatar size=\"md\" icon=\"person\" />\n<Avatar size=\"lg\" icon=\"group\" />\n<Avatar size=\"xl\" icon=\"settings\" />\n<Avatar size=\"lg\" icon=\"star\" shape={AvatarShape::Squircle} />\n<Avatar size=\"md\" icon=\"palette\" shape={AvatarShape::Square} />".to_string()} language={"rust".to_string()} />

                // ── Fallback Text Avatar ──
                <Demo title="Fallback Text (Initials)">
                    <div style="display: flex; gap: 16px; align-items: center;">
                        <Avatar size="md" initials="AB" />
                        <Avatar size="md" initials="CD" />
                        <Avatar size="md" initials="EF" />
                        <Avatar size="lg" initials="JD" />
                        <Avatar size="lg" initials="RS" shape={AvatarShape::Squircle} />
                        <Avatar size="xl" initials="MT" shape={AvatarShape::Square} />
                    </div>
                </Demo>
                <CodeBlock code={"<Avatar size=\"md\" initials=\"AB\" />\n<Avatar size=\"md\" initials=\"CD\" />\n<Avatar size=\"lg\" initials=\"JD\" />\n<Avatar size=\"lg\" initials=\"RS\" shape={AvatarShape::Squircle} />\n<Avatar size=\"xl\" initials=\"MT\" shape={AvatarShape::Square} />".to_string()} language={"rust".to_string()} />

                // ── Image Avatar ──
                <Demo title="Image Avatar">
                    <div style="display: flex; gap: 16px; align-items: center;">
                        <Avatar size="md" src="https://i.pravatar.cc/80?img=1" />
                        <Avatar size="lg" src="https://i.pravatar.cc/80?img=2" />
                        <Avatar size="xl" src="https://i.pravatar.cc/80?img=3" />
                        <Avatar size="lg" src="https://i.pravatar.cc/80?img=4" shape={AvatarShape::Squircle} />
                        <Avatar size="lg" src="https://i.pravatar.cc/80?img=5" shape={AvatarShape::Square} />
                    </div>
                </Demo>
                <CodeBlock code={"// When src is provided, an <img> is rendered with object-fit: cover\n<Avatar size=\"md\" src=\"https://example.com/avatar.jpg\" />\n<Avatar size=\"lg\" src=\"https://example.com/avatar.jpg\" />\n<Avatar size=\"xl\" src=\"https://example.com/avatar.jpg\" />\n<Avatar size=\"lg\" src=\"https://example.com/avatar.jpg\" shape={AvatarShape::Squircle} />\n<Avatar size=\"lg\" src=\"https://example.com/avatar.jpg\" shape={AvatarShape::Square} />".to_string()} language={"rust".to_string()} />

                // ── Fallback Priority ──
                <Demo title="Fallback Priority">
                    <div style="display: flex; gap: 16px; align-items: center;">
                        <Avatar size="lg" src="https://invalid-url.example.com/broken.jpg" initials="AB" />
                        <Avatar size="lg" initials="CD" />
                        <Avatar size="lg" icon="person" />
                        <Avatar size="lg" />
                    </div>
                </Demo>
                <CodeBlock code={"// Rendering priority: src (image) > initials (text) > icon > default person icon\n// If src URL fails, initials or icon are shown as fallback\n<Avatar size=\"lg\" src=\"invalid.jpg\" initials=\"AB\" /> // shows initials if image fails\n<Avatar size=\"lg\" initials=\"CD\" />   // shows \"CD\"\n<Avatar size=\"lg\" icon=\"person\" />   // shows person icon\n<Avatar size=\"lg\" />                  // shows default person icon".to_string()} language={"rust".to_string()} />

                <PropTable rows={avatar_props} />
            </Section>

            <Section title="Shapes">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"The Shape component applies MD3 shape tokens to its children using border-radius \
                      or CSS clip-path. It provides 13 built-in shape types covering common UI patterns \
                      from simple rounding to expressive geometric forms."}
                </p>

                // ── CSS-Based Shapes ──
                <Demo title="CSS-Based Shapes (border-radius)">
                    <div style="display: flex; gap: 16px; align-items: flex-end; flex-wrap: wrap;">
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 6px;">
                            <Shape shape={ShapeType::Circle}><div style={format!("width: 64px; height: 64px; background-color: {}; display: flex; align-items: center; justify-content: center; font-size: 11px; font-weight: 500; color: {};", theme.colors.primary_container, theme.colors.on_primary_container)}>{"Circle"}</div></Shape>
                            <span style={format!("font-size: 10px; color: {};", theme.colors.on_surface_variant)}>{"Circle"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 6px;">
                            <Shape shape={ShapeType::Pill}><div style={format!("padding: 12px 24px; background-color: {}; font-weight: 500; font-size: 12px; color: {};", theme.colors.primary_container, theme.colors.on_primary_container)}>{"Pill"}</div></Shape>
                            <span style={format!("font-size: 10px; color: {};", theme.colors.on_surface_variant)}>{"Pill"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 6px;">
                            <Shape shape={ShapeType::RoundedSmall}><div style={format!("padding: 12px 20px; background-color: {}; font-weight: 500; font-size: 12px; color: {};", theme.colors.secondary_container, theme.colors.on_secondary_container)}>{"RoundedSm"}</div></Shape>
                            <span style={format!("font-size: 10px; color: {};", theme.colors.on_surface_variant)}>{"RoundedSmall (8px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 6px;">
                            <Shape shape={ShapeType::RoundedMedium}><div style={format!("padding: 12px 20px; background-color: {}; font-weight: 500; font-size: 12px; color: {};", theme.colors.secondary_container, theme.colors.on_secondary_container)}>{"RoundedMd"}</div></Shape>
                            <span style={format!("font-size: 10px; color: {};", theme.colors.on_surface_variant)}>{"RoundedMedium (12px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 6px;">
                            <Shape shape={ShapeType::RoundedLarge}><div style={format!("padding: 12px 20px; background-color: {}; font-weight: 500; font-size: 12px; color: {};", theme.colors.secondary_container, theme.colors.on_secondary_container)}>{"RoundedLg"}</div></Shape>
                            <span style={format!("font-size: 10px; color: {};", theme.colors.on_surface_variant)}>{"RoundedLarge (16px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 6px;">
                            <Shape shape={ShapeType::RoundedExtraLarge}><div style={format!("padding: 12px 20px; background-color: {}; font-weight: 500; font-size: 12px; color: {};", theme.colors.secondary_container, theme.colors.on_secondary_container)}>{"RoundedXL"}</div></Shape>
                            <span style={format!("font-size: 10px; color: {};", theme.colors.on_surface_variant)}>{"RoundedExtraLarge (28px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 6px;">
                            <Shape shape={ShapeType::Square}><div style={format!("padding: 12px 20px; background-color: {}; font-weight: 500; font-size: 12px; color: {};", theme.colors.surface_container_highest, theme.colors.on_surface)}>{"Square"}</div></Shape>
                            <span style={format!("font-size: 10px; color: {};", theme.colors.on_surface_variant)}>{"Square (0px)"}</span>
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={"<Shape shape={ShapeType::Circle}><div>...</div></Shape>\n<Shape shape={ShapeType::Pill}><div>...</div></Shape>\n<Shape shape={ShapeType::RoundedSmall}><div>...</div></Shape>    // 8px\n<Shape shape={ShapeType::RoundedMedium}><div>...</div></Shape>   // 12px\n<Shape shape={ShapeType::RoundedLarge}><div>...</div></Shape>    // 16px\n<Shape shape={ShapeType::RoundedExtraLarge}><div>...</div></Shape> // 28px\n<Shape shape={ShapeType::Square}><div>...</div></Shape>          // 0px".to_string()} language={"rust".to_string()} />

                // ── Partial Round Shapes ──
                <Demo title="Partial Round Shapes">
                    <div style="display: flex; gap: 16px; align-items: flex-end; flex-wrap: wrap;">
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 6px;">
                            <Shape shape={ShapeType::TopRound}><div style={format!("padding: 12px 20px; background-color: {}; font-weight: 500; font-size: 12px; color: {};", theme.colors.tertiary_container, theme.colors.on_tertiary_container)}>{"Top Round"}</div></Shape>
                            <span style={format!("font-size: 10px; color: {};", theme.colors.on_surface_variant)}>{"TopRound"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 6px;">
                            <Shape shape={ShapeType::BottomRound}><div style={format!("padding: 12px 20px; background-color: {}; font-weight: 500; font-size: 12px; color: {};", theme.colors.tertiary_container, theme.colors.on_tertiary_container)}>{"Bottom Round"}</div></Shape>
                            <span style={format!("font-size: 10px; color: {};", theme.colors.on_surface_variant)}>{"BottomRound"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 6px;">
                            <Shape shape={ShapeType::LeftRound}><div style={format!("padding: 12px 20px; background-color: {}; font-weight: 500; font-size: 12px; color: {};", theme.colors.tertiary_container, theme.colors.on_tertiary_container)}>{"Left Round"}</div></Shape>
                            <span style={format!("font-size: 10px; color: {};", theme.colors.on_surface_variant)}>{"LeftRound"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 6px;">
                            <Shape shape={ShapeType::RightRound}><div style={format!("padding: 12px 20px; background-color: {}; font-weight: 500; font-size: 12px; color: {};", theme.colors.tertiary_container, theme.colors.on_tertiary_container)}>{"Right Round"}</div></Shape>
                            <span style={format!("font-size: 10px; color: {};", theme.colors.on_surface_variant)}>{"RightRound"}</span>
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={"// Partial rounding — only specific corners are rounded\n<Shape shape={ShapeType::TopRound}><div>...</div></Shape>\n<Shape shape={ShapeType::BottomRound}><div>...</div></Shape>\n<Shape shape={ShapeType::LeftRound}><div>...</div></Shape>\n<Shape shape={ShapeType::RightRound}><div>...</div></Shape>".to_string()} language={"rust".to_string()} />

                // ── Clip-Path Shapes ──
                <Demo title="Clip-Path Shapes (expressive)">
                    <div style="display: flex; gap: 16px; align-items: flex-end; flex-wrap: wrap;">
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 6px;">
                            <Shape shape={ShapeType::Beveled}><div style={format!("width: 80px; height: 80px; background-color: {}; display: flex; align-items: center; justify-content: center; font-size: 11px; font-weight: 600; color: {};", theme.colors.primary_container, theme.colors.on_primary_container)}>{"Beveled"}</div></Shape>
                            <span style={format!("font-size: 10px; color: {};", theme.colors.on_surface_variant)}>{"Beveled"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 6px;">
                            <Shape shape={ShapeType::Diamond}><div style={format!("width: 80px; height: 80px; background-color: {}; display: flex; align-items: center; justify-content: center; font-size: 11px; font-weight: 600; color: {};", theme.colors.secondary_container, theme.colors.on_secondary_container)}>{"Diamond"}</div></Shape>
                            <span style={format!("font-size: 10px; color: {};", theme.colors.on_surface_variant)}>{"Diamond"}</span>
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={"// Clip-path based shapes for expressive geometric forms\n<Shape shape={ShapeType::Beveled}>\n    <div style=\"width: 80px; height: 80px; ...\">{\"Beveled\"}</div>\n</Shape>\n<Shape shape={ShapeType::Diamond}>\n    <div style=\"width: 80px; height: 80px; ...\">{\"Diamond\"}</div>\n</Shape>".to_string()} language={"rust".to_string()} />

                <PropTable rows={shape_props} />
            </Section>
        </>
    }
}
