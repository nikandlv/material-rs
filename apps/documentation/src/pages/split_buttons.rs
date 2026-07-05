use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn SplitButtonsPage() -> Html {
    let split_button_props = vec![
        PropRow { name: "label".into(), r#type: "String".into(), default_value: "required".into(), description: "Text displayed on the primary action area.".into() },
        PropRow { name: "icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Leading Material icon name shown before the label.".into() },
        PropRow { name: "variant".into(), r#type: "ButtonVariant".into(), default_value: "Filled".into(), description: "Visual variant: Filled or Outlined.".into() },
        PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Disables both the primary action and dropdown.".into() },
        PropRow { name: "onclick".into(), r#type: "Callback<MouseEvent>".into(), default_value: "default".into(), description: "Click handler for the primary action area.".into() },
        PropRow { name: "on_dropdown_click".into(), r#type: "Callback<MouseEvent>".into(), default_value: "default".into(), description: "Click handler for the dropdown trigger area.".into() },
    ];

    html! {
        <>
            <Section title="Split Button">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"A split button combines a primary action with a dropdown trigger in a single component. The main area executes the primary action while the trailing chevron opens a secondary menu."}
                </p>

                // ── Filled Split Button ──
                <Demo title="Filled">
                    <div style="display: flex; flex-wrap: wrap; gap: 12px; align-items: center;">
                        <SplitButton label="Save Action" icon="save" variant={ButtonVariant::Filled} onclick={Callback::from(|_| {})} on_dropdown_click={Callback::from(|_| {})} />
                    </div>
                </Demo>
                <CodeBlock code={"<SplitButton\n    label=\"Save Action\"\n    icon=\"save\"\n    variant={ButtonVariant::Filled}\n    onclick={Callback::from(|_| {})}\n    on_dropdown_click={Callback::from(|_| {})}\n/>".to_string()} language={"rust".to_string()} />

                // ── Outlined Split Button ──
                <Demo title="Outlined">
                    <div style="display: flex; flex-wrap: wrap; gap: 12px; align-items: center;">
                        <SplitButton label="Send Mail" icon="send" variant={ButtonVariant::Outlined} onclick={Callback::from(|_| {})} on_dropdown_click={Callback::from(|_| {})} />
                    </div>
                </Demo>
                <CodeBlock code={"<SplitButton\n    label=\"Send Mail\"\n    icon=\"send\"\n    variant={ButtonVariant::Outlined}\n    onclick={Callback::from(|_| {})}\n    on_dropdown_click={Callback::from(|_| {})}\n/>".to_string()} language={"rust".to_string()} />

                // ── Disabled Split Button ──
                <Demo title="Disabled">
                    <div style="display: flex; flex-wrap: wrap; gap: 12px; align-items: center;">
                        <SplitButton label="Disabled Action" icon="lock" disabled={true} onclick={Callback::from(|_| {})} on_dropdown_click={Callback::from(|_| {})} />
                    </div>
                </Demo>
                <CodeBlock code={"<SplitButton\n    label=\"Disabled Action\"\n    icon=\"lock\"\n    disabled={true}\n    onclick={Callback::from(|_| {})}\n    on_dropdown_click={Callback::from(|_| {})}\n/>".to_string()} language={"rust".to_string()} />

                // ── All Variants Side by Side ──
                <Demo title="All Variants">
                    <div style="display: flex; flex-wrap: wrap; gap: 12px; align-items: center;">
                        <SplitButton label="Filled" icon="edit" variant={ButtonVariant::Filled} onclick={Callback::from(|_| {})} on_dropdown_click={Callback::from(|_| {})} />
                        <SplitButton label="Outlined" icon="edit" variant={ButtonVariant::Outlined} onclick={Callback::from(|_| {})} on_dropdown_click={Callback::from(|_| {})} />
                        <SplitButton label="Disabled" icon="edit" disabled={true} onclick={Callback::from(|_| {})} on_dropdown_click={Callback::from(|_| {})} />
                    </div>
                </Demo>
                <CodeBlock code={"<SplitButton label=\"Filled\" icon=\"edit\" variant={ButtonVariant::Filled}\n    onclick={Callback::from(|_| {})} on_dropdown_click={Callback::from(|_| {})} />\n<SplitButton label=\"Outlined\" icon=\"edit\" variant={ButtonVariant::Outlined}\n    onclick={Callback::from(|_| {})} on_dropdown_click={Callback::from(|_| {})} />\n<SplitButton label=\"Disabled\" icon=\"edit\" disabled={true}\n    onclick={Callback::from(|_| {})} on_dropdown_click={Callback::from(|_| {})} />".to_string()} language={"rust".to_string()} />

                <PropTable rows={split_button_props} />
            </Section>
        </>
    }
}
