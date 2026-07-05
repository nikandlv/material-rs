use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn ButtonsPage() -> Html {
    let button_props = vec![
        PropRow { name: "label".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Text displayed on the button.".into() },
        PropRow { name: "icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Leading Material icon name (Google Font ligature).".into() },
        PropRow { name: "trailing_icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Trailing Material icon name shown after the label.".into() },
        PropRow { name: "variant".into(), r#type: "ButtonVariant".into(), default_value: "Filled".into(), description: "Visual variant: Filled, FilledTonal, FilledTertiary, Outlined, Text, Elevated, Icon, Fab, SmallFab, LargeFab, ExtendedFab, or Tertiary variants.".into() },
        PropRow { name: "size".into(), r#type: "ButtonSize".into(), default_value: "Medium".into(), description: "Button size: Small (32px), Medium (40px), or Large (48px). Applies to standard buttons.".into() },
        PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Disables interaction and applies muted styling.".into() },
        PropRow { name: "full_width".into(), r#type: "bool".into(), default_value: "false".into(), description: "Expands button to fill the container width.".into() },
        PropRow { name: "onclick".into(), r#type: "Callback<MouseEvent>".into(), default_value: "default".into(), description: "Click event handler.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
        PropRow { name: "style".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Inline CSS style override.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Child content, overrides label rendering if provided.".into() },
    ];

    let icon_button_props = vec![
        PropRow { name: "icon".into(), r#type: "String".into(), default_value: "required".into(), description: "Material icon name.".into() },
        PropRow { name: "variant".into(), r#type: "IconButtonVariant".into(), default_value: "Standard".into(), description: "Visual variant: Standard, Filled, Tonal, or Outlined.".into() },
        PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Disables interaction and applies muted styling.".into() },
        PropRow { name: "label".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Accessible label (aria-label).".into() },
        PropRow { name: "onclick".into(), r#type: "Callback<MouseEvent>".into(), default_value: "default".into(), description: "Click event handler.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    let fab_sizes = vec![
        PropRow { name: "SmallFab".into(), r#type: "ButtonVariant".into(), default_value: "-".into(), description: "40x40px compact FAB.".into() },
        PropRow { name: "Fab".into(), r#type: "ButtonVariant".into(), default_value: "-".into(), description: "56x56px standard FAB.".into() },
        PropRow { name: "LargeFab".into(), r#type: "ButtonVariant".into(), default_value: "-".into(), description: "96x96px large FAB.".into() },
        PropRow { name: "ExtendedFab".into(), r#type: "ButtonVariant".into(), default_value: "-".into(), description: "56px tall pill FAB with label text.".into() },
        PropRow { name: "FabTertiary".into(), r#type: "ButtonVariant".into(), default_value: "-".into(), description: "Tertiary-colored variant of standard FAB.".into() },
        PropRow { name: "SmallFabTertiary".into(), r#type: "ButtonVariant".into(), default_value: "-".into(), description: "Tertiary-colored 40px FAB.".into() },
        PropRow { name: "LargeFabTertiary".into(), r#type: "ButtonVariant".into(), default_value: "-".into(), description: "Tertiary-colored 96px FAB.".into() },
        PropRow { name: "ExtendedFabTertiary".into(), r#type: "ButtonVariant".into(), default_value: "-".into(), description: "Tertiary-colored extended FAB.".into() },
    ];

    html! {
        <>
            <Section title="Button">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Buttons communicate actions users can take. They are typically placed throughout your UI, in places like dialogs, forms, and cards."}
                </p>

                // ── Button Variants ──
                <Demo title="Variants">
                    <div style="display: flex; flex-wrap: wrap; gap: 12px; align-items: center;">
                        <Button label="Filled" variant={ButtonVariant::Filled} />
                        <Button label="Filled Tonal" variant={ButtonVariant::FilledTonal} />
                        <Button label="Filled Tertiary" variant={ButtonVariant::FilledTertiary} />
                        <Button label="Outlined" variant={ButtonVariant::Outlined} />
                        <Button label="Text" variant={ButtonVariant::Text} />
                        <Button label="Elevated" variant={ButtonVariant::Elevated} />
                    </div>
                </Demo>
                <CodeBlock code={"<Button label=\"Filled\" variant={ButtonVariant::Filled} />\n<Button label=\"Filled Tonal\" variant={ButtonVariant::FilledTonal} />\n<Button label=\"Filled Tertiary\" variant={ButtonVariant::FilledTertiary} />\n<Button label=\"Outlined\" variant={ButtonVariant::Outlined} />\n<Button label=\"Text\" variant={ButtonVariant::Text} />\n<Button label=\"Elevated\" variant={ButtonVariant::Elevated} />".to_string()} language={"rust".to_string()} />

                // ── Button Sizes ──
                <Demo title="Sizes">
                    <div style="display: flex; flex-wrap: wrap; gap: 12px; align-items: center;">
                        <Button label="Small" size={ButtonSize::Small} />
                        <Button label="Medium" size={ButtonSize::Medium} />
                        <Button label="Large" size={ButtonSize::Large} />
                    </div>
                </Demo>
                <CodeBlock code={"<Button label=\"Small\" size={ButtonSize::Small} />\n<Button label=\"Medium\" size={ButtonSize::Medium} />\n<Button label=\"Large\" size={ButtonSize::Large} />".to_string()} language={"rust".to_string()} />

                // ── Icons ──
                <Demo title="Icons">
                    <div style="display: flex; flex-wrap: wrap; gap: 12px; align-items: center;">
                        <Button label="Leading Icon" icon="favorite" variant={ButtonVariant::Filled} />
                        <Button label="Trailing Icon" trailing_icon="arrow_forward" variant={ButtonVariant::Filled} />
                        <Button label="Both Icons" icon="favorite" trailing_icon="arrow_forward" variant={ButtonVariant::Outlined} />
                        <Button label="Disabled" icon="lock" disabled={true} variant={ButtonVariant::Filled} />
                    </div>
                </Demo>
                <CodeBlock code={"<Button label=\"Leading Icon\" icon=\"favorite\" variant={ButtonVariant::Filled} />\n<Button label=\"Trailing Icon\" trailing_icon=\"arrow_forward\" variant={ButtonVariant::Filled} />\n<Button label=\"Both Icons\" icon=\"favorite\" trailing_icon=\"arrow_forward\" variant={ButtonVariant::Outlined} />\n<Button label=\"Disabled\" icon=\"lock\" disabled={true} variant={ButtonVariant::Filled} />".to_string()} language={"rust".to_string()} />

                // ── Full Width ──
                <Demo title="Full Width">
                    <div style="width: 100%; max-width: 400px;">
                        <Button label="Full Width Button" variant={ButtonVariant::Filled} full_width={true} />
                    </div>
                </Demo>
                <CodeBlock code={"<Button label=\"Full Width Button\" variant={ButtonVariant::Filled} full_width={true} />".to_string()} language={"rust".to_string()} />

                // ── Disabled State ──
                <Demo title="Disabled State">
                    <div style="display: flex; flex-wrap: wrap; gap: 12px; align-items: center;">
                        <Button label="Filled" disabled={true} />
                        <Button label="Tonal" variant={ButtonVariant::FilledTonal} disabled={true} />
                        <Button label="Outlined" variant={ButtonVariant::Outlined} disabled={true} />
                        <Button label="Text" variant={ButtonVariant::Text} disabled={true} />
                    </div>
                </Demo>
                <CodeBlock code={"<Button label=\"Filled\" disabled={true} />\n<Button label=\"Tonal\" variant={ButtonVariant::FilledTonal} disabled={true} />\n<Button label=\"Outlined\" variant={ButtonVariant::Outlined} disabled={true} />\n<Button label=\"Text\" variant={ButtonVariant::Text} disabled={true} />".to_string()} language={"rust".to_string()} />

                // ── Props Table ──
                <PropTable rows={button_props} />
            </Section>

            <Section title="Icon Button">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Icon buttons allow users to take actions and make choices with a single tap. They display icons instead of text and are commonly used in toolbars and app bars."}
                </p>

                <Demo title="Icon Button Variants">
                    <div style="display: flex; flex-wrap: wrap; gap: 16px; align-items: center;">
                        <IconButton icon="favorite" variant={IconButtonVariant::Standard} label="Standard" />
                        <IconButton icon="favorite" variant={IconButtonVariant::Filled} label="Filled" />
                        <IconButton icon="favorite" variant={IconButtonVariant::Tonal} label="Tonal" />
                        <IconButton icon="favorite" variant={IconButtonVariant::Outlined} label="Outlined" />
                        <IconButton icon="favorite" variant={IconButtonVariant::Standard} disabled={true} label="Disabled" />
                    </div>
                </Demo>
                <CodeBlock code={"<IconButton icon=\"favorite\" variant={IconButtonVariant::Standard} label=\"Standard\" />\n<IconButton icon=\"favorite\" variant={IconButtonVariant::Filled} label=\"Filled\" />\n<IconButton icon=\"favorite\" variant={IconButtonVariant::Tonal} label=\"Tonal\" />\n<IconButton icon=\"favorite\" variant={IconButtonVariant::Outlined} label=\"Outlined\" />\n<IconButton icon=\"favorite\" variant={IconButtonVariant::Standard} disabled={true} label=\"Disabled\" />".to_string()} language={"rust".to_string()} />

                <PropTable rows={icon_button_props} />
            </Section>

            <Section title="Floating Action Button (FAB)">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"FABs are floating circular buttons that represent the primary action on a screen. They come in multiple sizes and can be extended to include a label."}
                </p>

                <Demo title="FAB Sizes">
                    <div style="display: flex; flex-wrap: wrap; gap: 16px; align-items: center;">
                        <Button variant={ButtonVariant::SmallFab} icon="add" />
                        <Button variant={ButtonVariant::Fab} icon="add" />
                        <Button variant={ButtonVariant::LargeFab} icon="add" />
                        <Button variant={ButtonVariant::ExtendedFab} icon="edit" label="Create" />
                    </div>
                </Demo>
                <CodeBlock code={"<Button variant={ButtonVariant::SmallFab} icon=\"add\" />\n<Button variant={ButtonVariant::Fab} icon=\"add\" />\n<Button variant={ButtonVariant::LargeFab} icon=\"add\" />\n<Button variant={ButtonVariant::ExtendedFab} icon=\"edit\" label=\"Create\" />".to_string()} language={"rust".to_string()} />

                <Demo title="FAB Tertiary Variants">
                    <div style="display: flex; flex-wrap: wrap; gap: 16px; align-items: center;">
                        <Button variant={ButtonVariant::SmallFabTertiary} icon="add" />
                        <Button variant={ButtonVariant::FabTertiary} icon="add" />
                        <Button variant={ButtonVariant::LargeFabTertiary} icon="add" />
                        <Button variant={ButtonVariant::ExtendedFabTertiary} icon="edit" label="Create" />
                    </div>
                </Demo>
                <CodeBlock code={"<Button variant={ButtonVariant::SmallFabTertiary} icon=\"add\" />\n<Button variant={ButtonVariant::FabTertiary} icon=\"add\" />\n<Button variant={ButtonVariant::LargeFabTertiary} icon=\"add\" />\n<Button variant={ButtonVariant::ExtendedFabTertiary} icon=\"edit\" label=\"Create\" />".to_string()} language={"rust".to_string()} />

                <Demo title="FAB Sizes Reference">
                    <div style="display: flex; flex-wrap: wrap; gap: 24px; align-items: flex-end;">
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <Button variant={ButtonVariant::SmallFab} icon="add" />
                            <span style="font-size: 12px;">{"Small (40px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <Button variant={ButtonVariant::Fab} icon="add" />
                            <span style="font-size: 12px;">{"Regular (56px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <Button variant={ButtonVariant::LargeFab} icon="add" />
                            <span style="font-size: 12px;">{"Large (96px)"}</span>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <Button variant={ButtonVariant::ExtendedFab} icon="edit" label="Compose" />
                            <span style="font-size: 12px;">{"Extended (auto width)"}</span>
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={"<Button variant={ButtonVariant::SmallFab} icon=\"add\" />    // 40x40px\n<Button variant={ButtonVariant::Fab} icon=\"add\" />         // 56x56px\n<Button variant={ButtonVariant::LargeFab} icon=\"add\" />     // 96x96px\n<Button variant={ButtonVariant::ExtendedFab} icon=\"edit\" label=\"Compose\" />  // auto width".to_string()} language={"rust".to_string()} />

                <PropTable rows={fab_sizes} />
            </Section>
        </>
    }
}
