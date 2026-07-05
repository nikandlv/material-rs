use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn AlertBoxPage() -> Html {
    let alert_props = vec![
        PropRow { name: "message".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "The alert message content displayed in the body.".into() },
        PropRow { name: "severity".into(), r#type: "AlertSeverity".into(), default_value: "Info".into(), description: "Visual severity: Info, Success, Warning, or Error. Controls color and icon.".into() },
        PropRow { name: "title".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Optional title text displayed as a heading above the message.".into() },
        PropRow { name: "open".into(), r#type: "bool".into(), default_value: "true".into(), description: "Controls visibility. Set to false to hide the alert.".into() },
        PropRow { name: "dismissible".into(), r#type: "bool".into(), default_value: "true".into(), description: "When true, shows a close button to dismiss the alert.".into() },
        PropRow { name: "on_dismiss".into(), r#type: "Callback<()>".into(), default_value: "default".into(), description: "Fired when the user clicks the dismiss (close) button.".into() },
        PropRow { name: "action_label".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Text for an optional action button displayed alongside the alert.".into() },
        PropRow { name: "on_action".into(), r#type: "Callback<()>".into(), default_value: "default".into(), description: "Fired when the action button is clicked.".into() },
        PropRow { name: "full_width".into(), r#type: "bool".into(), default_value: "true".into(), description: "Expands the alert to fill the container width.".into() },
    ];

    html! {
        <>
            <Section title="Alert Box">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Alert boxes display brief, important messages to users. They indicate \
                      a state that requires the user's attention and can include optional \
                      action buttons for contextual responses."}
                </p>

                // ── Severity Levels ──
                <Demo title="Severity Levels">
                    <div style="display: flex; flex-direction: column; gap: 12px; width: 100%; max-width: 560px;">
                        <AlertBox message="A new software update is available. See what's new in version 3.0." title="Info" severity={AlertSeverity::Info} />
                        <AlertBox message="Your changes have been saved successfully." title="Success" severity={AlertSeverity::Success} />
                        <AlertBox message="Your storage is almost full. Free up space to continue." title="Warning" severity={AlertSeverity::Warning} />
                        <AlertBox message="Unable to connect to the server. Please check your network connection." title="Error" severity={AlertSeverity::Error} />
                    </div>
                </Demo>
                <CodeBlock code={"<AlertBox message=\"A new software update is available.\" title=\"Info\" severity={AlertSeverity::Info} />\n<AlertBox message=\"Your changes have been saved successfully.\" title=\"Success\" severity={AlertSeverity::Success} />\n<AlertBox message=\"Your storage is almost full.\" title=\"Warning\" severity={AlertSeverity::Warning} />\n<AlertBox message=\"Unable to connect to the server.\" title=\"Error\" severity={AlertSeverity::Error} />".to_string()} language={"rust".to_string()} />

                // ── Dismissible Alert ──
                <Demo title="Dismissible Alert">
                    <div style="width: 100%; max-width: 560px;">
                        <AlertBox message="This alert can be dismissed by clicking the close button." title="Dismissible" severity={AlertSeverity::Info} dismissible={true} />
                    </div>
                </Demo>
                <CodeBlock code={"<AlertBox\n    message=\"This alert can be dismissed by clicking the close button.\"\n    title=\"Dismissible\"\n    severity={AlertSeverity::Info}\n    dismissible={true}\n/>".to_string()} language={"rust".to_string()} />

                // ── Alert with Action Button ──
                <Demo title="Alert with Action Button">
                    <div style="width: 100%; max-width: 560px;">
                        <AlertBox message="A new software update is available. See what's new in version 3.0." title="Software Update" severity={AlertSeverity::Info} action_label="Update" />
                    </div>
                </Demo>
                <CodeBlock code={"<AlertBox\n    message=\"A new software update is available.\"\n    title=\"Software Update\"\n    severity={AlertSeverity::Info}\n    action_label=\"Update\"\n/>".to_string()} language={"rust".to_string()} />

                // ── Non-Dismissible Alert ──
                <Demo title="Non-Dismissible Alert">
                    <div style="width: 100%; max-width: 560px;">
                        <AlertBox message="This alert cannot be dismissed by the user." title="Persistent" severity={AlertSeverity::Warning} dismissible={false} />
                    </div>
                </Demo>
                <CodeBlock code={"<AlertBox\n    message=\"This alert cannot be dismissed by the user.\"\n    title=\"Persistent\"\n    severity={AlertSeverity::Warning}\n    dismissible={false}\n/>".to_string()} language={"rust".to_string()} />

                <PropTable rows={alert_props} />
            </Section>
        </>
    }
}
