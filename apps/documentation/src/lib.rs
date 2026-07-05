pub mod app_context;
pub mod pages;
pub mod routes;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

use app_context::{AppState, theme_for_palette};
use material_rs::components::*;
use material_rs::theme::{Direction, MaterialThemeProvider, Theme};
use routes::Route;
use stylist::{css, yew::Global};
use yew::prelude::*;
use yew_router::prelude::*;

// ─── Drawer Navigation Data ──────────────────────────────────────────

const DRAWER_ENTRIES: &[(&str, &str, &str, &str, Route)] = &[
    ("home", "home", "Home", "", Route::Home),
    ("buttons", "smart_button", "Buttons & FABs", "Form Controls", Route::Buttons),
    ("text-fields", "text_fields", "Text Fields", "Form Controls", Route::TextFields),
    ("selection", "check_circle", "Selection Controls", "Form Controls", Route::Selection),
    ("slider", "linear_scale", "Slider", "Form Controls", Route::Slider),
    ("split-buttons", "call_split", "Split Buttons", "Form Controls", Route::SplitButtons),
    ("select-textarea", "arrow_drop_down_circle", "Select & Text Area", "Form Controls", Route::SelectTextarea),
    ("toggle-buttons", "toggle_on", "Toggle Buttons", "Form Controls", Route::ToggleButtons),
    ("chips", "label", "Chips", "Data Display", Route::Chips),
    ("cards", "credit_card", "Cards", "Data Display", Route::Cards),
    ("badges", "new_releases", "Badges", "Data Display", Route::Badges),
    ("list", "list", "List Items", "Data Display", Route::List),
    ("image-list", "photo_library", "Image List", "Data Display", Route::ImageList),
    ("avatars-shapes", "person", "Avatars & Shapes", "Data Display", Route::AvatarsShapes),
    ("shape-gallery", "palette", "Shape Gallery", "Data Display", Route::ShapeGallery),
    ("tables", "table_chart", "Tables", "Data Display", Route::Tables),
    ("code-view", "code", "Code View", "Data Display", Route::CodeView),
    ("icon", "emoji_symbols", "Icon", "Data Display", Route::Icon),
    ("carousel", "view_carousel", "Carousel", "Data Display", Route::Carousel),
    ("accordions", "expand_more", "Accordions", "Data Display", Route::Accordions),
    ("progress", "hourglass_empty", "Progress", "Feedback", Route::Progress),
    ("dialog", "web_asset", "Dialog", "Feedback", Route::Dialog),
    ("alert-box", "info", "Alert Box", "Feedback", Route::AlertBox),
    ("skeleton", "view_carousel", "Skeleton", "Feedback", Route::Skeleton),
    ("steppers", "timeline", "Steppers", "Feedback", Route::Steppers),
    ("bottom-sheet", "expand_less", "Bottom Sheet", "Feedback", Route::BottomSheet),
    ("menu", "menu", "Menu", "Feedback", Route::Menu),
    ("snackbar", "notifications_active", "Snackbar", "Feedback", Route::Snackbar),
    ("tooltip", "tooltip", "Tooltip", "Feedback", Route::Tooltip),
    ("tabs", "tab", "Tabs", "Navigation", Route::Tabs),
    ("breadcrumb", "chevron_right", "Breadcrumb", "Navigation", Route::Breadcrumb),
    ("navigation-drawer", "menu", "Navigation Drawer", "Navigation", Route::NavigationDrawer),
    ("navigation-rail", "swap_vert", "Navigation Rail", "Navigation", Route::NavigationRail),
    ("navigation-bar", "bottom_navigation", "Navigation Bar", "Navigation", Route::NavigationBar),
    ("tab-bar", "tab", "Tab Bar", "Navigation", Route::TabBar),
    ("top-app-bar", "web", "Top App Bar", "Navigation", Route::TopAppBar),
    ("typography", "text_format", "Typography", "Layout", Route::Typography),
    ("box", "crop_square", "Box Utility", "Layout", Route::Box),
    ("grid", "grid_on", "Grid & Container", "Layout", Route::Grid),
    ("ripple", "touch_app", "Ripple", "Layout", Route::Ripple),
    ("toolbar", "toolbar", "Toolbar", "Layout", Route::Toolbar),
    ("divider", "horizontal_rule", "Divider", "Layout", Route::Divider),
    ("container", "crop_free", "Container", "Layout", Route::Container),
    ("spacer", "space_bar", "Spacer", "Layout", Route::Spacer),
    ("theme-tokens", "palette", "Theme Tokens", "Theme", Route::ThemeTokens),
    ("showcase-login", "login", "Login Page", "Showcases", Route::ShowcaseLogin),
    ("showcase-admin", "dashboard", "Admin Dashboard", "Showcases", Route::ShowcaseAdmin),
    ("showcase-email", "mail", "Email App", "Showcases", Route::ShowcaseEmail),
    ("showcase-settings", "settings", "Settings", "Showcases", Route::ShowcaseSettings),
    ("showcase-ecommerce", "store", "Store", "Showcases", Route::ShowcaseEcommerce),
    ("showcase-blog", "article", "Blog", "Showcases", Route::ShowcaseBlog),
];

