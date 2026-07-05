pub mod accordions;
pub mod alert_box_page;
pub mod avatars_shapes;
pub mod badges;
pub mod bottom_sheet_page;
pub mod box_page;
pub mod breadcrumb_page;
pub mod buttons_page;
pub mod cards;
pub mod carousel_page;
pub mod chips;
pub mod code_view_page;
pub mod container_page;
pub mod dialog_page;
pub mod divider_page;
pub mod grid_page;
pub mod home;
pub mod icon_page;
pub mod image_list_page;
pub mod landing;
pub mod list_page;
pub mod menu_page;
pub mod navigation_bar_page;
pub mod navigation_drawer_page;
pub mod navigation_rail_page;
pub mod not_found;
pub mod progress_page;
pub mod ripple_page;
pub mod select_textarea;
pub mod selection;
pub mod shape_gallery;
pub mod showcase_admin;
pub mod showcase_blog;
pub mod showcase_ecommerce;
pub mod showcase_email;
pub mod showcase_login;
pub mod showcase_settings;
pub mod skeleton_page;
pub mod slider;
pub mod snackbar_page;
pub mod spacer_page;
pub mod split_buttons;
pub mod steppers_page;
pub mod tab_bar_page;
pub mod tables_page;
pub mod tabs_page;
pub mod text_fields;
pub mod theme_tokens;
pub mod toggle_buttons;
pub mod toolbar_page;
pub mod tooltip_page;
pub mod top_app_bar_page;
pub mod typography_page;

pub use accordions::AccordionsPage;
pub use alert_box_page::AlertBoxPage;
pub use avatars_shapes::AvatarsShapesPage;
pub use badges::BadgesPage;
pub use bottom_sheet_page::BottomSheetPage;
pub use box_page::BoxPage;
pub use breadcrumb_page::BreadcrumbPage;
pub use buttons_page::ButtonsPage;
pub use cards::CardsPage;
pub use carousel_page::CarouselPage;
pub use chips::ChipsPage;
pub use code_view_page::CodeViewPage;
pub use container_page::ContainerPage;
pub use dialog_page::DialogPage;
pub use divider_page::DividerPage;
pub use grid_page::GridPage;
pub use home::HomePage;
pub use icon_page::IconPage;
pub use image_list_page::ImageListPage;
pub use landing::LandingPage;
pub use list_page::ListPage;
pub use menu_page::MenuPage;
pub use navigation_bar_page::NavigationBarPage;
pub use navigation_drawer_page::NavigationDrawerPage;
pub use navigation_rail_page::NavigationRailPage;
pub use not_found::NotFoundPage;
pub use progress_page::ProgressPage;
pub use ripple_page::RipplePage;
pub use select_textarea::SelectTextareaPage;
pub use selection::SelectionPage;
pub use shape_gallery::ShapeGalleryPage;
pub use showcase_admin::ShowcaseAdminPage;
pub use showcase_blog::ShowcaseBlogPage;
pub use showcase_ecommerce::ShowcaseEcommercePage;
pub use showcase_email::ShowcaseEmailPage;
pub use showcase_login::ShowcaseLoginPage;
pub use showcase_settings::ShowcaseSettingsPage;
pub use skeleton_page::SkeletonPage;
pub use slider::SliderPage;
pub use snackbar_page::SnackbarPage;
pub use spacer_page::SpacerPage;
pub use split_buttons::SplitButtonsPage;
pub use steppers_page::SteppersPage;
pub use tab_bar_page::TabBarPage;
pub use tables_page::TablesPage;
pub use tabs_page::TabsPage;
pub use text_fields::TextFieldsPage;
pub use theme_tokens::ThemeTokensPage;
pub use toggle_buttons::ToggleButtonsPage;
pub use toolbar_page::ToolbarPage;
pub use tooltip_page::TooltipPage;
pub use top_app_bar_page::TopAppBarPage;
pub use typography_page::TypographyPage;

use material_rs::theme::Theme;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
use yew::prelude::*;

// ── Shared doc components ──

#[function_component]
pub fn Section(props: &SectionProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    html! {
        <section style="margin-bottom: 40px;">
            <h2 style={format!(
                "font-size: 14px; font-weight: 500; letter-spacing: 0.1px; \
                 color: {}; margin-bottom: 16px; text-transform: uppercase;",
                theme.colors.primary,
            )}>
                { &props.title }
            </h2>
            { props.children.clone() }
        </section>
    }
}

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub title: String,
    pub children: Children,
}

