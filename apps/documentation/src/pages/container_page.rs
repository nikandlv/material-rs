use yew::prelude::*;
use material_rs::components::Container;
use material_rs::theme::Theme;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn ContainerPage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let props = vec![
        PropRow { name: "max_width".into(), r#type: "String".into(), default_value: "\"lg\"".into(), description: "Maximum width breakpoint: \"xs\" (444px), \"sm\" (600px), \"md\" (905px), \"lg\" (1240px), \"xl\" (1440px), \"none\" (100%), or custom CSS value.".into() },
        PropRow { name: "disable_gutters".into(), r#type: "bool".into(), default_value: "false".into(), description: "Removes horizontal padding gutters.".into() },
        PropRow { name: "padding".into(), r#type: "String".into(), default_value: "\"medium\"".into(), description: "Horizontal padding: \"none\", \"small\" (8px), \"medium\" (16px), \"large\" (24px), or custom CSS.".into() },
        PropRow { name: "height".into(), r#type: "String".into(), default_value: "\"auto\"".into(), description: "Height constraint: \"auto\", \"min\", \"max\", or custom CSS value like \"100vh\".".into() },
        PropRow { name: "tag".into(), r#type: "String".into(), default_value: "\"div\"".into(), description: "HTML element tag: \"div\", \"main\", or \"section\".".into() },
        PropRow { name: "style".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Inline CSS style string.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Container content.".into() },
    ];

    html! {
        <>
            <Section title="Container">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Containers center-align page content and limit its maximum width. They provide \
                      responsive horizontal padding and optional gutters, following Material Design \
                      layout principles for readable, well-structured pages."}
                </p>

                // ── Default Container ──
                <Demo title="Default Container (lg)">
                    <Container>
                        <div style={format!("padding: 20px; background: {}; border-radius: 12px; text-align: center;", theme.colors.primary_container)}>
                            <span style={format!("color: {};", theme.colors.on_primary_container)}>{"Default container with max-width: 1240px"}</span>
                        </div>
                    </Container>
                </Demo>
                <CodeBlock code={r#"<Container>
    <div>{ "Default container with max-width: 1240px" }</div>
</Container>"#.to_string()} language={"rust".to_string()} />

                // ── Fixed Width Variants ──
                <Demo title="Fixed Width Variants">
                    <div style="display: flex; flex-direction: column; gap: 12px; width: 100%;">
                        <Container max_width="sm">
                            <div style={format!("padding: 16px; background: {}; border-radius: 8px; text-align: center;", theme.colors.secondary_container)}>
                                <span style={format!("color: {};", theme.colors.on_secondary_container)}>{"sm — 600px"}</span>
                            </div>
                        </Container>
                        <Container max_width="md">
                            <div style={format!("padding: 16px; background: {}; border-radius: 8px; text-align: center;", theme.colors.tertiary_container)}>
                                <span style={format!("color: {};", theme.colors.on_tertiary_container)}>{"md — 905px"}</span>
                            </div>
                        </Container>
                        <Container max_width="lg">
                            <div style={format!("padding: 16px; background: {}; border-radius: 8px; text-align: center;", theme.colors.primary_container)}>
                                <span style={format!("color: {};", theme.colors.on_primary_container)}>{"lg — 1240px (default)"}</span>
                            </div>
                        </Container>
                        <Container max_width="xl">
                            <div style={format!("padding: 16px; background: {}; border-radius: 8px; text-align: center;", theme.colors.surface_container_high)}>
                                <span style={format!("color: {};", theme.colors.on_surface)}>{"xl — 1440px"}</span>
                            </div>
                        </Container>
                    </div>
                </Demo>
                <CodeBlock code={r#"<Container max_width="sm">{"sm — 600px"}</Container>
<Container max_width="md">{"md — 905px"}</Container>
<Container max_width="lg">{"lg — 1240px (default)"}</Container>
<Container max_width="xl">{"xl — 1440px"}</Container>"#.to_string()} language={"rust".to_string()} />

                // ── Fluid Container ──
                <Demo title="Fluid Container">
                    <Container max_width="none">
                        <div style={format!("padding: 20px; background: {}; border-radius: 12px; text-align: center;", theme.colors.primary_container)}>
                            <span style={format!("color: {};", theme.colors.on_primary_container)}>{"Fluid container — full width (no max-width)"}</span>
                        </div>
                    </Container>
                </Demo>
                <CodeBlock code={r#"<Container max_width="none">
    <div>{ "Fluid container — full width" }</div>
</Container>"#.to_string()} language={"rust".to_string()} />

                // ── No Gutters ──
                <Demo title="No Gutters">
                    <Container disable_gutters={true}>
                        <div style={format!("padding: 20px; background: {}; border-radius: 12px; text-align: center;", theme.colors.secondary_container)}>
                            <span style={format!("color: {};", theme.colors.on_secondary_container)}>{"No gutters — no horizontal padding"}</span>
                        </div>
                    </Container>
                </Demo>
                <CodeBlock code={r#"<Container disable_gutters={true}>
    <div>{ "No gutters — no horizontal padding" }</div>
</Container>"#.to_string()} language={"rust".to_string()} />

                // ── Padding Variants ──
                <Demo title="Padding Variants">
                    <div style="display: flex; flex-direction: column; gap: 12px; width: 100%;">
                        <Container padding="none">
                            <div style={format!("padding: 12px; background: {}; border-radius: 8px; text-align: center;", theme.colors.surface_container_high)}>
                                <span style={format!("color: {};", theme.colors.on_surface)}>{"padding: none"}</span>
                            </div>
                        </Container>
                        <Container padding="small">
                            <div style={format!("padding: 12px; background: {}; border-radius: 8px; text-align: center;", theme.colors.surface_container_high)}>
                                <span style={format!("color: {};", theme.colors.on_surface)}>{"padding: small (8px)"}</span>
                            </div>
                        </Container>
                        <Container padding="medium">
                            <div style={format!("padding: 12px; background: {}; border-radius: 8px; text-align: center;", theme.colors.surface_container_high)}>
                                <span style={format!("color: {};", theme.colors.on_surface)}>{"padding: medium (16px) — default"}</span>
                            </div>
                        </Container>
                        <Container padding="large">
                            <div style={format!("padding: 12px; background: {}; border-radius: 8px; text-align: center;", theme.colors.surface_container_high)}>
                                <span style={format!("color: {};", theme.colors.on_surface)}>{"padding: large (24px)"}</span>
                            </div>
                        </Container>
                    </div>
                </Demo>
                <CodeBlock code={r#"<Container padding="none">{"padding: none"}</Container>
<Container padding="small">{"padding: small (8px)"}</Container>
<Container padding="medium">{"padding: medium (16px) — default"}</Container>
<Container padding="large">{"padding: large (24px)"}</Container>"#.to_string()} language={"rust".to_string()} />

                <PropTable rows={props} />
            </Section>
        </>
    }
}