fn build_drawer_items(active_key: &str) -> Vec<DrawerItem> {
    DRAWER_ENTRIES
        .iter()
        .map(|&(key, icon, label, section, _)| DrawerItem {
            key: key.into(),
            icon: icon.into(),
            label: label.into(),
            section: section.into(),
            active: active_key == key,
        })
        .collect()
}

fn route_to_drawer_key(route: &Route) -> &'static str {
    DRAWER_ENTRIES
        .iter()
        .find(|entry| &entry.4 == route)
        .map(|entry| entry.0)
        .unwrap_or("")
}

fn drawer_key_to_route(key: &str) -> Route {
    DRAWER_ENTRIES
        .iter()
        .find(|entry| entry.0 == key)
        .map(|entry| entry.4.clone())
        .unwrap_or(Route::Home)
}

fn toggle_state_field(
    state: &UseStateHandle<AppState>,
    field_fn: impl Fn(&mut AppState) -> &mut bool + 'static,
) -> Callback<MouseEvent> {
    let state = state.clone();
    Callback::from(move |_: MouseEvent| {
        let mut new = (*state).clone();
        let field = field_fn(&mut new);
        *field = !*field;
        state.set(new);
    })
}

// ─── App Root ────────────────────────────────────────────────────────

#[function_component]
pub fn App() -> Html {
    let state = use_state(AppState::default);
    let mut theme = theme_for_palette(state.palette, state.dark_mode);
    theme.direction = if state.rtl { Direction::Rtl } else { Direction::Ltr };
    let bg_color = theme.colors.surface.clone();
    let fg_color = theme.colors.on_surface.clone();
    let font = theme.font_family.clone();

    html! {
        <MaterialThemeProvider theme={theme}>
            <ContextProvider<UseStateHandle<AppState>> context={state.clone()}>
                <Global css={css!(r#"
                    .home-card-link { text-decoration: none; color: inherit; display: block; }
                    .home-card-link:hover { text-decoration: none; color: inherit; }
                    .home-card-link:visited { color: inherit; text-decoration: none; }
                "#)} />
                <BrowserRouter>
                    <div style={format!(
                        "color-scheme: {}; background-color: {}; color: {}; font-family: {}, sans-serif; transition: background-color 300ms cubic-bezier(0.2, 0, 0, 1), color 300ms cubic-bezier(0.2, 0, 0, 1);",
                        if state.dark_mode { "dark" } else { "light" },
                        bg_color, fg_color, font,
                    )}>
                        <Content state={state.clone()} />
                        <SnackbarOverlay state={state.clone()} />
                        <MenuOverlay state={state.clone()} />
                    </div>
                </BrowserRouter>
            </ContextProvider<UseStateHandle<AppState>>>
        </MaterialThemeProvider>
    }
}

// ─── Main Content ────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
pub struct ContentProps {
    pub state: UseStateHandle<AppState>,
}

#[function_component]
fn Content(props: &ContentProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let state_handle = &props.state;
    let current_route = use_route::<Route>().unwrap_or(Route::Landing);
    let active_key = route_to_drawer_key(&current_route);
    let is_landing = matches!(current_route, Route::Landing);

    // Mobile detection — browser only
    #[cfg(target_arch = "wasm32")]
    {
        let s = state_handle.clone();
        use_effect_with((), move |_| {
            let check = {
                let s = s.clone();
                move || {
                    let width = web_sys::window()
                        .unwrap()
                        .inner_width()
                        .unwrap()
                        .as_f64()
                        .unwrap_or(1200.0);
                    let is_mobile = width < 900.0;
                    let mut new = (*s).clone();
                    if new.is_mobile != is_mobile {
                        new.is_mobile = is_mobile;
                        new.drawer_open = !is_mobile;
                        s.set(new);
                    }
                }
            };
            check();

            let cb = wasm_bindgen::closure::Closure::wrap(std::boxed::Box::new(move || {
                check();
            })
                as std::boxed::Box<dyn FnMut()>);
            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("resize", cb.as_ref().unchecked_ref())
                .unwrap();
            cb.forget();
            || ()
        });
    }

    let on_dark_toggle = toggle_state_field(state_handle, |s| &mut s.dark_mode);
    let on_rtl_toggle = toggle_state_field(state_handle, |s| &mut s.rtl);
    let on_hamburger_click = toggle_state_field(state_handle, |s| &mut s.drawer_open);
    let toggle_palette_menu = toggle_state_field(state_handle, |s| &mut s.palette_menu_open);

    let drawer_items = build_drawer_items(active_key);
    let navigator = use_navigator();

    html! {
        <div style={format!(
            "background-color: {}; color: {}; font-family: {}, sans-serif; transition: background-color 300ms cubic-bezier(0.2, 0, 0, 1), color 300ms cubic-bezier(0.2, 0, 0, 1);",
            theme.colors.surface_container, theme.colors.on_surface, theme.font_family,
        )}>
            if !is_landing {
                <TopAppBar
                    title="Material RS Components"
                    logo="logo.svg"
                    nav_icon="menu"
                    on_nav_click={on_hamburger_click}
                    scrolled={true}
                    position={TopAppBarPosition::Sticky}
                    actions={vec![
                        html!{<Tooltip text="GitHub" position={TooltipPosition::Bottom}>
                            <Button variant={ButtonVariant::Text} label="GitHub" icon="code"
                                onclick={Callback::from(|_: MouseEvent| {
                                    #[cfg(target_arch = "wasm32")]
                                    web_sys::window().unwrap()
                                        .open_with_url_and_target("https://github.com/nikandlv/material-rs", "_blank")
                                        .ok();
                                })} />
                        </Tooltip>},
                        html!{<Tooltip text="Color Palette" position={TooltipPosition::Bottom}>
                            <IconButton icon="palette" variant={IconButtonVariant::Standard} id="palette-trigger" label="Color Palette" onclick={toggle_palette_menu.clone()} />
                        </Tooltip>},
                        html!{<Tooltip text="Toggle theme" position={TooltipPosition::BottomLeft}>
                            <IconButton icon={if state_handle.dark_mode { "light_mode" } else { "dark_mode" }} variant={IconButtonVariant::Standard} onclick={on_dark_toggle} label="Toggle theme" />
                        </Tooltip>},
                        html!{<Tooltip text="Toggle RTL" position={TooltipPosition::BottomLeft}>
                            <IconButton icon="info" variant={IconButtonVariant::Standard} onclick={on_rtl_toggle} label="Toggle RTL" />
                        </Tooltip>},
                    ]}
                />
            }

            <div style={format!("display: flex; min-height: calc(100vh - {}); max-height: calc(100vh - {});",
                if is_landing {"0px"} else {"64px"},
                if is_landing {"0px"} else {"64px"},
            )}>
                if !state_handle.is_mobile && !is_landing {
                    <NavigationDrawer
                        variant={DrawerVariant::Persistent}
                        open={state_handle.drawer_open}
                        headline="Material RS"
                        items={drawer_items}
                        on_select={Callback::from({
                            let nav = navigator.clone();
                            move |key: String| {
                                if let Some(ref n) = nav {
                                    n.push(&drawer_key_to_route(&key));
                                }
                            }
                        })}
                        on_close={Callback::from({
                            let s = state_handle.clone();
                            move |_: ()| {
                                let mut new = (*s).clone();
                                new.drawer_open = false;
                                s.set(new);
                            }
                        })}
                    />
                }

                <Box style={format!(
                    "flex: 1; box-sizing: border-box; overflow-y: auto; padding: {}; height: 100%; max-height: calc(100dvh - {});",
                    if is_landing { "0" } else { "24px 32px" },
                    if is_landing {"0px"} else {"64px"},
                )}>
                    <yew_router::Switch<Route> render={
                        let state = props.state.clone();
                        move |route| switch(route, state.clone())
                    } />
                </Box>
            </div>
        </div>
    }
}

// ─── Route Switch ────────────────────────────────────────────────────

fn switch(route: Route, _state: UseStateHandle<AppState>) -> Html {
    match route {
        Route::Landing => html! { <pages::LandingPage /> },
        Route::Home => html! { <pages::HomePage /> },
        Route::Buttons => html! { <pages::ButtonsPage /> },
        Route::TextFields => html! { <pages::TextFieldsPage /> },
        Route::Selection => html! { <pages::SelectionPage /> },
        Route::Slider => html! { <pages::SliderPage /> },
        Route::Chips => html! { <pages::ChipsPage /> },
        Route::Cards => html! { <pages::CardsPage /> },
        Route::Progress => html! { <pages::ProgressPage /> },
        Route::Badges => html! { <pages::BadgesPage /> },
        Route::Dialog => html! { <pages::DialogPage /> },
        Route::List => html! { <pages::ListPage /> },
        Route::Tabs => html! { <pages::TabsPage /> },
        Route::Typography => html! { <pages::TypographyPage /> },
        Route::Box => html! { <pages::BoxPage /> },
        Route::Grid => html! { <pages::GridPage /> },
        Route::SplitButtons => html! { <pages::SplitButtonsPage /> },
        Route::SelectTextarea => html! { <pages::SelectTextareaPage /> },
        Route::Accordions => html! { <pages::AccordionsPage /> },
        Route::ToggleButtons => html! { <pages::ToggleButtonsPage /> },
        Route::AlertBox => html! { <pages::AlertBoxPage /> },
        Route::Skeleton => html! { <pages::SkeletonPage /> },
        Route::Breadcrumb => html! { <pages::BreadcrumbPage /> },
        Route::ImageList => html! { <pages::ImageListPage /> },
        Route::AvatarsShapes => html! { <pages::AvatarsShapesPage /> },
        Route::ShapeGallery => html! { <pages::ShapeGalleryPage /> },
        Route::Steppers => html! { <pages::SteppersPage /> },
        Route::Tables => html! { <pages::TablesPage /> },
        Route::NavigationDrawer => html! { <pages::NavigationDrawerPage /> },
        Route::BottomSheet => html! { <pages::BottomSheetPage /> },
        Route::ThemeTokens => html! { <pages::ThemeTokensPage /> },
        Route::CodeView => html! { <pages::CodeViewPage /> },
        Route::Carousel => html! { <pages::CarouselPage /> },
        Route::NavigationRail => html! { <pages::NavigationRailPage /> },
        Route::NavigationBar => html! { <pages::NavigationBarPage /> },
        Route::TabBar => html! { <pages::TabBarPage /> },
        Route::Ripple => html! { <pages::RipplePage /> },
        Route::Icon => html! { <pages::IconPage /> },
        Route::Menu => html! { <pages::MenuPage /> },
        Route::Snackbar => html! { <pages::SnackbarPage /> },
        Route::Tooltip => html! { <pages::TooltipPage /> },
        Route::Toolbar => html! { <pages::ToolbarPage /> },
        Route::TopAppBar => html! { <pages::TopAppBarPage /> },
        Route::Divider => html! { <pages::DividerPage /> },
        Route::Container => html! { <pages::ContainerPage /> },
        Route::Spacer => html! { <pages::SpacerPage /> },
        Route::ShowcaseLogin => html! { <pages::ShowcaseLoginPage /> },
        Route::ShowcaseAdmin => html! { <pages::ShowcaseAdminPage /> },
        Route::ShowcaseEmail => html! { <pages::ShowcaseEmailPage /> },
        Route::ShowcaseSettings => html! { <pages::ShowcaseSettingsPage /> },
        Route::ShowcaseEcommerce => html! { <pages::ShowcaseEcommercePage /> },
        Route::ShowcaseBlog => html! { <pages::ShowcaseBlogPage /> },
        Route::NotFound => html! { <pages::NotFoundPage /> },
    }
}

// ─── Overlays ────────────────────────────────────────────────────────

#[function_component]
fn SnackbarOverlay(props: &ContentProps) -> Html {
    let state_handle = &props.state;

    let dismiss = {
        let s = state_handle.clone();
        Callback::from(move |_: ()| {
            let mut new = (*s).clone();
            new.snackbar_visible = false;
            s.set(new);
        })
    };

    html! {
        <Snackbar
            visible={state_handle.snackbar_visible}
            message={state_handle.snackbar_message.clone()}
            action_label="Dismiss"
            on_dismiss={dismiss.clone()}
            on_action={dismiss}
        />
    }
}

#[function_component]
fn MenuOverlay(props: &ContentProps) -> Html {
    let state_handle = &props.state;

    let menu_items: Vec<MenuItem> = [
        app_context::ColorPalette::Purple,
        app_context::ColorPalette::Blue,
        app_context::ColorPalette::Green,
        app_context::ColorPalette::Red,
        app_context::ColorPalette::Orange,
    ]
    .iter()
    .map(|p| MenuItem {
        key: p.label().to_string(),
        label: p.label().to_string(),
        icon: if *p == state_handle.palette { "check".to_string() } else { String::new() },
        trailing_text: String::new(),
        disabled: false,
    })
    .collect();

    let on_select = {
        let s = state_handle.clone();
        Callback::from(move |key: String| {
            let palette = match key.as_str() {
                "Purple" => app_context::ColorPalette::Purple,
                "Blue" => app_context::ColorPalette::Blue,
                "Green" => app_context::ColorPalette::Green,
                "Red" => app_context::ColorPalette::Red,
                _ => app_context::ColorPalette::Orange,
            };
            let mut new = (*s).clone();
            new.palette = palette;
            new.palette_menu_open = false;
            s.set(new);
        })
    };

    let close_menu = {
        let s = state_handle.clone();
        Callback::from(move |_: ()| {
            let mut new = (*s).clone();
            new.palette_menu_open = false;
            s.set(new);
        })
    };

    html! {
        <Menu
            open={state_handle.palette_menu_open}
            on_close={close_menu}
            anchor_id="palette-trigger"
            items={menu_items}
            on_select={on_select}
        />
    }
}
