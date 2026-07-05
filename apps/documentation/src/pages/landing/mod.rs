use crate::app_context::{AppState, ColorPalette};
use crate::routes::Route;
use material_rs::components::box_layout::Box;
use material_rs::components::*;
use material_rs::theme::Theme;
use material_rs::utils::dynamic_style::dynamic_style;
use stylist::{
    css,
    yew::{Global, use_style},
};
use yew::prelude::*;
use yew_router::prelude::{Link, use_navigator};

pub fn with_alpha_hex(hex: &str, alpha: f64) -> String {
    if hex.len() == 7 {
        let a = (alpha * 255.0) as u8;
        format!("{}{:02X}", hex, a)
    } else {
        hex.to_string()
    }
}

#[function_component]
pub fn LandingPage() -> Html {
    let state = use_context::<UseStateHandle<AppState>>().expect("AppState context required");
    let app_state = (*state).clone();
    let theme = use_context::<Theme>().expect("Theme context required");
    let colors = &theme.colors;

    let navigator = use_navigator().unwrap();
    let on_nav_click = {
        let nav = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            nav.push(&Route::Home);
        })
    };

    let on_dark_toggle = {
        let state_handle = state.clone();
        Callback::from(move |_| {
            let mut new_state = (*state_handle).clone();
            new_state.dark_mode = !new_state.dark_mode;
            state_handle.set(new_state);
        })
    };
    let toggle_palette = {
        let state_handle = state.clone();
        Callback::from(move |_: MouseEvent| {
            let mut new_state = (*state_handle).clone();
            new_state.palette_menu_open = !new_state.palette_menu_open;
            state_handle.set(new_state);
        })
    };
    let _select_palette = {
        let state_handle = state.clone();
        Callback::from(move |key: String| {
            let mut new_state = (*state_handle).clone();
            new_state.palette = match key.as_str() {
                "blue" => ColorPalette::Blue,
                "green" => ColorPalette::Green,
                "red" => ColorPalette::Red,
                "orange" => ColorPalette::Orange,
                _ => ColorPalette::Purple,
            };
            new_state.palette_menu_open = false;
            state_handle.set(new_state);
        })
    };
    let _close_palette = {
        let state_handle = state.clone();
        Callback::from(move |_: ()| {
            let mut new_state = (*state_handle).clone();
            new_state.palette_menu_open = false;
            state_handle.set(new_state);
        })
    };

    let container = use_style!(r#"max-width: 1400px; margin: 0 auto; padding: 0 48px;"#);
    let section_style = use_style!(r#"padding: 60px 0; position: relative;"#);
    let section_header = use_style!(r#"text-align: center; margin-bottom: 48px;"#);
    let section_icon = use_style!(
        r#"width: 56px; height: 56px; border-radius: 16px; display: inline-flex; align-items: center; justify-content: center; margin-bottom: 20px;"#
    );
    let code_block = use_style!(
        r#"border-radius: 16px; overflow: hidden; border: 1px solid; text-align: left;"#
    );
    let code_body = use_style!(
        r#"padding: 20px; font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace; font-size: 14px; line-height: 1.7; overflow-x: auto; white-space: pre;"#
    );
    let code_border = dynamic_style(format!(
        "border-color: {}; background-color: {};",
        colors.outline_variant, colors.surface_container
    ));
    let code_bg = dynamic_style(format!(
        "background-color: {}; color: {};",
        colors.surface_container_low, colors.on_surface
    ));

    let active_tab = use_state(|| "cargo".to_string());
    let tab_key = (*active_tab).clone();
    let cargo_code = "[dependencies]\nmaterial-rs = \"0.1\"\n\n# Or from git:\n# material-rs = { git = \"https://github.com/user/material-rs\" }";
    let main_code = "use material_rs::components::*;\nuse material_rs::theme::{\n    MaterialThemeProvider,\n    ThemeBuilder, ThemeMode,\n};\n\n#[function_component]\nfn App() -> Html {\n    let theme = ThemeBuilder::new()\n        .mode(ThemeMode::Light)\n        .build();\n\n    html! {\n        <MaterialThemeProvider theme={theme}>\n            <Button label=\"Hello Material 3!\" />\n        </MaterialThemeProvider>\n    }\n}";
    let current_code = if tab_key == "cargo" {
        cargo_code
    } else {
        main_code
    };

    html! {
        <>
            <Global css={css!(r##"
                @keyframes lf1 { 0%, 100% { transform: translateY(0px) rotate(0deg); } 25% { transform: translateY(-18px) rotate(12deg); } 50% { transform: translateY(-8px) rotate(-6deg); } 75% { transform: translateY(-24px) rotate(8deg); } }
                @keyframes ld { 0%, 100% { transform: translate(0, 0) rotate(0deg); } 33% { transform: translate(12px, -20px) rotate(15deg); } 66% { transform: translate(-8px, -12px) rotate(-10deg); } }
                @keyframes ls { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }
                @keyframes lp { 0%, 100% { opacity: 0.15; transform: scale(1); } 50% { opacity: 0.25; transform: scale(1.1); } }
                @keyframes lg { 0%, 100% { opacity: 0.4; filter: blur(40px); } 50% { opacity: 0.6; filter: blur(60px); } }
                .lsh { position: absolute; pointer-events: none; z-index: 0; }
                .lg { position: absolute; width: 400px; height: 400px; border-radius: 50%; filter: blur(80px); animation: lg 8s ease-in-out infinite; pointer-events: none; z-index: 0; }
                a.hcl { text-decoration: none; color: inherit; display: block; height: 100%; }
            "##)} />

                <TopAppBar title="Material RS" logo="logo.svg" variant={TopAppBarVariant::Small} position={TopAppBarPosition::Sticky} nav_icon="menu"
                    on_nav_click={on_nav_click}
                    actions={html! { <>
                        <Link<Route> to={Route::Buttons}><Button variant={ButtonVariant::Text} label="Docs" icon="menu_book" /></Link<Route>>
                        <Link<Route> to={Route::ShowcaseLogin}><Button variant={ButtonVariant::Text} label="Showcases" icon="play_arrow" /></Link<Route>>
                        <Tooltip text="GitHub" position={TooltipPosition::BottomLeft}>
                            <Button variant={ButtonVariant::Text} label="GitHub" icon="code"
                                onclick={Callback::from(|_: MouseEvent| {
                                    #[cfg(target_arch = "wasm32")]
                                    web_sys::window().unwrap()
                                        .open_with_url_and_target("https://github.com/nikandlv/material-rs", "_blank")
                                        .ok();
                                })} />
                        </Tooltip>
                        <IconButton icon={if app_state.dark_mode { "light_mode" } else { "dark_mode" }} variant={IconButtonVariant::Standard} onclick={on_dark_toggle} label="Toggle theme" />
                        <IconButton icon="palette" variant={IconButtonVariant::Standard} id="palette-trigger" label="Palette" onclick={toggle_palette.clone()} />
                    </> }}
                />
            <Box class={container.get_class_name().to_string()}>


                <Hero />
                <WhySection />

                // ── Code Example ──
                <Box class={format!("{} {}", section_style.get_class_name(), dynamic_style(format!("background-color: {};", colors.surface_container)))}>
                    <Box class={section_header.get_class_name().to_string()}>
                        <Box class={section_icon.get_class_name().to_string()} style={dynamic_style(format!("background-color: {};", colors.tertiary_container))}><Icon name="code" size="28px" color={colors.on_tertiary_container.clone()} /></Box>
                        <Typography tag="h2" variant={TypographyVariant::HeadlineLarge} style={dynamic_style(format!("color: {}; font-weight: 300;", colors.on_surface))}>{ "Start building in minutes" }</Typography>
                        <Typography tag="p" variant={TypographyVariant::BodyLarge} style={dynamic_style(format!("color: {}; margin-top: 8px;", colors.on_surface_variant))}>{ "Install the crate, import the components, and wrap your app with the theme provider." }</Typography>
                    </Box>
                    <Box class={format!("{} {}", code_block.get_class_name(), code_border)}>
                        <TabBar tabs={vec![TabItem { key: "cargo".into(), label: "Cargo.toml".into(), icon: String::new(), active: tab_key == "cargo" }, TabItem { key: "main".into(), label: "main.rs".into(), icon: String::new(), active: tab_key == "main" }]}
                            on_select={let tab_handle = active_tab.clone(); Callback::from(move |k: String| tab_handle.set(k))} />
                        <pre class={format!("{} {}", code_body.get_class_name(), code_bg)}>{ current_code }</pre>
                    </Box>
                </Box>

                <ShowcaseSection />

                <CtaSection />
            </Box>

        </>
    }
}

// ═══════════════════════════════════════════════════════════════
// SECTION COMPONENTS
// ═══════════════════════════════════════════════════════════════

#[function_component]
fn Hero() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let colors = &theme.colors;
    let hero_style = use_style!(
        r#"padding: 60px 0 80px; position: relative; overflow: hidden; border-radius: 32px;"#
    );
    let content = use_style!(
        r#"max-width: 860px; margin: 0 auto; position: relative; z-index: 2; text-align: center;"#
    );
    let badge = use_style!(
        r#"display: inline-flex; align-items: center; gap: 6px; padding: 6px 16px; border-radius: 999px; font-size: 13px; font-weight: 500; margin-bottom: 24px;"#
    );
    let cta = use_style!(
        r#"display: flex; gap: 16px; justify-content: center; flex-wrap: wrap; margin-top: 32px;"#
    );
    let bg = dynamic_style(format!(
        "background: linear-gradient(160deg, {} 0%, {} 40%, {} 100%);",
        colors.primary_container, colors.tertiary_container, colors.secondary_container
    ));
    let badge_bg = dynamic_style(format!(
        "background-color: {}; color: {}; border: 1px solid {};",
        with_alpha_hex(&colors.primary, 0.12),
        colors.primary,
        with_alpha_hex(&colors.primary, 0.3)
    ));

    html! {
        <Box class={format!("{} {}", hero_style.get_class_name(), bg)}>
            <div class="lg" style={format!("background-color: {};", with_alpha_hex(&colors.primary, 0.3))} />
            <div class="lg" style={format!("background-color: {}; top: 20%; left: 20%;", with_alpha_hex(&colors.tertiary, 0.25))} />
            <div class="lsh" style="top: 10%; right: 10%; opacity: 0.18; animation: lf1 9s ease-in-out infinite;"><ShapeHeart size="5rem" color={colors.primary.clone()} /></div>
            <div class="lsh" style="bottom: 15%; left: 8%; opacity: 0.15; animation: ld 11s ease-in-out infinite;"><ShapeFlower size="4rem" color={colors.tertiary.clone()} /></div>
            <div class="lsh" style="top: 20%; left: 15%; opacity: 0.12; animation: ls 25s linear infinite;"><ShapeArch size="6rem" color={colors.primary.clone()} /></div>
            <div class="lsh" style="bottom: 10%; right: 12%; opacity: 0.15; animation: lp 7s ease-in-out infinite;"><ShapeGem size="4rem" color={colors.secondary.clone()} /></div>
            <Box class={content.get_class_name().to_string()}>
                <div class={badge.get_class_name().to_string()} style={badge_bg}>
                    <Icon name="auto_awesome" size="16px" color={colors.primary.clone()} />
                    <span style={dynamic_style(format!("color: {};", colors.primary))}>{ "v0.1 — Now with 40+ components" }</span>
                </div>
                <Typography tag="h1" variant={TypographyVariant::DisplayLarge} style={dynamic_style(format!("color: {}; margin-bottom: 20px; line-height: 1.05; letter-spacing: -1.5px; font-weight: 300;", colors.on_primary_container))}>{ "Material Design 3\nfor Rust & Yew" }</Typography>
                <Typography tag="p" variant={TypographyVariant::HeadlineSmall} style={dynamic_style(format!("color: {}; margin: 0 auto; max-width: 640px; line-height: 1.45; font-weight: 300;", with_alpha_hex(&colors.on_primary_container, 0.85)))}>{ "Build expressive, accessible interfaces with the full Material 3 specolors." }</Typography>
                <Box class={cta.get_class_name().to_string()}>
                    <Link<Route> to={Route::Buttons}><Button variant={ButtonVariant::Filled} label="Explore Components" icon="arrow_forward" size={ButtonSize::Large} /></Link<Route>>
                    <Link<Route> to={Route::ShowcaseLogin}><Button variant={ButtonVariant::Outlined} label="See Showcases" icon="play_arrow" size={ButtonSize::Large} /></Link<Route>>
                </Box>
                <Box display="flex" gap="12px" justify_content="center" flex_wrap="wrap" margin_top="24px">
                    { for [("40+", "Components", colors.primary_container.clone()), ("100%", "Type Safe", colors.secondary_container.clone()), ("0", "JS Deps", colors.tertiary_container.clone()), ("5", "Themes", colors.primary_container.clone())].iter().map(|(stat_value, stat_label, bg)| {
                        let value_text = stat_value.to_string();
                        let label_text = stat_label.to_string();
                        html! { <Badge count={0}><Box display="inline-flex" align_items="center" gap="8px" padding="8px 16px" border_radius="999px" style={dynamic_style(format!("background-color: {}; border: 1px solid {}; color: {};", bg, with_alpha_hex(&colors.on_primary_container, 0.12), colors.on_primary_container))}><span style={dynamic_style(format!("font-size: 16px; font-weight: 700; color: {};", colors.on_primary_container))}>{ html! { value_text } }</span><span style={dynamic_style(format!("font-size: 12px; color: {};", with_alpha_hex(&colors.on_primary_container, 0.7)))}>{ html! { label_text } }</span></Box></Badge> }
                    })}
                </Box>
            </Box>
        </Box>
    }
}

#[function_component]
fn WhySection() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let colors = &theme.colors;
    let section_container = use_style!(r#"padding: 60px 0; position: relative;"#);
    let header_style = use_style!(r#"text-align: center; margin-bottom: 48px;"#);
    let icon_box = use_style!(
        r#"width: 56px; height: 56px; border-radius: 16px; display: inline-flex; align-items: center; justify-content: center; margin-bottom: 20px;"#
    );
    let grid_layout =
        use_style!(r#"display: grid; grid-template-columns: repeat(3, 1fr); gap: 24px;"#);
    let card_style = use_style!(
        r#"padding: 28px; border-radius: 20px; border: 1px solid; transition: transform 250ms cubic-bezier(0.2, 0, 0, 1), box-shadow 250ms; &:hover { transform: translateY(-4px); box-shadow: 0 8px 32px rgba(0,0,0,0.08); }"#
    );
    let card_border_style = dynamic_style(format!(
        "background-color: {}; border-color: {};",
        colors.surface, colors.outline_variant
    ));

    html! {
        <Box class={section_container.get_class_name().to_string()}>
            <Box class={header_style.get_class_name().to_string()}>
                <Box class={icon_box.get_class_name().to_string()} style={dynamic_style(format!("background-color: {};", colors.primary_container))}><Icon name="auto_awesome" size="28px" color={colors.on_primary_container.clone()} /></Box>
                <Typography tag="h2" variant={TypographyVariant::HeadlineLarge} style={dynamic_style(format!("color: {}; font-weight: 300;", colors.on_surface))}>{ "Why Material RS?" }</Typography>
                <Typography tag="p" variant={TypographyVariant::BodyLarge} style={dynamic_style(format!("color: {}; margin-top: 8px;", colors.on_surface_variant))}>{ "A complete Material Design 3 implementation for the Yew framework, written entirely in Rust." }</Typography>
            </Box>
            <Box class={grid_layout.get_class_name().to_string()}>
                { for [("palette", "Dynamic Color", "Full MD3 color system with 5 palettes."), ("view_timeline", "40+ Components", "Every component follows the MD3 specification."), ("code", "Pure Rust / WASM", "Zero JavaScript dependencies. Full type safety."), ("translate", "RTL Built-in", "CSS logical properties auto-flip padding, margins, and positioning."), ("design_services", "Shape System", "31 expressive SVG shapes with full theming."), ("speed", "Motion & Animation", "Built-in easing curves and duration tokens.")].iter().map(|(icon, title, description)| {
                    let icon_name = icon.to_string();
                    let title_text = title.to_string();
                    let description_text = description.to_string();
                    html! { <Box class={format!("{} {}", card_style.get_class_name(), card_border_style)}><Box display="flex" align_items="center" gap="12px" margin_bottom="16px"><Box width="44px" height="44px" border_radius="12px" display="flex" align_items="center" justify_content="center" bg={colors.primary_container.clone()}><Icon name={icon_name} size="22px" color={colors.on_primary_container.clone()} /></Box><Typography tag="h3" variant={TypographyVariant::TitleMedium} style={dynamic_style(format!("color: {}; font-weight: 600;", colors.on_surface))}>{ html! { title_text } }</Typography></Box><Typography tag="p" variant={TypographyVariant::BodyMedium} style={dynamic_style(format!("color: {}; line-height: 1.6;", colors.on_surface_variant))}>{ html! { description_text } }</Typography></Box> }
                })}
            </Box>
        </Box>
    }
}

#[function_component]
fn ShowcaseSection() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let colors = &theme.colors;
    let section_container = use_style!(r#"padding: 60px 0; position: relative;"#);
    let header_style = use_style!(r#"text-align: center; margin-bottom: 48px;"#);
    let icon_box = use_style!(
        r#"width: 56px; height: 56px; border-radius: 16px; display: inline-flex; align-items: center; justify-content: center; margin-bottom: 20px;"#
    );
    let grid_layout = use_style!(
        r#"display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 20px;"#
    );
    let card_style = use_style!(
        r#"padding: 28px; border-radius: 20px; border: 1px solid; transition: transform 250ms cubic-bezier(0.2, 0, 0, 1), box-shadow 250ms; &:hover { transform: translateY(-4px); box-shadow: 0 8px 32px rgba(0,0,0,0.08); }"#
    );
    let card_border_style = dynamic_style(format!(
        "background-color: {}; border-color: {};",
        colors.surface, colors.outline_variant
    ));

    html! {
        <Box class={section_container.get_class_name().to_string()}>
            <Box class={header_style.get_class_name().to_string()}>
                <Box class={icon_box.get_class_name().to_string()} style={dynamic_style(format!("background-color: {};", colors.secondary_container))}><Icon name="web" size="28px" color={colors.on_secondary_container.clone()} /></Box>
                <Typography tag="h2" variant={TypographyVariant::HeadlineLarge} style={dynamic_style(format!("color: {}; font-weight: 300;", colors.on_surface))}>{ "Real-world showcases" }</Typography>
                <Typography tag="p" variant={TypographyVariant::BodyLarge} style={dynamic_style(format!("color: {}; margin-top: 8px;", colors.on_surface_variant))}>{ "See how Material RS powers complete application interfaces." }</Typography>
            </Box>
            <Box class={grid_layout.get_class_name().to_string()}>
                { for [("login", "Login Page", "Authentication flow with form validation, social login, and remember-me.", Route::ShowcaseLogin), ("dashboard", "Admin Dashboard", "Persistent drawer, stat cards, data tables, progress metrics.", Route::ShowcaseAdmin), ("mail", "Email App", "Folder sidebar, message list with avatars, star toggles.", Route::ShowcaseEmail), ("article", "Blog", "Featured posts, article grid, sidebar with categories.", Route::ShowcaseBlog), ("tune", "Settings", "Sidebar navigation, profile form, notification toggles.", Route::ShowcaseSettings), ("store", "E-commerce Store", "Product grid, filter chips, ratings, pagination.", Route::ShowcaseEcommerce)].iter().map(|(icon, title, description, route)| {
                    let icon_name = icon.to_string();
                    let title_text = title.to_string();
                    let description_text = description.to_string();
                    let route_target = route.clone();
                    html! { <Link<Route> to={route_target} classes="hcl"><Box class={format!("{} {}", card_style.get_class_name(), card_border_style)} padding="24px"><Box display="flex" align_items="center" gap="12px" margin_bottom="12px"><Icon name={icon_name} size="24px" color={colors.primary.clone()} /><Typography tag="h3" variant={TypographyVariant::TitleMedium} style={dynamic_style(format!("color: {}; font-weight: 600;", colors.on_surface))}>{ html! { title_text } }</Typography></Box><Typography tag="p" variant={TypographyVariant::BodySmall} style={dynamic_style(format!("color: {};", colors.on_surface_variant))}>{ html! { description_text } }</Typography></Box></Link<Route>> }
                })}
            </Box>
        </Box>
    }
}

#[function_component]
fn CtaSection() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let colors = &theme.colors;
    let hero = use_style!(
        r#"padding: 60px 0 80px; position: relative; overflow: hidden; border-radius: 32px;"#
    );
    let content = use_style!(
        r#"max-width: 860px; margin: 0 auto; position: relative; z-index: 2; text-align: center;"#
    );
    let bg = dynamic_style(format!(
        "background: linear-gradient(160deg, {} 0%, {} 40%, {} 100%);",
        colors.primary_container, colors.tertiary_container, colors.secondary_container
    ));

    html! {
        <Box margin_bottom="64px" class={format!("{} {}", hero.get_class_name(), bg)} text_align="center">
            <div class="lsh" style="top: 10%; right: 10%; opacity: 0.12; animation: lf1 10s ease-in-out infinite;"><ShapeHeart size="4rem" color={colors.on_primary_container.clone()} /></div>
            <div class="lsh" style="bottom: 15%; left: 8%; opacity: 0.1; animation: ld 12s ease-in-out infinite;"><ShapeFlower size="3rem" color={colors.on_primary_container.clone()} /></div>
            <div class="lsh" style="top: 20%; left: 20%; opacity: 0.08; animation: ls 20s linear infinite;"><ShapeArch size="5rem" color={colors.on_primary_container.clone()} /></div>
            <Box class={content.get_class_name().to_string()}>
                <Typography tag="h2" variant={TypographyVariant::HeadlineLarge} style={dynamic_style(format!("color: {}; margin-bottom: 12px; font-weight: 300;", colors.on_primary_container))}>{ "Ready to build?" }</Typography>
                <Typography tag="p" variant={TypographyVariant::BodyLarge} style={dynamic_style(format!("color: {}; margin-bottom: 32px;", with_alpha_hex(&colors.on_primary_container, 0.8)))}>{ "Join the growing community of developers building with Material RS." }</Typography>
                <Box display="flex" margin_top="12px" gap="16px" justify_content="center" flex_wrap="wrap">
                    <Link<Route> to={Route::Buttons}><Button variant={ButtonVariant::Filled} label="Get Started" icon="arrow_forward" size={ButtonSize::Large} /></Link<Route>>
                    <Link<Route> to={Route::ThemeTokens}><Button variant={ButtonVariant::Outlined} label="Explore Theme" icon="palette" size={ButtonSize::Large} /></Link<Route>>
                </Box>
                <Box margin_top="48px" padding_top="32px" style={dynamic_style(format!("border-top: 1px solid {};", with_alpha_hex(&colors.on_primary_container, 0.15)))}>
                    <Box display="flex" align_items="center" justify_content="center" gap="12px" margin_bottom="16px">
                        <ShapeArch size="2rem" color={colors.on_primary_container.clone()} />
                        <Typography tag="span" variant={TypographyVariant::TitleMedium} style={dynamic_style(format!("color: {}; font-weight: 600;", colors.on_primary_container))}>{ "Material RS" }</Typography>
                    </Box>
                    <Typography tag="p" variant={TypographyVariant::BodyMedium} style={dynamic_style(format!("color: {}; max-width: 480px; margin: 0 auto 20px; line-height: 1.6;", with_alpha_hex(&colors.on_primary_container, 0.8)))}>{ "Material Design 3 Expressive component library for Rust and Yew. Open source, MIT licensed." }</Typography>
                    <Box display="flex" gap="24px" justify_content="center" flex_wrap="wrap">
                        <Link<Route> to={Route::Buttons}><Typography tag="span" variant={TypographyVariant::LabelMedium} style={dynamic_style(format!("color: {};", with_alpha_hex(&colors.on_primary_container, 0.9)))}>{ "Components" }</Typography></Link<Route>>
                        <Link<Route> to={Route::ThemeTokens}><Typography tag="span" variant={TypographyVariant::LabelMedium} style={dynamic_style(format!("color: {};", with_alpha_hex(&colors.on_primary_container, 0.9)))}>{ "Theme" }</Typography></Link<Route>>
                        <Link<Route> to={Route::ShowcaseAdmin}><Typography tag="span" variant={TypographyVariant::LabelMedium} style={dynamic_style(format!("color: {};", with_alpha_hex(&colors.on_primary_container, 0.9)))}>{ "Showcases" }</Typography></Link<Route>>
                        <a href="https://github.com/nikandlv/material-rs" target="_blank" rel="noopener noreferrer"
                            style={dynamic_style(format!("color: {}; text-decoration: none;", with_alpha_hex(&colors.on_primary_container, 0.9)))}>
                            <Typography tag="span" variant={TypographyVariant::LabelMedium}>{ "GitHub" }</Typography>
                        </a>
                    </Box>
                    <Typography tag="p" variant={TypographyVariant::BodySmall} style={dynamic_style(format!("color: {}; margin-top: 20px;", with_alpha_hex(&colors.on_primary_container, 0.6)))}>{ "Built with Rust, Yew, and Material Design 3 Expressive." }</Typography>
                </Box>
            </Box>
        </Box>
    }
}
