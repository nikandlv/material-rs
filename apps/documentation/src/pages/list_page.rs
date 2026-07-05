use yew::prelude::*;
use material_rs::components::{ListItem, Divider, DividerVariant, IconButton, IconButtonVariant};
use material_rs::theme::Theme;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn ListPage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let item_props = vec![
        PropRow { name: "headline".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Primary text displayed on the first line.".into() },
        PropRow { name: "supporting_text".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Secondary text shown on the second line. When non-empty, the item becomes a two-line variant.".into() },
        PropRow { name: "overline".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Small text shown above the headline. When non-empty, the item becomes a three-line variant.".into() },
        PropRow { name: "leading".into(), r#type: "Html".into(), default_value: "Html::default()".into(), description: "Leading content (icon, avatar, checkbox, etc.) placed before the text area.".into() },
        PropRow { name: "trailing".into(), r#type: "Html".into(), default_value: "Html::default()".into(), description: "Trailing content (icon, badge, toggle, etc.) placed after the text area.".into() },
        PropRow { name: "interactive".into(), r#type: "bool".into(), default_value: "true".into(), description: "When false, removes hover/press states and blocks click callbacks.".into() },
        PropRow { name: "onclick".into(), r#type: "Callback<MouseEvent>".into(), default_value: "Callback::noop()".into(), description: "Fires when the item is clicked (only when interactive is true).".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    html! {
        <>
            <Section title="List">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Lists display multiple lines of text with optional leading and trailing content. \
                      They come in three density variants — one-line (56px), two-line (72px), and \
                      three-line (88px) — each automatically determined by which text props are set."}
                </p>

                // ── One-Line Item ──
                <Demo title="One-Line Item">
                    <div style="width: 100%; max-width: 400px; border-radius: 12px; overflow: hidden;">
                        <ListItem headline="Inbox" />
                    </div>
                </Demo>
                <CodeBlock code={"<ListItem headline=\"Inbox\" />".to_string()} language={"rust".to_string()} />

                // ── Two-Line Item ──
                <Demo title="Two-Line Item">
                    <div style="width: 100%; max-width: 400px; border-radius: 12px; overflow: hidden;">
                        <ListItem headline="Megan Miller" supporting_text="Hey, are we still on for lunch?" />
                    </div>
                </Demo>
                <CodeBlock code={"<ListItem\n    headline=\"Megan Miller\"\n    supporting_text=\"Hey, are we still on for lunch?\"\n/>".to_string()} language={"rust".to_string()} />

                // ── Three-Line Item ──
                <Demo title="Three-Line Item">
                    <div style="width: 100%; max-width: 400px; border-radius: 12px; overflow: hidden;">
                        <ListItem
                            headline="Project Update"
                            supporting_text="The new design system is ready for review and implementation"
                            overline="DESIGN"
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<ListItem\n    headline=\"Project Update\"\n    supporting_text=\"The new design system is ready for review and implementation\"\n    overline=\"DESIGN\"\n/>".to_string()} language={"rust".to_string()} />

                // ── With Icons ──
                <Demo title="With Leading Icons">
                    <div style="width: 100%; max-width: 400px; border-radius: 12px; overflow: hidden;">
                        <ListItem
                            headline="Mail"
                            leading={html!{<span class="material-symbols-outlined">{"mail"}</span>}}
                        />
                        <ListItem
                            headline="Phone"
                            supporting_text="+1 (555) 123-4567"
                            leading={html!{<span class="material-symbols-outlined">{"phone"}</span>}}
                        />
                        <ListItem
                            headline="Email from Alex"
                            supporting_text="Design specs attached for the new dashboard layout"
                            leading={html!{<span class="material-symbols-outlined">{"email"}</span>}}
                            overline="FAVORITES"
                        />
                    </div>
                </Demo>
                <CodeBlock code={"// One-line with icon\n<ListItem\n    headline=\"Mail\"\n    leading={html!{<span class=\"material-symbols-outlined\">{\"mail\"}</span>}}\n/>\n\n// Two-line with icon\n<ListItem\n    headline=\"Phone\"\n    supporting_text=\"+1 (555) 123-4567\"\n    leading={html!{<span class=\"material-symbols-outlined\">{\"phone\"}</span>}}\n/>\n\n// Three-line with icon\n<ListItem\n    headline=\"Email from Alex\"\n    supporting_text=\"Design specs attached for the new dashboard layout\"\n    leading={html!{<span class=\"material-symbols-outlined\">{\"email\"}</span>}}\n    overline=\"FAVORITES\"\n/>".to_string()} language={"rust".to_string()} />

                // ── With Trailing Content ──
                <Demo title="With Trailing Content">
                    <div style="width: 100%; max-width: 400px; border-radius: 12px; overflow: hidden;">
                        <ListItem
                            headline="Settings"
                            leading={html!{<span class="material-symbols-outlined">{"settings"}</span>}}
                            trailing={html!{<IconButton icon="chevron_right" variant={IconButtonVariant::Standard} />}}
                        />
                        <ListItem
                            headline="Notifications"
                            supporting_text="Email, SMS, Push"
                            leading={html!{<span class="material-symbols-outlined">{"notifications"}</span>}}
                            trailing={html!{<span class="material-symbols-outlined" style={format!("color: {};", theme.colors.on_surface_variant)}>{"chevron_right"}</span>}}
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<ListItem\n    headline=\"Settings\"\n    leading={html!{<span class=\"material-symbols-outlined\">{\"settings\"}</span>}}\n    trailing={html!{<IconButton icon=\"chevron_right\" variant={IconButtonVariant::Standard} />}}\n/>\n\n<ListItem\n    headline=\"Notifications\"\n    supporting_text=\"Email, SMS, Push\"\n    leading={html!{<span class=\"material-symbols-outlined\">{\"notifications\"}</span>}}\n    trailing={html!{\n        <span class=\"material-symbols-outlined\">{\"chevron_right\"}</span>\n    }}\n/>".to_string()} language={"rust".to_string()} />

                // ── Disabled / Non-Interactive ──
                <Demo title="Non-Interactive">
                    <div style="width: 100%; max-width: 400px; border-radius: 12px; overflow: hidden;">
                        <ListItem
                            headline="Active Item"
                            supporting_text="This item responds to clicks"
                            leading={html!{<span class="material-symbols-outlined">{"check_circle"}</span>}}
                        />
                        <Divider variant={DividerVariant::Inset} />
                        <ListItem
                            headline="Disabled Item"
                            supporting_text="This item does not respond to interaction"
                            leading={html!{<span class="material-symbols-outlined">{"block"}</span>}}
                            interactive={false}
                        />
                    </div>
                </Demo>
                <CodeBlock code={"// Interactive (default)\n<ListItem\n    headline=\"Active Item\"\n    supporting_text=\"This item responds to clicks\"\n    leading={html!{<span class=\"material-symbols-outlined\">{\"check_circle\"}</span>}}\n/>\n\n// Non-interactive\n<ListItem\n    headline=\"Disabled Item\"\n    supporting_text=\"This item does not respond to interaction\"\n    leading={html!{<span class=\"material-symbols-outlined\">{\"block\"}</span>}}\n    interactive={false}\n/>".to_string()} language={"rust".to_string()} />

                // ── Full Showcase ──
                <Demo title="Full Showcase">
                    <div style="width: 100%; max-width: 400px; border-radius: 12px; overflow: hidden;">
                        <ListItem headline="Inbox" leading={html!{<span class="material-symbols-outlined">{"inbox"}</span>}} trailing={html!{<span style={format!("font-size: 14px; color: {};", theme.colors.on_surface_variant)}>{"24"}</span>}} />
                        <ListItem headline="Starred" leading={html!{<span class="material-symbols-outlined">{"star"}</span>}} />
                        <Divider variant={DividerVariant::Inset} />
                        <ListItem headline="Megan Miller" supporting_text="Hey, are we still on for lunch?" leading={html!{<span class="material-symbols-outlined">{"account_circle"}</span>}} trailing={html!{<span style={format!("font-size: 12px; color: {};", theme.colors.on_surface_variant)}>{"2:30 PM"}</span>}} />
                        <ListItem headline="Design Review" supporting_text="The new component library is looking great" overline="WORK" leading={html!{<span class="material-symbols-outlined">{"design_services"}</span>}} />
                        <Divider variant={DividerVariant::Inset} />
                        <ListItem headline="Archived" leading={html!{<span class="material-symbols-outlined">{"archive"}</span>}} interactive={false} />
                    </div>
                </Demo>
                <CodeBlock code={"<ListItem\n    headline=\"Inbox\"\n    leading={html!{<span class=\"material-symbols-outlined\">{\"inbox\"}</span>}}\n    trailing={html!{<span style=\"font-size: 14px;\">{\"24\"}</span>}}\n/>\n<ListItem\n    headline=\"Starred\"\n    leading={html!{<span class=\"material-symbols-outlined\">{\"star\"}</span>}}\n/>\n<Divider variant={DividerVariant::Inset} />\n<ListItem\n    headline=\"Megan Miller\"\n    supporting_text=\"Hey, are we still on for lunch?\"\n    leading={html!{<span class=\"material-symbols-outlined\">{\"account_circle\"}</span>}}\n    trailing={html!{<span style=\"font-size: 12px;\">{\"2:30 PM\"}</span>}}\n/>\n<ListItem\n    headline=\"Design Review\"\n    supporting_text=\"The new component library is looking great\"\n    overline=\"WORK\"\n    leading={html!{<span class=\"material-symbols-outlined\">{\"design_services\"}</span>}}\n/>\n<Divider variant={DividerVariant::Inset} />\n<ListItem\n    headline=\"Archived\"\n    leading={html!{<span class=\"material-symbols-outlined\">{\"archive\"}</span>}}\n    interactive={false}\n/>".to_string()} language={"rust".to_string()} />

                // ── Props Table ──
                <PropTable rows={item_props} />
            </Section>
        </>
    }
}
