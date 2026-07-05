use yew::prelude::*;
use material_rs::components::{Badge, BadgeSize, IconButton, IconButtonVariant};
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn BadgesPage() -> Html {
    let badge_props = vec![
        PropRow { name: "count".into(), r#type: "usize".into(), default_value: "0".into(), description: "Numeric count displayed in the badge. Zero hides it unless dot is true. Values above 999 display as \"999+\".".into() },
        PropRow { name: "dot".into(), r#type: "bool".into(), default_value: "false".into(), description: "Shows a small dot indicator instead of a number.".into() },
        PropRow { name: "is_hidden".into(), r#type: "bool".into(), default_value: "false".into(), description: "Forces the badge to be hidden even when count > 0 or dot is true.".into() },
        PropRow { name: "size".into(), r#type: "BadgeSize".into(), default_value: "BadgeSize::Medium".into(), description: "Badge size: Small (6px dot), Medium (16px), or Large (20px).".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "The element the badge is anchored to (e.g. an IconButton).".into() },
    ];

    html! {
        <>
            <Section title="Badge">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Badges are small overlay indicators that show counts, status, or \
                      notifications on top of another element. They are commonly placed \
                      on icon buttons to indicate unread messages, pending actions, or \
                      new items. Badges can display a numeric count, a dot, or be hidden."}
                </p>

                // ── Count Badge ──
                <Demo title="Count Badge">
                    <div style="display: flex; gap: 32px; align-items: center;">
                        <Badge count={1}>
                            <IconButton icon="mail" variant={IconButtonVariant::Standard} label="Mail" />
                        </Badge>
                        <Badge count={5}>
                            <IconButton icon="inbox" variant={IconButtonVariant::Standard} label="Inbox" />
                        </Badge>
                        <Badge count={42}>
                            <IconButton icon="notifications" variant={IconButtonVariant::Filled} label="Notifications" />
                        </Badge>
                    </div>
                </Demo>
                <CodeBlock code={"<Badge count={1}>\n    <IconButton icon=\"mail\" variant={IconButtonVariant::Standard} label=\"Mail\" />\n</Badge>\n<Badge count={5}>\n    <IconButton icon=\"inbox\" variant={IconButtonVariant::Standard} label=\"Inbox\" />\n</Badge>\n<Badge count={42}>\n    <IconButton icon=\"notifications\" variant={IconButtonVariant::Filled} label=\"Notifications\" />\n</Badge>".to_string()} language={"rust".to_string()} />

                // ── Dot Badge ──
                <Demo title="Dot Badge">
                    <div style="display: flex; gap: 32px; align-items: center;">
                        <Badge dot={true}>
                            <IconButton icon="chat" variant={IconButtonVariant::Standard} label="Chat" />
                        </Badge>
                        <Badge dot={true}>
                            <IconButton icon="favorite" variant={IconButtonVariant::Tonal} label="Favorites" />
                        </Badge>
                        <Badge dot={true}>
                            <IconButton icon="circle_notifications" variant={IconButtonVariant::Outlined} label="Alerts" />
                        </Badge>
                    </div>
                </Demo>
                <CodeBlock code={"<Badge dot={true}>\n    <IconButton icon=\"chat\" variant={IconButtonVariant::Standard} label=\"Chat\" />\n</Badge>\n<Badge dot={true}>\n    <IconButton icon=\"favorite\" variant={IconButtonVariant::Tonal} label=\"Favorites\" />\n</Badge>\n<Badge dot={true}>\n    <IconButton icon=\"circle_notifications\" variant={IconButtonVariant::Outlined} label=\"Alerts\" />\n</Badge>".to_string()} language={"rust".to_string()} />

                // ── Max Count (999+) ──
                <Demo title="Large Count (999+)">
                    <div style="display: flex; gap: 32px; align-items: center;">
                        <Badge count={150}>
                            <IconButton icon="mail" variant={IconButtonVariant::Standard} label="Mail" />
                        </Badge>
                        <Badge count={999}>
                            <IconButton icon="notifications" variant={IconButtonVariant::Tonal} label="Notifications" />
                        </Badge>
                        <Badge count={1234}>
                            <IconButton icon="forum" variant={IconButtonVariant::Filled} label="Forum" />
                        </Badge>
                    </div>
                </Demo>
                <CodeBlock code={"// Counts above 999 display as \"999+\"\n<Badge count={150}>\n    <IconButton icon=\"mail\" variant={IconButtonVariant::Standard} label=\"Mail\" />\n</Badge>\n<Badge count={999}>\n    <IconButton icon=\"notifications\" variant={IconButtonVariant::Tonal} label=\"Notifications\" />\n</Badge>\n<Badge count={1234}>\n    <IconButton icon=\"forum\" variant={IconButtonVariant::Filled} label=\"Forum\" />\n</Badge>".to_string()} language={"rust".to_string()} />

                // ── Hidden Badge ──
                <Demo title="Hidden Badge">
                    <div style="display: flex; gap: 32px; align-items: center;">
                        <Badge count={0}>
                            <IconButton icon="mail" variant={IconButtonVariant::Standard} label="No Count" />
                        </Badge>
                        <Badge count={5} is_hidden={true}>
                            <IconButton icon="inbox" variant={IconButtonVariant::Tonal} label="Hidden" />
                        </Badge>
                        <Badge dot={true} is_hidden={true}>
                            <IconButton icon="chat" variant={IconButtonVariant::Outlined} label="Hidden Dot" />
                        </Badge>
                    </div>
                </Demo>
                <CodeBlock code={"// count={0} hides automatically\n<Badge count={0}>\n    <IconButton icon=\"mail\" variant={IconButtonVariant::Standard} label=\"No Count\" />\n</Badge>\n\n// is_hidden forces badge to disappear\n<Badge count={5} is_hidden={true}>\n    <IconButton icon=\"inbox\" variant={IconButtonVariant::Tonal} label=\"Hidden\" />\n</Badge>\n<Badge dot={true} is_hidden={true}>\n    <IconButton icon=\"chat\" variant={IconButtonVariant::Outlined} label=\"Hidden Dot\" />\n</Badge>".to_string()} language={"rust".to_string()} />

                // ── Badge Sizes ──
                <Demo title="Badge Sizes">
                    <div style="display: flex; gap: 32px; align-items: center;">
                        <Badge count={3} size={BadgeSize::Small}>
                            <IconButton icon="mail" variant={IconButtonVariant::Standard} label="Small Badge" />
                        </Badge>
                        <Badge count={3} size={BadgeSize::Medium}>
                            <IconButton icon="inbox" variant={IconButtonVariant::Standard} label="Medium Badge" />
                        </Badge>
                        <Badge count={3} size={BadgeSize::Large}>
                            <IconButton icon="notifications" variant={IconButtonVariant::Standard} label="Large Badge" />
                        </Badge>
                    </div>
                </Demo>
                <CodeBlock code={"<Badge count={3} size={BadgeSize::Small}>\n    <IconButton icon=\"mail\" variant={IconButtonVariant::Standard} label=\"Small\" />\n</Badge>\n<Badge count={3} size={BadgeSize::Medium}>\n    <IconButton icon=\"inbox\" variant={IconButtonVariant::Standard} label=\"Medium\" />\n</Badge>\n<Badge count={3} size={BadgeSize::Large}>\n    <IconButton icon=\"notifications\" variant={IconButtonVariant::Standard} label=\"Large\" />\n</Badge>".to_string()} language={"rust".to_string()} />

                <PropTable rows={badge_props} />
            </Section>
        </>
    }
}
