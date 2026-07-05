use yew::prelude::*;
use material_rs::components::{ToggleButtonGroup, ToggleButtonItem, ToggleGroupMode};
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

/// Toggle button groups let users select one or more options from a set.
/// They are commonly used for text formatting controls (bold, italic,
/// underline), alignment pickers, or any scenario where a compact
/// selection UI is needed. Groups can operate in **Single** mode
/// (mutually exclusive) or **Multi** mode (independent toggles).

#[function_component]
pub fn ToggleButtonsPage() -> Html {
    let single_selected = use_state(|| vec!["center".to_string()]);
    let multi_selected = use_state(|| vec!["bold".to_string(), "italic".to_string()]);

    html! {
        <>
            <Section title="Toggle Button Group">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Toggle button groups display a set of options where users can \
                      select one (single mode) or multiple (multi mode) items. They \
                      provide a compact, self-contained selection UI with icons and labels."}
                </p>

                // ── Single Select ──
                <Demo title="Single Select">
                    <div style="width: 100%; max-width: 560px;">
                        <ToggleButtonGroup
                            items={vec![
                                ToggleButtonItem { key: "left".into(), label: "Left".into(), icon: "format_align_left".into() },
                                ToggleButtonItem { key: "center".into(), label: "Center".into(), icon: "format_align_center".into() },
                                ToggleButtonItem { key: "right".into(), label: "Right".into(), icon: "format_align_right".into() },
                                ToggleButtonItem { key: "justify".into(), label: "Justify".into(), icon: "format_align_justify".into() },
                            ]}
                            selected={(*single_selected).clone()}
                            mode={ToggleGroupMode::Single}
                            onchange={let s = single_selected.clone(); Callback::from(move |v: Vec<String>| s.set(v))}
                        />
                    </div>
                </Demo>
                <CodeBlock code={SINGLE_CODE.to_string()} language={"rust".to_string()} />

                // ── Multi Select ──
                <Demo title="Multi Select">
                    <div style="width: 100%; max-width: 560px;">
                        <ToggleButtonGroup
                            items={vec![
                                ToggleButtonItem { key: "bold".into(), label: "Bold".into(), icon: "format_bold".into() },
                                ToggleButtonItem { key: "italic".into(), label: "Italic".into(), icon: "format_italic".into() },
                                ToggleButtonItem { key: "underline".into(), label: "Underline".into(), icon: "format_underlined".into() },
                                ToggleButtonItem { key: "strikethrough".into(), label: "Strike".into(), icon: "strikethrough_s".into() },
                            ]}
                            selected={(*multi_selected).clone()}
                            mode={ToggleGroupMode::Multi}
                            onchange={let s = multi_selected.clone(); Callback::from(move |v: Vec<String>| s.set(v))}
                        />
                    </div>
                </Demo>
                <CodeBlock code={MULTI_CODE.to_string()} language={"rust".to_string()} />

                // ── Disabled ──
                <Demo title="Disabled Group">
                    <div style="width: 100%; max-width: 560px;">
                        <ToggleButtonGroup
                            items={vec![
                                ToggleButtonItem { key: "left".into(), label: "Left".into(), icon: "format_align_left".into() },
                                ToggleButtonItem { key: "center".into(), label: "Center".into(), icon: "format_align_center".into() },
                                ToggleButtonItem { key: "right".into(), label: "Right".into(), icon: "format_align_right".into() },
                            ]}
                            selected={vec!["left".into()]}
                            mode={ToggleGroupMode::Single}
                            disabled={true}
                            onchange={Callback::from(|_: Vec<String>| {})}
                        />
                    </div>
                </Demo>
                <CodeBlock code={DISABLED_CODE.to_string()} language={"rust".to_string()} />

                // ── Props Table ──
                <PropTable rows={vec![
                    PropRow { name: "items".into(), r#type: "Vec<ToggleButtonItem>".into(), default_value: "(required)".into(), description: "List of toggle items to render. Each has key, label, and optional icon.".into() },
                    PropRow { name: "selected".into(), r#type: "Vec<String>".into(), default_value: "vec![]".into(), description: "Keys of currently selected items.".into() },
                    PropRow { name: "mode".into(), r#type: "ToggleGroupMode".into(), default_value: "ToggleGroupMode::Single".into(), description: "Single (one selection) or Multi (independent selections).".into() },
                    PropRow { name: "onchange".into(), r#type: "Callback<Vec<String>>".into(), default_value: "Callback::noop()".into(), description: "Fires with the updated list of selected keys when the selection changes.".into() },
                    PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Disables all buttons in the group.".into() },
                ]} />
            </Section>
        </>
    }
}

// ── Code snippets ─────────────────────────────────────────────────

const SINGLE_CODE: &str = r#"<ToggleButtonGroup
    items={vec![
        ToggleButtonItem { key: "left".into(), label: "Left".into(), icon: "format_align_left".into() },
        ToggleButtonItem { key: "center".into(), label: "Center".into(), icon: "format_align_center".into() },
        ToggleButtonItem { key: "right".into(), label: "Right".into(), icon: "format_align_right".into() },
        ToggleButtonItem { key: "justify".into(), label: "Justify".into(), icon: "format_align_justify".into() },
    ]}
    selected={selected}
    mode={ToggleGroupMode::Single}
    onchange={Callback::from(move |v: Vec<String>| set_selected(v))}
/>"#;

const MULTI_CODE: &str = r#"<ToggleButtonGroup
    items={vec![
        ToggleButtonItem { key: "bold".into(), label: "Bold".into(), icon: "format_bold".into() },
        ToggleButtonItem { key: "italic".into(), label: "Italic".into(), icon: "format_italic".into() },
        ToggleButtonItem { key: "underline".into(), label: "Underline".into(), icon: "format_underlined".into() },
        ToggleButtonItem { key: "strikethrough".into(), label: "Strike".into(), icon: "strikethrough_s".into() },
    ]}
    selected={selected}
    mode={ToggleGroupMode::Multi}
    onchange={Callback::from(move |v: Vec<String>| set_selected(v))}
/>"#;

const DISABLED_CODE: &str = r#"<ToggleButtonGroup
    items={vec![
        ToggleButtonItem { key: "left".into(), label: "Left".into(), icon: "format_align_left".into() },
        ToggleButtonItem { key: "center".into(), label: "Center".into(), icon: "format_align_center".into() },
        ToggleButtonItem { key: "right".into(), label: "Right".into(), icon: "format_align_right".into() },
    ]}
    selected={vec!["left".into()]}
    mode={ToggleGroupMode::Single}
    disabled={true}
    onchange={Callback::from(|_| {})}
/>"#;
