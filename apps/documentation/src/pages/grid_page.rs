use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn GridPage() -> Html {
    let container_props = vec![
        PropRow { name: "max_width".into(), r#type: "String".into(), default_value: "\"lg\"".into(), description: "Maximum width breakpoint: \"xs\" (444px), \"sm\" (600px), \"md\" (905px), \"lg\" (1240px), \"xl\" (1440px), \"none\" (100%), or custom CSS value.".into() },
        PropRow { name: "padding".into(), r#type: "String".into(), default_value: "\"medium\"".into(), description: "Horizontal padding: \"none\" (0), \"small\" (8px), \"medium\" (16px), \"large\" (24px), or custom CSS value.".into() },
        PropRow { name: "disable_gutters".into(), r#type: "bool".into(), default_value: "false".into(), description: "When true, removes all side padding/margin gutters.".into() },
        PropRow { name: "height".into(), r#type: "String".into(), default_value: "\"auto\"".into(), description: "Height constraint: \"auto\", \"min\", \"max\", or custom CSS value like \"100vh\".".into() },
        PropRow { name: "tag".into(), r#type: "String".into(), default_value: "\"div\"".into(), description: "HTML element tag: div, main, section.".into() },
        PropRow { name: "style".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Custom inline CSS style override.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Child content (typically Grid items).".into() },
    ];

    let grid_props = vec![
        PropRow { name: "container".into(), r#type: "bool".into(), default_value: "false".into(), description: "Renders as a CSS Grid container when true.".into() },
        PropRow { name: "item".into(), r#type: "bool".into(), default_value: "false".into(), description: "Renders as a grid item cell when true.".into() },
        PropRow { name: "columns".into(), r#type: "u32".into(), default_value: "12".into(), description: "Total grid columns for the container.".into() },
        PropRow { name: "spacing".into(), r#type: "String".into(), default_value: "\"16px\"".into(), description: "Gap between grid items. Raw number is treated as pixels.".into() },
        PropRow { name: "xs".into(), r#type: "Option<u32>".into(), default_value: "12".into(), description: "Column span on extra-small screens (0-599px). 1-12.".into() },
        PropRow { name: "sm".into(), r#type: "Option<u32>".into(), default_value: "inherits xs".into(), description: "Column span on small screens (600-904px). Inherits xs if not set.".into() },
        PropRow { name: "md".into(), r#type: "Option<u32>".into(), default_value: "inherits sm".into(), description: "Column span on medium screens (905-1239px). Inherits sm if not set.".into() },
        PropRow { name: "lg".into(), r#type: "Option<u32>".into(), default_value: "inherits md".into(), description: "Column span on large screens (1240-1439px). Inherits md if not set.".into() },
        PropRow { name: "xl".into(), r#type: "Option<u32>".into(), default_value: "inherits lg".into(), description: "Column span on extra-large screens (1440px+). Inherits lg if not set.".into() },
        PropRow { name: "justify_content".into(), r#type: "Option<String>".into(), default_value: "\"stretch\"".into(), description: "CSS justify-content for the grid container.".into() },
        PropRow { name: "align_items".into(), r#type: "Option<String>".into(), default_value: "\"stretch\"".into(), description: "CSS align-items for the grid container.".into() },
        PropRow { name: "tag".into(), r#type: "String".into(), default_value: "\"div\"".into(), description: "HTML element tag: div, section, main, span.".into() },
        PropRow { name: "style".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Custom inline CSS style override.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Child content.".into() },
    ];

    html! {
        <>
            <Section title="Container">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Container is a center-aligned layout wrapper that constrains page content to maximum \
                      widths matching MD3 breakpoints. It provides consistent horizontal padding and \
                      auto-centering for all content within it."}
                </p>

                <Demo title="Container Sizes">
                    <div style="display: flex; flex-direction: column; gap: 12px; width: 100%;">
                        <Container max_width="xs">
                            <Box bg="primary-container" px="16px" py="12px" border_radius="8px">
                                <span>{"max_width=\"xs\" (444px)"}</span>
                            </Box>
                        </Container>
                        <Container max_width="sm">
                            <Box bg="secondary-container" px="16px" py="12px" border_radius="8px">
                                <span>{"max_width=\"sm\" (600px)"}</span>
                            </Box>
                        </Container>
                        <Container max_width="md">
                            <Box bg="primary-container" px="16px" py="12px" border_radius="8px">
                                <span>{"max_width=\"md\" (905px)"}</span>
                            </Box>
                        </Container>
                    </div>
                </Demo>
                <CodeBlock code={"<Container max_width=\"xs\">\n    <Box bg=\"primary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n        <span>{\"max_width=xs (444px)\"}</span>\n    </Box>\n</Container>\n\n<Container max_width=\"sm\">\n    <Box bg=\"secondary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n        <span>{\"max_width=sm (600px)\"}</span>\n    </Box>\n</Container>\n\n<Container max_width=\"md\">\n    <Box bg=\"primary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n        <span>{\"max_width=md (905px)\"}</span>\n    </Box>\n</Container>".to_string()} language={"rust".to_string()} />

                <Demo title="Fluid Container">
                    <Container max_width="none" padding="none">
                        <Box bg="primary" px="16px" py="12px" border_radius="8px">
                            <span style="color: #FFFFFF;">{"Fluid container (no max-width constraint, full width)"}</span>
                        </Box>
                    </Container>
                </Demo>
                <CodeBlock code={"<Container max_width=\"none\" padding=\"none\">\n    <Box bg=\"primary\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n        <span style=\"color: #FFFFFF;\">{\"Fluid container (full width)\"}</span>\n    </Box>\n</Container>".to_string()} language={"rust".to_string()} />

                <PropTable rows={container_props} />
            </Section>

            <Section title="Grid">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Grid provides a responsive 12-column layout system using CSS Grid. Containers create \
                      the grid context, and items span columns at different responsive breakpoints \
                      (xs, sm, md, lg, xl). If a breakpoint is not specified, it inherits from the \
                      next smaller breakpoint."}
                </p>

                // ── Basic 12-Column Grid ──
                <Demo title="Basic 12-Column Grid">
                    <Container max_width="lg" padding="none">
                        <Grid container={true} spacing="16px">
                            <Grid item={true} xs={12}>
                                <Box bg="primary-container" px="16px" py="12px" border_radius="8px">
                                    <span>{"xs=12 (full width)"}</span>
                                </Box>
                            </Grid>
                            <Grid item={true} xs={6}>
                                <Box bg="secondary-container" px="16px" py="12px" border_radius="8px">
                                    <span>{"xs=6"}</span>
                                </Box>
                            </Grid>
                            <Grid item={true} xs={6}>
                                <Box bg="secondary-container" px="16px" py="12px" border_radius="8px">
                                    <span>{"xs=6"}</span>
                                </Box>
                            </Grid>
                            <Grid item={true} xs={4}>
                                <Box bg="primary-container" px="16px" py="12px" border_radius="8px">
                                    <span>{"xs=4"}</span>
                                </Box>
                            </Grid>
                            <Grid item={true} xs={4}>
                                <Box bg="primary-container" px="16px" py="12px" border_radius="8px">
                                    <span>{"xs=4"}</span>
                                </Box>
                            </Grid>
                            <Grid item={true} xs={4}>
                                <Box bg="primary-container" px="16px" py="12px" border_radius="8px">
                                    <span>{"xs=4"}</span>
                                </Box>
                            </Grid>
                            <Grid item={true} xs={3}>
                                <Box bg="surface-container" px="16px" py="12px" border_radius="8px" border="1px solid #CAC4D0">
                                    <span>{"xs=3"}</span>
                                </Box>
                            </Grid>
                            <Grid item={true} xs={3}>
                                <Box bg="surface-container" px="16px" py="12px" border_radius="8px" border="1px solid #CAC4D0">
                                    <span>{"xs=3"}</span>
                                </Box>
                            </Grid>
                            <Grid item={true} xs={3}>
                                <Box bg="surface-container" px="16px" py="12px" border_radius="8px" border="1px solid #CAC4D0">
                                    <span>{"xs=3"}</span>
                                </Box>
                            </Grid>
                            <Grid item={true} xs={3}>
                                <Box bg="surface-container" px="16px" py="12px" border_radius="8px" border="1px solid #CAC4D0">
                                    <span>{"xs=3"}</span>
                                </Box>
                            </Grid>
                        </Grid>
                    </Container>
                </Demo>
                <CodeBlock code={"<Container max_width=\"lg\" padding=\"none\">\n    <Grid container={true} spacing=\"16px\">\n        <Grid item={true} xs={12}>\n            <Box bg=\"primary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n                <span>{\"xs=12 (full width)\"}</span>\n            </Box>\n        </Grid>\n        <Grid item={true} xs={6}>\n            <Box bg=\"secondary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n                <span>{\"xs=6\"}</span>\n            </Box>\n        </Grid>\n        <Grid item={true} xs={6}>\n            <Box bg=\"secondary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n                <span>{\"xs=6\"}</span>\n            </Box>\n        </Grid>\n        <Grid item={true} xs={4}>\n            <Box bg=\"primary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n                <span>{\"xs=4\"}</span>\n            </Box>\n        </Grid>\n        <Grid item={true} xs={4}>\n            <Box bg=\"primary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n                <span>{\"xs=4\"}</span>\n            </Box>\n        </Grid>\n        <Grid item={true} xs={4}>\n            <Box bg=\"primary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n                <span>{\"xs=4\"}</span>\n            </Box>\n        </Grid>\n    </Grid>\n</Container>".to_string()} language={"rust".to_string()} />

                // ── Responsive Breakpoints ──
                <Demo title="Responsive Breakpoints">
                    <Container max_width="lg" padding="none">
                        <Grid container={true} spacing="16px">
                            <Grid item={true} xs={12} sm={6} md={4} lg={3}>
                                <Box bg="primary-container" px="16px" py="12px" border_radius="8px">
                                    <span>{"xs=12, sm=6, md=4, lg=3"}</span>
                                </Box>
                            </Grid>
                            <Grid item={true} xs={12} sm={6} md={4} lg={3}>
                                <Box bg="secondary-container" px="16px" py="12px" border_radius="8px">
                                    <span>{"xs=12, sm=6, md=4, lg=3"}</span>
                                </Box>
                            </Grid>
                            <Grid item={true} xs={12} sm={6} md={4} lg={3}>
                                <Box bg="primary-container" px="16px" py="12px" border_radius="8px">
                                    <span>{"xs=12, sm=6, md=4, lg=3"}</span>
                                </Box>
                            </Grid>
                            <Grid item={true} xs={12} sm={6} md={4} lg={3}>
                                <Box bg="secondary-container" px="16px" py="12px" border_radius="8px">
                                    <span>{"xs=12, sm=6, md=4, lg=3"}</span>
                                </Box>
                            </Grid>
                        </Grid>
                    </Container>
                </Demo>
                <CodeBlock code={"<Grid container={true} spacing=\"16px\">\n    <Grid item={true} xs={12} sm={6} md={4} lg={3}>\n        <Box bg=\"primary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n            <span>{\"xs=12, sm=6, md=4, lg=3\"}</span>\n        </Box>\n    </Grid>\n    <Grid item={true} xs={12} sm={6} md={4} lg={3}>\n        <Box bg=\"secondary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n            <span>{\"xs=12, sm=6, md=4, lg=3\"}</span>\n        </Box>\n    </Grid>\n    <Grid item={true} xs={12} sm={6} md={4} lg={3}>\n        <Box bg=\"primary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n            <span>{\"xs=12, sm=6, md=4, lg=3\"}</span>\n        </Box>\n    </Grid>\n    <Grid item={true} xs={12} sm={6} md={4} lg={3}>\n        <Box bg=\"secondary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n            <span>{\"xs=12, sm=6, md=4, lg=3\"}</span>\n        </Box>\n    </Grid>\n</Grid>".to_string()} language={"rust".to_string()} />

                // ── Nested Grid ──
                <Demo title="Nested Grid">
                    <Container max_width="lg" padding="none">
                        <Grid container={true} spacing="16px">
                            <Grid item={true} xs={12} md={8}>
                                <Box bg="primary-container" px="16px" py="12px" border_radius="8px">
                                    <span style="font-weight: 500;">{"Main Content (xs=12, md=8)"}</span>
                                </Box>
                                <Grid container={true} spacing="8px" style="margin-top: 8px;">
                                    <Grid item={true} xs={6}>
                                        <Box bg="surface-container" px="12px" py="8px" border_radius="6px">
                                            <span>{"Nested 1"}</span>
                                        </Box>
                                    </Grid>
                                    <Grid item={true} xs={6}>
                                        <Box bg="surface-container" px="12px" py="8px" border_radius="6px">
                                            <span>{"Nested 2"}</span>
                                        </Box>
                                    </Grid>
                                </Grid>
                            </Grid>
                            <Grid item={true} xs={12} md={4}>
                                <Box bg="secondary-container" px="16px" py="12px" border_radius="8px">
                                    <span style="font-weight: 500;">{"Sidebar (xs=12, md=4)"}</span>
                                </Box>
                            </Grid>
                        </Grid>
                    </Container>
                </Demo>
                <CodeBlock code={"<Grid container={true} spacing=\"16px\">\n    <Grid item={true} xs={12} md={8}>\n        <Box bg=\"primary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n            <span>{\"Main Content (xs=12, md=8)\"}</span>\n        </Box>\n        <Grid container={true} spacing=\"8px\" style=\"margin-top: 8px;\">\n            <Grid item={true} xs={6}>\n                <Box bg=\"surface-container\" px=\"12px\" py=\"8px\" border_radius=\"6px\">\n                    <span>{\"Nested 1\"}</span>\n                </Box>\n            </Grid>\n            <Grid item={true} xs={6}>\n                <Box bg=\"surface-container\" px=\"12px\" py=\"8px\" border_radius=\"6px\">\n                    <span>{\"Nested 2\"}</span>\n                </Box>\n            </Grid>\n        </Grid>\n    </Grid>\n    <Grid item={true} xs={12} md={4}>\n        <Box bg=\"secondary-container\" px=\"16px\" py=\"12px\" border_radius=\"8px\">\n            <span>{\"Sidebar (xs=12, md=4)\"}</span>\n        </Box>\n    </Grid>\n</Grid>".to_string()} language={"rust".to_string()} />

                // ── Custom Spacing ──
                <Demo title="Custom Spacing">
                    <div style="display: flex; flex-direction: column; gap: 16px; width: 100%;">
                        <div>
                            <span style="font-size: 12px; color: #49454F; margin-bottom: 4px; display: block;">{"spacing=\"8px\""}</span>
                            <Container max_width="lg" padding="none">
                                <Grid container={true} spacing="8px">
                                    <Grid item={true} xs={4}>
                                        <Box bg="primary-container" px="12px" py="8px" border_radius="6px">
                                            <span>{"A"}</span>
                                        </Box>
                                    </Grid>
                                    <Grid item={true} xs={4}>
                                        <Box bg="primary-container" px="12px" py="8px" border_radius="6px">
                                            <span>{"B"}</span>
                                        </Box>
                                    </Grid>
                                    <Grid item={true} xs={4}>
                                        <Box bg="primary-container" px="12px" py="8px" border_radius="6px">
                                            <span>{"C"}</span>
                                        </Box>
                                    </Grid>
                                </Grid>
                            </Container>
                        </div>
                        <div>
                            <span style="font-size: 12px; color: #49454F; margin-bottom: 4px; display: block;">{"spacing=\"32px\""}</span>
                            <Container max_width="lg" padding="none">
                                <Grid container={true} spacing="32px">
                                    <Grid item={true} xs={4}>
                                        <Box bg="secondary-container" px="12px" py="8px" border_radius="6px">
                                            <span>{"A"}</span>
                                        </Box>
                                    </Grid>
                                    <Grid item={true} xs={4}>
                                        <Box bg="secondary-container" px="12px" py="8px" border_radius="6px">
                                            <span>{"B"}</span>
                                        </Box>
                                    </Grid>
                                    <Grid item={true} xs={4}>
                                        <Box bg="secondary-container" px="12px" py="8px" border_radius="6px">
                                            <span>{"C"}</span>
                                        </Box>
                                    </Grid>
                                </Grid>
                            </Container>
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={"// Tight spacing\n<Grid container={true} spacing=\"8px\">\n    <Grid item={true} xs={4}><Box ...>{\"A\"}</Box></Grid>\n    <Grid item={true} xs={4}><Box ...>{\"B\"}</Box></Grid>\n    <Grid item={true} xs={4}><Box ...>{\"C\"}</Box></Grid>\n</Grid>\n\n// Wide spacing\n<Grid container={true} spacing=\"32px\">\n    <Grid item={true} xs={4}><Box ...>{\"A\"}</Box></Grid>\n    <Grid item={true} xs={4}><Box ...>{\"B\"}</Box></Grid>\n    <Grid item={true} xs={4}><Box ...>{\"C\"}</Box></Grid>\n</Grid>".to_string()} language={"rust".to_string()} />

                <PropTable rows={grid_props} />
            </Section>
        </>
    }
}
