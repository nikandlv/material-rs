use yew::prelude::*;
use material_rs::components::{CodeView, CodeLanguage};
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn CodeViewPage() -> Html {
    let rust_code = r#"use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let count = use_state(|| 0);

    let onclick = {
        let count = count.clone();
        Callback::from(move |_: MouseEvent| {
            count.set(*count + 1);
        })
    };

    html! {
        <div>
            <h1>{ format!("Count: {}", *count) }</h1>
            <button onclick={onclick}>{ "Click me" }</button>
        </div>
    }
}"#;

    let js_code = r#"import { useState } from 'react';

function Counter() {
    const [count, setCount] = useState(0);

    return (
        <div>
            <h1>Count: {count}</h1>
            <button onClick={() => setCount(count + 1)}>
                Click me
            </button>
        </div>
    );
}

export default Counter;"#;

    let css_code = r#".card {
    display: flex;
    flex-direction: column;
    padding: 16px;
    border-radius: 12px;
    background-color: var(--surface-container-low);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.12);
    transition: transform 200ms ease, box-shadow 200ms ease;
}

.card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}"#;

    let html_code = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Material RS</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div id="app"></div>
    <script src="app.js"></script>
</body>
</html>"#;

    let plain_code = r#"This is plain text without any syntax highlighting.
It can be used for logs, output, or any non-code content.
Each line is rendered as-is with the monospace font."#;

    let code_view_props = vec![
        PropRow { name: "code".into(), r#type: "String".into(), default_value: "(required)".into(), description: "The source code string to display in the code block.".into() },
        PropRow { name: "language".into(), r#type: "Language".into(), default_value: "Language::Rust".into(), description: "Programming language for syntax highlighting: Rust, JavaScript, Css, Html, or Plain.".into() },
        PropRow { name: "show_line_numbers".into(), r#type: "bool".into(), default_value: "true".into(), description: "Whether to display line numbers in the gutter on the left side.".into() },
        PropRow { name: "show_copy".into(), r#type: "bool".into(), default_value: "true".into(), description: "Whether to show the copy-to-clipboard button in the header.".into() },
        PropRow { name: "title".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Optional filename or title displayed in the header bar above the code.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute for the root element.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names applied to the root element.".into() },
    ];

    html! {
        <>
            <Section title="Code View">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"CodeView is a syntax-highlighted code block component with line numbers, \
                      copy-to-clipboard functionality, and MD3-themed styling. It supports \
                      Rust, JavaScript, CSS, HTML, and plain text via syntect-based highlighting."}
                </p>
            </Section>

            // ── Rust Example ──
            <Section title="Rust">
                <Demo title="Rust Syntax Highlighting">
                    <div style="width: 100%; max-width: 640px;">
                        <CodeView code={rust_code.to_string()} language={CodeLanguage::Rust} title={"app.rs".to_string()} />
                    </div>
                </Demo>
                <CodeBlock code={"<CodeView\n    code={rust_code.to_string()}\n    language={CodeLanguage::Rust}\n    title={\"app.rs\".to_string()}\n/>".to_string()} language={"rust".to_string()} />
            </Section>

            // ── JavaScript Example ──
            <Section title="JavaScript">
                <Demo title="JavaScript Syntax Highlighting">
                    <div style="width: 100%; max-width: 640px;">
                        <CodeView code={js_code.to_string()} language={CodeLanguage::JavaScript} title={"Counter.jsx".to_string()} />
                    </div>
                </Demo>
                <CodeBlock code={"<CodeView\n    code={js_code.to_string()}\n    language={CodeLanguage::JavaScript}\n    title={\"Counter.jsx\".to_string()}\n/>".to_string()} language={"rust".to_string()} />
            </Section>

            // ── CSS Example ──
            <Section title="CSS">
                <Demo title="CSS Syntax Highlighting">
                    <div style="width: 100%; max-width: 640px;">
                        <CodeView code={css_code.to_string()} language={CodeLanguage::Css} title={"styles.css".to_string()} />
                    </div>
                </Demo>
                <CodeBlock code={"<CodeView\n    code={css_code.to_string()}\n    language={CodeLanguage::Css}\n    title={\"styles.css\".to_string()}\n/>".to_string()} language={"rust".to_string()} />
            </Section>

            // ── HTML Example ──
            <Section title="HTML">
                <Demo title="HTML Syntax Highlighting">
                    <div style="width: 100%; max-width: 640px;">
                        <CodeView code={html_code.to_string()} language={CodeLanguage::Html} title={"index.html".to_string()} />
                    </div>
                </Demo>
                <CodeBlock code={"<CodeView\n    code={html_code.to_string()}\n    language={CodeLanguage::Html}\n    title={\"index.html\".to_string()}\n/>".to_string()} language={"rust".to_string()} />
            </Section>

            // ── Plain Text ──
            <Section title="Plain Text">
                <Demo title="No Syntax Highlighting">
                    <div style="width: 100%; max-width: 640px;">
                        <CodeView code={plain_code.to_string()} language={CodeLanguage::Plain} title={"readme.txt".to_string()} />
                    </div>
                </Demo>
                <CodeBlock code={"<CodeView\n    code={plain_code.to_string()}\n    language={CodeLanguage::Plain}\n    title={\"readme.txt\".to_string()}\n/>".to_string()} language={"rust".to_string()} />
            </Section>

            // ── Without Line Numbers ──
            <Section title="Without Line Numbers">
                <Demo title="Hidden Line Numbers">
                    <div style="width: 100%; max-width: 640px;">
                        <CodeView code={rust_code.to_string()} language={CodeLanguage::Rust} show_line_numbers={false} title={"app.rs".to_string()} />
                    </div>
                </Demo>
                <CodeBlock code={"<CodeView\n    code={rust_code.to_string()}\n    language={CodeLanguage::Rust}\n    show_line_numbers={false}\n    title={\"app.rs\".to_string()}\n/>".to_string()} language={"rust".to_string()} />
            </Section>

            // ── Without Copy Button ──
            <Section title="Without Copy Button">
                <Demo title="Hidden Copy Button">
                    <div style="width: 100%; max-width: 640px;">
                        <CodeView code={rust_code.to_string()} language={CodeLanguage::Rust} show_copy={false} title={"app.rs".to_string()} />
                    </div>
                </Demo>
                <CodeBlock code={"<CodeView\n    code={rust_code.to_string()}\n    language={CodeLanguage::Rust}\n    show_copy={false}\n    title={\"app.rs\".to_string()}\n/>".to_string()} language={"rust".to_string()} />
            </Section>

            // ── Custom Title ──
            <Section title="Custom Title">
                <Demo title="Custom Header Title">
                    <div style="width: 100%; max-width: 640px;">
                        <CodeView code={"fn main() {\n    println!(\"Hello, Material RS!\");\n}".to_string()} language={CodeLanguage::Rust} title={"src/main.rs".to_string()} />
                    </div>
                </Demo>
                <CodeBlock code={"// The title prop appears in the header bar above the code\n<CodeView\n    code={rust_fn.to_string()}\n    language={CodeLanguage::Rust}\n    title={\"src/main.rs\".to_string()}\n/>".to_string()} language={"rust".to_string()} />
            </Section>

            // ── Props Table ──
            <Section title="Props">
                <PropTable rows={code_view_props} />
            </Section>
        </>
    }
}
