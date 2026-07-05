use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn BreadcrumbPage() -> Html {
    let breadcrumb_props = vec![
        PropRow { name: "items".into(), r#type: "Vec<BreadcrumbItem>".into(), default_value: "required".into(), description: "List of breadcrumb items to render. The last item is shown as the current page (non-clickable).".into() },
        PropRow { name: "separator".into(), r#type: "String".into(), default_value: "\"/\"".into(), description: "Character or string displayed between each breadcrumb item.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
    ];

    let item_props = vec![
        PropRow { name: "label".into(), r#type: "String".into(), default_value: "required".into(), description: "Display text for this breadcrumb item.".into() },
        PropRow { name: "href".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "URL for the link. Empty string renders a non-link item (current page).".into() },
        PropRow { name: "icon".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Optional Material icon name displayed before the label.".into() },
    ];

    let default_code = r#"<Breadcrumb items={vec![
    BreadcrumbItem { label: "Home".into(), href: "/".into(), icon: String::new() },
    BreadcrumbItem { label: "Library".into(), href: "/library".into(), icon: String::new() },
    BreadcrumbItem { label: "Data".into(), href: String::new(), icon: String::new() },
]} />"#;

    let separator_code = r#"<Breadcrumb separator={">".to_string()} items={vec![
    BreadcrumbItem { label: "Home".into(), href: "/".into(), icon: String::new() },
    BreadcrumbItem { label: "Docs".into(), href: "/docs".into(), icon: String::new() },
    BreadcrumbItem { label: "Components".into(), href: "/docs/components".into(), icon: String::new() },
    BreadcrumbItem { label: "Breadcrumb".into(), href: String::new(), icon: String::new() },
]} />"#;

    let icon_code = r#"<Breadcrumb separator={">".to_string()} items={vec![
    BreadcrumbItem { label: "Home".into(), href: "/".into(), icon: "home".into() },
    BreadcrumbItem { label: "Projects".into(), href: "/projects".into(), icon: "folder".into() },
    BreadcrumbItem { label: "Material RS".into(), href: String::new(), icon: "code".into() },
]} />"#;

    html! {
        <>
            <Section title="Breadcrumb">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Breadcrumbs show the user's current location within a navigational \
                      hierarchy. They provide links back to each previous page the user \
                      visited and can include icons alongside labels."}
                </p>

                <Demo title="Default Breadcrumb">
                    <div style="width: 100%; max-width: 560px;">
                        <Breadcrumb items={vec![
                            BreadcrumbItem { label: "Home".into(), href: "/".into(), icon: String::new() },
                            BreadcrumbItem { label: "Library".into(), href: "/library".into(), icon: String::new() },
                            BreadcrumbItem { label: "Data".into(), href: String::new(), icon: String::new() },
                        ]} />
                    </div>
                </Demo>
                <CodeBlock code={default_code.to_string()} language={"rust".to_string()} />

                <Demo title="Custom Separator">
                    <div style="width: 100%; max-width: 560px;">
                        <Breadcrumb separator={">".to_string()} items={vec![
                            BreadcrumbItem { label: "Home".into(), href: "/".into(), icon: String::new() },
                            BreadcrumbItem { label: "Docs".into(), href: "/docs".into(), icon: String::new() },
                            BreadcrumbItem { label: "Components".into(), href: "/docs/components".into(), icon: String::new() },
                            BreadcrumbItem { label: "Breadcrumb".into(), href: String::new(), icon: String::new() },
                        ]} />
                    </div>
                </Demo>
                <CodeBlock code={separator_code.to_string()} language={"rust".to_string()} />

                <Demo title="Breadcrumb with Icons">
                    <div style="width: 100%; max-width: 560px;">
                        <Breadcrumb separator={">".to_string()} items={vec![
                            BreadcrumbItem { label: "Home".into(), href: "/".into(), icon: "home".into() },
                            BreadcrumbItem { label: "Projects".into(), href: "/projects".into(), icon: "folder".into() },
    BreadcrumbItem { label: "Material RS".into(), href: String::new(), icon: "code".into() },
                        ]} />
                    </div>
                </Demo>
                <CodeBlock code={icon_code.to_string()} language={"rust".to_string()} />

                <PropTable rows={breadcrumb_props} />
            </Section>

            <Section title="BreadcrumbItem">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Each item in a breadcrumb trail is represented by a BreadcrumbItem. \
                      Items with an href render as clickable links; the last item (the current \
                      page) typically omits the href and is styled as active."}
                </p>
                <PropTable rows={item_props} />
            </Section>
        </>
    }
}
