use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn TabBarPage() -> Html {
    let tab_item_props = vec![
        PropRow { name: "key".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Unique identifier for the tab. Used as the active key value.".into() },
        PropRow { name: "label".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Text displayed on the tab.".into() },
        PropRow { name: "icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Material icon name shown above the label. When set, the tab becomes icon+label variant.".into() },
        PropRow { name: "active".into(), r#type: "bool".into(), default_value: "false".into(), description: "Whether this tab is currently selected.".into() },
    ];

    let tab_bar_props = vec![
        PropRow { name: "tabs".into(), r#type: "Vec<TabItem>".into(), default_value: "(required)".into(), description: "Vector of tab definitions to render.".into() },
        PropRow { name: "on_select".into(), r#type: "Callback<String>".into(), default_value: "Callback::noop()".into(), description: "Fires with the key of the tab the user selected.".into() },
        PropRow { name: "scrollable".into(), r#type: "bool".into(), default_value: "false".into(), description: "When true, tabs use fixed widths and the bar scrolls horizontally. When false, tabs stretch evenly.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    // Demo state
    let selected_fixed = use_state(|| "tab1".to_string());
    let selected_scrollable = use_state(|| "tab1".to_string());
    let selected_icons = use_state(|| "home".to_string());
    let selected_active = use_state(|| "tab2".to_string());

    let fixed_code = r#"let selected = use_state(|| "tab1".to_string());

<TabBar
    tabs={vec![
        TabItem { key: "tab1".into(), label: "Albums".into(), icon: String::new(), active: *selected == "tab1" },
        TabItem { key: "tab2".into(), label: "Photos".into(), icon: String::new(), active: *selected == "tab2" },
        TabItem { key: "tab3".into(), label: "Shared".into(), icon: String::new(), active: *selected == "tab3" },
    ]}
    on_select={let s = selected.clone(); Callback::from(move |key: String| s.set(key))}
/>"#;

    let scrollable_code = r#"let selected = use_state(|| "tab1".to_string());

<TabBar
    scrollable={true}
    tabs={vec![
        TabItem { key: "tab1".into(), label: "Overview".into(), icon: String::new(), active: *selected == "tab1" },
        TabItem { key: "tab2".into(), label: "Analytics".into(), icon: String::new(), active: *selected == "tab2" },
        TabItem { key: "tab3".into(), label: "Revenue".into(), icon: String::new(), active: *selected == "tab3" },
        TabItem { key: "tab4".into(), label: "Audience".into(), icon: String::new(), active: *selected == "tab4" },
        TabItem { key: "tab5".into(), label: "Conversions".into(), icon: String::new(), active: *selected == "tab5" },
        TabItem { key: "tab6".into(), label: "Realtime".into(), icon: String::new(), active: *selected == "tab6" },
    ]}
    on_select={let s = selected.clone(); Callback::from(move |key: String| s.set(key))}
/>"#;

    let icons_code = r#"let selected = use_state(|| "home".to_string());

<TabBar
    tabs={vec![
        TabItem { key: "home".into(), label: "Home".into(), icon: "home".into(), active: *selected == "home" },
        TabItem { key: "search".into(), label: "Search".into(), icon: "search".into(), active: *selected == "search" },
        TabItem { key: "favorites".into(), label: "Favorites".into(), icon: "favorite".into(), active: *selected == "favorites" },
        TabItem { key: "profile".into(), label: "Profile".into(), icon: "person".into(), active: *selected == "profile" },
    ]}
    on_select={let s = selected.clone(); Callback::from(move |key: String| s.set(key))}
/>"#;

    let active_code = r#"let selected = use_state(|| "tab2".to_string());

<TabBar
    tabs={vec![
        TabItem { key: "tab1".into(), label: "Overview".into(), icon: String::new(), active: *selected == "tab1" },
        TabItem { key: "tab2".into(), label: "Details".into(), icon: "info".into(), active: *selected == "tab2" },
        TabItem { key: "tab3".into(), label: "Settings".into(), icon: "settings".into(), active: *selected == "tab3" },
    ]}
    on_select={let s = selected.clone(); Callback::from(move |key: String| s.set(key))}
/>"#;

    html! {
        <>
            <Section title="Tab Bar">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Tab bars organize content across multiple views. Each tab shows a different section \
                      and the user switches between them by tapping. A sliding indicator visually marks \
                      the active tab. Tab bars support both fixed (evenly distributed) and scrollable \
                      (variable width) layouts."}
                </p>

                // ── Fixed Tabs ──
                <Demo title="Fixed Tabs">
                    <TabBar
                        tabs={vec![
                            TabItem { key: "tab1".into(), label: "Albums".into(), icon: String::new(), active: *selected_fixed == "tab1" },
                            TabItem { key: "tab2".into(), label: "Photos".into(), icon: String::new(), active: *selected_fixed == "tab2" },
                            TabItem { key: "tab3".into(), label: "Shared".into(), icon: String::new(), active: *selected_fixed == "tab3" },
                        ]}
                        on_select={let s = selected_fixed.clone(); Callback::from(move |key: String| s.set(key))}
                    />
                    <div style="padding: 16px; font-size: 14px; color: var(--md-sys-color-on-surface-variant);">
                        { format!("Selected: {}", *selected_fixed) }
                    </div>
                </Demo>
                <CodeBlock code={fixed_code.to_string()} language={"rust".to_string()} />

                // ── Scrollable Tabs ──
                <Demo title="Scrollable Tabs">
                    <TabBar
                        scrollable={true}
                        tabs={vec![
                            TabItem { key: "tab1".into(), label: "Overview".into(), icon: String::new(), active: *selected_scrollable == "tab1" },
                            TabItem { key: "tab2".into(), label: "Analytics".into(), icon: String::new(), active: *selected_scrollable == "tab2" },
                            TabItem { key: "tab3".into(), label: "Revenue".into(), icon: String::new(), active: *selected_scrollable == "tab3" },
                            TabItem { key: "tab4".into(), label: "Audience".into(), icon: String::new(), active: *selected_scrollable == "tab4" },
                            TabItem { key: "tab5".into(), label: "Conversions".into(), icon: String::new(), active: *selected_scrollable == "tab5" },
                            TabItem { key: "tab6".into(), label: "Realtime".into(), icon: String::new(), active: *selected_scrollable == "tab6" },
                        ]}
                        on_select={let s = selected_scrollable.clone(); Callback::from(move |key: String| s.set(key))}
                    />
                    <div style="padding: 16px; font-size: 14px; color: var(--md-sys-color-on-surface-variant);">
                        { format!("Selected: {}", *selected_scrollable) }
                    </div>
                </Demo>
                <CodeBlock code={scrollable_code.to_string()} language={"rust".to_string()} />

                // ── Tabs with Icons ──
                <Demo title="Tabs with Icons">
                    <TabBar
                        tabs={vec![
                            TabItem { key: "home".into(), label: "Home".into(), icon: "home".into(), active: *selected_icons == "home" },
                            TabItem { key: "search".into(), label: "Search".into(), icon: "search".into(), active: *selected_icons == "search" },
                            TabItem { key: "favorites".into(), label: "Favorites".into(), icon: "favorite".into(), active: *selected_icons == "favorites" },
                            TabItem { key: "profile".into(), label: "Profile".into(), icon: "person".into(), active: *selected_icons == "profile" },
                        ]}
                        on_select={let s = selected_icons.clone(); Callback::from(move |key: String| s.set(key))}
                    />
                    <div style="padding: 16px; font-size: 14px; color: var(--md-sys-color-on-surface-variant);">
                        { format!("Selected: {}", *selected_icons) }
                    </div>
                </Demo>
                <CodeBlock code={icons_code.to_string()} language={"rust".to_string()} />

                // ── Active Tab with Icons ──
                <Demo title="Active Tab with Icons">
                    <TabBar
                        tabs={vec![
                            TabItem { key: "tab1".into(), label: "Overview".into(), icon: String::new(), active: *selected_active == "tab1" },
                            TabItem { key: "tab2".into(), label: "Details".into(), icon: "info".into(), active: *selected_active == "tab2" },
                            TabItem { key: "tab3".into(), label: "Settings".into(), icon: "settings".into(), active: *selected_active == "tab3" },
                        ]}
                        on_select={let s = selected_active.clone(); Callback::from(move |key: String| s.set(key))}
                    />
                    <div style="padding: 16px; font-size: 14px; color: var(--md-sys-color-on-surface-variant);">
                        { format!("Selected: {}", *selected_active) }
                    </div>
                </Demo>
                <CodeBlock code={active_code.to_string()} language={"rust".to_string()} />

                // ── Props Tables ──
                <h3 style="font-size: 16px; font-weight: 600; margin: 32px 0 12px;">{"TabItem"}</h3>
                <PropTable rows={tab_item_props} />

                <h3 style="font-size: 16px; font-weight: 600; margin: 32px 0 12px;">{"TabBar"}</h3>
                <PropTable rows={tab_bar_props} />
            </Section>
        </>
    }
}
