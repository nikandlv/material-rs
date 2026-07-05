use super::CodeBlock;
use crate::routes::Route;
use material_rs::components::box_layout::{Box, BoxTag};
use material_rs::components::switch::Switch;
use material_rs::components::*;
use material_rs::theme::Theme;
use material_rs::utils::classes::combine_classes;
use material_rs::utils::dynamic_style::dynamic_style;
use stylist::{css, yew::Global};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn HomePage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let colors = &theme.colors;

    let demo_switch = use_state(|| true);
    let demo_text = use_state(String::new);
    let demo_count = use_state(|| 0u32);
    let demo_selected = use_state(|| "Yew".to_string());

    // ── Dynamic styles (theme-dependent) ──

    let hero_wrap = dynamic_style(format!(
        "width:100%;padding:100px 40px 120px;position:relative;overflow:hidden;border-radius:32px;text-align:center;margin-bottom:80px;\
         background:linear-gradient(145deg,{} 0%,{} 40%,{} 80%,{} 100%);",
        colors.primary_container,
        colors.tertiary_container,
        colors.secondary_container,
        colors.primary_container
    ));
    let hero_content = dynamic_style("margin:0 auto;position:relative;z-index:2;".into());
    let hero_actions =
        dynamic_style("display:flex;gap:16px;justify-content:center;flex-wrap:wrap;".into());
    let section_max = dynamic_style("margin:0 auto;padding:0 24px;".into());
    let feature_grid = dynamic_style(
        "display:grid;grid-template-columns:repeat(auto-fit,minmax(260px,1fr));gap:20px;".into(),
    );
    let icon_box = dynamic_style(format!(
        "width:52px;height:52px;border-radius:16px;display:flex;align-items:center;justify-content:center;\
         background:linear-gradient(135deg,{} 0%,{} 100%);margin-bottom:16px;",
        colors.primary_container, colors.tertiary_container
    ));
    let install_wrap = dynamic_style(format!(
        "padding:36px;border-radius:24px;margin:0 auto;\
         background:linear-gradient(135deg,{} 0%,{} 100%);border:1px solid {};",
        colors.surface_container_low, colors.surface_container, colors.outline_variant
    ));
    let feat_list = dynamic_style(format!(
        "margin:0;padding:0;line-height:2.4;list-style-type:none;color:{};",
        colors.on_surface_variant
    ));
    let feat_item = dynamic_style("display:flex;align-items:center;gap:10px;".into());
    let comp_icon = dynamic_style(format!(
        "width:44px;height:44px;border-radius:13px;display:flex;align-items:center;justify-content:center;flex-shrink:0;\
         background:linear-gradient(135deg,{} 0%,{} 100%);",
        colors.primary_container, colors.secondary_container
    ));
    let demo_wrap = dynamic_style(format!(
        "padding:52px 28px;border-radius:28px;margin-bottom:80px;\
         background:linear-gradient(180deg,{} 0%,{} 100%);border:1px solid {};",
        colors.surface_container_low, colors.surface_container, colors.outline_variant
    ));
    let demo_grid = dynamic_style(
        "display:grid;grid-template-columns:1fr 1fr;gap:28px;align-items:start;".into(),
    );
    let demo_card = dynamic_style(format!(
        "padding:24px;border-radius:20px;background-color:{};border:1px solid {};",
        colors.surface_container, colors.outline_variant
    ));
    let stat_row = dynamic_style(
        "display:grid;grid-template-columns:repeat(3,1fr);gap:20px;margin-bottom:80px;".into(),
    );
    let feat_box_bg = dynamic_style(format!(
        "background-color:{};border-left:3px solid {};padding:18px 22px;border-radius:12px;margin-top:24px;",
        colors.surface_container_high, colors.primary
    ));
    let shape_base = dynamic_style("position:absolute;pointer-events:none;z-index:1;".into());
    let shape_floating_1 =
        dynamic_style("top:10%;right:5%;opacity:.18;animation:s1 8s ease-in-out infinite;".into());
    let shape_floating_2 = dynamic_style(
        "bottom:8%;left:3%;opacity:.16;animation:s2 10s ease-in-out infinite;".into(),
    );
    let shape_floating_3 =
        dynamic_style("top:6%;left:10%;opacity:.16;animation:s3 7s ease-in-out infinite;".into());
    let shape_floating_4 =
        dynamic_style("bottom:5%;right:4%;opacity:.12;animation:s4 22s linear infinite;".into());
    let shape_floating_5 = dynamic_style(
        "top:48%;right:2%;opacity:.14;animation:s5 6s ease-in-out infinite 1.5s;".into(),
    );
    let shape_floating_6 = dynamic_style(
        "bottom:18%;left:20%;opacity:.10;animation:s4 30s linear infinite reverse;".into(),
    );
    let shape_floating_7 = dynamic_style(
        "top:18%;left:30%;opacity:.08;animation:s1 11s ease-in-out infinite 3s;".into(),
    );
    let section_header_style = dynamic_style(format!(
        "color:{};text-align:center;margin-bottom:8px;font-weight:500;",
        colors.on_surface
    ));
    let section_sub_style = dynamic_style(format!(
        "color:{};text-align:center;margin-bottom:44px;line-height:1.6;",
        colors.on_surface_variant
    ));
    let section_gap = dynamic_style("margin-bottom:80px;".into());

    let comp_sections = vec![
        (
            "Buttons & FABs",
            "smart_button",
            Route::Buttons,
            "Buttons, floating action buttons, and icon buttons",
        ),
        (
            "Text Fields",
            "text_fields",
            Route::TextFields,
            "Filled, outlined, plain, underline, and dense text inputs",
        ),
        (
            "Selection Controls",
            "check_circle",
            Route::Selection,
            "Switches, checkboxes, and radio buttons",
        ),
        (
            "Slider",
            "linear_scale",
            Route::Slider,
            "Continuous range slider with tick marks",
        ),
        (
            "Chips",
            "label",
            Route::Chips,
            "Assist, filter, and input chips",
        ),
        (
            "Cards",
            "credit_card",
            Route::Cards,
            "Elevated, filled, and outlined cards",
        ),
        (
            "Progress",
            "hourglass_empty",
            Route::Progress,
            "Linear, circular, and wavy progress bars",
        ),
        (
            "Badges",
            "new_releases",
            Route::Badges,
            "Notification badges and dot indicators",
        ),
        (
            "Dialog",
            "web_asset",
            Route::Dialog,
            "Modal dialogs with confirm/dismiss actions",
        ),
        (
            "List",
            "list",
            Route::List,
            "One-line, two-line, and three-line list items",
        ),
        ("Tabs", "tab", Route::Tabs, "Tab bar navigation"),
        (
            "Typography",
            "text_format",
            Route::Typography,
            "MD3 type scale system",
        ),
        (
            "Box",
            "crop_square",
            Route::Box,
            "MUI-equivalent box layout component",
        ),
        (
            "Grid",
            "grid_on",
            Route::Grid,
            "Responsive grid and container system",
        ),
        (
            "Split Buttons",
            "call_split",
            Route::SplitButtons,
            "Buttons with dropdown action",
        ),
        (
            "Select & Text Area",
            "arrow_drop_down_circle",
            Route::SelectTextarea,
            "Dropdown selects and multiline text areas",
        ),
        (
            "Accordions",
            "expand_more",
            Route::Accordions,
            "Collapsible content panels",
        ),
        (
            "Toggle Buttons",
            "toggle_on",
            Route::ToggleButtons,
            "Single and multi-select toggle button groups",
        ),
        (
            "Alert Box",
            "info",
            Route::AlertBox,
            "Info, success, warning, and error alerts",
        ),
        (
            "Skeleton",
            "view_carousel",
            Route::Skeleton,
            "Text, circular, and card skeleton placeholders",
        ),
        (
            "Breadcrumb",
            "chevron_right",
            Route::Breadcrumb,
            "Navigation breadcrumb trails",
        ),
        (
            "Image List",
            "photo_library",
            Route::ImageList,
            "Standard, masonry, and quilted image grids",
        ),
        (
            "Avatars & Shapes",
            "person",
            Route::AvatarsShapes,
            "User avatars and shape clipping",
        ),
        (
            "Shape Gallery",
            "palette",
            Route::ShapeGallery,
            "Expressive SVG shape collection",
        ),
        (
            "Steppers",
            "timeline",
            Route::Steppers,
            "Progress stepper with step indicators",
        ),
        (
            "Tables",
            "table_chart",
            Route::Tables,
            "Data tables with selection, search, and pagination",
        ),
        (
            "Navigation Drawer",
            "menu",
            Route::NavigationDrawer,
            "Persistent and modal navigation drawers",
        ),
        (
            "Bottom Sheet",
            "expand_less",
            Route::BottomSheet,
            "Standard, modal, and expanding bottom sheets",
        ),
        (
            "Carousel",
            "view_carousel",
            Route::Carousel,
            "Horizontal scrolling items with snap behavior",
        ),
        (
            "Icon",
            "emoji_symbols",
            Route::Icon,
            "Material Symbols icon component with weight variants",
        ),
        (
            "Menu",
            "more_vert",
            Route::Menu,
            "Dropdown menus with smart positioning",
        ),
        (
            "Snackbar",
            "notifications_active",
            Route::Snackbar,
            "Brief messages with optional action",
        ),
        (
            "Tooltip",
            "info",
            Route::Tooltip,
            "Context hints on hover and focus",
        ),
        (
            "Code View",
            "code",
            Route::CodeView,
            "Syntax-highlighted code blocks",
        ),
        (
            "Theme Tokens",
            "palette",
            Route::ThemeTokens,
            "Color scheme and design token reference",
        ),
    ];

    let install_code = r#"// Cargo.toml
