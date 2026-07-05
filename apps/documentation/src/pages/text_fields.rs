use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn TextFieldsPage() -> Html {
    let text_filled = use_state(String::new);
    let on_change_filled = {
        let text_filled = text_filled.clone();
        Callback::from(move |val: String| text_filled.set(val))
    };

    let text_outlined = use_state(String::new);
    let on_change_outlined = {
        let text_outlined = text_outlined.clone();
        Callback::from(move |val: String| text_outlined.set(val))
    };

    let text_plain = use_state(String::new);
    let on_change_plain = {
        let text_plain = text_plain.clone();
        Callback::from(move |val: String| text_plain.set(val))
    };

    let text_underline = use_state(String::new);
    let on_change_underline = {
        let text_underline = text_underline.clone();
        Callback::from(move |val: String| text_underline.set(val))
    };

    let text_dense = use_state(String::new);
    let on_change_dense = {
        let text_dense = text_dense.clone();
        Callback::from(move |val: String| text_dense.set(val))
    };

    let text_error = use_state(String::new);
    let on_change_error = {
        let text_error = text_error.clone();
        Callback::from(move |val: String| text_error.set(val))
    };

    let text_icons = use_state(String::new);
    let on_change_icons = {
        let text_icons = text_icons.clone();
        Callback::from(move |val: String| text_icons.set(val))
    };

    let text_password = use_state(String::new);
    let on_change_password = {
        let text_password = text_password.clone();
        Callback::from(move |val: String| text_password.set(val))
    };

    let text_required = use_state(String::new);
    let on_change_required = {
        let text_required = text_required.clone();
        Callback::from(move |val: String| text_required.set(val))
    };

    let text_maxlen = use_state(String::new);
    let on_change_maxlen = {
        let text_maxlen = text_maxlen.clone();
        Callback::from(move |val: String| text_maxlen.set(val))
    };

    let props_table = vec![
        PropRow { name: "label".into(), r#type: "String".into(), default_value: "required".into(), description: "Floating label text displayed above the input.".into() },
        PropRow { name: "variant".into(), r#type: "TextFieldVariant".into(), default_value: "Filled".into(), description: "Visual variant: Filled, Outlined, Plain, Underline, or Dense.".into() },
        PropRow { name: "value".into(), r#type: "String".into(), default_value: "required".into(), description: "Current input value.".into() },
        PropRow { name: "onchange".into(), r#type: "Callback<String>".into(), default_value: "required".into(), description: "Callback fired when the input value changes.".into() },
        PropRow { name: "placeholder".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Placeholder text shown when the field is focused and empty.".into() },
        PropRow { name: "helper_text".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Helper text displayed below the field.".into() },
        PropRow { name: "error_text".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Error message displayed when error is true.".into() },
        PropRow { name: "error".into(), r#type: "bool".into(), default_value: "false".into(), description: "Puts the field in error state with red styling.".into() },
        PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Disables input and applies muted styling.".into() },
        PropRow { name: "leading_icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Material icon shown at the start of the input.".into() },
        PropRow { name: "trailing_icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Material icon shown at the end of the input.".into() },
        PropRow { name: "input_type".into(), r#type: "String".into(), default_value: "\"text\"".into(), description: "HTML input type: text, password, email, number, etc.".into() },
        PropRow { name: "required".into(), r#type: "bool".into(), default_value: "false".into(), description: "Marks the field as required for form validation.".into() },
        PropRow { name: "max_length".into(), r#type: "usize".into(), default_value: "0".into(), description: "Maximum character count. 0 means no limit. Shows a counter when set.".into() },
        PropRow { name: "full_width".into(), r#type: "bool".into(), default_value: "true".into(), description: "Expands to fill the parent container width.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    html! {
        <>
            <Section title="Text Field">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Text fields let users enter and edit text. They come in multiple visual variants to fit different contexts, from dense data entry to clean minimal forms."}
                </p>

                // ── Variants ──
                <Demo title="Filled Variant">
                    <div style="width: 100%; max-width: 400px;">
                        <TextField
                            label="Filled"
                            variant={TextFieldVariant::Filled}
                            value={(*text_filled).clone()}
                            onchange={on_change_filled.clone()}
                            helper_text="Helper text below the field"
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<TextField\n    label=\"Filled\"\n    variant={TextFieldVariant::Filled}\n    value={(*text_filled).clone()}\n    onchange={on_change_filled.clone()}\n    helper_text=\"Helper text below the field\"\n/>".to_string()} language={"rust".to_string()} />

                <Demo title="Outlined Variant">
                    <div style="width: 100%; max-width: 400px;">
                        <TextField
                            label="Outlined"
                            variant={TextFieldVariant::Outlined}
                            value={(*text_outlined).clone()}
                            onchange={on_change_outlined.clone()}
                            helper_text="Bordered style"
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<TextField\n    label=\"Outlined\"\n    variant={TextFieldVariant::Outlined}\n    value={(*text_outlined).clone()}\n    onchange={on_change_outlined.clone()}\n    helper_text=\"Bordered style\"\n/>".to_string()} language={"rust".to_string()} />

                <Demo title="Plain Variant">
                    <div style="width: 100%; max-width: 400px;">
                        <TextField
                            label="Plain"
                            variant={TextFieldVariant::Plain}
                            value={(*text_plain).clone()}
                            onchange={on_change_plain.clone()}
                            helper_text="No background, no border"
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<TextField\n    label=\"Plain\"\n    variant={TextFieldVariant::Plain}\n    value={(*text_plain).clone()}\n    onchange={on_change_plain.clone()}\n    helper_text=\"No background, no border\"\n/>".to_string()} language={"rust".to_string()} />

                <Demo title="Underline Variant">
                    <div style="width: 100%; max-width: 400px;">
                        <TextField
                            label="Underline"
                            variant={TextFieldVariant::Underline}
                            value={(*text_underline).clone()}
                            onchange={on_change_underline.clone()}
                            helper_text="Bottom border indicator"
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<TextField\n    label=\"Underline\"\n    variant={TextFieldVariant::Underline}\n    value={(*text_underline).clone()}\n    onchange={on_change_underline.clone()}\n    helper_text=\"Bottom border indicator\"\n/>".to_string()} language={"rust".to_string()} />

                <Demo title="Dense Variant">
                    <div style="width: 100%; max-width: 400px;">
                        <TextField
                            label="Dense"
                            variant={TextFieldVariant::Dense}
                            value={(*text_dense).clone()}
                            onchange={on_change_dense.clone()}
                            helper_text="Compact 40px height"
                            leading_icon="person"
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<TextField\n    label=\"Dense\"\n    variant={TextFieldVariant::Dense}\n    value={(*text_dense).clone()}\n    onchange={on_change_dense.clone()}\n    helper_text=\"Compact 40px height\"\n    leading_icon=\"person\"\n/>".to_string()} language={"rust".to_string()} />

                // ── Icons ──
                <Demo title="Leading & Trailing Icons">
                    <div style="display: flex; flex-direction: column; gap: 24px; width: 100%; max-width: 400px;">
                        <TextField
                            label="Search"
                            variant={TextFieldVariant::Filled}
                            value={(*text_icons).clone()}
                            onchange={on_change_icons.clone()}
                            leading_icon="search"
                            helper_text="Leading icon"
                        />
                        <TextField
                            label="Email"
                            variant={TextFieldVariant::Outlined}
                            value={String::new()}
                            onchange={Callback::from(|_| {})}
                            trailing_icon="check_circle"
                            helper_text="Trailing icon"
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<TextField\n    label=\"Search\"\n    variant={TextFieldVariant::Filled}\n    value={(*text_icons).clone()}\n    onchange={on_change_icons.clone()}\n    leading_icon=\"search\"\n    helper_text=\"Leading icon\"\n/>\n\n<TextField\n    label=\"Email\"\n    variant={TextFieldVariant::Outlined}\n    value={String::new()}\n    onchange={Callback::from(|_| {})}\n    trailing_icon=\"check_circle\"\n    helper_text=\"Trailing icon\"\n/>".to_string()} language={"rust".to_string()} />

                // ── Error State ──
                <Demo title="Error State">
                    <div style="display: flex; flex-direction: column; gap: 24px; width: 100%; max-width: 400px;">
                        <TextField
                            label="Username"
                            variant={TextFieldVariant::Filled}
                            value={(*text_error).clone()}
                            onchange={on_change_error.clone()}
                            error={true}
                            error_text="This field is required"
                        />
                        <TextField
                            label="Password"
                            variant={TextFieldVariant::Outlined}
                            value={String::new()}
                            onchange={Callback::from(|_| {})}
                            error={true}
                            error_text="Password must be at least 8 characters"
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<TextField\n    label=\"Username\"\n    variant={TextFieldVariant::Filled}\n    value={(*text_error).clone()}\n    onchange={on_change_error.clone()}\n    error={true}\n    error_text=\"This field is required\"\n/>\n\n<TextField\n    label=\"Password\"\n    variant={TextFieldVariant::Outlined}\n    value={String::new()}\n    onchange={Callback::from(|_| {})}\n    error={true}\n    error_text=\"Password must be at least 8 characters\"\n/>".to_string()} language={"rust".to_string()} />

                // ── Disabled State ──
                <Demo title="Disabled State">
                    <div style="width: 100%; max-width: 400px;">
                        <TextField
                            label="Disabled"
                            variant={TextFieldVariant::Filled}
                            value="Cannot edit this"
                            onchange={Callback::from(|_| {})}
                            disabled={true}
                            helper_text="This field is disabled"
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<TextField\n    label=\"Disabled\"\n    variant={TextFieldVariant::Filled}\n    value=\"Cannot edit this\"\n    onchange={Callback::from(|_| {})}\n    disabled={true}\n    helper_text=\"This field is disabled\"\n/>".to_string()} language={"rust".to_string()} />

                // ── Input Types ──
                <Demo title="Input Types">
                    <div style="width: 100%; max-width: 400px;">
                        <TextField
                            label="Password"
                            variant={TextFieldVariant::Filled}
                            value={(*text_password).clone()}
                            onchange={on_change_password.clone()}
                            input_type="password"
                            trailing_icon="visibility"
                            helper_text="Enter your password"
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<TextField\n    label=\"Password\"\n    variant={TextFieldVariant::Filled}\n    value={(*text_password).clone()}\n    onchange={on_change_password.clone()}\n    input_type=\"password\"\n    trailing_icon=\"visibility\"\n    helper_text=\"Enter your password\"\n/>".to_string()} language={"rust".to_string()} />

                // ── Required & Max Length ──
                <Demo title="Required & Max Length">
                    <div style="display: flex; flex-direction: column; gap: 24px; width: 100%; max-width: 400px;">
                        <TextField
                            label="Required Field"
                            variant={TextFieldVariant::Filled}
                            value={(*text_required).clone()}
                            onchange={on_change_required.clone()}
                            required={true}
                            helper_text="This field is required"
                        />
                        <TextField
                            label="Bio (max 50 chars)"
                            variant={TextFieldVariant::Outlined}
                            value={(*text_maxlen).clone()}
                            onchange={on_change_maxlen.clone()}
                            max_length={50}
                            helper_text="Tell us about yourself"
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<TextField\n    label=\"Required Field\"\n    variant={TextFieldVariant::Filled}\n    value={(*text_required).clone()}\n    onchange={on_change_required.clone()}\n    required={true}\n    helper_text=\"This field is required\"\n/>\n\n<TextField\n    label=\"Bio (max 50 chars)\"\n    variant={TextFieldVariant::Outlined}\n    value={(*text_maxlen).clone()}\n    onchange={on_change_maxlen.clone()}\n    max_length={50}\n    helper_text=\"Tell us about yourself\"\n/>".to_string()} language={"rust".to_string()} />

                // ── All Variants Side-by-Side ──
                <Demo title="All Variants at a Glance">
                    <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 24px; width: 100%;">
                        <TextField label="Filled" variant={TextFieldVariant::Filled} value={String::new()} onchange={Callback::from(|_| {})} />
                        <TextField label="Outlined" variant={TextFieldVariant::Outlined} value={String::new()} onchange={Callback::from(|_| {})} />
                        <TextField label="Plain" variant={TextFieldVariant::Plain} value={String::new()} onchange={Callback::from(|_| {})} />
                        <TextField label="Underline" variant={TextFieldVariant::Underline} value={String::new()} onchange={Callback::from(|_| {})} />
                        <TextField label="Dense" variant={TextFieldVariant::Dense} value={String::new()} onchange={Callback::from(|_| {})} />
                    </div>
                </Demo>
                <CodeBlock code={"<TextField label=\"Filled\" variant={TextFieldVariant::Filled} value={String::new()} onchange={Callback::from(|_| {})} />\n<TextField label=\"Outlined\" variant={TextFieldVariant::Outlined} value={String::new()} onchange={Callback::from(|_| {})} />\n<TextField label=\"Plain\" variant={TextFieldVariant::Plain} value={String::new()} onchange={Callback::from(|_| {})} />\n<TextField label=\"Underline\" variant={TextFieldVariant::Underline} value={String::new()} onchange={Callback::from(|_| {})} />\n<TextField label=\"Dense\" variant={TextFieldVariant::Dense} value={String::new()} onchange={Callback::from(|_| {})} />".to_string()} language={"rust".to_string()} />

                // ── Props Table ──
                <PropTable rows={props_table} />
            </Section>
        </>
    }
}
