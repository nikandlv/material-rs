use yew::prelude::*;
use material_rs::components::{Spacer, Box};
use material_rs::theme::Theme;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn SpacerPage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let props = vec![
        PropRow { name: "width".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Fixed width (e.g. \"16px\"). When set, the spacer uses inline-block with this width instead of flex expansion.".into() },
        PropRow { name: "height".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Fixed height (e.g. \"24px\"). When set, the spacer uses inline-block with this height instead of flex expansion.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    html! {
        <>
            <Section title="Spacer">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Spacer is a flexbox utility component that creates empty space in layouts. \
                      Without props it expands to fill available space (flex: 1). With width or \
                      height props it renders as a fixed-size inline block, useful for consistent \
                      gaps between elements."}
                </p>

                // ── Flex Spacer ──
                <Demo title="Flex Spacer">
                    <Box display="flex" align_items="center" style="width: 100%; background: var(--md-sys-color-surface-container-high); border-radius: 8px; padding: 12px;">
                        <div style={format!("padding: 8px 16px; background: {}; border-radius: 6px; color: {}; font-size: 13px;", theme.colors.primary, theme.colors.on_primary)}>
                            {"Left"}
                        </div>
                        <Spacer />
                        <div style={format!("padding: 8px 16px; background: {}; border-radius: 6px; color: {}; font-size: 13px;", theme.colors.primary, theme.colors.on_primary)}>
                            {"Right"}
                        </div>
                    </Box>
                </Demo>
                <CodeBlock code={r#"<Box display="flex" align_items="center">
    <div>{ "Left" }</div>
    <Spacer />
    <div>{ "Right" }</div>
</Box>"#.to_string()} language={"rust".to_string()} />

                // ── Multiple Flex Spacers ──
                <Demo title="Multiple Flex Spacers (Even Distribution)">
                    <Box display="flex" align_items="center" style="width: 100%; background: var(--md-sys-color-surface-container-high); border-radius: 8px; padding: 12px;">
                        <div style={format!("padding: 8px 16px; background: {}; border-radius: 6px; color: {}; font-size: 13px;", theme.colors.secondary, theme.colors.on_secondary)}>
                            {"A"}
                        </div>
                        <Spacer />
                        <div style={format!("padding: 8px 16px; background: {}; border-radius: 6px; color: {}; font-size: 13px;", theme.colors.secondary, theme.colors.on_secondary)}>
                            {"B"}
                        </div>
                        <Spacer />
                        <div style={format!("padding: 8px 16px; background: {}; border-radius: 6px; color: {}; font-size: 13px;", theme.colors.secondary, theme.colors.on_secondary)}>
                            {"C"}
                        </div>
                    </Box>
                </Demo>
                <CodeBlock code={r#"<Box display="flex" align_items="center">
    <div>{ "A" }</div>
    <Spacer />
    <div>{ "B" }</div>
    <Spacer />
    <div>{ "C" }</div>
</Box>"#.to_string()} language={"rust".to_string()} />

                // ── Fixed Width Spacer ──
                <Demo title="Fixed Width Spacer">
                    <Box display="flex" align_items="center" style="width: 100%; background: var(--md-sys-color-surface-container-high); border-radius: 8px; padding: 12px;">
                        <div style={format!("padding: 8px 16px; background: {}; border-radius: 6px; color: {}; font-size: 13px;", theme.colors.tertiary, theme.colors.on_tertiary)}>
                            {"Before"}
                        </div>
                        <Spacer width="48px" />
                        <div style={format!("padding: 8px 16px; background: {}; border-radius: 6px; color: {}; font-size: 13px;", theme.colors.tertiary, theme.colors.on_tertiary)}>
                            {"After"}
                        </div>
                    </Box>
                </Demo>
                <CodeBlock code={r#"<Box display="flex" align_items="center">
    <div>{ "Before" }</div>
    <Spacer width="48px" />
    <div>{ "After" }</div>
</Box>"#.to_string()} language={"rust".to_string()} />

                // ── Fixed Height Spacer ──
                <Demo title="Fixed Height Spacer">
                    <div style={format!("background: {}; border-radius: 8px; padding: 16px;", theme.colors.surface_container_high)}>
                        <div style={format!("padding: 8px 16px; background: {}; border-radius: 6px; color: {}; font-size: 13px; width: fit-content;", theme.colors.primary, theme.colors.on_primary)}>
                            {"Top content"}
                        </div>
                        <Spacer height="32px" />
                        <div style={format!("padding: 8px 16px; background: {}; border-radius: 6px; color: {}; font-size: 13px; width: fit-content;", theme.colors.primary, theme.colors.on_primary)}>
                            {"Bottom content (32px gap)"}
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={r#"<div>
    <div>{ "Top content" }</div>
    <Spacer height="32px" />
    <div>{ "Bottom content (32px gap)" }</div>
</div>"#.to_string()} language={"rust".to_string()} />

                // ── Spacer with Box Layout ──
                <Demo title="Using Spacer with Box for Layout">
                    <Box display="flex" align_items="stretch" style="width: 100%; height: 120px; background: var(--md-sys-color-surface-container-high); border-radius: 8px; padding: 12px; gap: 8px;">
                        <Box display="flex" align_items="center" justify_content="center" style={format!("flex: 1; background: {}; border-radius: 6px; color: {};", theme.colors.primary_container, theme.colors.on_primary_container)}>
                            {"1"}
                        </Box>
                        <Spacer width="16px" />
                        <Box display="flex" align_items="center" justify_content="center" style={format!("flex: 2; background: {}; border-radius: 6px; color: {};", theme.colors.secondary_container, theme.colors.on_secondary_container)}>
                            {"2 (double width)"}
                        </Box>
                        <Spacer width="16px" />
                        <Box display="flex" align_items="center" justify_content="center" style={format!("flex: 1; background: {}; border-radius: 6px; color: {};", theme.colors.tertiary_container, theme.colors.on_tertiary_container)}>
                            {"3"}
                        </Box>
                    </Box>
                </Demo>
                <CodeBlock code={r#"<Box display="flex" align_items="stretch" style="height: 120px;">
    <Box display="flex" align_items="center" justify_content="center"
         style="flex: 1; background: ...;">
        { "1" }
    </Box>
    <Spacer width="16px" />
    <Box display="flex" align_items="center" justify_content="center"
         style="flex: 2; background: ...;">
        { "2 (double width)" }
    </Box>
    <Spacer width="16px" />
    <Box display="flex" align_items="center" justify_content="center"
         style="flex: 1; background: ...;">
        { "3" }
    </Box>
</Box>"#.to_string()} language={"rust".to_string()} />

                <PropTable rows={props} />
            </Section>
        </>
    }
}
