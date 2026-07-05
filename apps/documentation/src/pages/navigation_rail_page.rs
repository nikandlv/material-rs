use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn NavigationRailPage() -> Html {
    let rail_dest_props = vec![
        PropRow { name: "key".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Unique identifier for the destination. Used as the value in on_select.".into() },
        PropRow { name: "icon".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Material icon name displayed for the destination.".into() },
        PropRow { name: "active_icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Icon shown when the destination is active. Falls back to icon if empty.".into() },
        PropRow { name: "label".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Text label displayed below the icon.".into() },
        PropRow { name: "active".into(), r#type: "bool".into(), default_value: "false".into(), description: "Whether this destination is currently selected.".into() },
    ];

    let rail_props = vec![
        PropRow { name: "destinations".into(), r#type: "Vec<RailDestination>".into(), default_value: "(required)".into(), description: "Vector of navigation destinations to render.".into() },
        PropRow { name: "on_select".into(), r#type: "Callback<String>".into(), default_value: "Callback::noop()".into(), description: "Fires with the key of the destination the user selected.".into() },
        PropRow { name: "header".into(), r#type: "Children".into(), default_value: "None".into(), description: "Header content rendered above destinations, typically a FAB.".into() },
        PropRow { name: "footer".into(), r#type: "Children".into(), default_value: "None".into(), description: "Footer content rendered below destinations.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    // Demo state
    let selected_basic = use_state(|| "home".to_string());
    let selected_header = use_state(|| "home".to_string());
    let selected_footer = use_state(|| "home".to_string());
    let selected_active = use_state(|| "favorites".to_string());

    let basic_code = r#"let selected = use_state(|| "home".to_string());

<NavigationRail
    destinations={vec![
        RailDestination { key: "home".into(), icon: "home".into(), active_icon: String::new(), label: "Home".into(), active: *selected == "home" },
        RailDestination { key: "search".into(), icon: "search".into(), active_icon: String::new(), label: "Search".into(), active: *selected == "search" },
        RailDestination { key: "favorites".into(), icon: "favorite".into(), active_icon: String::new(), label: "Favorites".into(), active: *selected == "favorites" },
    ]}
    on_select={let s = selected.clone(); Callback::from(move |key: String| s.set(key))}
/>"#;

    let header_code = r#"let selected = use_state(|| "home".to_string());

<NavigationRail
    destinations={vec![
        RailDestination { key: "home".into(), icon: "home".into(), active_icon: String::new(), label: "Home".into(), active: *selected == "home" },
        RailDestination { key: "search".into(), icon: "search".into(), active_icon: String::new(), label: "Search".into(), active: *selected == "search" },
        RailDestination { key: "favorites".into(), icon: "favorite".into(), active_icon: String::new(), label: "Favorites".into(), active: *selected == "favorites" },
    ]}
    header={html! { <Button variant={ButtonVariant::Fab} icon="add" /> }}
    on_select={let s = selected.clone(); Callback::from(move |key: String| s.set(key))}
/>"#;

    let footer_code = r#"let selected = use_state(|| "home".to_string());

<NavigationRail
    destinations={vec![
        RailDestination { key: "home".into(), icon: "home".into(), active_icon: String::new(), label: "Home".into(), active: *selected == "home" },
        RailDestination { key: "settings".into(), icon: "settings".into(), active_icon: String::new(), label: "Settings".into(), active: *selected == "settings" },
    ]}
    footer={html! {
        <IconButton icon="settings" variant={IconButtonVariant::Standard} />
    }}
    on_select={let s = selected.clone(); Callback::from(move |key: String| s.set(key))}
/>"#;

    let active_code = r#"let selected = use_state(|| "favorites".to_string());

<NavigationRail
    destinations={vec![
        RailDestination { key: "home".into(), icon: "home".into(), active_icon: "home".into(), label: "Home".into(), active: *selected == "home" },
        RailDestination { key: "search".into(), icon: "search".into(), active_icon: "manage_search".into(), label: "Search".into(), active: *selected == "search" },
        RailDestination { key: "favorites".into(), icon: "favorite_border".into(), active_icon: "favorite".into(), label: "Favorites".into(), active: *selected == "favorites" },
    ]}
    on_select={let s = selected.clone(); Callback::from(move |key: String| s.set(key))}
/>"#;

    html! {
        <>
            <Section title="Navigation Rail">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"A navigation rail is a side navigation component for wider layouts (tablets and desktops). \
                      It displays 3 to 7 destinations as icons with labels. The rail can include a header \
                      area for a FAB and a footer area for additional actions."}
                </p>

                // ── Basic Rail ──
                <Demo title="Basic Navigation Rail">
                    <div style="display: flex; height: 320px;">
                        <NavigationRail
                            destinations={vec![
                                RailDestination { key: "home".into(), icon: "home".into(), active_icon: String::new(), label: "Home".into(), active: *selected_basic == "home" },
                                RailDestination { key: "search".into(), icon: "search".into(), active_icon: String::new(), label: "Search".into(), active: *selected_basic == "search" },
                                RailDestination { key: "favorites".into(), icon: "favorite".into(), active_icon: String::new(), label: "Favorites".into(), active: *selected_basic == "favorites" },
                                RailDestination { key: "settings".into(), icon: "settings".into(), active_icon: String::new(), label: "Settings".into(), active: *selected_basic == "settings" },
                            ]}
                            on_select={let s = selected_basic.clone(); Callback::from(move |key: String| s.set(key))}
                        />
                        <div style="flex: 1; padding: 24px; font-size: 14px; color: var(--md-sys-color-on-surface-variant);">
                            { format!("Selected: {}", *selected_basic) }
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={basic_code.to_string()} language={"rust".to_string()} />

                // ── Rail with Header FAB ──
                <Demo title="Navigation Rail with Header FAB">
                    <div style="display: flex; height: 320px;">
                        <NavigationRail
                            destinations={vec![
                                RailDestination { key: "home".into(), icon: "home".into(), active_icon: String::new(), label: "Home".into(), active: *selected_header == "home" },
                                RailDestination { key: "search".into(), icon: "search".into(), active_icon: String::new(), label: "Search".into(), active: *selected_header == "search" },
                                RailDestination { key: "favorites".into(), icon: "favorite".into(), active_icon: String::new(), label: "Favorites".into(), active: *selected_header == "favorites" },
                            ]}
                            header={html! { <Button variant={ButtonVariant::Fab} icon="add" /> }}
                            on_select={let s = selected_header.clone(); Callback::from(move |key: String| s.set(key))}
                        />
                        <div style="flex: 1; padding: 24px; font-size: 14px; color: var(--md-sys-color-on-surface-variant);">
                            { format!("Selected: {}", *selected_header) }
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={header_code.to_string()} language={"rust".to_string()} />

                // ── Rail with Footer ──
                <Demo title="Navigation Rail with Footer">
                    <div style="display: flex; height: 320px;">
                        <NavigationRail
                            destinations={vec![
                                RailDestination { key: "home".into(), icon: "home".into(), active_icon: String::new(), label: "Home".into(), active: *selected_footer == "home" },
                                RailDestination { key: "settings".into(), icon: "settings".into(), active_icon: String::new(), label: "Settings".into(), active: *selected_footer == "settings" },
                            ]}
                            footer={html! {
                                <IconButton icon="settings" variant={IconButtonVariant::Standard} />
                            }}
                            on_select={let s = selected_footer.clone(); Callback::from(move |key: String| s.set(key))}
                        />
                        <div style="flex: 1; padding: 24px; font-size: 14px; color: var(--md-sys-color-on-surface-variant);">
                            { format!("Selected: {}", *selected_footer) }
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={footer_code.to_string()} language={"rust".to_string()} />

                // ── Rail with Active Selection ──
                <Demo title="Navigation Rail with Active Selection">
                    <div style="display: flex; height: 320px;">
                        <NavigationRail
                            destinations={vec![
                                RailDestination { key: "home".into(), icon: "home".into(), active_icon: "home".into(), label: "Home".into(), active: *selected_active == "home" },
                                RailDestination { key: "search".into(), icon: "search".into(), active_icon: "manage_search".into(), label: "Search".into(), active: *selected_active == "search" },
                                RailDestination { key: "favorites".into(), icon: "favorite_border".into(), active_icon: "favorite".into(), label: "Favorites".into(), active: *selected_active == "favorites" },
                            ]}
                            on_select={let s = selected_active.clone(); Callback::from(move |key: String| s.set(key))}
                        />
                        <div style="flex: 1; padding: 24px; font-size: 14px; color: var(--md-sys-color-on-surface-variant);">
                            { format!("Selected: {}", *selected_active) }
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={active_code.to_string()} language={"rust".to_string()} />

                // ── Props Tables ──
                <h3 style="font-size: 16px; font-weight: 600; margin: 32px 0 12px;">{"RailDestination"}</h3>
                <PropTable rows={rail_dest_props} />

                <h3 style="font-size: 16px; font-weight: 600; margin: 32px 0 12px;">{"NavigationRail"}</h3>
                <PropTable rows={rail_props} />
            </Section>
        </>
    }
}
