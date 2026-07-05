use yew::prelude::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo, ColorSwatch};

#[function_component]
pub fn ThemeTokensPage() -> Html {
    let theme = use_context::<material_rs::theme::Theme>().expect("Theme context required");

    let color_props = vec![
        PropRow { name: "primary".into(), r#type: "String".into(), default_value: "\"#6750A4\"".into(), description: "The primary brand color used for key components like FABs, buttons, and active states.".into() },
        PropRow { name: "on_primary".into(), r#type: "String".into(), default_value: "\"#FFFFFF\"".into(), description: "Text/icon color used on top of the primary color.".into() },
        PropRow { name: "primary_container".into(), r#type: "String".into(), default_value: "\"#EADDFF\"".into(), description: "A less prominent variant of primary for containers and subtle backgrounds.".into() },
        PropRow { name: "on_primary_container".into(), r#type: "String".into(), default_value: "\"#21005D\"".into(), description: "Text/icon color used on top of primary_container.".into() },
        PropRow { name: "secondary".into(), r#type: "String".into(), default_value: "\"#625B71\"".into(), description: "Used for less prominent components that complement primary actions.".into() },
        PropRow { name: "on_secondary".into(), r#type: "String".into(), default_value: "\"#FFFFFF\"".into(), description: "Text/icon color used on top of the secondary color.".into() },
        PropRow { name: "secondary_container".into(), r#type: "String".into(), default_value: "\"#E8DEF8\"".into(), description: "A less prominent variant of secondary for containers.".into() },
        PropRow { name: "on_secondary_container".into(), r#type: "String".into(), default_value: "\"#1D192B\"".into(), description: "Text/icon color used on top of secondary_container.".into() },
        PropRow { name: "tertiary".into(), r#type: "String".into(), default_value: "\"#7D5260\"".into(), description: "Used for contrasting accents and additional emphasis.".into() },
        PropRow { name: "error".into(), r#type: "String".into(), default_value: "\"#B3261E\"".into(), description: "Used for error states, destructive actions, and validation messages.".into() },
        PropRow { name: "surface".into(), r#type: "String".into(), default_value: "\"#FFFBFE\"".into(), description: "The default background color for screens and large containers.".into() },
        PropRow { name: "on_surface".into(), r#type: "String".into(), default_value: "\"#1C1B1F\"".into(), description: "Text/icon color used on top of the surface color.".into() },
        PropRow { name: "surface_variant".into(), r#type: "String".into(), default_value: "\"#E7E0EC\"".into(), description: "A subtle variation of surface used for dividers, thin borders, and tinted backgrounds.".into() },
        PropRow { name: "outline".into(), r#type: "String".into(), default_value: "\"#79747E\"".into(), description: "Used for borders, dividers, and emphasis on less prominent elements.".into() },
        PropRow { name: "outline_variant".into(), r#type: "String".into(), default_value: "\"#CAC4D0\"".into(), description: "A lighter variant of outline for subtle borders and dividers.".into() },
    ];

    let typography_props = vec![
        PropRow { name: "display_large".into(), r#type: "TypeRole".into(), default_value: "57px / 64px".into(), description: "57px font-size, 64px line-height, weight 400. Used for the largest display text.".into() },
        PropRow { name: "display_medium".into(), r#type: "TypeRole".into(), default_value: "45px / 52px".into(), description: "45px font-size, 52px line-height, weight 400.".into() },
        PropRow { name: "display_small".into(), r#type: "TypeRole".into(), default_value: "36px / 44px".into(), description: "36px font-size, 44px line-height, weight 400.".into() },
        PropRow { name: "headline_large".into(), r#type: "TypeRole".into(), default_value: "32px / 40px".into(), description: "32px font-size, 40px line-height, weight 400. Section headlines.".into() },
        PropRow { name: "headline_medium".into(), r#type: "TypeRole".into(), default_value: "28px / 36px".into(), description: "28px font-size, 36px line-height, weight 400.".into() },
        PropRow { name: "headline_small".into(), r#type: "TypeRole".into(), default_value: "24px / 32px".into(), description: "24px font-size, 32px line-height, weight 400.".into() },
        PropRow { name: "title_large".into(), r#type: "TypeRole".into(), default_value: "22px / 28px".into(), description: "22px font-size, 28px line-height, weight 400. AppBar titles, large section headers.".into() },
        PropRow { name: "title_medium".into(), r#type: "TypeRole".into(), default_value: "16px / 24px".into(), description: "16px font-size, 24px line-height, weight 500. List item titles, tab labels.".into() },
        PropRow { name: "title_small".into(), r#type: "TypeRole".into(), default_value: "14px / 20px".into(), description: "14px font-size, 20px line-height, weight 500.".into() },
        PropRow { name: "body_large".into(), r#type: "TypeRole".into(), default_value: "16px / 24px".into(), description: "16px font-size, 24px line-height, weight 400. Main body text.".into() },
        PropRow { name: "body_medium".into(), r#type: "TypeRole".into(), default_value: "14px / 20px".into(), description: "14px font-size, 20px line-height, weight 400. Default body text.".into() },
        PropRow { name: "body_small".into(), r#type: "TypeRole".into(), default_value: "12px / 16px".into(), description: "12px font-size, 16px line-height, weight 400. Captions, helper text.".into() },
        PropRow { name: "label_large".into(), r#type: "TypeRole".into(), default_value: "14px / 20px".into(), description: "14px font-size, 20px line-height, weight 500. Button labels, chip text.".into() },
        PropRow { name: "label_medium".into(), r#type: "TypeRole".into(), default_value: "12px / 16px".into(), description: "12px font-size, 16px line-height, weight 500. Badge labels, navigation labels.".into() },
        PropRow { name: "label_small".into(), r#type: "TypeRole".into(), default_value: "11px / 16px".into(), description: "11px font-size, 16px line-height, weight 500. Overlines, smallest labels.".into() },
    ];

    let shape_props = vec![
        PropRow { name: "none".into(), r#type: "String".into(), default_value: "\"0px\"".into(), description: "No rounding. Sharp corners.".into() },
        PropRow { name: "extra_small".into(), r#type: "String".into(), default_value: "\"4px\"".into(), description: "4px radius. Used for dividers, progress bars.".into() },
        PropRow { name: "small".into(), r#type: "String".into(), default_value: "\"8px\"".into(), description: "8px radius. Used for switches, checkboxes.".into() },
        PropRow { name: "medium".into(), r#type: "String".into(), default_value: "\"12px\"".into(), description: "12px radius. Used for text fields, menus, tooltips.".into() },
        PropRow { name: "large".into(), r#type: "String".into(), default_value: "\"16px\"".into(), description: "16px radius. Used for navigation bars, search bars.".into() },
        PropRow { name: "extra_large".into(), r#type: "String".into(), default_value: "\"28px\"".into(), description: "28px radius. Used for cards, dialogs, navigation drawers.".into() },
        PropRow { name: "full".into(), r#type: "String".into(), default_value: "\"9999px\"".into(), description: "Fully rounded. Used for buttons, FABs, chips, pills.".into() },
    ];

    html! {
        <>
            <Section title="Theme Tokens">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Material Design 3 defines a complete set of design tokens for color, typography, and shape. \
                      These tokens ensure visual consistency across all components and are accessible through \
                      the Theme context provided at the root of your application."}
                </p>
            </Section>

            // ── Color Tokens ──
            <Section title="Color Tokens">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 16px;">
                    {"The color scheme defines all MD3 color roles. Each role maps to a specific semantic purpose \
                      in the interface. Access these via theme.colors.*."}
                </p>

                <Demo title="Primary Colors">
                    <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 16px;">
                        <ColorSwatch label="primary" color={theme.colors.primary.clone()} />
                        <ColorSwatch label="on-primary" color={theme.colors.on_primary.clone()} />
                        <ColorSwatch label="primary-container" color={theme.colors.primary_container.clone()} />
                        <ColorSwatch label="on-primary-container" color={theme.colors.on_primary_container.clone()} />
                    </div>
                </Demo>
                <CodeBlock code={"// Access primary color tokens\nlet primary = theme.colors.primary;              // \"#6750A4\"\nlet on_primary = theme.colors.on_primary;          // \"#FFFFFF\"\nlet primary_container = theme.colors.primary_container; // \"#EADDFF\"\nlet on_primary_container = theme.colors.on_primary_container; // \"#21005D\"".to_string()} language={"rust".to_string()} />

                <Demo title="Secondary Colors">
                    <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 16px;">
                        <ColorSwatch label="secondary" color={theme.colors.secondary.clone()} />
                        <ColorSwatch label="on-secondary" color={theme.colors.on_secondary.clone()} />
                        <ColorSwatch label="secondary-container" color={theme.colors.secondary_container.clone()} />
                        <ColorSwatch label="on-secondary-container" color={theme.colors.on_secondary_container.clone()} />
                    </div>
                </Demo>
                <CodeBlock code={"let secondary = theme.colors.secondary;             // \"#625B71\"\nlet on_secondary = theme.colors.on_secondary;       // \"#FFFFFF\"\nlet secondary_container = theme.colors.secondary_container; // \"#E8DEF8\"\nlet on_secondary_container = theme.colors.on_secondary_container; // \"#1D192B\"".to_string()} language={"rust".to_string()} />

                <Demo title="Tertiary & Error Colors">
                    <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 16px;">
                        <ColorSwatch label="tertiary" color={theme.colors.tertiary.clone()} />
                        <ColorSwatch label="error" color={theme.colors.error.clone()} />
                        <ColorSwatch label="on-error" color={theme.colors.on_error.clone()} />
                        <ColorSwatch label="error-container" color={theme.colors.error_container.clone()} />
                    </div>
                </Demo>
                <CodeBlock code={"let tertiary = theme.colors.tertiary;               // \"#7D5260\"\nlet error = theme.colors.error;                     // \"#B3261E\"\nlet on_error = theme.colors.on_error;               // \"#FFFFFF\"\nlet error_container = theme.colors.error_container;  // \"#F9DEDC\"".to_string()} language={"rust".to_string()} />

                <Demo title="Surface Colors">
                    <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 16px;">
                        <ColorSwatch label="surface" color={theme.colors.surface.clone()} />
                        <ColorSwatch label="on-surface" color={theme.colors.on_surface.clone()} />
                        <ColorSwatch label="surface-variant" color={theme.colors.surface_variant.clone()} />
                        <ColorSwatch label="on-surface-variant" color={theme.colors.on_surface_variant.clone()} />
                    </div>
                </Demo>
                <CodeBlock code={"let surface = theme.colors.surface;                 // \"#FFFBFE\"\nlet on_surface = theme.colors.on_surface;           // \"#1C1B1F\"\nlet surface_variant = theme.colors.surface_variant; // \"#E7E0EC\"\nlet on_surface_variant = theme.colors.on_surface_variant; // \"#49454F\"".to_string()} language={"rust".to_string()} />

                <Demo title="Surface Container Levels">
                    <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 16px;">
                        <ColorSwatch label="container-lowest" color={theme.colors.surface_container_lowest.clone()} />
                        <ColorSwatch label="container-low" color={theme.colors.surface_container_low.clone()} />
                        <ColorSwatch label="container" color={theme.colors.surface_container.clone()} />
                        <ColorSwatch label="container-high" color={theme.colors.surface_container_high.clone()} />
                        <ColorSwatch label="container-highest" color={theme.colors.surface_container_highest.clone()} />
                    </div>
                </Demo>
                <CodeBlock code={"// Surface container levels (lowest → highest)\nlet c0 = theme.surface_container(0); // surface_container_lowest\nlet c1 = theme.surface_container(1); // surface_container_low\nlet c2 = theme.surface_container(2); // surface_container\nlet c3 = theme.surface_container(3); // surface_container_high\nlet c4 = theme.surface_container(4); // surface_container_highest".to_string()} language={"rust".to_string()} />

                <Demo title="Outline Colors">
                    <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 16px;">
                        <ColorSwatch label="outline" color={theme.colors.outline.clone()} />
                        <ColorSwatch label="outline-variant" color={theme.colors.outline_variant.clone()} />
                    </div>
                </Demo>
                <CodeBlock code={"let outline = theme.colors.outline;                 // \"#79747E\"\nlet outline_variant = theme.colors.outline_variant; // \"#CAC4D0\"".to_string()} language={"rust".to_string()} />

                <Demo title="Inverse & Scrim Colors">
                    <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 16px;">
                        <ColorSwatch label="inverse-surface" color={theme.colors.inverse_surface.clone()} />
                        <ColorSwatch label="inverse-on-surface" color={theme.colors.inverse_on_surface.clone()} />
                        <ColorSwatch label="inverse-primary" color={theme.colors.inverse_primary.clone()} />
                        <ColorSwatch label="scrim" color={theme.colors.scrim.clone()} />
                    </div>
                </Demo>
                <CodeBlock code={"let inverse_surface = theme.colors.inverse_surface;   // \"#313033\"\nlet inverse_on_surface = theme.colors.inverse_on_surface; // \"#F4EFF4\"\nlet inverse_primary = theme.colors.inverse_primary;  // \"#D0BCFF\"\nlet scrim = theme.colors.scrim;                       // \"#000000\"".to_string()} language={"rust".to_string()} />

                <PropTable rows={color_props} />
            </Section>

            // ── Typography Tokens ──
            <Section title="Typography Tokens">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 16px;">
                    {"The MD3 type scale provides 15 roles organized into display, headline, title, body, and label \
                      categories. Each role specifies font-size, line-height, font-weight, tracking, and text-transform. \
                      Access via theme.typography.*."}
                </p>

                <Demo title="Display Scale">
                    <div style="display: flex; flex-direction: column; gap: 8px;">
                        <div style={format!("font-size: {}px; line-height: {}px; font-weight: {}; letter-spacing: {}px; color: {};",
                            theme.typography.display_large.font_size,
                            theme.typography.display_large.line_height,
                            theme.typography.display_large.weight.as_str(),
                            theme.typography.display_large.tracking,
                            theme.colors.on_surface,
                        )}>{"Display Large (57px)"}</div>
                        <div style={format!("font-size: {}px; line-height: {}px; font-weight: {}; letter-spacing: {}px; color: {};",
                            theme.typography.display_medium.font_size,
                            theme.typography.display_medium.line_height,
                            theme.typography.display_medium.weight.as_str(),
                            theme.typography.display_medium.tracking,
                            theme.colors.on_surface,
                        )}>{"Display Medium (45px)"}</div>
                        <div style={format!("font-size: {}px; line-height: {}px; font-weight: {}; letter-spacing: {}px; color: {};",
                            theme.typography.display_small.font_size,
                            theme.typography.display_small.line_height,
                            theme.typography.display_small.weight.as_str(),
                            theme.typography.display_small.tracking,
                            theme.colors.on_surface,
                        )}>{"Display Small (36px)"}</div>
                    </div>
                </Demo>
                <CodeBlock code={"// Display scale — largest text for hero sections\nlet dl = &theme.typography.display_large;\n// font-size: 57px, line-height: 64px, weight: 400, tracking: -0.25\n\nlet dm = &theme.typography.display_medium;\n// font-size: 45px, line-height: 52px, weight: 400\n\nlet ds = &theme.typography.display_small;\n// font-size: 36px, line-height: 44px, weight: 400\n\n// Generate CSS for any role:\nlet css = dl.to_css();\n// \"font-size: 57px; line-height: 64px; font-weight: 400; letter-spacing: -0.25px;\"".to_string()} language={"rust".to_string()} />

                <Demo title="Headline Scale">
                    <div style="display: flex; flex-direction: column; gap: 8px;">
                        <div style={format!("font-size: {}px; line-height: {}px; font-weight: {}; color: {};",
                            theme.typography.headline_large.font_size,
                            theme.typography.headline_large.line_height,
                            theme.typography.headline_large.weight.as_str(),
                            theme.colors.on_surface,
                        )}>{"Headline Large (32px)"}</div>
                        <div style={format!("font-size: {}px; line-height: {}px; font-weight: {}; color: {};",
                            theme.typography.headline_medium.font_size,
                            theme.typography.headline_medium.line_height,
                            theme.typography.headline_medium.weight.as_str(),
                            theme.colors.on_surface,
                        )}>{"Headline Medium (28px)"}</div>
                        <div style={format!("font-size: {}px; line-height: {}px; font-weight: {}; color: {};",
                            theme.typography.headline_small.font_size,
                            theme.typography.headline_small.line_height,
                            theme.typography.headline_small.weight.as_str(),
                            theme.colors.on_surface,
                        )}>{"Headline Small (24px)"}</div>
                    </div>
                </Demo>
                <CodeBlock code={"// Headline scale — section headings\nlet hl = &theme.typography.headline_large;  // 32px / 40px, weight 400\nlet hm = &theme.typography.headline_medium; // 28px / 36px, weight 400\nlet hs = &theme.typography.headline_small;  // 24px / 32px, weight 400".to_string()} language={"rust".to_string()} />

                <Demo title="Title Scale">
                    <div style="display: flex; flex-direction: column; gap: 8px;">
                        <div style={format!("font-size: {}px; line-height: {}px; font-weight: {}; color: {};",
                            theme.typography.title_large.font_size,
                            theme.typography.title_large.line_height,
                            theme.typography.title_large.weight.as_str(),
                            theme.colors.on_surface,
                        )}>{"Title Large (22px)"}</div>
                        <div style={format!("font-size: {}px; line-height: {}px; font-weight: {}; color: {};",
                            theme.typography.title_medium.font_size,
                            theme.typography.title_medium.line_height,
                            theme.typography.title_medium.weight.as_str(),
                            theme.colors.on_surface,
                        )}>{"Title Medium (16px, Medium)"}</div>
                        <div style={format!("font-size: {}px; line-height: {}px; font-weight: {}; color: {};",
                            theme.typography.title_small.font_size,
                            theme.typography.title_small.line_height,
                            theme.typography.title_small.weight.as_str(),
                            theme.colors.on_surface,
                        )}>{"Title Small (14px, Medium)"}</div>
                    </div>
                </Demo>
                <CodeBlock code={"// Title scale — AppBar titles, list item headers\nlet tl = &theme.typography.title_large;  // 22px / 28px, weight 400\nlet tm = &theme.typography.title_medium; // 16px / 24px, weight 500\nlet ts = &theme.typography.title_small;  // 14px / 20px, weight 500".to_string()} language={"rust".to_string()} />

                <Demo title="Body Scale">
                    <div style="display: flex; flex-direction: column; gap: 8px;">
                        <div style={format!("font-size: {}px; line-height: {}px; font-weight: {}; color: {};",
                            theme.typography.body_large.font_size,
                            theme.typography.body_large.line_height,
                            theme.typography.body_large.weight.as_str(),
                            theme.colors.on_surface,
                        )}>{"Body Large (16px) — main paragraph text"}</div>
                        <div style={format!("font-size: {}px; line-height: {}px; font-weight: {}; color: {};",
                            theme.typography.body_medium.font_size,
                            theme.typography.body_medium.line_height,
                            theme.typography.body_medium.weight.as_str(),
                            theme.colors.on_surface,
                        )}>{"Body Medium (14px) — default body text"}</div>
                        <div style={format!("font-size: {}px; line-height: {}px; font-weight: {}; color: {};",
                            theme.typography.body_small.font_size,
                            theme.typography.body_small.line_height,
                            theme.typography.body_small.weight.as_str(),
                            theme.colors.on_surface,
                        )}>{"Body Small (12px) — captions and helper text"}</div>
                    </div>
                </Demo>
                <CodeBlock code={"// Body scale — paragraph and reading text\nlet bl = &theme.typography.body_large;  // 16px / 24px, weight 400\nlet bm = &theme.typography.body_medium; // 14px / 20px, weight 400\nlet bs = &theme.typography.body_small;  // 12px / 16px, weight 400".to_string()} language={"rust".to_string()} />

                <Demo title="Label Scale">
                    <div style="display: flex; flex-direction: column; gap: 8px;">
                        <div style={format!("font-size: {}px; line-height: {}px; font-weight: {}; color: {}; text-transform: uppercase; letter-spacing: {}px;",
                            theme.typography.label_large.font_size,
                            theme.typography.label_large.line_height,
                            theme.typography.label_large.weight.as_str(),
                            theme.colors.on_surface,
                            theme.typography.label_large.tracking,
                        )}>{"LABEL LARGE (14px, Medium)"}</div>
                        <div style={format!("font-size: {}px; line-height: {}px; font-weight: {}; color: {}; letter-spacing: {}px;",
                            theme.typography.label_medium.font_size,
                            theme.typography.label_medium.line_height,
                            theme.typography.label_medium.weight.as_str(),
                            theme.colors.on_surface,
                            theme.typography.label_medium.tracking,
                        )}>{"Label Medium (12px, Medium)"}</div>
                        <div style={format!("font-size: {}px; line-height: {}px; font-weight: {}; color: {}; letter-spacing: {}px;",
                            theme.typography.label_small.font_size,
                            theme.typography.label_small.line_height,
                            theme.typography.label_small.weight.as_str(),
                            theme.colors.on_surface,
                            theme.typography.label_small.tracking,
                        )}>{"Label Small (11px, Medium)"}</div>
                    </div>
                </Demo>
                <CodeBlock code={"// Label scale — buttons, chips, badges, navigation\nlet ll = &theme.typography.label_large;  // 14px / 20px, weight 500\nlet lm = &theme.typography.label_medium; // 12px / 16px, weight 500\nlet ls = &theme.typography.label_small;  // 11px / 16px, weight 500\n\n// Each TypeRole has:\n// .font_size  -> f64\n// .line_height -> f64\n// .weight     -> FontWeight (Regular=400, Medium=500, SemiBold=600, Bold=700)\n// .tracking   -> f64 (letter-spacing in px)\n// .text_case  -> TextCase (None, Upper, Lower, Capitalize)\n// .to_css()   -> generates a CSS property string".to_string()} language={"rust".to_string()} />

                <PropTable rows={typography_props} />
            </Section>

            // ── Shape Tokens ──
            <Section title="Shape Tokens">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 16px;">
                    {"Shape tokens define border-radius values from sharp corners (none) to fully rounded (full/pill). \
                      Each token maps to a specific MD3 size category. Access via theme.shapes.*."}
                </p>

                <Demo title="Shape Scale">
                    <div style="display: flex; gap: 16px; align-items: flex-end; flex-wrap: wrap;">
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <div style={format!("width: 80px; height: 80px; background-color: {}; border-radius: {}; border: 1px solid {};",
                                theme.colors.primary_container, theme.shapes.none, theme.colors.outline_variant,
                            )} />
                            <span style={format!("font-size: 11px; color: {};", theme.colors.on_surface_variant)}>{"none (0px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <div style={format!("width: 80px; height: 80px; background-color: {}; border-radius: {}; border: 1px solid {};",
                                theme.colors.primary_container, theme.shapes.extra_small, theme.colors.outline_variant,
                            )} />
                            <span style={format!("font-size: 11px; color: {};", theme.colors.on_surface_variant)}>{"extra_small (4px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <div style={format!("width: 80px; height: 80px; background-color: {}; border-radius: {}; border: 1px solid {};",
                                theme.colors.primary_container, theme.shapes.small, theme.colors.outline_variant,
                            )} />
                            <span style={format!("font-size: 11px; color: {};", theme.colors.on_surface_variant)}>{"small (8px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <div style={format!("width: 80px; height: 80px; background-color: {}; border-radius: {}; border: 1px solid {};",
                                theme.colors.primary_container, theme.shapes.medium, theme.colors.outline_variant,
                            )} />
                            <span style={format!("font-size: 11px; color: {};", theme.colors.on_surface_variant)}>{"medium (12px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <div style={format!("width: 80px; height: 80px; background-color: {}; border-radius: {}; border: 1px solid {};",
                                theme.colors.primary_container, theme.shapes.large, theme.colors.outline_variant,
                            )} />
                            <span style={format!("font-size: 11px; color: {};", theme.colors.on_surface_variant)}>{"large (16px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <div style={format!("width: 80px; height: 80px; background-color: {}; border-radius: {}; border: 1px solid {};",
                                theme.colors.primary_container, theme.shapes.extra_large, theme.colors.outline_variant,
                            )} />
                            <span style={format!("font-size: 11px; color: {};", theme.colors.on_surface_variant)}>{"extra_large (28px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <div style={format!("width: 80px; height: 80px; background-color: {}; border-radius: {}; border: 1px solid {};",
                                theme.colors.primary_container, theme.shapes.full, theme.colors.outline_variant,
                            )} />
                            <span style={format!("font-size: 11px; color: {};", theme.colors.on_surface_variant)}>{"full (9999px)"}</span>
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={"// Shape tokens — border-radius values from sharp to fully rounded\nlet none = &theme.shapes.none;           // \"0px\"\nlet xs = &theme.shapes.extra_small;      // \"4px\"\nlet sm = &theme.shapes.small;            // \"8px\"\nlet md = &theme.shapes.medium;           // \"12px\"\nlet lg = &theme.shapes.large;            // \"16px\"\nlet xl = &theme.shapes.extra_large;      // \"28px\"\nlet full = &theme.shapes.full;           // \"9999px\"\n\n// Generate CSS:\nlet css = theme.shapes.radius_css(md); // \"border-radius: 12px;\"\n\n// Component-specific defaults:\nuse material_rs::theme::shape::ComponentShapes;\nlet card_radius = ComponentShapes::for_component(\"card\"); // \"28px\"\nlet btn_radius = ComponentShapes::for_component(\"button\"); // \"9999px\"".to_string()} language={"rust".to_string()} />

                <PropTable rows={shape_props} />
            </Section>
        </>
    }
}
