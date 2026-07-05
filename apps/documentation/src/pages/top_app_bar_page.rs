use yew::prelude::*;
use material_rs::components::{TopAppBar, TopAppBarVariant, TopAppBarPosition, IconButton, IconButtonVariant};
use material_rs::theme::Theme;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn TopAppBarPage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let props = vec![
        PropRow { name: "title".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Title text displayed in the app bar.".into() },
        PropRow { name: "variant".into(), r#type: "TopAppBarVariant".into(), default_value: "TopAppBarVariant::Small".into(), description: "Size variant: Small (64px), Medium (112px), or Large (152px).".into() },
        PropRow { name: "position".into(), r#type: "TopAppBarPosition".into(), default_value: "TopAppBarPosition::Fixed".into(), description: "CSS position: Fixed, Sticky, or Static.".into() },
        PropRow { name: "nav_icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Material icon name for the navigation button.".into() },
        PropRow { name: "on_nav_click".into(), r#type: "Callback<MouseEvent>".into(), default_value: "default".into(), description: "Click handler for the navigation icon button.".into() },
        PropRow { name: "scrolled".into(), r#type: "bool".into(), default_value: "false".into(), description: "When true, collapses Medium/Large variants down to 64px and shows a surface shadow.".into() },
        PropRow { name: "actions".into(), r#type: "Children".into(), default_value: "default".into(), description: "Trailing action content rendered in the top row.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    html! {
        <>
            <Section title="Top App Bar">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Top app bars display information and actions relating to the current screen. \
                      Material Design 3 defines three size variants: Small (64px), Medium (112px), \
                      and Large (152px). Medium and Large bars collapse to 64px when scrolled. \
                      Positioning can be Fixed, Sticky, or Static."}
                </p>

                // ── Small Top App Bar ──
                <Demo title="Small Top App Bar">
                    <div style="background: var(--md-sys-color-surface-container); border-radius: 12px; overflow: hidden;">
                        <TopAppBar
                            title="Small App Bar"
                            variant={TopAppBarVariant::Small}
                            nav_icon="menu"
                            position={TopAppBarPosition::Static}
                            actions={
                                html!{
                                    <IconButton icon="search" variant={IconButtonVariant::Standard} label="Search" />
                                }
                            }
                        />
                        <div style="height: 200px; padding: 16px; display: flex; align-items: center; justify-content: center;">
                            <span style={format!("color: {};", theme.colors.on_surface_variant)}>{"Content area"}</span>
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={r#"<TopAppBar
    title="Small App Bar"
    variant={TopAppBarVariant::Small}
    nav_icon="menu"
    actions={html!{
        <IconButton icon="search" variant={IconButtonVariant::Standard} label="Search" />
    }}
/>"#.to_string()} language={"rust".to_string()} />

                // ── Medium Top App Bar ──
                <Demo title="Medium Top App Bar">
                    <div style="background: var(--md-sys-color-surface-container); border-radius: 12px; overflow: hidden;">
                        <TopAppBar
                            title="Medium App Bar"
                            variant={TopAppBarVariant::Medium}
                            nav_icon="menu"
                            position={TopAppBarPosition::Static}
                        />
                        <div style="height: 200px; padding: 16px; display: flex; align-items: center; justify-content: center;">
                            <span style={format!("color: {};", theme.colors.on_surface_variant)}>{"Content area"}</span>
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={r#"<TopAppBar
    title="Medium App Bar"
    variant={TopAppBarVariant::Medium}
    nav_icon="menu"
/>"#.to_string()} language={"rust".to_string()} />

                // ── Large Top App Bar ──
                <Demo title="Large Top App Bar">
                    <div style="background: var(--md-sys-color-surface-container); border-radius: 12px; overflow: hidden;">
                        <TopAppBar
                            title="Large App Bar"
                            variant={TopAppBarVariant::Large}
                            nav_icon="menu"
                            position={TopAppBarPosition::Static}
                        />
                        <div style="height: 200px; padding: 16px; display: flex; align-items: center; justify-content: center;">
                            <span style={format!("color: {};", theme.colors.on_surface_variant)}>{"Content area"}</span>
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={r#"<TopAppBar
    title="Large App Bar"
    variant={TopAppBarVariant::Large}
    nav_icon="menu"
/>"#.to_string()} language={"rust".to_string()} />

                // ── Fixed vs Sticky vs Static ──
                <Demo title="Position Variants">
                    <div style="display: flex; gap: 12px; flex-wrap: wrap;">
                        <div style="flex: 1; min-width: 200px; background: var(--md-sys-color-surface-container); border-radius: 12px; overflow: hidden;">
                            <TopAppBar title="Fixed" variant={TopAppBarVariant::Small} nav_icon="menu" position={TopAppBarPosition::Fixed} />
                            <div style="height: 80px;" />
                        </div>
                        <div style="flex: 1; min-width: 200px; background: var(--md-sys-color-surface-container); border-radius: 12px; overflow: hidden;">
                            <TopAppBar title="Sticky" variant={TopAppBarVariant::Small} nav_icon="menu" position={TopAppBarPosition::Sticky} />
                            <div style="height: 80px;" />
                        </div>
                        <div style="flex: 1; min-width: 200px; background: var(--md-sys-color-surface-container); border-radius: 12px; overflow: hidden;">
                            <TopAppBar title="Static" variant={TopAppBarVariant::Small} nav_icon="menu" position={TopAppBarPosition::Static} />
                            <div style="height: 80px;" />
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={r#"<TopAppBar title="Fixed" position={TopAppBarPosition::Fixed} />
<TopAppBar title="Sticky" position={TopAppBarPosition::Sticky} />
<TopAppBar title="Static" position={TopAppBarPosition::Static} />"#.to_string()} language={"rust".to_string()} />

                <PropTable rows={props} />
            </Section>
        </>
    }
}
