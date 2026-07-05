use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn NavigationDrawerPage() -> Html {
    let persistent_open = use_state(|| true);
    let modal_open = use_state(|| false);
    let section_open = use_state(|| true);

    let drawer_props = vec![
        PropRow { name: "variant".into(), r#type: "DrawerVariant".into(), default_value: "Modal".into(), description: "Drawer behavior: Standard (always visible), Modal (overlay), or Persistent (pushes content).".into() },
        PropRow { name: "open".into(), r#type: "bool".into(), default_value: "true".into(), description: "Controls whether the drawer is visible.".into() },
        PropRow { name: "items".into(), r#type: "Vec<DrawerItem>".into(), default_value: "required".into(), description: "Navigation items to display in the drawer.".into() },
        PropRow { name: "on_select".into(), r#type: "Callback<String>".into(), default_value: "default".into(), description: "Fires when an item is selected, passing the item's key.".into() },
        PropRow { name: "on_close".into(), r#type: "Callback<()>".into(), default_value: "default".into(), description: "Fires when the drawer requests to close (modal dismiss).".into() },
        PropRow { name: "headline".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Title text displayed at the top of the drawer.".into() },
    ];

    let drawer_item_props = vec![
        PropRow { name: "key".into(), r#type: "String".into(), default_value: "required".into(), description: "Unique identifier for this item, passed to on_select.".into() },
        PropRow { name: "icon".into(), r#type: "String".into(), default_value: "required".into(), description: "Material icon name displayed before the label.".into() },
        PropRow { name: "label".into(), r#type: "String".into(), default_value: "required".into(), description: "Display text for the navigation item.".into() },
        PropRow { name: "section".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Groups items under a section header. Empty string means no section.".into() },
        PropRow { name: "active".into(), r#type: "bool".into(), default_value: "false".into(), description: "Marks this item as the currently active/selected navigation target.".into() },
    ];

    let drawer_variant_props = vec![
        PropRow { name: "Standard".into(), r#type: "DrawerVariant".into(), default_value: "-".into(), description: "Always visible side panel that coexists with content.".into() },
        PropRow { name: "Modal".into(), r#type: "DrawerVariant".into(), default_value: "-".into(), description: "Overlay drawer that blocks interaction with content behind it.".into() },
        PropRow { name: "Persistent".into(), r#type: "DrawerVariant".into(), default_value: "-".into(), description: "Pushes content aside when open, collapses when closed.".into() },
    ];

    html! {
        <>
            <Section title="Navigation Drawer">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Navigation drawers provide a consistent way to navigate between sections of an application. They support three variants: Standard (always visible), Modal (overlay with backdrop), and Persistent (pushes content aside)."}
                </p>

                // ── Persistent Drawer ──
                <Demo title="Persistent Drawer">
                    <div style="display: flex; flex-direction: column; gap: 16px;">
                        <div style="display: flex; gap: 12px; flex-wrap: wrap;">
                            <Button label={if *persistent_open { "Close Drawer" } else { "Open Drawer" }} variant={ButtonVariant::FilledTonal} onclick={let lo = persistent_open.clone(); Callback::from(move |_: MouseEvent| lo.set(!*lo))} />
                        </div>
                        <div style="display: flex; width: 100%; height: 300px; border: 1px solid var(--md-sys-color-outline-variant); border-radius: 12px; overflow: hidden; background-color: var(--md-sys-color-surface-container-low);">
                            <NavigationDrawer variant={DrawerVariant::Persistent} open={*persistent_open} headline="My App"
                                items={vec![
                                    DrawerItem { key: "home".into(), icon: "home".into(), label: "Home".into(), section: "".into(), active: true },
                                    DrawerItem { key: "inbox".into(), icon: "inbox".into(), label: "Inbox".into(), section: "".into(), active: false },
                                    DrawerItem { key: "settings".into(), icon: "settings".into(), label: "Settings".into(), section: "".into(), active: false },
                                ]} on_select={Callback::from(|_| {})} />
                            <div style="flex: 1; padding: 24px; display: flex; flex-direction: column; gap: 12px; justify-content: center; align-items: center;">
                                <span style="font-size: 24px; font-weight: 500; color: var(--md-sys-color-on-surface);">{"Content Area"}</span>
                                <span style="font-size: 14px; color: var(--md-sys-color-on-surface-variant); text-align: center; max-width: 400px;">
                                    {"The persistent drawer pushes the content area horizontally when toggled, creating a smooth responsive layout."}
                                </span>
                            </div>
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={"let open = use_state(|| true);\n\n<Button\n    label={if *open { \"Close Drawer\" } else { \"Open Drawer\" }}\n    variant={ButtonVariant::FilledTonal}\n    onclick={let lo = open.clone(); Callback::from(move |_: MouseEvent| lo.set(!*lo))}\n/>\n\n<NavigationDrawer\n    variant={DrawerVariant::Persistent}\n    open={*open}\n    headline=\"My App\"\n    items={vec![\n        DrawerItem { key: \"home\".into(), icon: \"home\".into(), label: \"Home\".into(), section: \"\".into(), active: true },\n        DrawerItem { key: \"inbox\".into(), icon: \"inbox\".into(), label: \"Inbox\".into(), section: \"\".into(), active: false },\n        DrawerItem { key: \"settings\".into(), icon: \"settings\".into(), label: \"Settings\".into(), section: \"\".into(), active: false },\n    ]}\n    on_select={Callback::from(|_| {})}\n/>".to_string()} language={"rust".to_string()} />

                // ── Modal Drawer ──
                <Demo title="Modal Drawer">
                    <div style="display: flex; flex-direction: column; gap: 16px;">
                        <Button label="Open Modal Drawer" variant={ButtonVariant::Filled} onclick={let mo = modal_open.clone(); Callback::from(move |_: MouseEvent| mo.set(true))} />
                        <NavigationDrawer variant={DrawerVariant::Modal} open={*modal_open} headline="Quick Actions"
                            items={vec![
                                DrawerItem { key: "compose".into(), icon: "edit".into(), label: "Compose".into(), section: "".into(), active: false },
                                DrawerItem { key: "archive".into(), icon: "archive".into(), label: "Archive".into(), section: "".into(), active: false },
                                DrawerItem { key: "delete".into(), icon: "delete".into(), label: "Delete".into(), section: "".into(), active: false },
                            ]}
                            on_select={let mo = modal_open.clone(); Callback::from(move |_: String| mo.set(false))}
                            on_close={let mo = modal_open.clone(); Callback::from(move |()| mo.set(false))} />
                    </div>
                </Demo>
                <CodeBlock code={"let modal_open = use_state(|| false);\n\n<Button\n    label=\"Open Modal Drawer\"\n    variant={ButtonVariant::Filled}\n    onclick={let mo = modal_open.clone(); Callback::from(move |_: MouseEvent| mo.set(true))}\n/>\n\n<NavigationDrawer\n    variant={DrawerVariant::Modal}\n    open={*modal_open}\n    headline=\"Quick Actions\"\n    items={vec![\n        DrawerItem { key: \"compose\".into(), icon: \"edit\".into(), label: \"Compose\".into(), section: \"\".into(), active: false },\n        DrawerItem { key: \"archive\".into(), icon: \"archive\".into(), label: \"Archive\".into(), section: \"\".into(), active: false },\n        DrawerItem { key: \"delete\".into(), icon: \"delete\".into(), label: \"Delete\".into(), section: \"\".into(), active: false },\n    ]}\n    on_select={let mo = modal_open.clone(); Callback::from(move |_: String| mo.set(false))}\n    on_close={let mo = modal_open.clone(); Callback::from(move |()| mo.set(false))}\n/>".to_string()} language={"rust".to_string()} />

                // ── Drawer with Sections ──
                <Demo title="Grouped Sections">
                    <div style="display: flex; flex-direction: column; gap: 16px;">
                        <div style="display: flex; gap: 12px; flex-wrap: wrap;">
                            <Button label={if *section_open { "Close" } else { "Open" }} variant={ButtonVariant::FilledTonal} onclick={let so = section_open.clone(); Callback::from(move |_: MouseEvent| so.set(!*so))} />
                        </div>
                        <div style="display: flex; width: 100%; height: 400px; border: 1px solid var(--md-sys-color-outline-variant); border-radius: 12px; overflow: hidden; background-color: var(--md-sys-color-surface-container-low);">
                            <NavigationDrawer variant={DrawerVariant::Persistent} open={*section_open} headline="Navigation"
                                items={vec![
                                    DrawerItem { key: "dashboard".into(), icon: "dashboard".into(), label: "Dashboard".into(), section: "Main".into(), active: true },
                                    DrawerItem { key: "analytics".into(), icon: "analytics".into(), label: "Analytics".into(), section: "Main".into(), active: false },
                                    DrawerItem { key: "profile".into(), icon: "person".into(), label: "Profile".into(), section: "Account".into(), active: false },
                                    DrawerItem { key: "preferences".into(), icon: "tune".into(), label: "Preferences".into(), section: "Account".into(), active: false },
                                    DrawerItem { key: "help".into(), icon: "help".into(), label: "Help".into(), section: "Support".into(), active: false },
                                ]} on_select={Callback::from(|_| {})} />
                            <div style="flex: 1; padding: 24px; display: flex; flex-direction: column; gap: 12px; justify-content: center; align-items: center;">
                                <span style="font-size: 24px; font-weight: 500; color: var(--md-sys-color-on-surface);">{"Grouped Content"}</span>
                                <span style="font-size: 14px; color: var(--md-sys-color-on-surface-variant); text-align: center; max-width: 400px;">
                                    {"Items are organized into sections with headers. Use the section field to group related navigation items."}
                                </span>
                            </div>
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={"<NavigationDrawer\n    variant={DrawerVariant::Persistent}\n    open={*open}\n    headline=\"Navigation\"\n    items={vec![\n        DrawerItem { key: \"dashboard\".into(), icon: \"dashboard\".into(), label: \"Dashboard\".into(), section: \"Main\".into(), active: true },\n        DrawerItem { key: \"analytics\".into(), icon: \"analytics\".into(), label: \"Analytics\".into(), section: \"Main\".into(), active: false },\n        DrawerItem { key: \"profile\".into(), icon: \"person\".into(), label: \"Profile\".into(), section: \"Account\".into(), active: false },\n        DrawerItem { key: \"preferences\".into(), icon: \"tune\".into(), label: \"Preferences\".into(), section: \"Account\".into(), active: false },\n        DrawerItem { key: \"help\".into(), icon: \"help\".into(), label: \"Help\".into(), section: \"Support\".into(), active: false },\n    ]}\n    on_select={Callback::from(|_| {})}\n/>".to_string()} language={"rust".to_string()} />

                <PropTable rows={drawer_props} />

                <h3 style="font-size: 18px; font-weight: 500; margin-top: 32px; margin-bottom: 16px; color: var(--md-sys-color-on-surface);">{"DrawerItem"}</h3>
                <PropTable rows={drawer_item_props} />

                <h3 style="font-size: 18px; font-weight: 500; margin-top: 32px; margin-bottom: 16px; color: var(--md-sys-color-on-surface);">{"DrawerVariant"}</h3>
                <PropTable rows={drawer_variant_props} />
            </Section>
        </>
    }
}