[dependencies]
material-rs = { path = "../material-rs/packages/core" }

// main.rs
use material_rs::components::*;
use material_rs::theme::{MaterialThemeProvider, ThemeBuilder, ThemeMode};

#[function_component]
fn App() -> Html {
    let theme = ThemeBuilder::new()
        .mode(ThemeMode::Light)
        .build();

    html! {
        <MaterialThemeProvider theme={theme}>
            <Button label="Hello Material 3!" />
        </MaterialThemeProvider>
    }
}"#;

    html! {
            <>
                <Global css={css!(r#"
                @keyframes s1{0%,100%{transform:translateY(0) rotate(0)}25%{transform:translateY(-14px) rotate(8deg)}50%{transform:translateY(-6px) rotate(-4deg)}75%{transform:translateY(-20px) rotate(6deg)}}
                @keyframes s2{0%,100%{transform:translate(0,0) rotate(0)}25%{transform:translate(10px,-14px) rotate(12deg)}50%{transform:translate(-6px,-22px) rotate(-8deg)}75%{transform:translate(14px,-10px) rotate(5deg)}}
                @keyframes s3{0%,100%{transform:translateY(0) scale(1)}50%{transform:translateY(-16px) scale(1.08)}}
                @keyframes s4{from{transform:rotate(0)}to{transform:rotate(360deg)}}
                @keyframes s5{0%,100%{opacity:.18;transform:scale(1)}50%{opacity:.3;transform:scale(1.12)}}
                a.home-link{text-decoration:none;color:inherit;display:block;height:100%;}
                .docs-card-hover:hover{transform:translateY(-6px);box-shadow:0 8px 24px rgba(0,0,0,0.08);}
            "#)} />

                // ═══════════════════════════════════════════
                // HERO SECTION
                // ═══════════════════════════════════════════
                <Box class={hero_wrap} tag={BoxTag::Section}>
                    // Floating decorative shapes
                    <Box class={combine_classes(&shape_base, &shape_floating_1)}><ShapeHeart size="5rem" color={colors.primary.clone()} /></Box>
                    <Box class={combine_classes(&shape_base, &shape_floating_2)}><ShapeFlower size="4.5rem" color={colors.tertiary.clone()} /></Box>
                    <Box class={combine_classes(&shape_base, &shape_floating_3)}><ShapeGem size="3.5rem" color={colors.secondary.clone()} /></Box>
                    <Box class={combine_classes(&shape_base, &shape_floating_4)}><ShapeArch size="5.5rem" color={colors.primary.clone()} /></Box>
                    <Box class={combine_classes(&shape_base, &shape_floating_5)}><ShapeSunny size="4rem" color={colors.tertiary.clone()} /></Box>
                    <Box class={combine_classes(&shape_base, &shape_floating_6)}><ShapeBurst size="3rem" color={colors.error.clone()} /></Box>
                    <Box class={combine_classes(&shape_base, &shape_floating_7)}><ShapePuffy size="3.5rem" color={colors.primary.clone()} /></Box>

                    <Box class={hero_content}>
                        <Box display="inline-flex" align_items="center" gap="8px" padding="6px 18px"
                            margin_bottom="20px"
                            bg={colors.on_primary_container.clone()}
                            border_radius="9999px">
                            <Icon name="auto_awesome" size="16px" color={colors.primary_container.clone()} />
                            <Typography tag="span" variant={TypographyVariant::LabelMedium}
                                style={format!("color:{};letter-spacing:0.4px;font-weight:500;", colors.primary_container)}>
                                { "Documentation" }
                            </Typography>
                        </Box>

                        <Typography tag="h1" variant={TypographyVariant::DisplayLarge}
                            style={format!("color:{};margin-bottom:20px;line-height:1.08;font-weight:400;letter-spacing:-0.5px;", colors.on_primary_container)}>
                            { "Material RS" }
                        </Typography>

                        <Typography tag="h2" variant={TypographyVariant::HeadlineSmall}
                            style={format!("color:{};margin-bottom:44px;max-width:620px;margin-left:auto;margin-right:auto;margin-top:0;\
                                            line-height:1.5;font-weight:400;opacity:0.85;", colors.on_primary_container)}>
                            { "A comprehensive Material Design 3 Expressive component library for Rust and Yew. Build beautiful, accessible, and performant user interfaces." }
                        </Typography>

                        <Box class={hero_actions}>
                            <Link<Route> to={Route::Buttons}>
                                <Button variant={ButtonVariant::Filled} label="Get Started" icon="arrow_forward" size={ButtonSize::Large} />
                            </Link<Route>>
                            <Link<Route> to={Route::ThemeTokens}>
                                <Button variant={ButtonVariant::Outlined} label="View Tokens" icon="palette" size={ButtonSize::Large} />
                            </Link<Route>>
                            <Link<Route> to={Route::ShowcaseLogin}>
                                <Button variant={ButtonVariant::Text} label="See Showcases" icon="play_arrow" size={ButtonSize::Large} />
                            </Link<Route>>
                        </Box>
                    </Box>
                </Box>

                // ═══════════════════════════════════════════
                // LIVE DEMO SECTION
                // ═══════════════════════════════════════════
                <Box class={combine_classes(&section_max, &demo_wrap)} padding="52px 28px" border_radius="28px" margin_bottom="80px"
                    style={format!("margin-left:auto;margin-right:auto;max-width:1120px;")}>
                    <Typography tag="h2" variant={TypographyVariant::HeadlineLarge}
                        class={section_header_style.clone()}>
                        { "See It in Action" }
                    </Typography>
                    <Typography tag="p" variant={TypographyVariant::BodyLarge}
                        class={section_sub_style.clone()}>
                        { "Interactive components running live — try them yourself. Every component responds to state changes with smooth Material 3 motion." }
                    </Typography>

                    <Box class={demo_grid}>
                        // ── Left Column ──
                        <Box display="flex" flex_direction="column" gap="24px">
                            // Switch demo
                            <Box class={demo_card.clone()} padding="24px" border_radius="20px">
                                <Box display="flex" align_items="center" gap="10px" margin_bottom="16px">
                                    <Box width="32px" height="32px" border_radius="10px" bg={colors.primary_container.clone()}
                                        display="flex" align_items="center" justify_content="center">
                                        <Icon name="toggle_on" size="18px" color={colors.on_primary_container.clone()} />
                                    </Box>
                                    <Typography tag="h3" variant={TypographyVariant::TitleMedium}
                                        style={format!("color:{};font-weight:600;margin:0;", colors.on_surface)}>
                                        { "Switch" }
                                    </Typography>
                                </Box>
                                <Switch label="Enable notifications" checked={*demo_switch}
                                    onchange={let switch_handle = demo_switch.clone(); Callback::from(move |v: bool| switch_handle.set(v))} />
                                <Typography tag="p" variant={TypographyVariant::BodySmall}
                                    style={format!("color:{};margin-top:10px;", colors.on_surface_variant)}>
                                    { html! { format!("State: {}", if *demo_switch { "ON" } else { "OFF" }) } }
                                </Typography>
                            </Box>

                            // Chips demo
                            <Box class={demo_card.clone()} padding="24px" border_radius="20px">
                                <Box display="flex" align_items="center" gap="10px" margin_bottom="16px">
                                    <Box width="32px" height="32px" border_radius="10px" bg={colors.tertiary_container.clone()}
                                        display="flex" align_items="center" justify_content="center">
                                        <Icon name="label" size="18px" color={colors.on_tertiary_container.clone()} />
                                    </Box>
                                    <Typography tag="h3" variant={TypographyVariant::TitleMedium}
                                        style={format!("color:{};font-weight:600;margin:0;", colors.on_surface)}>
                                        { "Chips" }
                                    </Typography>
                                </Box>
                                <Box display="flex" gap="8px" flex_wrap="wrap">
                                    { for ["React", "Vue", "Svelte", "Yew"].iter().map(|tech| {
                                        let label = tech.to_string();
                                        let selected = *demo_selected == label;
                                        let selected_handle = demo_selected.clone();
                                        html! {
                                            <Chip label={label.clone()} chip_type={ChipType::Filter} selected={selected}
                                                on_select={let handle = selected_handle.clone(); let chip_label = label.clone(); Callback::from(move |v: bool| {
                                                    if v { handle.set(chip_label.clone()); }
                                                })} />
                                        }
                                    })}
                                </Box>
                                <Typography tag="p" variant={TypographyVariant::BodySmall}
                                    style={format!("color:{};margin-top:10px;", colors.on_surface_variant)}>
                                    { html! { format!("Selected: {}", *demo_selected) } }
                                </Typography>
                            </Box>

                            // Counter demo
                            <Box class={demo_card.clone()} padding="24px" border_radius="20px">
                                <Box display="flex" align_items="center" gap="10px" margin_bottom="16px">
                                    <Box width="32px" height="32px" border_radius="10px" bg={colors.secondary_container.clone()}
                                        display="flex" align_items="center" justify_content="center">
                                        <Icon name="pin" size="18px" color={colors.on_secondary_container.clone()} />
                                    </Box>
                                    <Typography tag="h3" variant={TypographyVariant::TitleMedium}
                                        style={format!("color:{};font-weight:600;margin:0;", colors.on_surface)}>
                                        { "Counter" }
                                    </Typography>
                                </Box>
                                <Box display="flex" align_items="center" gap="16px">
                                    <IconButton icon="remove" variant={IconButtonVariant::Outlined}
                                        label="Decrement"
                                        onclick={let counter_handle = demo_count.clone(); Callback::from(move |_: MouseEvent| { if *counter_handle > 0 { counter_handle.set(*counter_handle - 1); } })} />
                                    <Typography tag="span" variant={TypographyVariant::HeadlineMedium}
                                        style={format!("color:{};font-weight:700;min-width:60px;text-align:center;", colors.on_surface)}>
                                        { html! { demo_count.to_string() } }
                                    </Typography>
                                    <IconButton icon="add" variant={IconButtonVariant::Outlined}
                                        label="Increment"
                                        onclick={let counter_handle = demo_count.clone(); Callback::from(move |_: MouseEvent| counter_handle.set(*counter_handle + 1))} />
                                </Box>
                            </Box>
                        </Box>

                        // ── Right Column ──
                        <Box display="flex" flex_direction="column" gap="24px">
                            // Text field demo
                            <Box class={demo_card.clone()} padding="24px" border_radius="20px">
                                <Box display="flex" align_items="center" gap="10px" margin_bottom="16px">
                                    <Box width="32px" height="32px" border_radius="10px" bg={colors.primary_container.clone()}
                                        display="flex" align_items="center" justify_content="center">
                                        <Icon name="text_fields" size="18px" color={colors.on_primary_container.clone()} />
                                    </Box>
                                    <Typography tag="h3" variant={TypographyVariant::TitleMedium}
                                        style={format!("color:{};font-weight:600;margin:0;", colors.on_surface)}>
                                        { "Text Field" }
                                    </Typography>
                                </Box>
                                <TextField label="Search components..." value={(*demo_text).clone()} leading_icon="search"
                                    onchange={let text_handle = demo_text.clone(); Callback::from(move |v: String| text_handle.set(v))} />
                            </Box>

                            // Alert demo
                            <Box class={demo_card.clone()} padding="24px" border_radius="20px">
                                <Box display="flex" align_items="center" gap="10px" margin_bottom="16px">
                                    <Box width="32px" height="32px" border_radius="10px" bg={colors.tertiary_container.clone()}
                                        display="flex" align_items="center" justify_content="center">
                                        <Icon name="info" size="18px" color={colors.on_tertiary_container.clone()} />
                                    </Box>
                                    <Typography tag="h3" variant={TypographyVariant::TitleMedium}
                                        style={format!("color:{};font-weight:600;margin:0;", colors.on_surface)}>
                                        { "Alert Box" }
                                    </Typography>
                                </Box>
                                <AlertBox severity={AlertSeverity::Success} title="Great choice!" message="Material-rs gives you a full component kit for production apps." />
                            </Box>

                            // Progress demo
                            <Box class={demo_card.clone()} padding="24px" border_radius="20px">
                                <Box display="flex" align_items="center" gap="10px" margin_bottom="16px">
                                    <Box width="32px" height="32px" border_radius="10px" bg={colors.secondary_container.clone()}
                                        display="flex" align_items="center" justify_content="center">
                                        <Icon name="hourglass_empty" size="18px" color={colors.on_secondary_container.clone()} />
                                    </Box>
                                    <Typography tag="h3" variant={TypographyVariant::TitleMedium}
                                        style={format!("color:{};font-weight:600;margin:0;", colors.on_surface)}>
                                        { "Progress" }
                                    </Typography>
                                </Box>
                                <Box display="flex" flex_direction="column" gap="14px">
                                    <Box display="flex" align_items="center" gap="12px">
                                        <Box flex_grow="1">
                                            <ProgressIndicator progress_type={ProgressType::Linear} value={Some(0.72)} />
                                        </Box>
                                        <Typography tag="span" variant={TypographyVariant::LabelSmall}
                                            style={format!("color:{};font-weight:500;", colors.on_surface_variant)}>
                                            { "72%" }
                                        </Typography>
                                    </Box>
                                    <Box display="flex" align_items="center" gap="12px">
                                        <ProgressIndicator progress_type={ProgressType::Circular} value={Some(0.58)} />
                                        <Typography tag="span" variant={TypographyVariant::LabelSmall}
                                            style={format!("color:{};font-weight:500;", colors.on_surface_variant)}>
                                            { "58%" }
                                        </Typography>
                                    </Box>
                                </Box>
                            </Box>
                        </Box>
                    </Box>
                </Box>

                // ═══════════════════════════════════════════
                // STATS SECTION
                // ═══════════════════════════════════════════
                <Box class={combine_classes(&section_max, &section_gap)}>
                    <Box class={stat_row}>
                        { for [
                            ("40+", "Components", "Material 3 spec compliant"),
                            ("0", "JS Dependencies", "Pure Rust / WASM"),
                            ("5", "Color Palettes", "Dynamic seed theming"),
                        ].iter().map(|(num, label, sub)| {
                            let stat_value = num.to_string();
                            let stat_label = label.to_string();
                            let stat_subtitle = sub.to_string();
                            html! {
                                <Card variant={CardVariant::Elevated}>
                                    <Box padding="36px 20px" text_align="center">
                                        <Typography tag="div" variant={TypographyVariant::DisplaySmall}
                                            style={format!("color:{};font-weight:700;margin-bottom:6px;letter-spacing:-1px;", colors.primary)}>
                                            { html! { stat_value } }
                                        </Typography>
                                        <Typography tag="div" variant={TypographyVariant::TitleMedium}
                                            style={format!("color:{};font-weight:600;", colors.on_surface)}>
                                            { html! { stat_label } }
                                        </Typography>
                                        <Typography tag="p" variant={TypographyVariant::BodySmall}
                                            style={format!("color:{};margin-top:6px;", colors.on_surface_variant)}>
                                            { html! { stat_subtitle } }
                                        </Typography>
                                    </Box>
                                </Card>
                            }
                        })}
                    </Box>
                </Box>

                // ═══════════════════════════════════════════
                // FEATURES SECTION
                // ═══════════════════════════════════════════
                <Box class={combine_classes(&section_max, &section_gap)}>
                    <Typography tag="h2" variant={TypographyVariant::HeadlineLarge}
                        class={section_header_style.clone()}>
                        { "Why Material RS?" }
                    </Typography>
                    <Typography tag="p" variant={TypographyVariant::BodyLarge}
                        class={section_sub_style.clone()}>
                        { "Everything you need to build production-grade Material Design 3 applications in Rust with confidence and speed." }
                    </Typography>

                    <Box class={feature_grid}>
                        { for [
                            ("widgets", "40+ Components", "Buttons, cards, dialogs, tables, navigation, and every building block for complete applications."),
                            ("palette", "Material 3 Expressive", "Full MD3 spec with dynamic color generation, typography scale, shape families, and motion tokens."),
                            ("memory", "Pure Rust / WASM", "Zero JavaScript. Compiles to WebAssembly with full type safety and native performance."),
                            ("translate", "RTL Support", "Built-in right-to-left layout mirroring via Theme direction for internationalization."),
                            ("code", "Syntax Highlighting", "Built-in code view component with syntax highlighting for Rust, JS, CSS, HTML, and more."),
                            ("tune", "Fully Themeable", "Light/dark mode, 5 color palettes, custom typography, shape tokens, and elevation scales."),
                        ].iter().map(|(icon, title, description)| {
                            let icon_name = icon.to_string();
                            let title_text = title.to_string();
                            let description_text = description.to_string();
                            html! {
                                <Card variant={CardVariant::Elevated} interactive={true} class="docs-card-hover">
                                    <Box padding="24px 20px" display="flex" flex_direction="column" gap="4px">
                                        <Box class={icon_box.clone()}>
                                            <Icon name={icon_name} color={colors.on_primary_container.clone()} />
                                        </Box>
                                        <Typography tag="h3" variant={TypographyVariant::TitleLarge}
                                            style={format!("margin-bottom:8px;font-weight:600;color:{};", colors.on_surface)}>
                                            { html! { title_text } }
                                        </Typography>
                                        <Typography tag="p" variant={TypographyVariant::BodyMedium}
                                            style={format!("color:{};line-height:1.6;", colors.on_surface_variant)}>
                                            { html! { description_text } }
                                        </Typography>
                                    </Box>
                                </Card>
                            }
                        })}
                    </Box>
                </Box>

                // ═══════════════════════════════════════════
                // QUICK START / INSTALL SECTION
                // ═══════════════════════════════════════════
                <Box class={section_gap.clone()}>
                    <Box class={install_wrap}>
                        <Box display="flex" align_items="center" gap="12px" margin_bottom="20px">
                            <Box width="36px" height="36px" border_radius="10px" bg={colors.primary_container.clone()}
                                display="flex" align_items="center" justify_content="center">
                                <Icon name="terminal" size="20px" color={colors.on_primary_container.clone()} />
                            </Box>
                            <Typography tag="h3" variant={TypographyVariant::TitleLarge}
                                style={format!("color:{};font-weight:600;margin:0;", colors.on_surface)}>
                                { "Quick Start" }
                            </Typography>
                        </Box>

                        <Typography tag="p" variant={TypographyVariant::BodyLarge}
                            style={format!("color:{};margin-bottom:24px;line-height:1.7;", colors.on_surface_variant)}>
                            { "Add Material RS to your Cargo.toml and wrap your app with MaterialThemeProvider. You'll have access to all 40+ components instantly." }
                        </Typography>

                        <CodeBlock code={install_code.to_string()} language={"rust".to_string()} />

                        <Box class={feat_box_bg} padding="18px 22px" border_radius="12px" margin_top="24px">
                            <Typography tag="h4" variant={TypographyVariant::TitleSmall}
                                style={format!("color:{};margin-bottom:12px;font-weight:600;", colors.on_surface)}>
                                { "What You Get" }
                            </Typography>
                            <Box tag={BoxTag::Ul} class={feat_list}>
                                { for [
                                    "40+ MD3 Expressive components with full API documentation",
                                    "Light/dark mode with 5 color palette seeds",
                                    "RTL support via Theme direction property",
                                    "Syntax-highlighted code views (Rust, JS, CSS, HTML)",
                                    "Pure Rust — compiles to WASM, zero JS dependencies",
                                ].iter().map(|item| {
                                    let feature_text = item.to_string();
                                    html! {
                                        <Box tag={BoxTag::Li} class={feat_item.clone()}>
                                            <Icon name="check_circle" size="18px" color={colors.primary.clone()} />
                                            <Typography tag="span" variant={TypographyVariant::BodyMedium}
                                                style={format!("color:{};", colors.on_surface_variant)}>
                                                { html! { feature_text } }
                                            </Typography>
                                        </Box>
                                    }
                                })}
                            </Box>
                        </Box>
                    </Box>
                </Box>

                // ═══════════════════════════════════════════
                // COMPONENT REFERENCE GRID
                // ═══════════════════════════════════════════
                <Box class={section_max.clone()} margin_bottom="80px">
                    <Typography tag="h2" variant={TypographyVariant::HeadlineLarge}
                        class={section_header_style.clone()}>
                        { html! { format!("All {} Components", comp_sections.len()) } }
                    </Typography>
                    <Typography tag="p" variant={TypographyVariant::BodyLarge}
                        class={section_sub_style.clone()}>
                        { "Explore every component in our library. Each page includes interactive demos, prop documentation, and usage examples." }
                    </Typography>
    <Spacer />
                            <Grid container={true} spacing="16px">
                        { for comp_sections.iter().map(|(title, icon, route, description)| {
                            let title_text = title.to_string();
                            let icon_name = icon.to_string();
                            let description_text = description.to_string();
                            let route_target = route.clone();
                            html! {
                               <Grid item={true} xs={12} md={6} lg={4} xl={3}>
                                <Link<Route> to={route_target} classes="home-link">
                                    <Card variant={CardVariant::Elevated} interactive={true} class="docs-card-hover">
                                        <Box display="flex" align_items="center" gap="16px" padding="6px 0">
                                            <Box class={comp_icon.clone()}>
                                                <Icon name={icon_name} size="22px" color={colors.on_primary_container.clone()} />
                                            </Box>
                                            <Box min_width="0" flex_grow="1">
                                                <Typography tag="div" variant={TypographyVariant::TitleSmall}
                                                    style={format!("font-weight:600;color:{};margin-bottom:3px;", colors.on_surface)}>
                                                    { html! { title_text } }
                                                </Typography>
                                                <Typography tag="p" variant={TypographyVariant::BodySmall}
                                                    style={format!("color:{};white-space:nowrap;overflow:hidden;text-overflow:ellipsis;", colors.on_surface_variant)}>
                                                    { html! { description_text } }
                                                </Typography>
                                            </Box>
                                            <Icon name="chevron_right" size="20px" color={colors.primary.clone()} />
                                        </Box>
                                    </Card>
                                </Link<Route>>
                                </Grid>
                            }
                        })}
                    </Grid>
                </Box>

                // ═══════════════════════════════════════════
                // FOOTER
                // ═══════════════════════════════════════════
                <Box class={section_max.clone()} tag={BoxTag::Footer} padding="56px 0" text_align="center"
                    border_top={format!("1px solid {}", colors.outline_variant)}>
                    <Box display="flex" flex_direction="column" align_items="center" gap="16px">
                        <Box display="flex" align_items="center" gap="10px">
                            <ShapeGem size="1.8rem" color={colors.primary.clone()} />
                            <Typography tag="span" variant={TypographyVariant::TitleMedium}
                                style={format!("font-weight:600;color:{};", colors.on_surface)}>
                            { "Material RS" }
                        </Typography>
                    </Box>
                    <Typography tag="p" variant={TypographyVariant::BodyMedium}
                        style={format!("color:{};line-height:1.6;max-width:480px;", colors.on_surface_variant)}>
                        { "Built with Rust, Yew, and Stylist. Open source under MIT License." }
                        </Typography>
                    </Box>
                </Box>
            </>
        }
}
