use yew::prelude::*;
use material_rs::components::*;
use material_rs::theme::Theme;
use super::{Section, Demo, CodeBlock, PropTable, PropRow};

#[function_component]
pub fn IconPage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let icon_props = vec![
        PropRow { name: "name".into(), r#type: "String".into(), default_value: "required".into(), description: "Material icon name (ligature). E.g., \"home\", \"search\", \"close\".".into() },
        PropRow { name: "size".into(), r#type: "String".into(), default_value: "\"24px\"".into(), description: "Icon size as a CSS value.".into() },
        PropRow { name: "color".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Icon color. Falls back to the current text color when empty.".into() },
        PropRow { name: "weight".into(), r#type: "IconWeight".into(), default_value: "Regular".into(), description: "Font weight variant: Regular, Filled, Light, Bold.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    let weight_props = vec![
        PropRow { name: "Regular".into(), r#type: "IconWeight".into(), default_value: "Default".into(), description: "Standard weight (400), no fill.".into() },
        PropRow { name: "Filled".into(), r#type: "IconWeight".into(), default_value: "".into(), description: "Filled variant (fill=1, weight=400).".into() },
        PropRow { name: "Light".into(), r#type: "IconWeight".into(), default_value: "".into(), description: "Light variant (weight=300).".into() },
        PropRow { name: "Bold".into(), r#type: "IconWeight".into(), default_value: "".into(), description: "Bold variant (weight=700).".into() },
    ];

    html! {
        <>
            <Section title="Icon">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Icons are visual indicators that represent actions, objects, or concepts. \
                      Material Symbols provide a consistent, customizable set of icons with \
                      adjustable weight, size, and color."}
                </p>

                <Demo title="Basic Icons">
                    <div style="display: flex; align-items: center; gap: 16px; flex-wrap: wrap;">
                        <Icon name="home" />
                        <Icon name="search" />
                        <Icon name="settings" />
                        <Icon name="favorite" />
                        <Icon name="star" />
                        <Icon name="notifications" />
                        <Icon name="delete" />
                        <Icon name="edit" />
                        <Icon name="share" />
                        <Icon name="bookmark" />
                    </div>
                </Demo>
                <CodeBlock code={"<Icon name=\"home\" />\n<Icon name=\"search\" />\n<Icon name=\"settings\" />\n<Icon name=\"favorite\" />\n<Icon name=\"star\" />\n<Icon name=\"notifications\" />".to_string()} language={"rust".to_string()} />

                <Demo title="Custom Size">
                    <div style="display: flex; align-items: flex-end; gap: 16px;">
                        <Icon name="home" size="16px" />
                        <Icon name="home" size="24px" />
                        <Icon name="home" size="32px" />
                        <Icon name="home" size="48px" />
                        <Icon name="home" size="64px" />
                    </div>
                </Demo>
                <CodeBlock code={"<Icon name=\"home\" size=\"16px\" />\n<Icon name=\"home\" size=\"24px\" />\n<Icon name=\"home\" size=\"32px\" />\n<Icon name=\"home\" size=\"48px\" />\n<Icon name=\"home\" size=\"64px\" />".to_string()} language={"rust".to_string()} />

                <Demo title="Custom Color">
                    <div style="display: flex; align-items: center; gap: 16px;">
                        <Icon name="favorite" color={theme.colors.error.clone()} size="32px" />
                        <Icon name="star" color={theme.colors.primary.clone()} size="32px" />
                        <Icon name="thumb_up" color={"#4CAF50".to_string()} size="32px" />
                        <Icon name="warning" color={"#FF9800".to_string()} size="32px" />
                        <Icon name="info" color={"#2196F3".to_string()} size="32px" />
                    </div>
                </Demo>
                <CodeBlock code={"<Icon name=\"favorite\" color={theme.colors.error.clone()} size=\"32px\" />\n<Icon name=\"star\" color={theme.colors.primary.clone()} size=\"32px\" />\n<Icon name=\"thumb_up\" color=\"#4CAF50\" size=\"32px\" />\n<Icon name=\"warning\" color=\"#FF9800\" size=\"32px\" />\n<Icon name=\"info\" color=\"#2196F3\" size=\"32px\" />".to_string()} language={"rust".to_string()} />

                <Demo title="Weight Variants">
                    <div style="display: flex; align-items: center; gap: 24px;">
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
                            <Icon name="home" size="32px" weight={IconWeight::Light} />
                            <span style="font-size: 12px; color: var(--md-sys-color-on-surface-variant);">{"Light"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
                            <Icon name="home" size="32px" weight={IconWeight::Regular} />
                            <span style="font-size: 12px; color: var(--md-sys-color-on-surface-variant);">{"Regular"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
                            <Icon name="home" size="32px" weight={IconWeight::Bold} />
                            <span style="font-size: 12px; color: var(--md-sys-color-on-surface-variant);">{"Bold"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
                            <Icon name="home" size="32px" weight={IconWeight::Filled} />
                            <span style="font-size: 12px; color: var(--md-sys-color-on-surface-variant);">{"Filled"}</span>
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={"<Icon name=\"home\" size=\"32px\" weight={IconWeight::Light} />\n<Icon name=\"home\" size=\"32px\" weight={IconWeight::Regular} />\n<Icon name=\"home\" size=\"32px\" weight={IconWeight::Bold} />\n<Icon name=\"home\" size=\"32px\" weight={IconWeight::Filled} />".to_string()} language={"rust".to_string()} />

                <PropTable rows={icon_props} />
            </Section>

            <Section title="IconWeight">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"IconWeight controls the font variation settings of the icon, \
                      affecting its fill and stroke weight."}
                </p>
                <PropTable rows={weight_props} />
            </Section>
        </>
    }
}
