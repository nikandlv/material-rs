use yew::prelude::*;
use material_rs::components::*;
use material_rs::theme::Theme;
use super::{Section, Demo, CodeBlock, PropTable, PropRow};

#[function_component]
pub fn SnackbarPage() -> Html {
    let _theme = use_context::<Theme>().expect("Theme context required");

    let basic_open = use_state(|| false);
    let action_open = use_state(|| false);
    let auto_open = use_state(|| false);

    let snackbar_props = vec![
        PropRow { name: "visible".into(), r#type: "bool".into(), default_value: "false".into(), description: "Controls whether the snackbar is shown.".into() },
        PropRow { name: "message".into(), r#type: "String".into(), default_value: "required".into(), description: "Message text displayed in the snackbar.".into() },
        PropRow { name: "action_label".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Optional action button label. Hidden when empty.".into() },
        PropRow { name: "on_action".into(), r#type: "Callback<()>".into(), default_value: "default".into(), description: "Fires when the action button is clicked.".into() },
        PropRow { name: "on_dismiss".into(), r#type: "Callback<()>".into(), default_value: "default".into(), description: "Fires when the snackbar is dismissed.".into() },
        PropRow { name: "duration".into(), r#type: "u32".into(), default_value: "5000".into(), description: "Auto-dismiss duration in ms. 0 disables.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS classes.".into() },
    ];

    html! {
        <>
            <Section title="Snackbar">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Snackbars provide brief messages about a process at the bottom of the screen. \
                      They can include an action button and auto-dismiss after a set duration."}
                </p>

                <Demo title="Basic">
                    <Button label="Show Snackbar" variant={ButtonVariant::FilledTonal}
                        onclick={let s = basic_open.clone(); Callback::from(move |_: MouseEvent| s.set(true))} />
                    <Snackbar visible={*basic_open} message="Message sent successfully"
                        on_dismiss={let s = basic_open.clone(); Callback::from(move |_: ()| s.set(false))} />
                </Demo>
                <CodeBlock code={"let visible = use_state(|| false);\n\n<Snackbar\n    visible={*visible}\n    message=\"Message sent successfully\"\n    on_dismiss={let s = visible.clone();\n        Callback::from(move |_: ()| s.set(false))}\n/>".to_string()} language={"rust".to_string()} />

                <Demo title="With Action">
                    <Button label="Show with Action" variant={ButtonVariant::FilledTonal}
                        onclick={let s = action_open.clone(); Callback::from(move |_: MouseEvent| s.set(true))} />
                    <Snackbar visible={*action_open} message="File deleted" action_label="Undo"
                        on_action={let s = action_open.clone(); Callback::from(move |_: ()| s.set(false))}
                        on_dismiss={let s = action_open.clone(); Callback::from(move |_: ()| s.set(false))} />
                </Demo>

                <Demo title="Auto-Dismiss">
                    <Button label="Auto-Dismiss (5s)" variant={ButtonVariant::FilledTonal}
                        onclick={let s = auto_open.clone(); Callback::from(move |_: MouseEvent| s.set(true))} />
                    <Snackbar visible={*auto_open} message="This auto-dismisses after 5 seconds"
                        duration={5000}
                        on_dismiss={let s = auto_open.clone(); Callback::from(move |_: ()| s.set(false))} />
                </Demo>

                <PropTable rows={snackbar_props} />
            </Section>
        </>
    }
}
