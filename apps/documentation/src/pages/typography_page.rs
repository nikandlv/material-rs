use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn TypographyPage() -> Html {
    let typography_props = vec![
        PropRow { name: "tag".into(), r#type: "String".into(), default_value: "\"p\"".into(), description: "HTML tag to render (h1-h6, p, span, div, label, a). Defaults to \"p\".".into() },
        PropRow { name: "variant".into(), r#type: "TypographyVariant".into(), default_value: "BodyMedium".into(), description: "MD3 type role variant that controls font family, size, weight, line height, and letter spacing.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Text content rendered inside the element.".into() },
        PropRow { name: "style".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Custom inline CSS style override.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
    ];

    html! {
        <>
            <Section title="Typography">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Typography renders text using the official Material Design 3 type scale roles \
                      (Display, Headline, Title, Body, Label) onto customizable HTML tags. Each variant \
                      maps to a specific font family, size, weight, line height, and letter spacing from the theme."}
                </p>

                // ── All Variants ──
                <Demo title="Typography Variants">
                    <div style="display: flex; flex-direction: column; gap: 16px; width: 100%;">
                        <Typography tag="h1" variant={TypographyVariant::DisplayLarge}>{"Display Large"}</Typography>
                        <Typography tag="h2" variant={TypographyVariant::DisplayMedium}>{"Display Medium"}</Typography>
                        <Typography tag="h3" variant={TypographyVariant::DisplaySmall}>{"Display Small"}</Typography>

                        <Typography tag="h1" variant={TypographyVariant::HeadlineLarge}>{"Headline Large"}</Typography>
                        <Typography tag="h2" variant={TypographyVariant::HeadlineMedium}>{"Headline Medium"}</Typography>
                        <Typography tag="h3" variant={TypographyVariant::HeadlineSmall}>{"Headline Small"}</Typography>

                        <Typography tag="h4" variant={TypographyVariant::TitleLarge}>{"Title Large"}</Typography>
                        <Typography tag="h5" variant={TypographyVariant::TitleMedium}>{"Title Medium"}</Typography>
                        <Typography tag="h6" variant={TypographyVariant::TitleSmall}>{"Title Small"}</Typography>

                        <Typography tag="p" variant={TypographyVariant::BodyLarge}>{"Body Large: Used for longer-form text, highly readable with Roboto at the default weight."}</Typography>
                        <Typography tag="p" variant={TypographyVariant::BodyMedium}>{"Body Medium: The default variant. Standard paragraph text for most UI content."}</Typography>
                        <Typography tag="p" variant={TypographyVariant::BodySmall}>{"Body Small: Compact text for secondary content and captions."}</Typography>

                        <Typography tag="span" variant={TypographyVariant::LabelLarge}>{"LABEL LARGE"}</Typography>
                        <Typography tag="span" variant={TypographyVariant::LabelMedium}>{"LABEL MEDIUM"}</Typography>
                        <Typography tag="span" variant={TypographyVariant::LabelSmall}>{"LABEL SMALL"}</Typography>
                    </div>
                </Demo>
                <CodeBlock code={"// Display variants\n<Typography tag=\"h1\" variant={TypographyVariant::DisplayLarge}>{\"Display Large\"}</Typography>\n<Typography tag=\"h2\" variant={TypographyVariant::DisplayMedium}>{\"Display Medium\"}</Typography>\n<Typography tag=\"h3\" variant={TypographyVariant::DisplaySmall}>{\"Display Small\"}</Typography>\n\n// Headline variants\n<Typography tag=\"h1\" variant={TypographyVariant::HeadlineLarge}>{\"Headline Large\"}</Typography>\n<Typography tag=\"h2\" variant={TypographyVariant::HeadlineMedium}>{\"Headline Medium\"}</Typography>\n<Typography tag=\"h3\" variant={TypographyVariant::HeadlineSmall}>{\"Headline Small\"}</Typography>\n\n// Title variants\n<Typography tag=\"h4\" variant={TypographyVariant::TitleLarge}>{\"Title Large\"}</Typography>\n<Typography tag=\"h5\" variant={TypographyVariant::TitleMedium}>{\"Title Medium\"}</Typography>\n<Typography tag=\"h6\" variant={TypographyVariant::TitleSmall}>{\"Title Small\"}</Typography>\n\n// Body variants\n<Typography tag=\"p\" variant={TypographyVariant::BodyLarge}>{\"Body Large\"}</Typography>\n<Typography tag=\"p\" variant={TypographyVariant::BodyMedium}>{\"Body Medium\"}</Typography>\n<Typography tag=\"p\" variant={TypographyVariant::BodySmall}>{\"Body Small\"}</Typography>\n\n// Label variants\n<Typography tag=\"span\" variant={TypographyVariant::LabelLarge}>{\"LABEL LARGE\"}</Typography>\n<Typography tag=\"span\" variant={TypographyVariant::LabelMedium}>{\"LABEL MEDIUM\"}</Typography>\n<Typography tag=\"span\" variant={TypographyVariant::LabelSmall}>{\"LABEL SMALL\"}</Typography>".to_string()} language={"rust".to_string()} />

                // ── Custom Tags ──
                <Demo title="Custom HTML Tags">
                    <div style="display: flex; flex-direction: column; gap: 12px; width: 100%;">
                        <Typography tag="div" variant={TypographyVariant::BodyLarge}>{"Rendered as a div element"}</Typography>
                        <Typography tag="span" variant={TypographyVariant::BodyLarge}>{"Rendered as a span element"}</Typography>
                        <Typography tag="label" variant={TypographyVariant::BodyLarge}>{"Rendered as a label element"}</Typography>
                        <Typography tag="h6" variant={TypographyVariant::TitleLarge}>{"Rendered as an h6 element"}</Typography>
                    </div>
                </Demo>
                <CodeBlock code={"<Typography tag=\"div\" variant={TypographyVariant::BodyLarge}>{\"Rendered as a div\"}</Typography>\n<Typography tag=\"span\" variant={TypographyVariant::BodyLarge}>{\"Rendered as a span\"}</Typography>\n<Typography tag=\"label\" variant={TypographyVariant::BodyLarge}>{\"Rendered as a label\"}</Typography>\n<Typography tag=\"h6\" variant={TypographyVariant::TitleLarge}>{\"Rendered as an h6\"}</Typography>".to_string()} language={"rust".to_string()} />

                // ── Inline Styling ──
                <Demo title="Inline Styling">
                    <div style="display: flex; flex-direction: column; gap: 12px; width: 100%;">
                        <Typography tag="p" variant={TypographyVariant::BodyLarge} style="color: #6750A4; font-style: italic;">
                            {"Styled with custom color and italic text"}
                        </Typography>
                        <Typography tag="p" variant={TypographyVariant::BodyMedium} style="text-decoration: underline; letter-spacing: 2px;">
                            {"Underlined with increased letter spacing"}
                        </Typography>
                    </div>
                </Demo>
                <CodeBlock code={"<Typography tag=\"p\" variant={TypographyVariant::BodyLarge}\n    style=\"color: #6750A4; font-style: italic;\">\n    {\"Styled with custom color and italic text\"}\n</Typography>\n\n<Typography tag=\"p\" variant={TypographyVariant::BodyMedium}\n    style=\"text-decoration: underline; letter-spacing: 2px;\">\n    {\"Underlined with increased letter spacing\"}\n</Typography>".to_string()} language={"rust".to_string()} />

                <PropTable rows={typography_props} />
            </Section>
        </>
    }
}
