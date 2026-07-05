use yew::prelude::*;
use material_rs::components::{Card, CardVariant, Button, ButtonVariant};
use material_rs::theme::Theme;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn CardsPage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let card_props = vec![
        PropRow { name: "variant".into(), r#type: "CardVariant".into(), default_value: "CardVariant::Elevated".into(), description: "Visual variant: Elevated (shadow), Filled (flat surface), or Outlined (border).".into() },
        PropRow { name: "interactive".into(), r#type: "bool".into(), default_value: "false".into(), description: "Enables hover/press animations and ripple effect. Adds role=\"button\" and tabindex.".into() },
        PropRow { name: "onclick".into(), r#type: "Callback<MouseEvent>".into(), default_value: "default".into(), description: "Click event handler. Only meaningful when interactive is true.".into() },
        PropRow { name: "error".into(), r#type: "bool".into(), default_value: "false".into(), description: "Outlines the card in error color instead of the default outline variant border.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Card content rendered inside the container.".into() },
    ];

    html! {
        <>
            <Section title="Card">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Cards are surfaces that display content and actions on a single topic. \
                      Material Design 3 defines three card variants: Elevated cards float \
                      with a shadow, Filled cards use a flat surface color, and Outlined \
                      cards use a thin border. Interactive cards respond to hover and click."}
                </p>

                // ── Card Variants ──
                <Demo title="Variants">
                    <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(240px, 1fr)); gap: 16px;">
                        <Card variant={CardVariant::Elevated}>
                            <div style={format!("font-size: 16px; font-weight: 500; color: {}; margin-bottom: 8px;", theme.colors.on_surface)}>{"Elevated"}</div>
                            <div style={format!("font-size: 14px; color: {};", theme.colors.on_surface_variant)}>
                                {"Surface with a subtle shadow. The default card variant."}
                            </div>
                        </Card>
                        <Card variant={CardVariant::Filled}>
                            <div style={format!("font-size: 16px; font-weight: 500; color: {}; margin-bottom: 8px;", theme.colors.on_surface)}>{"Filled"}</div>
                            <div style={format!("font-size: 14px; color: {};", theme.colors.on_surface_variant)}>
                                {"Flat surface using the surface-container-highest color."}
                            </div>
                        </Card>
                        <Card variant={CardVariant::Outlined}>
                            <div style={format!("font-size: 16px; font-weight: 500; color: {}; margin-bottom: 8px;", theme.colors.on_surface)}>{"Outlined"}</div>
                            <div style={format!("font-size: 14px; color: {};", theme.colors.on_surface_variant)}>
                                {"Surface with a thin outline border."}
                            </div>
                        </Card>
                    </div>
                </Demo>
                <CodeBlock code={"<Card variant={CardVariant::Elevated}>\n    <div>{\"Elevated\"}</div>\n</Card>\n<Card variant={CardVariant::Filled}>\n    <div>{\"Filled\"}</div>\n</Card>\n<Card variant={CardVariant::Outlined}>\n    <div>{\"Outlined\"}</div>\n</Card>".to_string()} language={"rust".to_string()} />

                // ── Interactive Cards ──
                <Demo title="Interactive Cards">
                    <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(240px, 1fr)); gap: 16px;">
                        <Card variant={CardVariant::Elevated} interactive={true}>
                            <div style={format!("font-size: 16px; font-weight: 500; color: {}; margin-bottom: 8px;", theme.colors.on_surface)}>{"Interactive Elevated"}</div>
                            <div style={format!("font-size: 14px; color: {};", theme.colors.on_surface_variant)}>
                                {"Hover to see the elevation increase. Click for ripple."}
                            </div>
                        </Card>
                        <Card variant={CardVariant::Filled} interactive={true}>
                            <div style={format!("font-size: 16px; font-weight: 500; color: {}; margin-bottom: 8px;", theme.colors.on_surface)}>{"Interactive Filled"}</div>
                            <div style={format!("font-size: 14px; color: {};", theme.colors.on_surface_variant)}>
                                {"Hover lifts the card. Click triggers ripple effect."}
                            </div>
                        </Card>
                        <Card variant={CardVariant::Outlined} interactive={true}>
                            <div style={format!("font-size: 16px; font-weight: 500; color: {}; margin-bottom: 8px;", theme.colors.on_surface)}>{"Interactive Outlined"}</div>
                            <div style={format!("font-size: 14px; color: {};", theme.colors.on_surface_variant)}>
                                {"Hover adds elevation. Click shows ripple within the border."}
                            </div>
                        </Card>
                    </div>
                </Demo>
                <CodeBlock code={"<Card variant={CardVariant::Elevated} interactive={true}>\n    <div>{\"Interactive Elevated\"}</div>\n</Card>\n<Card variant={CardVariant::Filled} interactive={true}>\n    <div>{\"Interactive Filled\"}</div>\n</Card>\n<Card variant={CardVariant::Outlined} interactive={true}>\n    <div>{\"Interactive Outlined\"}</div>\n</Card>".to_string()} language={"rust".to_string()} />

                // ── Card with Click Handler ──
                <Demo title="Card with Click Handler">
                    <div style="max-width: 400px;">
                        <Card variant={CardVariant::Elevated} interactive={true}
                              onclick={Callback::from(|_| {
                                  #[cfg(target_arch = "wasm32")]
                                  web_sys::window().unwrap().alert_with_message("Card clicked!").unwrap();
                              })}>
                            <div style={format!("font-size: 16px; font-weight: 500; color: {}; margin-bottom: 8px;", theme.colors.on_surface)}>{"Click Me"}</div>
                            <div style={format!("font-size: 14px; color: {};", theme.colors.on_surface_variant)}>
                                {"This card fires an onclick callback when clicked."}
                            </div>
                        </Card>
                    </div>
                </Demo>
                <CodeBlock code={"<Card\n    variant={CardVariant::Elevated}\n    interactive={true}\n    onclick={Callback::from(|_| { /* handle click */ })}\n>\n    <div>{\"Click Me\"}</div>\n</Card>".to_string()} language={"rust".to_string()} />

                // ── Card with Nested Content ──
                <Demo title="Card with Nested Content">
                    <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px;">
                        <Card variant={CardVariant::Elevated}>
                            <div style={format!("font-size: 16px; font-weight: 500; color: {}; margin-bottom: 8px;", theme.colors.on_surface)}>{"Event Invitation"}</div>
                            <div style={format!("font-size: 14px; color: {}; margin-bottom: 12px;", theme.colors.on_surface_variant)}>
                                {"You are invited to the Material Design 3 launch event."}
                            </div>
                            <div style="display: flex; gap: 8px; justify-content: flex-end;">
                                <Button label="Decline" variant={ButtonVariant::Text} />
                                <Button label="Accept" variant={ButtonVariant::Filled} />
                            </div>
                        </Card>
                        <Card variant={CardVariant::Outlined}>
                            <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px;">
                                <span class="material-symbols-outlined" style={format!("font-size: 40px; color: {};", theme.colors.primary)}>
                                    {"palette"}
                                </span>
                                <div>
                                    <div style={format!("font-size: 16px; font-weight: 500; color: {};", theme.colors.on_surface)}>{"Theme Settings"}</div>
                                    <div style={format!("font-size: 12px; color: {};", theme.colors.on_surface_variant)}>{"Customize your appearance"}</div>
                                </div>
                            </div>
                            <div style={format!("font-size: 14px; color: {};", theme.colors.on_surface_variant)}>
                                {"Configure color schemes, typography, and shape tokens."}
                            </div>
                        </Card>
                    </div>
                </Demo>
                <CodeBlock code={"<Card variant={CardVariant::Elevated}>\n    <div>{\"Event Invitation\"}</div>\n    <div>{\"You are invited...\"}</div>\n    <div style=\"display: flex; gap: 8px; justify-content: flex-end;\">\n        <Button label=\"Decline\" variant={ButtonVariant::Text} />\n        <Button label=\"Accept\" variant={ButtonVariant::Filled} />\n    </div>\n</Card>\n\n<Card variant={CardVariant::Outlined}>\n    <div style=\"display: flex; align-items: center; gap: 12px;\">\n        <span class=\"material-symbols-outlined\">{\"palette\"}</span>\n        <div>\n            <div>{\"Theme Settings\"}</div>\n            <div>{\"Customize your appearance\"}</div>\n        </div>\n    </div>\n</Card>".to_string()} language={"rust".to_string()} />

                // ── Card with Header Image ──
                <Demo title="Card with Header Image (Edge-to-Edge Media)">
                    <div style="max-width: 320px;">
                        <Card
                            variant={CardVariant::Outlined}
                            image="https://picsum.photos/seed/materialcard/400/225"
                            image_ratio="16/9"
                            image_alt="Material landscape"
                        >
                            <div style={format!("font-size: 18px; font-weight: 500; color: {}; margin-bottom: 8px;", theme.colors.on_surface)}>
                                {"Yosemite Valley"}
                            </div>
                            <div style={format!("font-size: 14px; color: {}; margin-bottom: 16px;", theme.colors.on_surface_variant)}>
                                {"Beautiful national park located in California's Sierra Nevada mountains."}
                            </div>
                            <div style="display: flex; gap: 8px;">
                                <Button label="Explore" variant={ButtonVariant::Filled} />
                                <Button label="Share" variant={ButtonVariant::Text} />
                            </div>
                        </Card>
                    </div>
                </Demo>
                <CodeBlock code={"<Card\n    variant={CardVariant::Outlined}\n    image=\"https://picsum.photos/seed/materialcard/400/225\"\n    image_ratio=\"16/9\"\n    image_alt=\"Material landscape\"\n>\n    <div style=\"font-size: 18px; font-weight: 500; margin-bottom: 8px;\">{\"Yosemite Valley\"}</div>\n    <div style=\"font-size: 14px; margin-bottom: 16px;\">{\"Beautiful national park...\"}</div>\n    <div style=\"display: flex; gap: 8px;\">\n        <Button label=\"Explore\" variant={ButtonVariant::Filled} />\n        <Button label=\"Share\" variant={ButtonVariant::Text} />\n    </div>\n</Card>".to_string()} language={"rust".to_string()} />

                // ── Error Card ──
                <Demo title="Error Card">
                    <div style="max-width: 400px;">
                        <Card variant={CardVariant::Outlined} error={true}>
                            <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 8px;">
                                <span class="material-symbols-outlined" style="font-size: 24px; color: var(--md-sys-color-error);">
                                    {"error"}
                                </span>
                                <div style={format!("font-size: 16px; font-weight: 500; color: {};", theme.colors.on_surface)}>{"Error"}</div>
                            </div>
                            <div style={format!("font-size: 14px; color: {};", theme.colors.on_surface_variant)}>
                                {"This card uses the error color for its outline border."}
                            </div>
                        </Card>
                    </div>
                </Demo>
                <CodeBlock code={"<Card variant={CardVariant::Outlined} error={true}>\n    <div style=\"display: flex; align-items: center; gap: 12px;\">\n        <span class=\"material-symbols-outlined\">{\"error\"}</span>\n        <div>{\"Error\"}</div>\n    </div>\n</Card>".to_string()} language={"rust".to_string()} />

                <PropTable rows={card_props} />
            </Section>
        </>
    }
}
