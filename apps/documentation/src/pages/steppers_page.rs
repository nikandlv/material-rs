use yew::prelude::*;
use material_rs::components::{Stepper, Step};
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn SteppersPage() -> Html {
    let step_props = vec![
        PropRow { name: "label".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Text label displayed next to the step circle.".into() },
        PropRow { name: "optional_text".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Secondary text shown below the label (e.g. a hint or description).".into() },
    ];

    let stepper_props = vec![
        PropRow { name: "steps".into(), r#type: "Vec<Step>".into(), default_value: "(required)".into(), description: "Vector of Step items to display.".into() },
        PropRow { name: "active_step".into(), r#type: "usize".into(), default_value: "0".into(), description: "Index of the currently active step (0-indexed). Steps before this are shown as completed.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    html! {
        <>
            <Section title="Stepper">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Steppers show progress through a multi-step process. Each step displays \
                      a numbered circle, a label, and optional secondary text. Completed steps \
                      show a checkmark, the active step is highlighted and scaled up, and \
                      future steps appear as outlined circles. Connectors between steps fill \
                      in as they are completed."}
                </p>

                // ── Step at Position 0 ──
                <Demo title="Step 1 Active">
                    <Stepper steps={vec![
                        Step { label: "Configure".into(), optional_text: String::new() },
                        Step { label: "Design".into(), optional_text: String::new() },
                        Step { label: "Review".into(), optional_text: String::new() },
                    ]} active_step={0} />
                </Demo>
                <CodeBlock code={"<Stepper\n    steps={vec![\n        Step { label: \"Configure\".into(), optional_text: String::new() },\n        Step { label: \"Design\".into(), optional_text: String::new() },\n        Step { label: \"Review\".into(), optional_text: String::new() },\n    ]}\n    active_step={0}\n/>".to_string()} language={"rust".to_string()} />

                // ── Step at Position 1 ──
                <Demo title="Step 2 Active">
                    <Stepper steps={vec![
                        Step { label: "Configure".into(), optional_text: String::new() },
                        Step { label: "Design".into(), optional_text: String::new() },
                        Step { label: "Review".into(), optional_text: String::new() },
                    ]} active_step={1} />
                </Demo>
                <CodeBlock code={"<Stepper\n    steps={vec![\n        Step { label: \"Configure\".into(), optional_text: String::new() },\n        Step { label: \"Design\".into(), optional_text: String::new() },\n        Step { label: \"Review\".into(), optional_text: String::new() },\n    ]}\n    active_step={1}\n/>".to_string()} language={"rust".to_string()} />

                // ── Step at Position 2 (Last) ──
                <Demo title="Step 3 Active">
                    <Stepper steps={vec![
                        Step { label: "Configure".into(), optional_text: String::new() },
                        Step { label: "Design".into(), optional_text: String::new() },
                        Step { label: "Review".into(), optional_text: String::new() },
                    ]} active_step={2} />
                </Demo>
                <CodeBlock code={"<Stepper\n    steps={vec![\n        Step { label: \"Configure\".into(), optional_text: String::new() },\n        Step { label: \"Design\".into(), optional_text: String::new() },\n        Step { label: \"Review\".into(), optional_text: String::new() },\n    ]}\n    active_step={2}\n/>".to_string()} language={"rust".to_string()} />

                // ── With Optional Text ──
                <Demo title="With Optional Text">
                    <Stepper steps={vec![
                        Step { label: "Account".into(), optional_text: "Enter your details".into() },
                        Step { label: "Payment".into(), optional_text: "Choose a method".into() },
                        Step { label: "Confirm".into(), optional_text: String::new() },
                    ]} active_step={1} />
                </Demo>
                <CodeBlock code={"<Stepper\n    steps={vec![\n        Step { label: \"Account\".into(), optional_text: \"Enter your details\".into() },\n        Step { label: \"Payment\".into(), optional_text: \"Choose a method\".into() },\n        Step { label: \"Confirm\".into(), optional_text: String::new() },\n    ]}\n    active_step={1}\n/>".to_string()} language={"rust".to_string()} />

                // ── All Steps Completed ──
                <Demo title="All Steps Completed">
                    <Stepper steps={vec![
                        Step { label: "Upload".into(), optional_text: String::new() },
                        Step { label: "Process".into(), optional_text: String::new() },
                        Step { label: "Deploy".into(), optional_text: String::new() },
                    ]} active_step={3} />
                </Demo>
                <CodeBlock code={"// Set active_step beyond the last index to mark all as completed\n<Stepper\n    steps={vec![\n        Step { label: \"Upload\".into(), optional_text: String::new() },\n        Step { label: \"Process\".into(), optional_text: String::new() },\n        Step { label: \"Deploy\".into(), optional_text: String::new() },\n    ]}\n    active_step={3}\n/>".to_string()} language={"rust".to_string()} />

                // ── Props Tables ──
                <PropTable rows={stepper_props} />
                <PropTable rows={step_props} />
            </Section>
        </>
    }
}
