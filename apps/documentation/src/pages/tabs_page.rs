use yew::prelude::*;
use material_rs::components::{TabBar, TabItem};
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn TabsPage() -> Html {
    let tab_item_props = vec![
        PropRow { name: "key".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Unique identifier for the tab. Used as the active key value.".into() },
        PropRow { name: "label".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Text displayed on the tab.".into() },
        PropRow { name: "icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Material icon name shown above the label. When set, the tab becomes icon+label variant.".into() },
        PropRow { name: "active".into(), r#type: "bool".into(), default_value: "false".into(), description: "Whether this tab is currently selected. Typically driven by the TabBar's active_key logic.".into() },
    ];

    let tab_bar_props = vec![
        PropRow { name: "tabs".into(), r#type: "Vec<TabItem>".into(), default_value: "(required)".into(), description: "Vector of tab definitions to render.".into() },
        PropRow { name: "on_select".into(), r#type: "Callback<String>".into(), default_value: "Callback::noop()".into(), description: "Fires with the key of the tab the user selected.".into() },
        PropRow { name: "scrollable".into(), r#type: "bool".into(), default_value: "false".into(), description: "When true, tabs use fixed widths and the bar scrolls horizontally. When false, tabs stretch evenly.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    // ── Demo state ──
    let selected_text_only = use_state(|| "tab1".to_string());
    let selected_icons = use_state(|| "home".to_string());
    let selected_scrollable = use_state(|| "tab1".to_string());

    html! {
        <>
            <Section title="Tabs">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Tabs organize content across multiple views. Each tab shows a different section \
                      and the user switches between them by tapping. A sliding indicator visually \
                      marks the active tab."}
                </p>

                // ── Text-Only Tabs ──
                <Demo title="Text-Only Tabs">
                    <TabBar
                        tabs={vec![
                            TabItem { key: "tab1".into(), label: "Albums".into(), icon: String::new(), active: *selected_text_only == "tab1" },
                            TabItem { key: "tab2".into(), label: "Photos".into(), icon: String::new(), active: *selected_text_only == "tab2" },
                            TabItem { key: "tab3".into(), label: "Shared".into(), icon: String::new(), active: *selected_text_only == "tab3" },
                        ]}
                        on_select={let s = selected_text_only.clone(); Callback::from(move |key: String| s.set(key))}
                    />
                    <div style="padding: 16px; font-size: 14px; color: var(--md-sys-color-on-surface-variant);">
                        { format!("Selected: {}", *selected_text_only) }
                    </div>
                </Demo>
                <CodeBlock code={"let selected = use_state(|| \"tab1\".to_string());\n\n<TabBar\n    tabs={vec![\n        TabItem { key: \"tab1\".into(), label: \"Albums\".into(), icon: String::new(), active: *selected == \"tab1\" },\n        TabItem { key: \"tab2\".into(), label: \"Photos\".into(), icon: String::new(), active: *selected == \"tab2\" },\n        TabItem { key: \"tab3\".into(), label: \"Shared\".into(), icon: String::new(), active: *selected == \"tab3\" },\n    ]}\n    on_select={let s = selected.clone(); Callback::from(move |key: String| s.set(key))}\n/>".to_string()} language={"rust".to_string()} />

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
                <CodeBlock code={"let selected = use_state(|| \"home\".to_string());\n\n<TabBar\n    tabs={vec![\n        TabItem { key: \"home\".into(), label: \"Home\".into(), icon: \"home\".into(), active: *selected == \"home\" },\n        TabItem { key: \"search\".into(), label: \"Search\".into(), icon: \"search\".into(), active: *selected == \"search\" },\n        TabItem { key: \"favorites\".into(), label: \"Favorites\".into(), icon: \"favorite\".into(), active: *selected == \"favorites\" },\n        TabItem { key: \"profile\".into(), label: \"Profile\".into(), icon: \"person\".into(), active: *selected == \"profile\" },\n    ]}\n    on_select={let s = selected.clone(); Callback::from(move |key: String| s.set(key))}\n/>".to_string()} language={"rust".to_string()} />

                // ── Scrollable Tabs (wide content) ──
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
                <CodeBlock code={"let selected = use_state(|| \"tab1\".to_string());\n\n<TabBar\n    scrollable={true}\n    tabs={vec![\n        TabItem { key: \"tab1\".into(), label: \"Overview\".into(), icon: String::new(), active: *selected == \"tab1\" },\n        TabItem { key: \"tab2\".into(), label: \"Analytics\".into(), icon: String::new(), active: *selected == \"tab2\" },\n        TabItem { key: \"tab3\".into(), label: \"Revenue\".into(), icon: String::new(), active: *selected == \"tab3\" },\n        TabItem { key: \"tab4\".into(), label: \"Audience\".into(), icon: String::new(), active: *selected == \"tab4\" },\n        TabItem { key: \"tab5\".into(), label: \"Conversions\".into(), icon: String::new(), active: *selected == \"tab5\" },\n        TabItem { key: \"tab6\".into(), label: \"Realtime\".into(), icon: String::new(), active: *selected == \"tab6\" },\n    ]}\n    on_select={let s = selected.clone(); Callback::from(move |key: String| s.set(key))}\n/>".to_string()} language={"rust".to_string()} />

                // ── Props Tables ──
                <h3 style="font-size: 16px; font-weight: 600; margin: 32px 0 12px;">{"TabItem"}</h3>
                <PropTable rows={tab_item_props} />

                <h3 style="font-size: 16px; font-weight: 600; margin: 32px 0 12px;">{"TabBar"}</h3>
                <PropTable rows={tab_bar_props} />
            </Section>
        </>
    }
}
