use yew::prelude::*;
use material_rs::components::{Chip, ChipType};
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

/// Chips are compact elements that represent an input, action, or filter.
/// Material Design 3 defines three chip types: **Assist** chips surface
/// contextual actions, **Filter** chips toggle selection states, and
/// **Input** chips represent discrete user-entered values with an option
/// to remove them.

#[function_component]
pub fn ChipsPage() -> Html {
    let filter_a = use_state(|| false);
    let filter_b = use_state(|| true);
    let filter_c = use_state(|| false);

    html! {
        <>
            <Section title="Chips">
                <p style="margin-bottom: 24px; color: var(--md-sys-color-on-surface-variant);">
                    {"Chips help users navigate, filter content, or enter information. \
                      Each chip type serves a distinct purpose: Assist chips trigger \
                      actions, Filter chips toggle states, and Input chips hold \
                      removable user values."}
                </p>
            </Section>

            // ── Assist Chips ──────────────────────────────────────────

            <Section title="Assist Chips">
                <Demo title="Default">
                    <div style="display: flex; flex-wrap: wrap; gap: 8px;">
                        <Chip label="Book flight" chip_type={ChipType::Assist} />
                        <Chip label="With icon" chip_type={ChipType::Assist} icon="info" />
                    </div>
                </Demo>
                <CodeBlock code={ASSIST_CODE.to_string()} language={"rust".to_string()} />

                <h3 style="font-size: 14px; font-weight: 600; margin: 24px 0 12px;">{"Props"}</h3>
                <PropTable rows={vec![
                    PropRow { name: "label".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Text displayed inside the chip.".into() },
                    PropRow { name: "chip_type".into(), r#type: "ChipType".into(), default_value: "ChipType::Assist".into(), description: "The chip variant: Assist, Filter, or Input.".into() },
                    PropRow { name: "icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Material icon name shown before the label.".into() },
                    PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Disables interaction and reduces opacity.".into() },
                ]} />
            </Section>

            // ── Filter Chips ──────────────────────────────────────────

            <Section title="Filter Chips">
                <Demo title="Selected / Unselected">
                    <div style="display: flex; flex-wrap: wrap; gap: 8px;">
                        <Chip label="Unselected" chip_type={ChipType::Filter} selected={*filter_a} on_select={let s = filter_a.clone(); Callback::from(move |v: bool| s.set(v))} />
                        <Chip label="Selected" chip_type={ChipType::Filter} selected={*filter_b} on_select={let s = filter_b.clone(); Callback::from(move |v: bool| s.set(v))} />
                        <Chip label="Toggle me" chip_type={ChipType::Filter} selected={*filter_c} on_select={let s = filter_c.clone(); Callback::from(move |v: bool| s.set(v))} />
                    </div>
                </Demo>
                <CodeBlock code={FILTER_CODE.to_string()} language={"rust".to_string()} />

                <h3 style="font-size: 14px; font-weight: 600; margin: 24px 0 12px;">{"Props"}</h3>
                <PropTable rows={vec![
                    PropRow { name: "label".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Text displayed inside the chip.".into() },
                    PropRow { name: "chip_type".into(), r#type: "ChipType".into(), default_value: "ChipType::Assist".into(), description: "Set to ChipType::Filter for toggle behaviour.".into() },
                    PropRow { name: "selected".into(), r#type: "bool".into(), default_value: "false".into(), description: "Whether the filter chip is currently active.".into() },
                    PropRow { name: "on_select".into(), r#type: "Callback<bool>".into(), default_value: "Callback::noop()".into(), description: "Fires with the new selected state when toggled.".into() },
                    PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Disables the chip and dims its appearance.".into() },
                ]} />
            </Section>

            // ── Input Chips ───────────────────────────────────────────

            <Section title="Input Chips">
                <Demo title="With Remove">
                    <div style="display: flex; flex-wrap: wrap; gap: 8px;">
                        <Chip label="React" chip_type={ChipType::Input} icon="code" on_remove={Callback::from(|_| {})} />
                        <Chip label="Rust" chip_type={ChipType::Input} icon="code" on_remove={Callback::from(|_| {})} />
                        <Chip label="TypeScript" chip_type={ChipType::Input} icon="code" on_remove={Callback::from(|_| {})} />
                    </div>
                </Demo>
                <CodeBlock code={INPUT_CODE.to_string()} language={"rust".to_string()} />

                <h3 style="font-size: 14px; font-weight: 600; margin: 24px 0 12px;">{"Props"}</h3>
                <PropTable rows={vec![
                    PropRow { name: "label".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Text displayed inside the chip.".into() },
                    PropRow { name: "chip_type".into(), r#type: "ChipType".into(), default_value: "ChipType::Assist".into(), description: "Set to ChipType::Input to show the remove button.".into() },
                    PropRow { name: "icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Material icon name rendered before the label.".into() },
                    PropRow { name: "on_remove".into(), r#type: "Callback<()>".into(), default_value: "Callback::noop()".into(), description: "Fires when the remove button is clicked.".into() },
                    PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Disables the chip and its remove button.".into() },
                ]} />
            </Section>

            // ── Disabled Chips ────────────────────────────────────────

            <Section title="Disabled Chips">
                <Demo title="All Types Disabled">
                    <div style="display: flex; flex-wrap: wrap; gap: 8px;">
                        <Chip label="Disabled assist" chip_type={ChipType::Assist} icon="info" disabled={true} />
                        <Chip label="Disabled filter" chip_type={ChipType::Filter} selected={true} disabled={true} />
                        <Chip label="Disabled input" chip_type={ChipType::Input} icon="tag" disabled={true} />
                    </div>
                </Demo>
                <CodeBlock code={DISABLED_CODE.to_string()} language={"rust".to_string()} />

                <h3 style="font-size: 14px; font-weight: 600; margin: 24px 0 12px;">{"Props"}</h3>
                <PropTable rows={vec![
                    PropRow { name: "label".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Text displayed inside the chip.".into() },
                    PropRow { name: "chip_type".into(), r#type: "ChipType".into(), default_value: "ChipType::Assist".into(), description: "The chip variant: Assist, Filter, or Input.".into() },
                    PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Prevents all interaction and reduces opacity to 38%.".into() },
                ]} />
            </Section>
        </>
    }
}

// ── Code snippets ─────────────────────────────────────────────────

const ASSIST_CODE: &str = r#"<Chip label="Book flight" chip_type={ChipType::Assist} />
<Chip label="With icon" chip_type={ChipType::Assist} icon="info" />"#;

const FILTER_CODE: &str = r#"<Chip
    label="Unselected"
    chip_type={ChipType::Filter}
    selected={selected}
    on_select={Callback::from(move |v: bool| set_selected(v))}
/>
<Chip
    label="Selected"
    chip_type={ChipType::Filter}
    selected={true}
    on_select={Callback::from(|_| {})}
/>
<Chip
    label="Toggle me"
    chip_type={ChipType::Filter}
    selected={selected2}
    on_select={Callback::from(move |v: bool| set_selected2(v))}
/>"#;

const INPUT_CODE: &str = r#"<Chip
    label="React"
    chip_type={ChipType::Input}
    icon="code"
    on_remove={Callback::from(|_| {})}
/>
<Chip
    label="Rust"
    chip_type={ChipType::Input}
    icon="code"
    on_remove={Callback::from(|_| {})}
/>
<Chip
    label="TypeScript"
    chip_type={ChipType::Input}
    icon="code"
    on_remove={Callback::from(|_| {})}
/>"#;

const DISABLED_CODE: &str = r#"<Chip label="Disabled assist"  chip_type={ChipType::Assist} icon="info" disabled={true} />
<Chip label="Disabled filter" chip_type={ChipType::Filter} selected={true} disabled={true} />
<Chip label="Disabled input"  chip_type={ChipType::Input}  icon="tag" disabled={true} />"#;
