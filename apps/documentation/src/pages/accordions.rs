use yew::prelude::*;
use material_rs::components::Accordion;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn AccordionsPage() -> Html {
    let expanded_a = use_state(|| false);
    let expanded_b = use_state(|| false);

    let accordion_props = vec![
        PropRow { name: "title".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Header text displayed in the accordion bar.".into() },
        PropRow { name: "icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Material icon name shown before the title.".into() },
        PropRow { name: "expanded".into(), r#type: "bool".into(), default_value: "false".into(), description: "Whether the accordion content is visible. Can be controlled externally.".into() },
        PropRow { name: "ontoggle".into(), r#type: "Callback<bool>".into(), default_value: "Callback::noop()".into(), description: "Fires with the new expanded state (true = expanded, false = collapsed) when toggled.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Content rendered inside the collapsible area.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    html! {
        <>
            <Section title="Accordions">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Accordions are expandable containers that reveal additional information when tapped. \
                      They are useful for grouping related content and reducing visual clutter. The \
                      content stays in the DOM at all times, animated with max-height and opacity transitions."}
                </p>

                // ── Single Accordion ──
                <Demo title="Single Accordion">
                    <div style="width: 100%; max-width: 560px;">
                        <Accordion title="What is Material Design?" icon="help">
                            {"Material Design is a design system built and supported by Google designers \
                              and developers. It provides guidelines, components, and tools to help teams \
                              build high-quality digital experiences across platforms."}
                        </Accordion>
                    </div>
                </Demo>
                <CodeBlock code={"<Accordion title=\"What is Material Design?\" icon=\"help\">\n    {\"Material Design is a design system built and supported by Google designers \\\n      and developers. It provides guidelines, components, and tools to help teams \\\n      build high-quality digital experiences across platforms.\"}\n</Accordion>".to_string()} language={"rust".to_string()} />

                // ── Multiple Accordions ──
                <Demo title="Multiple Accordions">
                    <div style="display: flex; flex-direction: column; gap: 8px; width: 100%; max-width: 560px;">
                        <Accordion title="Getting Started" icon="rocket_launch" expanded={false}>
                            {"To get started, install the Material RS crate and add it to your Cargo.toml. \
                              Then import the components you need and wrap your app in a Theme provider."}
                        </Accordion>
                        <Accordion title="Customization" icon="palette" expanded={false}>
                            {"All components accept a class prop for custom CSS classes. You can override \
                              theme tokens by providing a custom Theme context at the root of your app."}
                        </Accordion>
                        <Accordion title="Accessibility" icon="accessibility_new" expanded={false}>
                            {"Every component includes proper ARIA attributes and keyboard navigation \
                              support. Tab order and focus management follow WAI-ARIA authoring practices."}
                        </Accordion>
                    </div>
                </Demo>
                <CodeBlock code={"<Accordion title=\"Getting Started\" icon=\"rocket_launch\" expanded={false}>\n    {\"To get started, install the Material RS crate and add it to your Cargo.toml.\"}\n</Accordion>\n<Accordion title=\"Customization\" icon=\"palette\" expanded={false}>\n    {\"All components accept a class prop for custom CSS classes.\"}\n</Accordion>\n<Accordion title=\"Accessibility\" icon=\"accessibility_new\" expanded={false}>\n    {\"Every component includes proper ARIA attributes and keyboard navigation.\"}\n</Accordion>".to_string()} language={"rust".to_string()} />

                // ── Controlled Expand ──
                <Demo title="Controlled Expand (synchronized)">
                    <div style="width: 100%; max-width: 560px;">
                        <Accordion
                            title="First Section"
                            icon="looks_one"
                            expanded={*expanded_a}
                            ontoggle={let s = expanded_a.clone(); Callback::from(move |v: bool| s.set(v))}
                        >
                            {"This accordion is controlled by parent state. Toggling it will \
                              report its new state through the ontoggle callback."}
                        </Accordion>
                        <Accordion
                            title="Second Section"
                            icon="looks_two"
                            expanded={*expanded_b}
                            ontoggle={let s = expanded_b.clone(); Callback::from(move |v: bool| s.set(v))}
                        >
                            {"This accordion also reports its state independently. Both use \
                              separate state but could share one for mutually exclusive behavior."}
                        </Accordion>
                    </div>
                </Demo>
                <CodeBlock code={"let expanded_a = use_state(|| false);\nlet expanded_b = use_state(|| false);\n\n<Accordion\n    title=\"First Section\"\n    icon=\"looks_one\"\n    expanded={*expanded_a}\n    ontoggle={let s = expanded_a.clone(); Callback::from(move |v: bool| s.set(v))}\n>\n    {\"This accordion is controlled by parent state.\"}\n</Accordion>\n<Accordion\n    title=\"Second Section\"\n    icon=\"looks_two\"\n    expanded={*expanded_b}\n    ontoggle={let s = expanded_b.clone(); Callback::from(move |v: bool| s.set(v))}\n>\n    {\"This accordion also reports its state independently.\"}\n</Accordion>".to_string()} language={"rust".to_string()} />

                // ── With Custom Content ──
                <Demo title="With Custom Content">
                    <div style="width: 100%; max-width: 560px;">
                        <Accordion title="Configuration Options" icon="tune" expanded={false}>
                            <div style="display: flex; flex-direction: column; gap: 12px;">
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <span style="font-weight: 500;">{"Enable notifications"}</span>
                                    <span class="material-symbols-outlined" style="font-size: 20px; color: var(--md-sys-color-primary);">{"toggle_on"}</span>
                                </div>
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <span style="font-weight: 500;">{"Dark mode"}</span>
                                    <span class="material-symbols-outlined" style="font-size: 20px; color: var(--md-sys-color-on-surface-variant);">{"toggle_off"}</span>
                                </div>
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <span style="font-weight: 500;">{"Auto-save"}</span>
                                    <span class="material-symbols-outlined" style="font-size: 20px; color: var(--md-sys-color-primary);">{"toggle_on"}</span>
                                </div>
                            </div>
                        </Accordion>
                    </div>
                </Demo>
                <CodeBlock code={"<Accordion title=\"Configuration Options\" icon=\"tune\" expanded={false}>\n    <div style=\"display: flex; flex-direction: column; gap: 12px;\">\n        <div style=\"display: flex; justify-content: space-between; align-items: center;\">\n            <span style=\"font-weight: 500;\">{\"Enable notifications\"}</span>\n            <span class=\"material-symbols-outlined\">{\"toggle_on\"}</span>\n        </div>\n        <div style=\"display: flex; justify-content: space-between; align-items: center;\">\n            <span style=\"font-weight: 500;\">{\"Dark mode\"}</span>\n            <span class=\"material-symbols-outlined\">{\"toggle_off\"}</span>\n        </div>\n        <div style=\"display: flex; justify-content: space-between; align-items: center;\">\n            <span style=\"font-weight: 500;\">{\"Auto-save\"}</span>\n            <span class=\"material-symbols-outlined\">{\"toggle_on\"}</span>\n        </div>\n    </div>\n</Accordion>".to_string()} language={"rust".to_string()} />

                // ── Props Table ──
                <PropTable rows={accordion_props} />
            </Section>
        </>
    }
}
