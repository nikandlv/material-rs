use yew::prelude::*;
use material_rs::components::{ProgressIndicator, ProgressType};
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

/// Progress indicators communicate the status of an ongoing process.
/// Material Design 3 provides three visual types: **Linear** bars that
/// fill from left to right, **Circular** spinners, and **Wavy** squiggly
/// lines. Each type can be determinate (showing a known percentage) or
/// indeterminate (animated, for unknown durations).

#[function_component]
pub fn ProgressPage() -> Html {
    html! {
        <>
            <Section title="Progress Indicators">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Progress indicators show how long an operation will take or that \
                      work is happening. Use determinate indicators when the completion \
                      percentage is known, and indeterminate indicators for unknown \
                      durations."}
                </p>

                // ── Linear Determinate ──
                <Demo title="Linear Determinate">
                    <div style="width: 100%; max-width: 400px;">
                        <ProgressIndicator progress_type={ProgressType::Linear} value={Some(0.65)} />
                    </div>
                </Demo>
                <CodeBlock code={LINEAR_DET_CODE.to_string()} language={"rust".to_string()} />

                // ── Linear Indeterminate ──
                <Demo title="Linear Indeterminate">
                    <div style="width: 100%; max-width: 400px;">
                        <ProgressIndicator progress_type={ProgressType::Linear} />
                    </div>
                </Demo>
                <CodeBlock code={LINEAR_INDET_CODE.to_string()} language={"rust".to_string()} />

                // ── Circular Determinate ──
                <Demo title="Circular Determinate">
                    <div style="display: flex; gap: 24px; align-items: center;">
                        <ProgressIndicator progress_type={ProgressType::Circular} value={Some(0.25)} />
                        <ProgressIndicator progress_type={ProgressType::Circular} value={Some(0.50)} />
                        <ProgressIndicator progress_type={ProgressType::Circular} value={Some(0.75)} />
                    </div>
                </Demo>
                <CodeBlock code={CIRCULAR_DET_CODE.to_string()} language={"rust".to_string()} />

                // ── Circular Indeterminate ──
                <Demo title="Circular Indeterminate">
                    <div style="display: flex; gap: 24px; align-items: center;">
                        <ProgressIndicator progress_type={ProgressType::Circular} />
                    </div>
                </Demo>
                <CodeBlock code={CIRCULAR_INDET_CODE.to_string()} language={"rust".to_string()} />

                // ── Wavy ──
                <Demo title="Wavy (Squiggly)">
                    <div style="width: 100%; max-width: 400px;">
                        <ProgressIndicator progress_type={ProgressType::Wavy} />
                    </div>
                </Demo>
                <CodeBlock code={WAVY_CODE.to_string()} language={"rust".to_string()} />

                // ── Props Table ──
                <PropTable rows={vec![
                    PropRow { name: "progress_type".into(), r#type: "ProgressType".into(), default_value: "ProgressType::Linear".into(), description: "Visual variant: Linear, Circular, or Wavy.".into() },
                    PropRow { name: "value".into(), r#type: "Option<f64>".into(), default_value: "None".into(), description: "Determinate progress from 0.0 to 1.0. None renders an indeterminate animation.".into() },
                ]} />
            </Section>
        </>
    }
}

// ── Code snippets ─────────────────────────────────────────────────

const LINEAR_DET_CODE: &str = r#"<ProgressIndicator
    progress_type={ProgressType::Linear}
    value={Some(0.65)}
/>"#;

const LINEAR_INDET_CODE: &str = r#"<ProgressIndicator
    progress_type={ProgressType::Linear}
/>"#;

const CIRCULAR_DET_CODE: &str = r#"<ProgressIndicator
    progress_type={ProgressType::Circular}
    value={Some(0.75)}
/>"#;

const CIRCULAR_INDET_CODE: &str = r#"<ProgressIndicator
    progress_type={ProgressType::Circular}
/>"#;

const WAVY_CODE: &str = r#"<ProgressIndicator
    progress_type={ProgressType::Wavy}
/>"#;
