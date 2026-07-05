use yew::prelude::*;
use material_rs::components::{Button, ButtonVariant, Dialog};
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn DialogPage() -> Html {
    let basic_open = use_state(|| false);
    let icon_open = use_state(|| false);
    let custom_open = use_state(|| false);
    let destructive_open = use_state(|| false);

    let dialog_props = vec![
        PropRow { name: "open".into(), r#type: "bool".into(), default_value: "false".into(), description: "Controls whether the dialog is visible. Set to true to show.".into() },
        PropRow { name: "title".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Dialog title text displayed below the icon (if present).".into() },
        PropRow { name: "icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Optional Material icon name shown above the title.".into() },
        PropRow { name: "on_close".into(), r#type: "Callback<()>".into(), default_value: "default".into(), description: "Fires when the user dismisses the dialog (scrim click or close button).".into() },
        PropRow { name: "confirm_label".into(), r#type: "String".into(), default_value: "\"OK\"".into(), description: "Text for the confirm (primary) button.".into() },
        PropRow { name: "dismiss_label".into(), r#type: "String".into(), default_value: "\"Cancel\"".into(), description: "Text for the dismiss (text) button.".into() },
        PropRow { name: "on_confirm".into(), r#type: "Callback<()>".into(), default_value: "default".into(), description: "Fires when the confirm button is clicked. Also triggers on_close.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Dialog body content rendered between the title and action buttons.".into() },
    ];

    html! {
        <>
            <Section title="Dialog">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Dialogs inform users about a task and can contain critical information, \
                      require decisions, or involve multiple tasks. Material Design 3 dialogs \
                      include a surface with a title, content, and action buttons, overlaid \
                      on a blurred scrim backdrop."}
                </p>

                // ── Basic Dialog ──
                <Demo title="Basic Dialog">
                    <Button label="Open Basic Dialog" variant={ButtonVariant::FilledTonal}
                            onclick={let s = basic_open.clone(); Callback::from(move |_: MouseEvent| s.set(true))} />
                    <Dialog open={*basic_open}
                            title="Basic Dialog"
                            on_close={let s = basic_open.clone(); Callback::from(move |_: ()| s.set(false))}
                            on_confirm={let s = basic_open.clone(); Callback::from(move |_: ()| s.set(false))}>
                        {"This is a basic dialog with a title and action buttons. \
                          Click the scrim or Cancel to dismiss, or OK to confirm."}
                    </Dialog>
                </Demo>
                <CodeBlock code={"let open = use_state(|| false);\n\n<Button\n    label=\"Open Basic Dialog\"\n    variant={ButtonVariant::FilledTonal}\n    onclick={let s = open.clone();\n        Callback::from(move |_: MouseEvent| s.set(true))}\n/>\n<Dialog\n    open={*open}\n    title=\"Basic Dialog\"\n    on_close={let s = open.clone();\n        Callback::from(move |_: ()| s.set(false))}\n    on_confirm={let s = open.clone();\n        Callback::from(move |_: ()| s.set(false))}\n>\n    {\"This is a basic dialog with a title and action buttons.\"}\n</Dialog>".to_string()} language={"rust".to_string()} />

                // ── Dialog with Icon ──
                <Demo title="Dialog with Icon">
                    <Button label="Open Dialog with Icon" variant={ButtonVariant::FilledTonal}
                            onclick={let s = icon_open.clone(); Callback::from(move |_: MouseEvent| s.set(true))} />
                    <Dialog open={*icon_open}
                            title="Enable Notifications?"
                            icon="notifications"
                            confirm_label="Enable"
                            dismiss_label="Not Now"
                            on_close={let s = icon_open.clone(); Callback::from(move |_: ()| s.set(false))}
                            on_confirm={let s = icon_open.clone(); Callback::from(move |_: ()| s.set(false))}>
                        {"Would you like to receive push notifications for new messages? \
                          You can change this later in Settings."}
                    </Dialog>
                </Demo>
                <CodeBlock code={"let open = use_state(|| false);\n\n<Dialog\n    open={*open}\n    title=\"Enable Notifications?\"\n    icon=\"notifications\"\n    confirm_label=\"Enable\"\n    dismiss_label=\"Not Now\"\n    on_close={let s = open.clone();\n        Callback::from(move |_: ()| s.set(false))}\n    on_confirm={let s = open.clone();\n        Callback::from(move |_: ()| s.set(false))}\n>\n    {\"Would you like to receive push notifications?\"}\n</Dialog>".to_string()} language={"rust".to_string()} />

                // ── Dialog with Custom Buttons ──
                <Demo title="Dialog with Custom Buttons">
                    <Button label="Open Custom Dialog" variant={ButtonVariant::FilledTonal}
                            onclick={let s = custom_open.clone(); Callback::from(move |_: MouseEvent| s.set(true))} />
                    <Dialog open={*custom_open}
                            title="Save Changes?"
                            icon="save"
                            confirm_label="Save Draft"
                            dismiss_label="Discard"
                            on_close={let s = custom_open.clone(); Callback::from(move |_: ()| s.set(false))}
                            on_confirm={let s = custom_open.clone(); Callback::from(move |_: ()| s.set(false))}>
                        {"You have unsaved changes. Would you like to save a draft \
                          or discard your work?"}
                    </Dialog>
                </Demo>
                <CodeBlock code={"<Dialog\n    open={*open}\n    title=\"Save Changes?\"\n    icon=\"save\"\n    confirm_label=\"Save Draft\"\n    dismiss_label=\"Discard\"\n    on_close={let s = open.clone();\n        Callback::from(move |_: ()| s.set(false))}\n    on_confirm={let s = open.clone();\n        Callback::from(move |_: ()| s.set(false))}\n>\n    {\"You have unsaved changes.\"}\n</Dialog>".to_string()} language={"rust".to_string()} />

                // ── Destructive Action Dialog ──
                <Demo title="Destructive Action Dialog">
                    <Button label="Delete Account" variant={ButtonVariant::Text}
                            icon="delete"
                            onclick={let s = destructive_open.clone(); Callback::from(move |_: MouseEvent| s.set(true))} />
                    <Dialog open={*destructive_open}
                            title="Delete Account?"
                            icon="warning"
                            confirm_label="Delete"
                            dismiss_label="Keep Account"
                            on_close={let s = destructive_open.clone(); Callback::from(move |_: ()| s.set(false))}
                            on_confirm={let s = destructive_open.clone(); Callback::from(move |_: ()| s.set(false))}>
                        {"This action cannot be undone. All your data, projects, and \
                          settings will be permanently deleted."}
                    </Dialog>
                </Demo>
                <CodeBlock code={"<Dialog\n    open={*open}\n    title=\"Delete Account?\"\n    icon=\"warning\"\n    confirm_label=\"Delete\"\n    dismiss_label=\"Keep Account\"\n    on_close={let s = open.clone();\n        Callback::from(move |_: ()| s.set(false))}\n    on_confirm={let s = open.clone();\n        Callback::from(move |_: ()| s.set(false))}\n>\n    {\"This action cannot be undone.\"}\n</Dialog>".to_string()} language={"rust".to_string()} />

                <PropTable rows={dialog_props} />
            </Section>
        </>
    }
}
