use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn NavigationBarPage() -> Html {
    let nav_dest_props = vec![
        PropRow { name: "key".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Unique identifier for the destination. Used as the value in on_select.".into() },
        PropRow { name: "icon".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Material icon name displayed for the destination.".into() },
        PropRow { name: "active_icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Icon shown when the destination is active. Falls back to icon if empty.".into() },
        PropRow { name: "label".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Text label displayed below the icon.".into() },
        PropRow { name: "active".into(), r#type: "bool".into(), default_value: "false".into(), description: "Whether this destination is currently selected.".into() },
    ];

    let nav_bar_props = vec![
        PropRow { name: "destinations".into(), r#type: "Vec<NavDestination>".into(), default_value: "(required)".into(), description: "Vector of navigation destinations to render (3–5 items recommended).".into() },
        PropRow { name: "on_select".into(), r#type: "Callback<String>".into(), default_value: "Callback::noop()".into(), description: "Fires with the key of the destination the user selected.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    // Demo state
    let selected_three = use_state(|| "home".to_string());
    let selected_five = use_state(|| "home".to_string());
    let selected_badge = use_state(|| "home".to_string());
    let selected_active = use_state(|| "favorites".to_string());

    let three_code = r#"let selected = use_state(|| "home".to_string());

<NavigationBar
    destinations={vec![
        NavDestination { key: "home".into(), icon: "home".into(), active_icon: String::new(), label: "Home".into(), active: *selected == "home" },
        NavDestination { key: "search".into(), icon: "search".into(), active_icon: String::new(), label: "Search".into(), active: *selected == "search" },
        NavDestination { key: "favorites".into(), icon: "favorite".into(), active_icon: String::new(), label: "Favorites".into(), active: *selected == "favorites" },
    ]}
    on_select={let s = selected.clone(); Callback::from(move |key: String| s.set(key))}
/>"#;

    let five_code = r#"let selected = use_state(|| "home".to_string());

<NavigationBar
    destinations={vec![
        NavDestination { key: "home".into(), icon: "home".into(), active_icon: String::new(), label: "Home".into(), active: *selected == "home" },
        NavDestination { key: "search".into(), icon: "search".into(), active_icon: String::new(), label: "Search".into(), active: *selected == "search" },
        NavDestination { key: "favorites".into(), icon: "favorite".into(), active_icon: String::new(), label: "Favorites".into(), active: *selected == "favorites" },
        NavDestination { key: "notifications".into(), icon: "notifications".into(), active_icon: String::new(), label: "Alerts".into(), active: *selected == "notifications" },
        NavDestination { key: "profile".into(), icon: "person".into(), active_icon: String::new(), label: "Profile".into(), active: *selected == "profile" },
    ]}
    on_select={let s = selected.clone(); Callback::from(move |key: String| s.set(key))}
/>"#;

    let badge_code = r#"let selected = use_state(|| "home".to_string());

// Badge is applied via the icon container; wrap NavigationBar in a parent
// that positions the badge over the icon for the "notifications" item.
<NavigationBar
    destinations={vec![
        NavDestination { key: "home".into(), icon: "home".into(), active_icon: String::new(), label: "Home".into(), active: *selected == "home" },
        NavDestination { key: "search".into(), icon: "search".into(), active_icon: String::new(), label: "Search".into(), active: *selected == "search" },
        NavDestination { key: "notifications".into(), icon: "notifications".into(), active_icon: String::new(), label: "Alerts".into(), active: *selected == "notifications" },
        NavDestination { key: "favorites".into(), icon: "favorite".into(), active_icon: String::new(), label: "Favorites".into(), active: *selected == "favorites" },
    ]}
    on_select={let s = selected.clone(); Callback::from(move |key: String| s.set(key))}
/>"#;

    let active_code = r#"let selected = use_state(|| "favorites".to_string());

<NavigationBar
    destinations={vec![
        NavDestination { key: "home".into(), icon: "home".into(), active_icon: "home".into(), label: "Home".into(), active: *selected == "home" },
        NavDestination { key: "search".into(), icon: "search".into(), active_icon: "manage_search".into(), label: "Search".into(), active: *selected == "search" },
        NavDestination { key: "favorites".into(), icon: "favorite_border".into(), active_icon: "favorite".into(), label: "Favorites".into(), active: *selected == "favorites" },
        NavDestination { key: "profile".into(), icon: "person".into(), active_icon: "person".into(), label: "Profile".into(), active: *selected == "profile" },
    ]}
    on_select={let s = selected.clone(); Callback::from(move |key: String| s.set(key))}
/>"#;

    html! {
        <>
            <Section title="Navigation Bar">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"A navigation bar is a bottom navigation component for mobile layouts. It displays \
                      3 to 5 destinations as icons with labels. The active destination is highlighted \
                      with a secondary container indicator pill."}
                </p>

                // ── 3-Item Bottom Nav ──
                <Demo title="Bottom Navigation (3 Items)">
                    <div style="width: 100%; max-width: 400px;">
                        <NavigationBar
                            destinations={vec![
                                NavDestination { key: "home".into(), icon: "home".into(), active_icon: String::new(), label: "Home".into(), active: *selected_three == "home" },
                                NavDestination { key: "search".into(), icon: "search".into(), active_icon: String::new(), label: "Search".into(), active: *selected_three == "search" },
                                NavDestination { key: "favorites".into(), icon: "favorite".into(), active_icon: String::new(), label: "Favorites".into(), active: *selected_three == "favorites" },
                            ]}
                            on_select={let s = selected_three.clone(); Callback::from(move |key: String| s.set(key))}
                        />
                    </div>
                    <div style="padding: 16px; font-size: 14px; color: var(--md-sys-color-on-surface-variant);">
                        { format!("Selected: {}", *selected_three) }
                    </div>
                </Demo>
                <CodeBlock code={three_code.to_string()} language={"rust".to_string()} />

                // ── 5-Item Bottom Nav ──
                <Demo title="Bottom Navigation (5 Items)">
                    <div style="width: 100%; max-width: 500px;">
                        <NavigationBar
                            destinations={vec![
                                NavDestination { key: "home".into(), icon: "home".into(), active_icon: String::new(), label: "Home".into(), active: *selected_five == "home" },
                                NavDestination { key: "search".into(), icon: "search".into(), active_icon: String::new(), label: "Search".into(), active: *selected_five == "search" },
                                NavDestination { key: "favorites".into(), icon: "favorite".into(), active_icon: String::new(), label: "Favorites".into(), active: *selected_five == "favorites" },
                                NavDestination { key: "notifications".into(), icon: "notifications".into(), active_icon: String::new(), label: "Alerts".into(), active: *selected_five == "notifications" },
                                NavDestination { key: "profile".into(), icon: "person".into(), active_icon: String::new(), label: "Profile".into(), active: *selected_five == "profile" },
                            ]}
                            on_select={let s = selected_five.clone(); Callback::from(move |key: String| s.set(key))}
                        />
                    </div>
                    <div style="padding: 16px; font-size: 14px; color: var(--md-sys-color-on-surface-variant);">
                        { format!("Selected: {}", *selected_five) }
                    </div>
                </Demo>
                <CodeBlock code={five_code.to_string()} language={"rust".to_string()} />

                // ── Bottom Nav with Badge ──
                <Demo title="Bottom Navigation with Badge">
                    <div style="width: 100%; max-width: 400px;">
                        <NavigationBar
                            destinations={vec![
                                NavDestination { key: "home".into(), icon: "home".into(), active_icon: String::new(), label: "Home".into(), active: *selected_badge == "home" },
                                NavDestination { key: "search".into(), icon: "search".into(), active_icon: String::new(), label: "Search".into(), active: *selected_badge == "search" },
                                NavDestination { key: "notifications".into(), icon: "notifications".into(), active_icon: String::new(), label: "Alerts".into(), active: *selected_badge == "notifications" },
                                NavDestination { key: "favorites".into(), icon: "favorite".into(), active_icon: String::new(), label: "Favorites".into(), active: *selected_badge == "favorites" },
                            ]}
                            on_select={let s = selected_badge.clone(); Callback::from(move |key: String| s.set(key))}
                        />
                    </div>
                    <div style="padding: 16px; font-size: 14px; color: var(--md-sys-color-on-surface-variant);">
                        { format!("Selected: {}", *selected_badge) }
                    </div>
                </Demo>
                <CodeBlock code={badge_code.to_string()} language={"rust".to_string()} />

                // ── Bottom Nav with Active Selection ──
                <Demo title="Bottom Navigation with Active Icons">
                    <div style="width: 100%; max-width: 400px;">
                        <NavigationBar
                            destinations={vec![
                                NavDestination { key: "home".into(), icon: "home".into(), active_icon: "home".into(), label: "Home".into(), active: *selected_active == "home" },
                                NavDestination { key: "search".into(), icon: "search".into(), active_icon: "manage_search".into(), label: "Search".into(), active: *selected_active == "search" },
                                NavDestination { key: "favorites".into(), icon: "favorite_border".into(), active_icon: "favorite".into(), label: "Favorites".into(), active: *selected_active == "favorites" },
                                NavDestination { key: "profile".into(), icon: "person".into(), active_icon: "person".into(), label: "Profile".into(), active: *selected_active == "profile" },
                            ]}
                            on_select={let s = selected_active.clone(); Callback::from(move |key: String| s.set(key))}
                        />
                    </div>
                    <div style="padding: 16px; font-size: 14px; color: var(--md-sys-color-on-surface-variant);">
                        { format!("Selected: {}", *selected_active) }
                    </div>
                </Demo>
                <CodeBlock code={active_code.to_string()} language={"rust".to_string()} />

                // ── Props Tables ──
                <h3 style="font-size: 16px; font-weight: 600; margin: 32px 0 12px;">{"NavDestination"}</h3>
                <PropTable rows={nav_dest_props} />

                <h3 style="font-size: 16px; font-weight: 600; margin: 32px 0 12px;">{"NavigationBar"}</h3>
                <PropTable rows={nav_bar_props} />
            </Section>
        </>
    }
}
