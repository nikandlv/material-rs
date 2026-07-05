use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn RipplePage() -> Html {
    let ripple_props = vec![
        PropRow { name: "color".into(), r#type: "String".into(), default_value: "\"rgba(255, 255, 255, 0.24)\"".into(), description: "Ripple color in CSS format. Use \"transparent\" to disable the visual effect.".into() },
        PropRow { name: "duration".into(), r#type: "u32".into(), default_value: "550".into(), description: "Ripple animation duration in milliseconds.".into() },
    ];

    let default_code = r#"<div style="position: relative; overflow: hidden; border-radius: 12px;
     background: var(--md-sys-color-primary); color: var(--md-sys-color-on-primary);
     padding: 24px 32px; cursor: pointer; user-select: none;">
    <Ripple />
    {"Click me for a ripple"}
</div>"#;

    let color_code = r#"<div style="position: relative; overflow: hidden; border-radius: 12px;
     background: var(--md-sys-color-tertiary); color: var(--md-sys-color-on-tertiary);
     padding: 24px 32px; cursor: pointer; user-select: none;">
    <Ripple color="rgba(0, 0, 0, 0.12)" />
    {"Custom color ripple"}
</div>"#;

    let duration_code = r#"<div style="position: relative; overflow: hidden; border-radius: 12px;
     background: var(--md-sys-color-secondary); color: var(--md-sys-color-on-secondary);
     padding: 24px 32px; cursor: pointer; user-select: none;">
    <Ripple duration={200} />
    {"Fast ripple (200ms)"}
</div>"#;

    html! {
        <>
            <Section title="Ripple">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Ripple provides visual feedback on pointer interaction. It creates an expanding \
                      circular animation from the click point, following the Material Design 3 \
                      state layer specification. Place it inside a relatively positioned container."}
                </p>

                // ── Default Ripple ──
                <Demo title="Default Ripple">
                    <div style="position: relative; overflow: hidden; border-radius: 12px; background: var(--md-sys-color-primary); color: var(--md-sys-color-on-primary); padding: 24px 32px; cursor: pointer; user-select: none;">
                        <Ripple />
                        {"Click me for a ripple"}
                    </div>
                </Demo>
                <CodeBlock code={default_code.to_string()} language={"rust".to_string()} />

                // ── Custom Color ──
                <Demo title="Custom Color">
                    <div style="position: relative; overflow: hidden; border-radius: 12px; background: var(--md-sys-color-tertiary); color: var(--md-sys-color-on-tertiary); padding: 24px 32px; cursor: pointer; user-select: none;">
                        <Ripple color={"rgba(0, 0, 0, 0.12)".to_string()} />
                        {"Custom color ripple"}
                    </div>
                </Demo>
                <CodeBlock code={color_code.to_string()} language={"rust".to_string()} />

                // ── Fast Duration ──
                <Demo title="Fast Duration">
                    <div style="position: relative; overflow: hidden; border-radius: 12px; background: var(--md-sys-color-secondary); color: var(--md-sys-color-on-secondary); padding: 24px 32px; cursor: pointer; user-select: none;">
                        <Ripple duration={200} />
                        {"Fast ripple (200ms)"}
                    </div>
                </Demo>
                <CodeBlock code={duration_code.to_string()} language={"rust".to_string()} />

                // ── Props Table ──
                <h3 style="font-size: 16px; font-weight: 600; margin: 32px 0 12px;">{"Ripple"}</h3>
                <PropTable rows={ripple_props} />
            </Section>
        </>
    }
}
