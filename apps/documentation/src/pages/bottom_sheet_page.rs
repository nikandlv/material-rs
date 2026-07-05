use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn BottomSheetPage() -> Html {
    let modal_open = use_state(|| false);
    let standard_open = use_state(|| false);
    let expanding_open = use_state(|| false);

    let bottom_sheet_props = vec![
        PropRow { name: "open".into(), r#type: "bool".into(), default_value: "false".into(), description: "Controls whether the bottom sheet is visible.".into() },
        PropRow { name: "variant".into(), r#type: "BottomSheetVariant".into(), default_value: "Modal".into(), description: "Determines the sheet style: Standard, Modal, or Expanding.".into() },
        PropRow { name: "title".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Title text displayed at the top of the sheet (modal/expanding only).".into() },
        PropRow { name: "on_close".into(), r#type: "Callback<()>".into(), default_value: "default".into(), description: "Fires when the user dismisses the sheet (scrim click or drag down).".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Content rendered inside the sheet body.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    html! {
        <>
            <Section title="Bottom Sheet">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Bottom sheets surface content from the bottom edge of the screen. \
                      Material Design 3 provides three variants: Standard for inline content, \
                      Modal with a scrim backdrop, and Expanding for content that can fill more \
                      of the screen."}
                </p>

                // ── Modal Bottom Sheet ──
                <Demo title="Modal Bottom Sheet">
                    <Button label="Open Modal Sheet" variant={ButtonVariant::FilledTonal}
                            onclick={let s = modal_open.clone(); Callback::from(move |_: MouseEvent| s.set(true))} />
                    <BottomSheet open={*modal_open}
                                variant={BottomSheetVariant::Modal}
                                title="Modal Sheet"
                                on_close={let s = modal_open.clone(); Callback::from(move |_: ()| s.set(false))}>
                        <p style="margin: 0 0 12px;">
                            {"This is a modal bottom sheet. It displays a scrim backdrop and can \
                              be dismissed by tapping outside or dragging down."}
                        </p>
                        <Button label="Close" variant={ButtonVariant::Text}
                                onclick={let s = modal_open.clone(); Callback::from(move |_: MouseEvent| s.set(false))} />
                    </BottomSheet>
                </Demo>
                <CodeBlock code={"let open = use_state(|| false);\n\n<Button\n    label=\"Open Modal Sheet\"\n    variant={ButtonVariant::FilledTonal}\n    onclick={let s = open.clone();\n        Callback::from(move |_: MouseEvent| s.set(true))}\n/>\n<BottomSheet\n    open={*open}\n    variant={BottomSheetVariant::Modal}\n    title=\"Modal Sheet\"\n    on_close={let s = open.clone();\n        Callback::from(move |_: ()| s.set(false))}\n>\n    <p>{\"This is a modal bottom sheet.\"}</p>\n    <Button\n        label=\"Close\"\n        variant={ButtonVariant::Text}\n        onclick={let s = open.clone();\n            Callback::from(move |_: MouseEvent| s.set(false))}\n    />\n</BottomSheet>".to_string()} language={"rust".to_string()} />

                // ── Standard Bottom Sheet ──
                <Demo title="Standard Bottom Sheet">
                    <Button label="Open Standard Sheet" variant={ButtonVariant::FilledTonal}
                            onclick={let s = standard_open.clone(); Callback::from(move |_: MouseEvent| s.set(true))} />
                    <BottomSheet open={*standard_open}
                                variant={BottomSheetVariant::Standard}
                                on_close={let s = standard_open.clone(); Callback::from(move |_: ()| s.set(false))}>
                        <p style="margin: 0 0 12px;">
                            {"This is a standard bottom sheet. It has no scrim backdrop and \
                              no drag handle — ideal for inline content."}
                        </p>
                        <Button label="Close" variant={ButtonVariant::Text}
                                onclick={let s = standard_open.clone(); Callback::from(move |_: MouseEvent| s.set(false))} />
                    </BottomSheet>
                </Demo>
                <CodeBlock code={"let open = use_state(|| false);\n\n<Button\n    label=\"Open Standard Sheet\"\n    variant={ButtonVariant::FilledTonal}\n    onclick={let s = open.clone();\n        Callback::from(move |_: MouseEvent| s.set(true))}\n/>\n<BottomSheet\n    open={*open}\n    variant={BottomSheetVariant::Standard}\n    on_close={let s = open.clone();\n        Callback::from(move |_: ()| s.set(false))}\n>\n    <p>{\"This is a standard bottom sheet.\"}</p>\n    <Button\n        label=\"Close\"\n        variant={ButtonVariant::Text}\n        onclick={let s = open.clone();\n            Callback::from(move |_: MouseEvent| s.set(false))}\n    />\n</BottomSheet>".to_string()} language={"rust".to_string()} />

                // ── Expanding Bottom Sheet ──
                <Demo title="Expanding Bottom Sheet">
                    <Button label="Open Expanding Sheet" variant={ButtonVariant::FilledTonal}
                            onclick={let s = expanding_open.clone(); Callback::from(move |_: MouseEvent| s.set(true))} />
                    <BottomSheet open={*expanding_open}
                                variant={BottomSheetVariant::Expanding}
                                title="Expanding Sheet"
                                on_close={let s = expanding_open.clone(); Callback::from(move |_: ()| s.set(false))}>
                        <p style="margin: 0 0 12px;">
                            {"This is an expanding bottom sheet. It shows a drag handle and \
                              can be pulled up to reveal more content."}
                        </p>
                        <p style="margin: 0 0 12px;">
                            {"Drag it down to dismiss, or tap the scrim to close."}
                        </p>
                        <Button label="Close" variant={ButtonVariant::Text}
                                onclick={let s = expanding_open.clone(); Callback::from(move |_: MouseEvent| s.set(false))} />
                    </BottomSheet>
                </Demo>
                <CodeBlock code={"let open = use_state(|| false);\n\n<Button\n    label=\"Open Expanding Sheet\"\n    variant={ButtonVariant::FilledTonal}\n    onclick={let s = open.clone();\n        Callback::from(move |_: MouseEvent| s.set(true))}\n/>\n<BottomSheet\n    open={*open}\n    variant={BottomSheetVariant::Expanding}\n    title=\"Expanding Sheet\"\n    on_close={let s = open.clone();\n        Callback::from(move |_: ()| s.set(false))}\n>\n    <p>{\"This is an expanding bottom sheet.\"}</p>\n    <p>{\"Drag it down to dismiss.\"}</p>\n    <Button\n        label=\"Close\"\n        variant={ButtonVariant::Text}\n        onclick={let s = open.clone();\n            Callback::from(move |_: MouseEvent| s.set(false))}\n    />\n</BottomSheet>".to_string()} language={"rust".to_string()} />

                <PropTable rows={bottom_sheet_props} />
            </Section>
        </>
    }
}
