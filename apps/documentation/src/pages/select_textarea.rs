use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn SelectTextareaPage() -> Html {
    let select_value = use_state(|| "opt1".to_string());
    let select_icon_value = use_state(|| "opt1".to_string());
    let select_disabled_value = use_state(|| "opt1".to_string());
    let textarea_filled_value = use_state(String::new);
    let textarea_outlined_value = use_state(String::new);

    let select_props = vec![
        PropRow { name: "label".into(), r#type: "String".into(), default_value: "required".into(), description: "Floating label text above the select field.".into() },
        PropRow { name: "value".into(), r#type: "String".into(), default_value: "required".into(), description: "Currently selected option value.".into() },
        PropRow { name: "options".into(), r#type: "Vec<SelectOption>".into(), default_value: "required".into(), description: "List of available options to choose from.".into() },
        PropRow { name: "onchange".into(), r#type: "Callback<String>".into(), default_value: "default".into(), description: "Fires when the selection changes with the new value.".into() },
        PropRow { name: "leading_icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Material icon displayed before the select text.".into() },
        PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Disables interaction and applies muted styling.".into() },
        PropRow { name: "variant".into(), r#type: "TextFieldVariant".into(), default_value: "Filled".into(), description: "Text field variant for styling.".into() },
    ];

    let select_option_props = vec![
        PropRow { name: "value".into(), r#type: "String".into(), default_value: "required".into(), description: "Value associated with this option.".into() },
        PropRow { name: "label".into(), r#type: "String".into(), default_value: "required".into(), description: "Display text for this option.".into() },
    ];

    let textarea_props = vec![
        PropRow { name: "label".into(), r#type: "String".into(), default_value: "required".into(), description: "Floating label text above the text area.".into() },
        PropRow { name: "variant".into(), r#type: "TextFieldVariant".into(), default_value: "Filled".into(), description: "Visual variant: Filled or Outlined.".into() },
        PropRow { name: "value".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Current text content of the text area.".into() },
        PropRow { name: "onchange".into(), r#type: "Callback<String>".into(), default_value: "default".into(), description: "Fires when the text content changes.".into() },
        PropRow { name: "placeholder".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Placeholder text shown when empty.".into() },
        PropRow { name: "helper_text".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Helper text displayed below the field.".into() },
        PropRow { name: "error_text".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Error message displayed below the field when in error state.".into() },
        PropRow { name: "error".into(), r#type: "bool".into(), default_value: "false".into(), description: "Puts the text area in error state with red styling.".into() },
        PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Disables interaction and applies muted styling.".into() },
        PropRow { name: "rows".into(), r#type: "u32".into(), default_value: "3".into(), description: "Number of visible text rows.".into() },
    ];

    html! {
        <>
            <Section title="Select">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Select components let users choose a single option from a dropdown list. They support leading icons, helper text, error states, and individual option disabling."}
                </p>

                // ── Basic Select ──
                <Demo title="Basic Select">
                    <div style="width: 100%; max-width: 360px;">
                        <Select label="Choose an Option" value={(*select_value).clone()} onchange={let sv = select_value.clone(); Callback::from(move |v: String| sv.set(v))}
                            options={vec![
                                SelectOption { value: "opt1".into(), label: "Option One".into() },
                                SelectOption { value: "opt2".into(), label: "Option Two".into() },
                                SelectOption { value: "opt3".into(), label: "Option Three".into() },
                            ]}
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<Select\n    label=\"Choose an Option\"\n    value={(*select_value).clone()}\n    onchange={let sv = select_value.clone(); Callback::from(move |v: String| sv.set(v))}\n    options={vec![\n        SelectOption { value: \"opt1\".into(), label: \"Option One\".into() },\n        SelectOption { value: \"opt2\".into(), label: \"Option Two\".into() },\n        SelectOption { value: \"opt3\".into(), label: \"Option Three\".into() },\n    ]}\n/>".to_string()} language={"rust".to_string()} />

                // ── Select with Leading Icon ──
                <Demo title="Leading Icon">
                    <div style="width: 100%; max-width: 360px;">
                        <Select label="Material Select Box" value={(*select_icon_value).clone()} onchange={let sv = select_icon_value.clone(); Callback::from(move |v: String| sv.set(v))} leading_icon="layers"
                            options={vec![
                                SelectOption { value: "opt1".into(), label: "Option One".into() },
                                SelectOption { value: "opt2".into(), label: "Option Two".into() },
                                SelectOption { value: "opt3".into(), label: "Option Three".into() },
                            ]}
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<Select\n    label=\"Material Select Box\"\n    value={(*select_icon_value).clone()}\n    onchange={let sv = select_icon_value.clone(); Callback::from(move |v: String| sv.set(v))}\n    leading_icon=\"layers\"\n    options={vec![\n        SelectOption { value: \"opt1\".into(), label: \"Option One\".into() },\n        SelectOption { value: \"opt2\".into(), label: \"Option Two\".into() },\n        SelectOption { value: \"opt3\".into(), label: \"Option Three\".into() },\n    ]}\n/>".to_string()} language={"rust".to_string()} />

                // ── Disabled Select ──
                <Demo title="Disabled">
                    <div style="width: 100%; max-width: 360px;">
                        <Select label="Disabled Select" value={(*select_disabled_value).clone()} disabled={true} onchange={Callback::from(|_| {})}
                            options={vec![
                                SelectOption { value: "opt1".into(), label: "Option One".into() },
                                SelectOption { value: "opt2".into(), label: "Option Two".into() },
                            ]}
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<Select\n    label=\"Disabled Select\"\n    value={(*select_disabled_value).clone()}\n    disabled={true}\n    options={vec![\n        SelectOption { value: \"opt1\".into(), label: \"Option One\".into() },\n        SelectOption { value: \"opt2\".into(), label: \"Option Two\".into() },\n    ]}\n/>".to_string()} language={"rust".to_string()} />

                <PropTable rows={select_props} />

                <h3 style="font-size: 18px; font-weight: 500; margin-top: 32px; margin-bottom: 16px; color: var(--md-sys-color-on-surface);">{"SelectOption"}</h3>
                <PropTable rows={select_option_props} />
            </Section>

            <Section title="Text Area">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Text areas allow users to input multiple lines of text. They support floating labels, helper text, error states, and configurable row counts."}
                </p>

                // ── Filled Text Area ──
                <Demo title="Filled Variant">
                    <div style="width: 100%; max-width: 360px;">
                        <TextArea label="Filled Text Area" variant={TextFieldVariant::Filled} value={(*textarea_filled_value).clone()} onchange={let tv = textarea_filled_value.clone(); Callback::from(move |v: String| tv.set(v))} placeholder="Type your message here..." rows={4} helper_text="Helper text" />
                    </div>
                </Demo>
                <CodeBlock code={"<TextArea\n    label=\"Filled Text Area\"\n    variant={TextFieldVariant::Filled}\n    value={(*textarea_filled_value).clone()}\n    onchange={let tv = textarea_filled_value.clone(); Callback::from(move |v: String| tv.set(v))}\n    placeholder=\"Type your message here...\"\n    rows={4}\n    helper_text=\"Helper text\"\n/>".to_string()} language={"rust".to_string()} />

                // ── Outlined Text Area ──
                <Demo title="Outlined Variant">
                    <div style="width: 100%; max-width: 360px;">
                        <TextArea label="Outlined Text Area" variant={TextFieldVariant::Outlined} value={(*textarea_outlined_value).clone()} onchange={let tv = textarea_outlined_value.clone(); Callback::from(move |v: String| tv.set(v))} placeholder="Type multiline comments here..." rows={3} />
                    </div>
                </Demo>
                <CodeBlock code={"<TextArea\n    label=\"Outlined Text Area\"\n    variant={TextFieldVariant::Outlined}\n    value={(*textarea_outlined_value).clone()}\n    onchange={let tv = textarea_outlined_value.clone(); Callback::from(move |v: String| tv.set(v))}\n    placeholder=\"Type multiline comments here...\"\n    rows={3}\n/>".to_string()} language={"rust".to_string()} />

                <PropTable rows={textarea_props} />
            </Section>
        </>
    }
}
