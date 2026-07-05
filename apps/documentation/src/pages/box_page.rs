use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn BoxPage() -> Html {
    let box_props = vec![
        PropRow { name: "tag".into(), r#type: "String".into(), default_value: "\"div\"".into(), description: "HTML element tag: div, section, article, header, footer, aside, main, span.".into() },
        PropRow { name: "display".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS display property (flex, grid, block, inline-block, etc.).".into() },
        PropRow { name: "flex_direction".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS flex-direction (row, column, row-reverse, column-reverse).".into() },
        PropRow { name: "justify_content".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS justify-content for main axis alignment.".into() },
        PropRow { name: "align_items".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS align-items for cross axis alignment.".into() },
        PropRow { name: "gap".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS gap between flex/grid children.".into() },
        PropRow { name: "flex_wrap".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS flex-wrap (nowrap, wrap, wrap-reverse).".into() },
        PropRow { name: "padding".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS padding shorthand.".into() },
        PropRow { name: "px".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "Horizontal padding shorthand (padding-left + padding-right).".into() },
        PropRow { name: "py".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "Vertical padding shorthand (padding-top + padding-bottom).".into() },
        PropRow { name: "margin".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS margin shorthand.".into() },
        PropRow { name: "mx".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "Horizontal margin shorthand (margin-left + margin-right).".into() },
        PropRow { name: "my".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "Vertical margin shorthand (margin-top + margin-bottom).".into() },
        PropRow { name: "width".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS width.".into() },
        PropRow { name: "height".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS height.".into() },
        PropRow { name: "min_width".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS min-width.".into() },
        PropRow { name: "min_height".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS min-height.".into() },
        PropRow { name: "max_width".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS max-width.".into() },
        PropRow { name: "max_height".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS max-height.".into() },
        PropRow { name: "bg".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "Background color. Accepts theme names (primary, secondary, primary-container, surface, etc.) or raw CSS values.".into() },
        PropRow { name: "color".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "Text color. Accepts theme names (primary, secondary, tertiary, error, surface) or raw CSS values.".into() },
        PropRow { name: "border".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS border shorthand.".into() },
        PropRow { name: "border_radius".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS border-radius.".into() },
        PropRow { name: "box_shadow".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS box-shadow.".into() },
        PropRow { name: "position".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS position (relative, absolute, fixed, sticky).".into() },
        PropRow { name: "overflow".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS overflow shorthand.".into() },
        PropRow { name: "cursor".into(), r#type: "Option<String>".into(), default_value: "None".into(), description: "CSS cursor property.".into() },
        PropRow { name: "style".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Raw CSS string appended after typed props (highest specificity override).".into() },
        PropRow { name: "onclick".into(), r#type: "Callback<MouseEvent>".into(), default_value: "default".into(), description: "Click event handler.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Child content.".into() },
    ];

    html! {
        <>
            <Section title="Box">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Box is a generic CSS container that maps typed props directly to inline styles. \
                      It provides a typed API for all common CSS properties (display, flex, padding, margin, \
                      gap, dimensions, positioning, borders, backgrounds) without writing raw CSS strings. \
                      Color and background props resolve theme token names automatically."}
                </p>

                // ── Flex Row Layout ──
                <Demo title="Flex Row Layout">
                    <Box display="flex" flex_direction="row" gap="12px" align_items="center">
                        <Box bg="primary-container" px="16px" py="12px" border_radius="8px">
                            <span style="color: #6750A4;">{"Item 1"}</span>
                        </Box>
                        <Box bg="secondary-container" px="16px" py="12px" border_radius="8px">
                            <span style="color: #6750A4;">{"Item 2"}</span>
                        </Box>
                        <Box bg="surface-container" px="16px" py="12px" border_radius="8px" border="1px solid #CAC4D0">
                            <span>{"Item 3"}</span>
                        </Box>
                    </Box>
                </Demo>
                <CodeBlock code={"<Box display=\"flex\" flex_direction=\"row\" gap=\"12px\" align_items=\"center\">\n    <Box bg=\"primary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n        <span style=\"color: #6750A4;\">{\"Item 1\"}</span>\n    </Box>\n    <Box bg=\"secondary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n        <span style=\"color: #6750A4;\">{\"Item 2\"}</span>\n    </Box>\n    <Box bg=\"surface-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\"\n        border=\"1px solid #CAC4D0\">\n        <span>{\"Item 3\"}</span>\n    </Box>\n</Box>".to_string()} language={"rust".to_string()} />

                // ── Flex Column Layout ──
                <Demo title="Flex Column Layout">
                    <Box display="flex" flex_direction="column" gap="12px" width="300px">
                        <Box bg="primary" px="16px" py="12px" border_radius="8px">
                            <span style="color: #FFFFFF;">{"Primary"}</span>
                        </Box>
                        <Box bg="secondary" px="16px" py="12px" border_radius="8px">
                            <span style="color: #FFFFFF;">{"Secondary"}</span>
                        </Box>
                        <Box bg="tertiary" px="16px" py="12px" border_radius="8px">
                            <span style="color: #FFFFFF;">{"Tertiary"}</span>
                        </Box>
                    </Box>
                </Demo>
                <CodeBlock code={"<Box display=\"flex\" flex_direction=\"column\" gap=\"12px\" width=\"300px\">\n    <Box bg=\"primary\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n        <span style=\"color: #FFFFFF;\">{\"Primary\"}</span>\n    </Box>\n    <Box bg=\"secondary\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n        <span style=\"color: #FFFFFF;\">{\"Secondary\"}</span>\n    </Box>\n    <Box bg=\"tertiary\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n        <span style=\"color: #FFFFFF;\">{\"Tertiary\"}</span>\n    </Box>\n</Box>".to_string()} language={"rust".to_string()} />

                // ── Centered Content ──
                <Demo title="Centered Content">
                    <Box display="flex" justify_content="center" align_items="center" height="120px" bg="surface-container" border_radius="12px">
                        <Typography tag="span" variant={TypographyVariant::TitleMedium}>{"Centered"}</Typography>
                    </Box>
                </Demo>
                <CodeBlock code={"<Box display=\"flex\" justify_content=\"center\" align_items=\"center\"\n    height=\"120px\" bg=\"surface-container\" border_radius=\"12px\">\n    <Typography tag=\"span\" variant={TypographyVariant::TitleMedium}>{\"Centered\"}</Typography>\n</Box>".to_string()} language={"rust".to_string()} />

                // ── Padding & Margin ──
                <Demo title="Padding & Margin">
                    <Box bg="primary-container" px="24px" py="16px" border_radius="12px">
                        <Box bg="surface" px="16px" py="12px" border_radius="8px" border="1px solid #CAC4D0">
                            <span>{"Inner with padding"}</span>
                        </Box>
                    </Box>
                </Demo>
                <CodeBlock code={"<Box bg=\"primary-container\" px=\"24px\" py=\"16px\" border_radius=\"12px\">\n    <Box bg=\"surface\" px=\"16px\" py=\"12px\" border_radius=\"8px\"\n        border=\"1px solid #CAC4D0\">\n        <span>{\"Inner with padding\"}</span>\n    </Box>\n</Box>".to_string()} language={"rust".to_string()} />

                // ── Background Colors ──
                <Demo title="Background Colors">
                    <div style="display: flex; flex-wrap: wrap; gap: 12px;">
                        <Box bg="primary" px="16px" py="10px" border_radius="8px">
                            <span style="color: #FFFFFF;">{"primary"}</span>
                        </Box>
                        <Box bg="secondary" px="16px" py="10px" border_radius="8px">
                            <span style="color: #FFFFFF;">{"secondary"}</span>
                        </Box>
                        <Box bg="primary-container" px="16px" py="10px" border_radius="8px">
                            <span>{"primary-container"}</span>
                        </Box>
                        <Box bg="secondary-container" px="16px" py="10px" border_radius="8px">
                            <span>{"secondary-container"}</span>
                        </Box>
                        <Box bg="surface" px="16px" py="10px" border_radius="8px" border="1px solid #CAC4D0">
                            <span>{"surface"}</span>
                        </Box>
                        <Box bg="surface-container" px="16px" py="10px" border_radius="8px">
                            <span>{"surface-container"}</span>
                        </Box>
                        <Box bg="#FF6B35" px="16px" py="10px" border_radius="8px">
                            <span style="color: #FFFFFF;">{"raw CSS value"}</span>
                        </Box>
                    </div>
                </Demo>
                <CodeBlock code={"// Theme token colors (resolved automatically)\n<Box bg=\"primary\" ...>\n<Box bg=\"secondary\" ...>\n<Box bg=\"primary-container\" ...>\n<Box bg=\"secondary-container\" ...>\n<Box bg=\"surface\" ...>\n<Box bg=\"surface-container\" ...>\n\n// Raw CSS color values also work\n<Box bg=\"#FF6B35\" ...>".to_string()} language={"rust".to_string()} />

                // ── Border Radius ──
                <Demo title="Border Radius">
                    <div style="display: flex; flex-wrap: wrap; gap: 12px; align-items: center;">
                        <Box bg="primary-container" px="24px" py="16px" border_radius="0">
                            <span>{"No radius"}</span>
                        </Box>
                        <Box bg="primary-container" px="24px" py="16px" border_radius="8px">
                            <span>{"8px"}</span>
                        </Box>
                        <Box bg="primary-container" px="24px" py="16px" border_radius="16px">
                            <span>{"16px"}</span>
                        </Box>
                        <Box bg="primary-container" px="24px" py="16px" border_radius="50%">
                            <span>{"Circle"}</span>
                        </Box>
                    </div>
                </Demo>
                <CodeBlock code={"<Box bg=\"primary-container\" px=\"24px\" py=\"16px\" border_radius=\"0\">{\"No radius\"}</Box>\n<Box bg=\"primary-container\" px=\"24px\" py=\"16px\" border_radius=\"8px\">{\"8px\"}</Box>\n<Box bg=\"primary-container\" px=\"24px\" py=\"16px\" border_radius=\"16px\">{\"16px\"}</Box>\n<Box bg=\"primary-container\" px=\"24px\" py=\"16px\" border_radius=\"50%\">{\"Circle\"}</Box>".to_string()} language={"rust".to_string()} />

                // ── Gap Layouts ──
                <Demo title="Gap Layouts">
                    <div style="display: flex; flex-direction: column; gap: 16px; width: 100%;">
                        <Box display="flex" gap="8px" align_items="center">
                            <Box bg="primary" width="60px" height="60px" border_radius="8px" />
                            <Box bg="primary" width="60px" height="60px" border_radius="8px" />
                            <Box bg="primary" width="60px" height="60px" border_radius="8px" />
                            <span style="font-size: 12px; color: #49454F; margin-left: 8px;">{"gap=\"8px\""}</span>
                        </Box>
                        <Box display="flex" gap="24px" align_items="center">
                            <Box bg="secondary" width="60px" height="60px" border_radius="8px" />
                            <Box bg="secondary" width="60px" height="60px" border_radius="8px" />
                            <Box bg="secondary" width="60px" height="60px" border_radius="8px" />
                            <span style="font-size: 12px; color: #49454F; margin-left: 8px;">{"gap=\"24px\""}</span>
                        </Box>
                    </div>
                </Demo>
                <CodeBlock code={"<Box display=\"flex\" gap=\"8px\" align_items=\"center\">\n    <Box bg=\"primary\" width=\"60px\" height=\"60px\" border_radius=\"8px\" />\n    <Box bg=\"primary\" width=\"60px\" height=\"60px\" border_radius=\"8px\" />\n    <Box bg=\"primary\" width=\"60px\" height=\"60px\" border_radius=\"8px\" />\n</Box>\n\n<Box display=\"flex\" gap=\"24px\" align_items=\"center\">\n    <Box bg=\"secondary\" width=\"60px\" height=\"60px\" border_radius=\"8px\" />\n    <Box bg=\"secondary\" width=\"60px\" height=\"60px\" border_radius=\"8px\" />\n    <Box bg=\"secondary\" width=\"60px\" height=\"60px\" border_radius=\"8px\" />\n</Box>".to_string()} language={"rust".to_string()} />

                <PropTable rows={box_props} />
            </Section>
        </>
    }
}
