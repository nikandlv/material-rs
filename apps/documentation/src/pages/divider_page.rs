use yew::prelude::*;
use material_rs::components::{Divider, DividerVariant};
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn DividerPage() -> Html {
    let props = vec![
        PropRow { name: "variant".into(), r#type: "DividerVariant".into(), default_value: "DividerVariant::FullWidth".into(), description: "Visual variant: FullWidth (edge-to-edge), Inset (left margin), or Middle (indented for subheaders).".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    html! {
        <>
            <Section title="Divider">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Dividers separate content into clear groups. Material Design 3 defines \
                      three variants: FullWidth spans the entire container width, Inset provides \
                      a left margin for list alignment, and Middle adds an indented separator \
                      typically used beneath subheaders."}
                </p>

                // ── FullWidth Divider ──
                <Demo title="FullWidth Divider">
                    <div style="width: 100%; max-width: 500px;">
                        <p style="font-size: 14px; margin-bottom: 8px;">{"Content above"}</p>
                        <Divider variant={DividerVariant::FullWidth} />
                        <p style="font-size: 14px; margin-top: 8px;">{"Content below"}</p>
                    </div>
                </Demo>
                <CodeBlock code={r#"<p>{ "Content above" }</p>
<Divider variant={DividerVariant::FullWidth} />
<p>{ "Content below" }</p>"#.to_string()} language={"rust".to_string()} />

                // ── Inset Divider ──
                <Demo title="Inset Divider">
                    <div style="width: 100%; max-width: 500px;">
                        <p style="font-size: 14px; margin-bottom: 8px;">{"Content above"}</p>
                        <Divider variant={DividerVariant::Inset} />
                        <p style="font-size: 14px; margin-top: 8px;">{"Content below"}</p>
                    </div>
                </Demo>
                <CodeBlock code={r#"<p>{ "Content above" }</p>
<Divider variant={DividerVariant::Inset} />
<p>{ "Content below" }</p>"#.to_string()} language={"rust".to_string()} />

                // ── Middle Divider ──
                <Demo title="Middle Divider">
                    <div style="width: 100%; max-width: 500px;">
                        <p style="font-size: 14px; margin-bottom: 8px;">{"Content above"}</p>
                        <Divider variant={DividerVariant::Middle} />
                        <p style="font-size: 14px; margin-top: 8px;">{"Content below"}</p>
                    </div>
                </Demo>
                <CodeBlock code={r#"<p>{ "Content above" }</p>
<Divider variant={DividerVariant::Middle} />
<p>{ "Content below" }</p>"#.to_string()} language={"rust".to_string()} />

                // ── All Variants Comparison ──
                <Demo title="All Variants Comparison">
                    <div style="width: 100%; max-width: 500px;">
                        <p style="font-size: 12px; color: var(--md-sys-color-on-surface-variant); margin-bottom: 4px;">{"FullWidth"}</p>
                        <Divider variant={DividerVariant::FullWidth} />
                        <p style="font-size: 12px; color: var(--md-sys-color-on-surface-variant); margin: 8px 0 4px;">{"Inset"}</p>
                        <Divider variant={DividerVariant::Inset} />
                        <p style="font-size: 12px; color: var(--md-sys-color-on-surface-variant); margin: 8px 0 4px;">{"Middle"}</p>
                        <Divider variant={DividerVariant::Middle} />
                    </div>
                </Demo>
                <CodeBlock code={r#"<Divider variant={DividerVariant::FullWidth} />
<Divider variant={DividerVariant::Inset} />
<Divider variant={DividerVariant::Middle} />"#.to_string()} language={"rust".to_string()} />

                <PropTable rows={props} />
            </Section>
        </>
    }
}
