//! Material Design 3 Expressive Breadcrumb
//!
//! A navigation breadcrumb trail showing the hierarchy of the current page.
//! Follows MD3 expressive design with bold separators and hover animations.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

/// A single breadcrumb item.
#[derive(Clone, PartialEq)]
pub struct BreadcrumbItem {
    /// Display label for this breadcrumb.
    pub label: String,
    /// Optional link URL. If empty, the item is rendered as plain text (current page).
    pub href: String,
    /// Optional leading icon (Material Symbols ligature name).
    pub icon: String,
}

#[derive(Properties, PartialEq)]
pub struct BreadcrumbProps {
    /// List of breadcrumb items. The last item is treated as the current page.
    pub items: Vec<BreadcrumbItem>,

    /// Separator element between items. Defaults to "/".
    #[prop_or("/".into())]
    pub separator: String,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn Breadcrumb(props: &BreadcrumbProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let nav_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        flex-wrap: wrap;
        gap: 4px;
        font-family: ${font_family}, sans-serif;
        font-size: 14px;
        padding: 8px 0;
        "#,
        font_family = theme.font_family,
    );

    let link_style = use_style!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: 6px;
        padding: 4px 8px;
        border-radius: ${radius};
        color: ${link_color};
        text-decoration: none;
        font-weight: 500;
        font-size: 14px;
        transition: background-color 150ms cubic-bezier(0.2, 0, 0, 1),
                    color 150ms cubic-bezier(0.2, 0, 0, 1),
                    transform 100ms cubic-bezier(0.34, 1.56, 0.64, 1);
        outline: none;
        -webkit-tap-highlight-color: transparent;
        cursor: pointer;

        &:hover {
            background-color: ${hover_bg};
            transform: translateY(-1px);
        }

        &:active {
            transform: scale(0.97);
        }

        &:focus-visible {
            outline: 2px solid ${focus_ring};
            outline-offset: 2px;
        }
        "#,
        radius = theme.shapes.extra_small,
        link_color = theme.colors.primary,
        hover_bg = with_alpha(&theme.colors.primary, 0.08).unwrap_or_default(),
        focus_ring = theme.colors.primary,
    );

    let current_style = use_style!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: 6px;
        padding: 4px 8px;
        border-radius: ${radius};
        color: ${color};
        font-weight: 600;
        font-size: 14px;
        letter-spacing: 0.1px;
        "#,
        radius = theme.shapes.extra_small,
        color = theme.colors.on_surface,
    );

    let separator_style = format!(
        "color: {}; font-size: 16px; user-select: none; padding: 0 2px; flex-shrink: 0;",
        theme.colors.on_surface_variant
    );

    let component_override = theme.component_style("Breadcrumb").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    let items_html: Vec<Html> = props
        .items
        .iter()
        .enumerate()
        .flat_map(|(idx, item)| {
            let is_last = idx == props.items.len() - 1;
            let mut elements = vec![];

            // Separator before non-first items
            if idx > 0 {
                elements.push(html! {
                    <span style={separator_style.clone()} aria-hidden="true">
                        { &props.separator }
                    </span>
                });
            }

            if is_last {
                // Current page — not a link
                elements.push(html! {
                    <span
                        class={current_style.get_class_name().to_string()}
                        aria-current="page"
                    >
                        if !item.icon.is_empty() {
                            <Icon name={item.icon.clone()} size="18px" />
                        }
                        { &item.label }
                    </span>
                });
            } else {
                // Navigable ancestor
                elements.push(html! {
                    <a
                        class={link_style.get_class_name().to_string()}
                        href={item.href.clone()}
                    >
                        if !item.icon.is_empty() {
                            <div style="pointer-events: none; display: inline-flex; align-items: center;">
                                <Icon name={item.icon.clone()} size="18px" />
                            </div>
                        }
                        <span style="pointer-events: none;">{ &item.label }</span>
                    </a>
                });
            }

            elements
        })
        .collect();

    html! {
        <nav
            class={yew::classes![nav_style.get_class_name().to_string(), &props.class, &component_override]}
            id={props.id.clone()}
            aria-label="Breadcrumb"
        >
            <ol style="display: flex; align-items: center; flex-wrap: wrap; gap: 4px; list-style: none; margin: 0; padding: 0;">
                { for items_html.into_iter().map(|item| html! { <li>{ item }</li> }) }
            </ol>
        </nav>
    }
}
