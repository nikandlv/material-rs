//! Material Design 3 Code View Component
//!
//! A syntax-highlighted code block with line numbers, copy-to-clipboard,
//! and MD3-themed styling. Supports Rust, JavaScript, CSS, HTML, and plain text.

pub mod highlight;

use stylist::yew::{use_style, Global};
use yew::prelude::*;

use crate::components::button::{Button, ButtonVariant};
use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;
use highlight::{highlight, Language};

#[derive(Properties, PartialEq)]
pub struct CodeViewProps {
    /// Source code to display.
    pub code: String,

    /// Programming language for syntax highlighting.
    #[prop_or_default]
    pub language: Language,

    /// Whether to show line numbers.
    #[prop_or(true)]
    pub show_line_numbers: bool,

    /// Whether to show the copy button.
    #[prop_or(true)]
    pub show_copy: bool,

    /// Optional title/filename displayed in the header.
    #[prop_or_default]
    pub title: String,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn CodeView(props: &CodeViewProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let copied = use_state(|| false);

    let on_copy = {
        let code = props.code.clone();
        let copied = copied.clone();
        Callback::from(move |_: MouseEvent| {
            let code = code.clone();
            let copied = copied.clone();
            if let Some(window) = web_sys::window() {
                let nav = window.navigator();
                let clipboard = nav.clipboard();
                let code_clone = code.clone();
                let copied_clone = copied.clone();
                let _promise = clipboard.write_text(&code_clone);
                copied_clone.set(true);
                gloo_timers::callback::Timeout::new(2000, move || {
                    copied_clone.set(false);
                })
                .forget();
            }
        })
    };

    let tokens = highlight(&props.code, props.language);
    // Count lines matching the highlight function's output (source.lines())
    let line_count = if props.code.is_empty() { 1 } else { props.code.lines().count() };

    let container_style = use_style!(
        r#"
        display: flex;
        flex-direction: column;
        border-radius: 12px;
        overflow: hidden;
        border: 1px solid ${border};
        background-color: ${bg};
        font-family: 'JetBrains Mono', 'Fira Code', 'SF Mono', 'Consolas', monospace;
        "#,
        border = theme.colors.outline_variant,
        bg = theme.colors.surface_container,
    );

    let header_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 8px 16px;
        border-bottom: 1px solid ${border};
        background-color: ${bg};
        min-height: 36px;
        "#,
        border = theme.colors.outline_variant,
        bg = theme.colors.surface_container_high,
    );

    let code_container_style = use_style!(
        r#"
        display: flex;
        overflow-x: auto;
        font-size: 13px;
        line-height: 1.6;
        tab-size: 4;
        "#,
    );

    let gutter_style = use_style!(
        r#"
        display: flex;
        flex-direction: column;
        padding: 12px 0;
        text-align: end;
        user-select: none;
        -webkit-user-select: none;
        border-inline-end: 1px solid ${border};
        background-color: ${bg};
        "#,
        border = theme.colors.outline_variant,
        bg = theme.colors.surface_container_low,
    );

    let line_number_style = use_style!(
        r#"
        padding: 0 12px 0 12px;
        color: ${color};
        font-size: 13px;
        line-height: 1.6;
        min-width: 32px;
        text-align: end;
        "#,
        color = theme.colors.on_surface_variant,
    );

    let code_style = use_style!(
        r#"
        flex: 1;
        padding: 12px 16px;
        overflow-x: auto;
        white-space: pre;
        color: ${color};
        font-size: 13px;
        line-height: 1.6;
        margin: 0;
        "#,
        color = theme.colors.on_surface,
    );

    let highlighted_html = tokens.iter().map(|token| {
        if token.class.is_empty() {
            html! { <>{ &token.text }</> }
        } else {
            html! { <span class={token.class.clone()}>{ &token.text }</span> }
        }
    });

    let copy_label = if *copied { "Copied!" } else { "Copy" };
    let copy_icon = if *copied { "check" } else { "content_copy" };

    // Highlight CSS via Global (NOT use_style! which scopes selectors)
    let hl_css = format!(
        r#"
        .hl-comment {{ color: #6a737d; font-style: italic; }}
        .hl-string {{ color: #a5d6ff; }}
        .hl-keyword {{ color: #ff7b72; font-weight: 500; }}
        .hl-type {{ color: #79c0ff; }}
        .hl-number {{ color: #79c0ff; }}
        .hl-function {{ color: #d2a8ff; }}
        .hl-attr {{ color: #7ee787; }}
        .hl-punctuation {{ color: {on_surface}; }}
        "#,
        on_surface = theme.colors.on_surface,
    );

    let component_override = theme.component_style("CodeView").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div
            class={yew::classes![container_style.get_class_name().to_string(), &props.class, &component_override]}
            id={props.id.clone()}
            dir="ltr"
        >
            <Global css={hl_css.parse::<stylist::StyleSource>().expect("invalid css")} />

            <div class={header_style.get_class_name().to_string()}>
                if !props.title.is_empty() {
                    <span class="code-view-title">
                        { &props.title }
                    </span>
                } else {
                    <span />
                }

                if props.show_copy {
                    <Button
                        variant={ButtonVariant::Text}
                        icon={copy_icon}
                        label={copy_label}
                        onclick={on_copy}
                    />
                }
            </div>

            <div class={code_container_style.get_class_name().to_string()}>
                if props.show_line_numbers {
                    <div class={gutter_style.get_class_name().to_string()}>
                        { for (1..=line_count).map(|n| {
                            html! {
                                <div class={line_number_style.get_class_name().to_string()}>
                                    { n }
                                </div>
                            }
                        })}
                    </div>
                }

                <pre class={code_style.get_class_name().to_string()}>
                    <code>{ for highlighted_html }</code>
                </pre>
            </div>
        </div>
    }
}
