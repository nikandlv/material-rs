use yew::prelude::*;
use material_rs::components::*;
use material_rs::theme::Theme;

use super::{Section, Demo, CodeBlock, PropTable, PropRow};

#[function_component]
pub fn MenuPage() -> Html {
    let _theme = use_context::<Theme>().expect("Theme context required");

    let basic_open = use_state(|| false);
    let disabled_open = use_state(|| false);
    let icons_open = use_state(|| false);
    let anchor_open = use_state(|| false);

    let basic_open_cb = {
        let s = basic_open.clone();
        Callback::from(move |_: MouseEvent| s.set(true))
    };
    let basic_close_cb = {
        let s = basic_open.clone();
        Callback::from(move |_: ()| s.set(false))
    };
    let basic_select_cb = Callback::from(move |_key: String| {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"Selected".into());
    });
    let disabled_open_cb = {
        let s = disabled_open.clone();
        Callback::from(move |_: MouseEvent| s.set(true))
    };
    let disabled_close_cb = {
        let s = disabled_open.clone();
        Callback::from(move |_: ()| s.set(false))
    };
    let disabled_select_cb = Callback::from(move |_key: String| {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"Selected".into());
    });
    let icons_open_cb = {
        let s = icons_open.clone();
        Callback::from(move |_: MouseEvent| s.set(true))
    };
    let icons_close_cb = {
        let s = icons_open.clone();
        Callback::from(move |_: ()| s.set(false))
    };
    let icons_select_cb = Callback::from(move |_key: String| {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"Selected".into());
    });
    let anchor_open_cb = {
        let s = anchor_open.clone();
        Callback::from(move |_: MouseEvent| s.set(true))
    };
    let anchor_close_cb = {
        let s = anchor_open.clone();
        Callback::from(move |_: ()| s.set(false))
    };
    let anchor_select_cb = Callback::from(move |_key: String| {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"Selected".into());
    });

    let menu_props = vec![
        PropRow { name: "open".into(), r#type: "bool".into(), default_value: "false".into(), description: "Controls whether the menu is visible.".into() },
        PropRow { name: "items".into(), r#type: "Vec<MenuItem>".into(), default_value: "required".into(), description: "List of menu items to render.".into() },
        PropRow { name: "on_select".into(), r#type: "Callback<String>".into(), default_value: "default".into(), description: "Callback fired when an item is selected (passes the item key).".into() },
        PropRow { name: "on_close".into(), r#type: "Callback<()>".into(), default_value: "default".into(), description: "Callback fired when the menu is closed.".into() },
        PropRow { name: "anchor_id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id of the anchor element for positioning.".into() },
        PropRow { name: "match_anchor_width".into(), r#type: "bool".into(), default_value: "false".into(), description: "Whether the menu width should match the anchor's width.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    let item_props = vec![
        PropRow { name: "key".into(), r#type: "String".into(), default_value: "required".into(), description: "Unique identifier for the item.".into() },
        PropRow { name: "label".into(), r#type: "String".into(), default_value: "required".into(), description: "Display text for the menu item.".into() },
        PropRow { name: "icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Optional Material icon name displayed before the label.".into() },
        PropRow { name: "trailing_text".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Optional trailing text (e.g., keyboard shortcut).".into() },
        PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Whether the item is disabled and non-interactive.".into() },
    ];

    html! {
        <>
            <Section title="Menu">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Menus display a list of choices on a temporary surface. They appear \
                      when a user interacts with a button, action, or other control. \
                      Menus support icons, trailing text, disabled states, and smart \
                      positioning to stay within the viewport."}
                </p>

                <Demo title="Basic Menu">
                    <div style="display: flex; gap: 12px; align-items: center;">
                        <Button id="basic-menu-anchor" label="Open Menu" variant={ButtonVariant::FilledTonal}
                            onclick={basic_open_cb} />
                    </div>
                    <Menu
                        open={*basic_open}
                        anchor_id="basic-menu-anchor"
                        items={vec![
                            MenuItem { key: "profile".into(), label: "Profile".into(), icon: String::new(), trailing_text: String::new(), disabled: false },
                            MenuItem { key: "settings".into(), label: "Settings".into(), icon: String::new(), trailing_text: String::new(), disabled: false },
                            MenuItem { key: "help".into(), label: "Help & Support".into(), icon: String::new(), trailing_text: String::new(), disabled: false },
                        ]}
                        on_close={basic_close_cb}
                        on_select={basic_select_cb}
                    />
                </Demo>
                <CodeBlock code={"let open = use_state(|| false);\n\n<Button\n    id=\"menu-anchor\"\n    label=\"Open Menu\"\n    variant={ButtonVariant::FilledTonal}\n    onclick={let s = open.clone();\n        Callback::from(move |_: MouseEvent| s.set(true))}\n/>\n<Menu\n    open={*open}\n    anchor_id=\"menu-anchor\"\n    items={vec![\n        MenuItem { key: \"profile\".into(), label: \"Profile\".into(), ..default() },\n        MenuItem { key: \"settings\".into(), label: \"Settings\".into(), ..default() },\n        MenuItem { key: \"help\".into(), label: \"Help & Support\".into(), ..default() },\n    ]}\n    on_close={let s = open.clone();\n        Callback::from(move |_: ()| s.set(false))}\n    on_select={Callback::from(move |key: String| {\n        web_sys::console::log_1(&format!(\"Selected: {}\", key).into());\n    })}\n/>".to_string()} language={"rust".to_string()} />

                <Demo title="Disabled Items">
                    <div style="display: flex; gap: 12px; align-items: center;">
                        <Button id="disabled-menu-anchor" label="With Disabled Items" variant={ButtonVariant::FilledTonal}
                            onclick={disabled_open_cb} />
                    </div>
                    <Menu
                        open={*disabled_open}
                        anchor_id="disabled-menu-anchor"
                        items={vec![
                            MenuItem { key: "cut".into(), label: "Cut".into(), icon: "content_cut".into(), trailing_text: String::new(), disabled: false },
                            MenuItem { key: "copy".into(), label: "Copy".into(), icon: "content_copy".into(), trailing_text: String::new(), disabled: false },
                            MenuItem { key: "paste".into(), label: "Paste".into(), icon: "content_paste".into(), trailing_text: String::new(), disabled: true },
                            MenuItem { key: "delete".into(), label: "Delete".into(), icon: "delete".into(), trailing_text: String::new(), disabled: false },
                        ]}
                        on_close={disabled_close_cb}
                        on_select={disabled_select_cb}
                    />
                </Demo>
                <CodeBlock code={"<Menu\n    open={*open}\n    anchor_id=\"anchor\"\n    items={vec![\n        MenuItem { key: \"cut\".into(), label: \"Cut\".into(),\n            icon: \"content_cut\".into(), ..default() },\n        MenuItem { key: \"copy\".into(), label: \"Copy\".into(),\n            icon: \"content_copy\".into(), ..default() },\n        MenuItem { key: \"paste\".into(), label: \"Paste\".into(),\n            icon: \"content_paste\".into(), disabled: true },\n        MenuItem { key: \"delete\".into(), label: \"Delete\".into(),\n            icon: \"delete\".into(), ..default() },\n    ]}\n    on_close={let s = open.clone();\n        Callback::from(move |_: ()| s.set(false))}\n/>".to_string()} language={"rust".to_string()} />

                <Demo title="Icons & Trailing Text">
                    <div style="display: flex; gap: 12px; align-items: center;">
                        <Button id="icons-menu-anchor" label="Edit Menu" variant={ButtonVariant::FilledTonal}
                            onclick={icons_open_cb} />
                    </div>
                    <Menu
                        open={*icons_open}
                        anchor_id="icons-menu-anchor"
                        items={vec![
                            MenuItem { key: "undo".into(), label: "Undo".into(), icon: "undo".into(), trailing_text: "Ctrl+Z".into(), disabled: false },
                            MenuItem { key: "redo".into(), label: "Redo".into(), icon: "redo".into(), trailing_text: "Ctrl+Y".into(), disabled: false },
                            MenuItem { key: "cut".into(), label: "Cut".into(), icon: "content_cut".into(), trailing_text: "Ctrl+X".into(), disabled: false },
                            MenuItem { key: "copy".into(), label: "Copy".into(), icon: "content_copy".into(), trailing_text: "Ctrl+C".into(), disabled: false },
                            MenuItem { key: "paste".into(), label: "Paste".into(), icon: "content_paste".into(), trailing_text: "Ctrl+V".into(), disabled: false },
                        ]}
                        on_close={icons_close_cb}
                        on_select={icons_select_cb}
                    />
                </Demo>
                <CodeBlock code={"<Menu\n    open={*open}\n    anchor_id=\"anchor\"\n    items={vec![\n        MenuItem { key: \"undo\".into(), label: \"Undo\".into(),\n            icon: \"undo\".into(), trailing_text: \"Ctrl+Z\".into(), ..default() },\n        MenuItem { key: \"redo\".into(), label: \"Redo\".into(),\n            icon: \"redo\".into(), trailing_text: \"Ctrl+Y\".into(), ..default() },\n        MenuItem { key: \"cut\".into(), label: \"Cut\".into(),\n            icon: \"content_cut\".into(), trailing_text: \"Ctrl+X\".into(), ..default() },\n        MenuItem { key: \"copy\".into(), label: \"Copy\".into(),\n            icon: \"content_copy\".into(), trailing_text: \"Ctrl+C\".into(), ..default() },\n        MenuItem { key: \"paste\".into(), label: \"Paste\".into(),\n            icon: \"content_paste\".into(), trailing_text: \"Ctrl+V\".into(), ..default() },\n    ]}\n    on_close={let s = open.clone();\n        Callback::from(move |_: ()| s.set(false))}\n/>".to_string()} language={"rust".to_string()} />

                <Demo title="Anchor-Width Matching">
                    <div style="display: flex; gap: 12px; align-items: center; width: 300px;">
                        <Button id="anchor-width-menu" label="Full Width Menu" variant={ButtonVariant::FilledTonal}
                            style="width: 100%;"
                            onclick={anchor_open_cb} />
                    </div>
                    <Menu
                        open={*anchor_open}
                        anchor_id="anchor-width-menu"
                        match_anchor_width={true}
                        items={vec![
                            MenuItem { key: "small".into(), label: "Small".into(), icon: String::new(), trailing_text: String::new(), disabled: false },
                            MenuItem { key: "medium".into(), label: "Medium".into(), icon: String::new(), trailing_text: String::new(), disabled: false },
                            MenuItem { key: "large".into(), label: "Large".into(), icon: String::new(), trailing_text: String::new(), disabled: false },
                        ]}
                        on_close={anchor_close_cb}
                        on_select={anchor_select_cb}
                    />
                </Demo>
                <CodeBlock code={"<Button\n    id=\"anchor\"\n    label=\"Full Width Menu\"\n    style=\"width: 100%;\"\n    onclick={...}\n/>\n<Menu\n    open={*open}\n    anchor_id=\"anchor\"\n    match_anchor_width={true}\n    items={vec![\n        MenuItem { key: \"small\".into(), label: \"Small\".into(), ..default() },\n        MenuItem { key: \"medium\".into(), label: \"Medium\".into(), ..default() },\n        MenuItem { key: \"large\".into(), label: \"Large\".into(), ..default() },\n    ]}\n    on_close={let s = open.clone();\n        Callback::from(move |_: ()| s.set(false))}\n/>".to_string()} language={"rust".to_string()} />

                <PropTable rows={menu_props} />
            </Section>

            <Section title="MenuItem">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Each item in a menu is represented by a MenuItem struct. Items can include \
                      an icon, trailing text for shortcuts, and a disabled state."}
                </p>
                <PropTable rows={item_props} />
            </Section>
        </>
    }
}