/// Renders a code snippet with a copy button.
#[function_component]
pub fn CodeBlock(props: &CodeBlockProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let copied = use_state(|| false);

    let on_copy = {
        let _code = props.code.clone();
        let _copied = copied.clone();
        Callback::from(move |_: MouseEvent| {
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(window) = web_sys::window() {
                    let nav = window.navigator();
                    let clipboard = nav.clipboard();
                    let c = _code.clone();
                    let cs = _copied.clone();
                    let _ = clipboard.write_text(&c);
                    cs.set(true);
                    let cs2 = cs.clone();
                    let closure = wasm_bindgen::closure::Closure::once(move || {
                        cs2.set(false);
                    });
                    let _ = web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            closure.as_ref().unchecked_ref(),
                            2000,
                        );
                    closure.forget();
                }
            }
        })
    };

    html! {
        <div style={format!(
            "border-radius: 12px; overflow: hidden; border: 1px solid {}; margin: 16px 0;",
            theme.colors.outline_variant,
        )}>
            <div style={format!(
                "display: flex; align-items: center; justify-content: space-between; \
                 padding: 6px 12px; background-color: {}; border-bottom: 1px solid {};",
                theme.colors.surface_container_high, theme.colors.outline_variant,
            )}>
                if let Some(lang) = &props.language {
                    <span style={format!("font-size: 11px; color: {}; font-weight: 500;", theme.colors.on_surface_variant)}>
                        { lang }
                    </span>
                } else {
                    <span />
                }
                <button
                    onclick={on_copy}
                    style={format!(
                        "display: flex; align-items: center; gap: 4px; padding: 4px 10px; \
                         border: 1px solid {}; border-radius: 6px; background: transparent; \
                         color: {}; font-size: 12px; cursor: pointer; font-family: inherit;",
                        theme.colors.outline, theme.colors.on_surface_variant,
                    )}
                >
                    <span class="material-symbols-outlined" style="font-size: 16px;">
                        { if *copied { "check" } else { "content_copy" } }
                    </span>
                    { if *copied { "Copied!" } else { "Copy" } }
                </button>
            </div>
            <pre style={format!(
                "margin: 0; padding: 16px; overflow-x: auto; background-color: {}; \
                 font-size: 13px; line-height: 1.6; font-family: 'JetBrains Mono', 'Fira Code', monospace; \
                 color: {}; white-space: pre;",
                theme.colors.surface_container, theme.colors.on_surface,
            )}>
                <code>{ &props.code }</code>
            </pre>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CodeBlockProps {
    pub code: String,
    #[prop_or_default]
    pub language: Option<String>,
}

/// Renders a prop documentation table.
#[function_component]
pub fn PropTable(props: &PropTableProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    html! {
        <div style="overflow-x: auto; margin: 16px 0;">
            <table style={format!(
                "width: 100%; border-collapse: collapse; font-size: 13px; font-family: {}, sans-serif;",
                theme.font_family,
            )}>
                <thead>
                    <tr style={format!("border-bottom: 2px solid {}; background-color: {};", theme.colors.outline_variant, theme.colors.surface_container_low)}>
                        <th style="padding: 8px 12px; text-align: left; font-weight: 600;">{"Prop"}</th>
                        <th style="padding: 8px 12px; text-align: left; font-weight: 600;">{"Type"}</th>
                        <th style="padding: 8px 12px; text-align: left; font-weight: 600;">{"Default"}</th>
                        <th style="padding: 8px 12px; text-align: left; font-weight: 600;">{"Description"}</th>
                    </tr>
                </thead>
                <tbody>
                    { for props.rows.iter().map(|row| {
                        html! {
                            <tr style={format!("border-bottom: 1px solid {};", theme.colors.outline_variant)}>
                                <td style="padding: 8px 12px; font-family: monospace; color: {}; white-space: nowrap;">{ &row.name }</td>
                                <td style="padding: 8px 12px; font-family: monospace; color: {}; white-space: nowrap;">{ &row.r#type }</td>
                                <td style="padding: 8px 12px; font-family: monospace; color: {}; white-space: nowrap;">{ &row.default_value }</td>
                                <td style="padding: 8px 12px; color: {};">{ &row.description }</td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PropTableProps {
    pub rows: Vec<PropRow>,
}

#[derive(Clone, PartialEq)]
pub struct PropRow {
    pub name: String,
    pub r#type: String,
    pub default_value: String,
    pub description: String,
}

/// Renders a live demo with a label.
#[function_component]
pub fn Demo(props: &DemoProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    html! {
        <div style={format!(
            "border: 1px solid {}; border-radius: 12px; padding: 24px; margin: 12px 0; background-color: {};",
            theme.colors.outline_variant, theme.colors.surface,
        )}>
            if !props.title.is_empty() {
                <div style={format!("font-size: 12px; color: {}; margin-bottom: 16px; font-weight: 500;", theme.colors.on_surface_variant)}>
                    { &props.title }
                </div>
            }
            { props.children.clone() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DemoProps {
    #[prop_or_default]
    pub title: String,
    pub children: Children,
}

#[function_component]
pub fn ColorSwatch(props: &ColorSwatchProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    html! {
        <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
            <div style={format!(
                "width: 48px; height: 48px; border-radius: 12px; background-color: {}; \
                 border: 1px solid {};",
                props.color, theme.colors.outline_variant,
            )} />
            <span style={format!(
                "font-size: 10px; color: {}; text-align: center; word-break: break-all;",
                theme.colors.on_surface_variant
            )}>
                { &props.label }
            </span>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ColorSwatchProps {
    pub label: String,
    pub color: String,
}
