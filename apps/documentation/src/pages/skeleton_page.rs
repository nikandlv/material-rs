use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn SkeletonPage() -> Html {
    let skeleton_props = vec![
        PropRow { name: "shape".into(), r#type: "SkeletonShape".into(), default_value: "Text".into(), description: "Visual shape: Text (rectangle), Circle, or Rounded.".into() },
        PropRow { name: "width".into(), r#type: "String".into(), default_value: "\"100%\"".into(), description: "CSS width of the skeleton placeholder.".into() },
        PropRow { name: "height".into(), r#type: "String".into(), default_value: "\"16px\"".into(), description: "CSS height of the skeleton placeholder.".into() },
        PropRow { name: "border_radius".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Custom CSS border-radius override. Defaults are shape-dependent.".into() },
        PropRow { name: "animated".into(), r#type: "bool".into(), default_value: "true".into(), description: "Enables the shimmer animation. Set to false for a static placeholder.".into() },
    ];

    let skeleton_text_props = vec![
        PropRow { name: "lines".into(), r#type: "u32".into(), default_value: "3".into(), description: "Number of text lines to render.".into() },
        PropRow { name: "first_width".into(), r#type: "String".into(), default_value: "\"100%\"".into(), description: "CSS width of the first line.".into() },
        PropRow { name: "line_width".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "CSS width for subsequent lines. Defaults to 100%.".into() },
        PropRow { name: "line_height".into(), r#type: "String".into(), default_value: "\"14px\"".into(), description: "CSS height of each line.".into() },
        PropRow { name: "gap".into(), r#type: "String".into(), default_value: "\"8px\"".into(), description: "CSS gap between lines.".into() },
    ];

    let skeleton_card_props = vec![
        PropRow { name: "show_media".into(), r#type: "bool".into(), default_value: "true".into(), description: "Shows a media placeholder at the top of the card.".into() },
        PropRow { name: "media_height".into(), r#type: "String".into(), default_value: "\"200px\"".into(), description: "CSS height of the media placeholder.".into() },
        PropRow { name: "text_lines".into(), r#type: "u32".into(), default_value: "2".into(), description: "Number of text line placeholders below the media.".into() },
    ];

    html! {
        <>
            <Section title="Skeleton">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Skeleton loaders are placeholders shown while content is loading. \
                      They reduce perceived wait time by previewing the layout of the \
                      content that will eventually appear. Skeleton supports Text, Circle, \
                      and Rounded shapes, with optional shimmer animation."}
                </p>

                // ── Text Skeleton ──
                <Demo title="Text Skeleton">
                    <div style="width: 100%; max-width: 400px;">
                        <Skeleton shape={SkeletonShape::Text} width="100%" height="16px" />
                    </div>
                </Demo>
                <CodeBlock code={"<Skeleton shape={SkeletonShape::Text} width=\"100%\" height=\"16px\" />".to_string()} language={"rust".to_string()} />

                // ── Circle Skeleton ──
                <Demo title="Circle Skeleton">
                    <div style="display: flex; gap: 16px; align-items: center;">
                        <Skeleton shape={SkeletonShape::Circle} width="40px" height="40px" />
                        <Skeleton shape={SkeletonShape::Circle} width="56px" height="56px" />
                        <Skeleton shape={SkeletonShape::Circle} width="72px" height="72px" />
                    </div>
                </Demo>
                <CodeBlock code={"<Skeleton shape={SkeletonShape::Circle} width=\"40px\" height=\"40px\" />\n<Skeleton shape={SkeletonShape::Circle} width=\"56px\" height=\"56px\" />\n<Skeleton shape={SkeletonShape::Circle} width=\"72px\" height=\"72px\" />".to_string()} language={"rust".to_string()} />

                // ── Rounded Skeleton ──
                <Demo title="Rounded Skeleton">
                    <div style="display: flex; gap: 16px; align-items: center;">
                        <Skeleton shape={SkeletonShape::Rounded} width="120px" height="32px" />
                        <Skeleton shape={SkeletonShape::Rounded} width="160px" height="32px" />
                        <Skeleton shape={SkeletonShape::Rounded} width="200px" height="32px" />
                    </div>
                </Demo>
                <CodeBlock code={"<Skeleton shape={SkeletonShape::Rounded} width=\"120px\" height=\"32px\" />\n<Skeleton shape={SkeletonShape::Rounded} width=\"160px\" height=\"32px\" />\n<Skeleton shape={SkeletonShape::Rounded} width=\"200px\" height=\"32px\" />".to_string()} language={"rust".to_string()} />

                // ── Multi-Line Text ──
                <Demo title="Multi-Line Text">
                    <div style="width: 100%; max-width: 400px;">
                        <SkeletonText lines={4} first_width="80%" line_height="14px" gap="8px" />
                    </div>
                </Demo>
                <CodeBlock code={"<SkeletonText lines={4} first_width=\"80%\" line_height=\"14px\" gap=\"8px\" />".to_string()} language={"rust".to_string()} />

                // ── Card Skeleton ──
                <Demo title="Card Skeleton">
                    <div style="max-width: 320px;">
                        <SkeletonCard show_media={true} media_height="200px" text_lines={2} />
                    </div>
                </Demo>
                <CodeBlock code={"<SkeletonCard show_media={true} media_height=\"200px\" text_lines={2} />".to_string()} language={"rust".to_string()} />

                // ── Static (No Animation) ──
                <Demo title="Static (No Animation)">
                    <div style="width: 100%; max-width: 400px;">
                        <Skeleton shape={SkeletonShape::Text} width="60%" height="16px" animated={false} />
                    </div>
                </Demo>
                <CodeBlock code={"<Skeleton shape={SkeletonShape::Text} width=\"60%\" height=\"16px\" animated={false} />".to_string()} language={"rust".to_string()} />

                <PropTable rows={skeleton_props} />
            </Section>

            <Section title="SkeletonText">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"SkeletonText renders multiple text line placeholders with configurable \
                      widths and spacing. Use it to represent paragraphs or blocks of text \
                      that are loading."}
                </p>
                <PropTable rows={skeleton_text_props} />
            </Section>

            <Section title="SkeletonCard">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"SkeletonCard renders a card-shaped placeholder with an optional media \
                      area and configurable text lines. Ideal for representing content cards, \
                      list items, or feed entries."}
                </p>
                <PropTable rows={skeleton_card_props} />
            </Section>
        </>
    }
}
