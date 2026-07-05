use yew::prelude::*;
use material_rs::components::{Toolbar, IconButton, IconButtonVariant};
use material_rs::theme::Theme;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn ToolbarPage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let props = vec![
        PropRow { name: "leading".into(), r#type: "Html".into(), default_value: "Html::default()".into(), description: "Optional leading component rendered at the start of the toolbar.".into() },
        PropRow { name: "title".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Title text displayed inside the toolbar.".into() },
        PropRow { name: "actions".into(), r#type: "Vec<Html>".into(), default_value: "vec![]".into(), description: "Trailing action items rendered at the end of the toolbar.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Content rendered in the main area between leading and actions.".into() },
    ];

    html! {
        <>
            <Section title="Toolbar">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"A flex container for action headers, settings tools, or footer bars. \
                      The Toolbar provides a consistent 56px-tall surface with optional \
                      leading content, a title, and trailing action buttons."}
                </p>

                // ── Basic Toolbar ──
                <Demo title="Basic Toolbar">
                    <Toolbar title="Simple Toolbar" />
                </Demo>
                <CodeBlock code={r#"<Toolbar title="Simple Toolbar" />"#.to_string()} language={"rust".to_string()} />

                // ── Toolbar with Actions ──
                <Demo title="Toolbar with Actions">
                    <Toolbar title="Settings"
                        actions={vec![
                            html!{ <IconButton icon="search" variant={IconButtonVariant::Standard} label="Search" /> },
                            html!{ <IconButton icon="more_vert" variant={IconButtonVariant::Standard} label="More" /> },
                        ]}
                    />
                </Demo>
                <CodeBlock code={r#"<Toolbar
    title="Settings"
    actions={vec![
        html!{ <IconButton icon="search" variant={IconButtonVariant::Standard} label="Search" /> },
        html!{ <IconButton icon="more_vert" variant={IconButtonVariant::Standard} label="More" /> },
    ]}
/>"#.to_string()} language={"rust".to_string()} />

                // ── Toolbar with Leading Icon ──
                <Demo title="Toolbar with Leading Icon and Actions">
                    <Toolbar title="Navigation"
                        leading={html!{ <IconButton icon="menu" variant={IconButtonVariant::Standard} label="Menu" /> }}
                        actions={vec![
                            html!{ <IconButton icon="favorite" variant={IconButtonVariant::Standard} label="Favorite" /> },
                            html!{ <IconButton icon="share" variant={IconButtonVariant::Standard} label="Share" /> },
                        ]}
                    />
                </Demo>
                <CodeBlock code={r#"<Toolbar
    title="Navigation"
    leading={html!{
        <IconButton icon="menu" variant={IconButtonVariant::Standard} label="Menu" />
    }}
    actions={vec![
        html!{ <IconButton icon="favorite" variant={IconButtonVariant::Standard} label="Favorite" /> },
        html!{ <IconButton icon="share" variant={IconButtonVariant::Standard} label="Share" /> },
    ]}
/>"#.to_string()} language={"rust".to_string()} />

                // ── Toolbar with Children ──
                <Demo title="Toolbar with Custom Content">
                    <Toolbar title="Editor"
                        leading={html!{ <IconButton icon="arrow_back" variant={IconButtonVariant::Standard} label="Back" /> }}
                        actions={vec![
                            html!{ <IconButton icon="check" variant={IconButtonVariant::Standard} label="Save" /> },
                        ]}
                    >
                        <span style={format!("font-size: 13px; color: {};", theme.colors.on_surface_variant)}>
                            {"Editing document..."}
                        </span>
                    </Toolbar>
                </Demo>
                <CodeBlock code={r#"<Toolbar
    title="Editor"
    leading={html!{
        <IconButton icon="arrow_back" variant={IconButtonVariant::Standard} label="Back" />
    }}
    actions={vec![
        html!{ <IconButton icon="check" variant={IconButtonVariant::Standard} label="Save" /> },
    ]}
>
    <span>{ "Editing document..." }</span>
</Toolbar>"#.to_string()} language={"rust".to_string()} />

                <PropTable rows={props} />
            </Section>
        </>
    }
}
