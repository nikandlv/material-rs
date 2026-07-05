use yew::prelude::*;
use material_rs::components::{Switch, Checkbox, RadioButton};
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

/// Selection components — Switch, Checkbox, and RadioButton — let users
/// toggle between two or more states. Switch handles on/off toggles,
/// Checkbox supports independent multi-selection with an indeterminate
/// midpoint, and RadioButton enforces a single selection within a named
/// group.

#[function_component]
pub fn SelectionPage() -> Html {
    let switch_a = use_state(|| false);
    let switch_b = use_state(|| true);
    let checkbox_checked = use_state(|| false);
    let radio_value = use_state(|| "option1".to_string());

    html! {
        <>
            <Section title="Selection Components">
                <p style="margin-bottom: 24px; color: var(--md-sys-color-on-surface-variant);">
                    {"Selection controls let users choose one or more options from a set. \
                      Material Design 3 provides three selection types: Switch for on/off \
                      states, Checkbox for independent multi-selection, and RadioButton \
                      for mutually exclusive single selection."}
                </p>
            </Section>

            // ── Switch ────────────────────────────────────────────────

            <Section title="Switch">
                <Demo title="Default">
                    <div style="display: flex; flex-direction: column; gap: 16px;">
                        <Switch label="Toggle me" checked={*switch_a} onchange={let s = switch_a.clone(); Callback::from(move |v: bool| s.set(v))} />
                    </div>
                </Demo>
                <CodeBlock code={SWITCH_DEFAULT_CODE.to_string()} language={"rust".to_string()} />

                <Demo title="With Icons">
                    <div style="display: flex; flex-direction: column; gap: 16px;">
                        <Switch label="Icon switch" checked={*switch_b} onchange={let s = switch_b.clone(); Callback::from(move |v: bool| s.set(v))} checked_icon="check" unchecked_icon="close" />
                    </div>
                </Demo>
                <CodeBlock code={SWITCH_ICONS_CODE.to_string()} language={"rust".to_string()} />

                <Demo title="Disabled">
                    <div style="display: flex; flex-direction: column; gap: 16px;">
                        <Switch label="Disabled off" checked={false} disabled={true} onchange={Callback::from(|_| {})} />
                        <Switch label="Disabled on" checked={true} disabled={true} onchange={Callback::from(|_| {})} />
                    </div>
                </Demo>
                <CodeBlock code={SWITCH_DISABLED_CODE.to_string()} language={"rust".to_string()} />

                <h3 style="font-size: 14px; font-weight: 600; margin: 24px 0 12px;">{"Props"}</h3>
                <PropTable rows={vec![
                    PropRow { name: "checked".into(), r#type: "bool".into(), default_value: "false".into(), description: "Whether the switch is toggled on.".into() },
                    PropRow { name: "onchange".into(), r#type: "Callback<bool>".into(), default_value: "Callback::noop()".into(), description: "Fires with the new checked state when toggled.".into() },
                    PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Disables interaction and dims the switch.".into() },
                    PropRow { name: "label".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Optional label rendered next to the switch.".into() },
                    PropRow { name: "checked_icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Material icon name shown inside the thumb when checked.".into() },
                    PropRow { name: "unchecked_icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Material icon name shown inside the thumb when unchecked.".into() },
                ]} />
            </Section>

            // ── Checkbox ──────────────────────────────────────────────

            <Section title="Checkbox">
                <Demo title="Checked">
                    <div style="display: flex; flex-direction: column; gap: 16px;">
                        <Checkbox label="Checkbox" checked={*checkbox_checked} onchange={let c = checkbox_checked.clone(); Callback::from(move |v: bool| c.set(v))} />
                    </div>
                </Demo>
                <CodeBlock code={CHECKBOX_CHECKED_CODE.to_string()} language={"rust".to_string()} />

                <Demo title="Indeterminate">
                    <div style="display: flex; flex-direction: column; gap: 16px;">
                        <Checkbox label="Indeterminate" indeterminate={true} onchange={Callback::from(|_| {})} />
                    </div>
                </Demo>
                <CodeBlock code={CHECKBOX_INDETERMINATE_CODE.to_string()} language={"rust".to_string()} />

                <Demo title="Disabled">
                    <div style="display: flex; flex-direction: column; gap: 16px;">
                        <Checkbox label="Disabled unchecked" checked={false} disabled={true} onchange={Callback::from(|_| {})} />
                        <Checkbox label="Disabled checked" checked={true} disabled={true} onchange={Callback::from(|_| {})} />
                    </div>
                </Demo>
                <CodeBlock code={CHECKBOX_DISABLED_CODE.to_string()} language={"rust".to_string()} />

                <h3 style="font-size: 14px; font-weight: 600; margin: 24px 0 12px;">{"Props"}</h3>
                <PropTable rows={vec![
                    PropRow { name: "checked".into(), r#type: "bool".into(), default_value: "false".into(), description: "Whether the checkbox is checked.".into() },
                    PropRow { name: "indeterminate".into(), r#type: "bool".into(), default_value: "false".into(), description: "Shows a dash instead of a checkmark; mutually exclusive with checked visual.".into() },
                    PropRow { name: "onchange".into(), r#type: "Callback<bool>".into(), default_value: "Callback::noop()".into(), description: "Fires with the new checked state on click.".into() },
                    PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Prevents user interaction and dims the checkbox.".into() },
                    PropRow { name: "label".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Optional text label beside the checkbox.".into() },
                ]} />
            </Section>

            // ── RadioButton ───────────────────────────────────────────

            <Section title="RadioButton">
                <Demo title="Group with Selection">
                    <div style="display: flex; flex-direction: column; gap: 16px;">
                        <div style="font-size: 12px; color: var(--md-sys-color-on-surface-variant); font-weight: 500;">
                            {"Choose a size"}
                        </div>
                        <div style="display: flex; gap: 24px;">
                            <RadioButton label="Small" checked={*radio_value == "small"} onchange={let rv = radio_value.clone(); Callback::from(move |v: bool| { if v { rv.set("small".into()); } })} name="size-group" />
                            <RadioButton label="Medium" checked={*radio_value == "medium"} onchange={let rv = radio_value.clone(); Callback::from(move |v: bool| { if v { rv.set("medium".into()); } })} name="size-group" />
                            <RadioButton label="Large" checked={*radio_value == "large"} onchange={let rv = radio_value.clone(); Callback::from(move |v: bool| { if v { rv.set("large".into()); } })} name="size-group" />
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={RADIO_GROUP_CODE.to_string()} language={"rust".to_string()} />

                <Demo title="Disabled">
                    <div style="display: flex; gap: 24px;">
                        <RadioButton label="Disabled" checked={false} disabled={true} onchange={Callback::from(|_| {})} name="disabled-demo" />
                        <RadioButton label="Selected & disabled" checked={true} disabled={true} onchange={Callback::from(|_| {})} name="disabled-demo" />
                    </div>
                </Demo>
                <CodeBlock code={RADIO_DISABLED_CODE.to_string()} language={"rust".to_string()} />

                <h3 style="font-size: 14px; font-weight: 600; margin: 24px 0 12px;">{"Props"}</h3>
                <PropTable rows={vec![
                    PropRow { name: "checked".into(), r#type: "bool".into(), default_value: "false".into(), description: "Whether this radio button is the selected option.".into() },
                    PropRow { name: "onchange".into(), r#type: "Callback<bool>".into(), default_value: "Callback::noop()".into(), description: "Fires true when the radio button is selected.".into() },
                    PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Prevents selection and dims the control.".into() },
                    PropRow { name: "label".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Text label rendered beside the radio dot.".into() },
                    PropRow { name: "name".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Group name — radio buttons sharing the same name form an exclusive set.".into() },
                ]} />
            </Section>
        </>
    }
}

// ── Code snippets ─────────────────────────────────────────────────

const SWITCH_DEFAULT_CODE: &str = r#"<Switch
    label="Toggle me"
    checked={checked}
    onchange={Callback::from(move |v: bool| set_checked(v))}
/>"#;

const SWITCH_ICONS_CODE: &str = r#"<Switch
    label="Icon switch"
    checked={checked}
    onchange={Callback::from(move |v: bool| set_checked(v))}
    checked_icon="check"
    unchecked_icon="close"
/>"#;

const SWITCH_DISABLED_CODE: &str = r#"<Switch label="Disabled off" checked={false} disabled={true} />
<Switch label="Disabled on"  checked={true}  disabled={true} />"#;

const CHECKBOX_CHECKED_CODE: &str = r#"<Checkbox
    label="Checkbox"
    checked={checked}
    onchange={Callback::from(move |v: bool| set_checked(v))}
/>"#;

const CHECKBOX_INDETERMINATE_CODE: &str = r#"<Checkbox
    label="Indeterminate"
    indeterminate={true}
    onchange={Callback::from(|_| {})}
/>"#;

const CHECKBOX_DISABLED_CODE: &str = r#"<Checkbox label="Disabled unchecked" checked={false} disabled={true} />
<Checkbox label="Disabled checked"   checked={true}  disabled={true} />"#;

const RADIO_GROUP_CODE: &str = r#"<RadioButton
    label="Small"
    checked={selected == "small"}
    onchange={Callback::from(move |v: bool| {
        if v { set_selected("small".into()); }
    })}
    name="size-group"
/>
<RadioButton
    label="Medium"
    checked={selected == "medium"}
    onchange={Callback::from(move |v: bool| {
        if v { set_selected("medium".into()); }
    })}
    name="size-group"
/>
<RadioButton
    label="Large"
    checked={selected == "large"}
    onchange={Callback::from(move |v: bool| {
        if v { set_selected("large".into()); }
    })}
    name="size-group"
/>"#;

const RADIO_DISABLED_CODE: &str = r#"<RadioButton label="Disabled"          checked={false} disabled={true} />
<RadioButton label="Selected & disabled" checked={true}  disabled={true} />"#;
