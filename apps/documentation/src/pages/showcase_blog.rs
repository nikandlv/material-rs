use yew::prelude::*;
use material_rs::components::*;
use material_rs::components::box_layout::{Box, BoxTag};
use material_rs::theme::Theme;
use stylist::yew::use_style;
use super::{Section, Demo};

#[function_component]
pub fn ShowcaseBlogPage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let email_value = use_state(String::new);
    let email_onchange = {
        let email_value = email_value.clone();
        Callback::from(move |v: String| {
            email_value.set(v);
        })
    };

    let categories = [
        ("Tutorials", 12u32),
        ("Guides", 8),
        ("Updates", 15),
        ("News", 3),
        ("Releases", 6),
    ];

    let popular_tags = vec![
        "Rust", "WebAssembly", "Yew", "Material 3", "Components",
        "Accessibility", "Theming", "TypeScript", "WASM", "Open Source",
    ];

    // Static CSS classes via use_style!
    let desc_style = use_style!(r#"font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;"#);
    let topbar_actions = use_style!(r#"display: flex; align-items: center; gap: 4px;"#);
    let content_center = use_style!(r#"max-width: 1100px; margin: 0 auto;"#);
    let featured_content = use_style!(r#"padding: 24px 32px 32px;"#);
    let featured_title = use_style!(r#"font-size: 32px; font-weight: 400; margin-bottom: 12px; line-height: 1.3;"#);
    let featured_meta = use_style!(r#"font-size: 14px; margin-bottom: 16px; display: flex; align-items: center; gap: 8px;"#);
    let featured_body = use_style!(r#"font-size: 14px; line-height: 1.6; margin-bottom: 20px;"#);
    let featured_chips = use_style!(r#"display: flex; gap: 8px;"#);
    let section_title = use_style!(r#"font-size: 20px; font-weight: 500; margin-bottom: 20px;"#);
    let post_grid = use_style!(r#"display: grid; grid-template-columns: repeat(3, 1fr); gap: 20px; margin-bottom: 40px;"#);
    let post_content = use_style!(r#"padding: 20px;"#);
    let post_title = use_style!(r#"font-size: 18px; font-weight: 500; margin-bottom: 8px; line-height: 1.4;"#);
    let post_meta = use_style!(r#"font-size: 13px; margin-bottom: 12px; display: flex; align-items: center; gap: 6px;"#);
    let post_body = use_style!(r#"font-size: 13px; line-height: 1.5; margin-bottom: 16px;"#);
    let sidebar_grid = use_style!(r#"display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 20px;"#);
    let sidebar_card = use_style!(r#"border-radius: 20px; padding: 24px; box-shadow: 0 1px 3px rgba(0,0,0,0.12);"#);
    let sidebar_card_title = use_style!(r#"font-size: 18px; font-weight: 500; margin-bottom: 16px;"#);
    let category_item = use_style!(r#"display: flex; align-items: center; justify-content: space-between; padding: 10px 0; border-bottom: 1px solid var(--md-sys-color-outline-variant);"#);
    let category_name = use_style!(r#"font-size: 14px;"#);
    let category_count = use_style!(r#"font-size: 13px; font-weight: 500; padding: 2px 10px; border-radius: 12px;"#);
    let tags_wrap = use_style!(r#"display: flex; flex-wrap: wrap; gap: 8px;"#);
    let newsletter_desc = use_style!(r#"font-size: 13px; margin-bottom: 20px; line-height: 1.5;"#);
    let newsletter_field = use_style!(r#"margin-bottom: 16px;"#);
    let grid_auto = use_style!(r#"display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 12px;"#);
    let card_item = use_style!(r#"padding: 16px; border-radius: 12px; border: 1px solid var(--md-sys-color-outline-variant); background-color: var(--md-sys-color-surface-container-low);"#);
    let card_name = use_style!(r#"font-size: 14px; font-weight: 600; display: block; margin-bottom: 4px; color: var(--md-sys-color-on-surface);"#);
    let card_desc = use_style!(r#"font-size: 12px; line-height: 1.4; color: var(--md-sys-color-on-surface-variant);"#);

    html! {
        <>
            <Section title="Blog Showcase">
                <Box class={desc_style.get_class_name().to_string()}>
                    {"A full-page blog layout built entirely with Material RS components. \
                      This showcases TopAppBar, Card, Typography, Chip, TextField, Button, and Avatar \
                      working together to create a realistic Material Design 3 blog experience."}
                </Box>

                <Demo title="Interactive Blog Layout">
                    <Box
                        width="100%"
                        border_radius="16px"
                        overflow="hidden"
                        border={format!("1px solid {}", theme.colors.outline_variant)}
                        display="flex"
                        flex_direction="column"
                        height="1000px"
                    >
                        // ── Top App Bar ──
                        <Box
                            tag={BoxTag::Header}
                            display="flex"
                            align_items="center"
                            padding="8px 16px"
                            height="64px"
                            bg="primary-container"
                            color="on-primary-container"
                            flex_shrink="0"
                        >
                            <IconButton icon="menu" variant={IconButtonVariant::Standard} label="Menu" />
                            <Box tag={BoxTag::Span} font_size="22px" font_weight="400" margin_left="16px" flex="1" color="on-primary-container">
                                {"Material RS Blog"}
                            </Box>
                            <Box class={topbar_actions.get_class_name().to_string()}>
                                <IconButton icon="search" variant={IconButtonVariant::Standard} label="Search" />
                                <Box margin_left="8px">
                                    <Avatar initials="MR" size="sm" />
                                </Box>
                            </Box>
                        </Box>

                        // ── Content Area ──
                        <Box flex="1" overflow_y="auto" bg="surface-container-low" padding="32px">
                            <Box class={content_center.get_class_name().to_string()}>

                                // ── Featured Post ──
                                <Box
                                    border_radius="28px"
                                    overflow="hidden"
                                    bg="surface-container"
                                    box_shadow="0 1px 3px rgba(0,0,0,0.12), 0 1px 2px rgba(0,0,0,0.24)"
                                    margin_bottom="40px"
                                >
                                    // Gradient header
                                    <Box
                                        height="240px"
                                        display="flex"
                                        align_items="center"
                                        justify_content="center"
                                        style={format!("background: linear-gradient(135deg, {}, {});", theme.colors.primary_container, theme.colors.tertiary_container)}
                                    >
                                        <span class="material-symbols-outlined" style={format!("font-size: 72px; color: {}; opacity: 0.6;", theme.colors.on_primary_container)}>
                                            {"auto_stories"}
                                        </span>
                                    </Box>
                                    // Content
                                    <Box class={featured_content.get_class_name().to_string()}>
                                        <Box class={featured_title.get_class_name().to_string()} color="on-surface">
                                            {"Getting Started with Material 3 in Rust"}
                                        </Box>
                                        <Box class={featured_meta.get_class_name().to_string()} color="on-surface-variant">
                                            <Avatar initials="JD" size="xs" />
                                            <Box tag={BoxTag::Span}>{"John Doe"}</Box>
                                            <Box tag={BoxTag::Span} style="margin: 0 4px;">{"·"}</Box>
                                            <Box tag={BoxTag::Span}>{"June 28, 2026"}</Box>
                                            <Box tag={BoxTag::Span} style="margin: 0 4px;">{"·"}</Box>
                                            <Box tag={BoxTag::Span}>{"8 min read"}</Box>
                                        </Box>
                                        <Box class={featured_body.get_class_name().to_string()} color="on-surface-variant">
                                            {"Explore how to build beautiful, accessible user interfaces using Material RS and the Material Design 3 system. This comprehensive guide covers component setup, theme configuration, and best practices for creating production-ready applications with Rust and WebAssembly."}
                                        </Box>
                                        <Box class={featured_chips.get_class_name().to_string()}>
                                            <Chip label="Tutorial" chip_type={ChipType::Filter} />
                                            <Chip label="Rust" chip_type={ChipType::Filter} />
                                            <Chip label="Yew" chip_type={ChipType::Filter} />
                                        </Box>
                                    </Box>
                                </Box>

                                // ── Recent Posts Grid ──
                                <Box class={section_title.get_class_name().to_string()} color="on-surface">
                                    {"Recent Posts"}
                                </Box>
                                <Box class={post_grid.get_class_name().to_string()}>

                                    // Post 1
                                    <Box
                                        border_radius="20px"
                                        overflow="hidden"
                                        bg="surface-container"
                                        box_shadow="0 1px 3px rgba(0,0,0,0.12), 0 1px 2px rgba(0,0,0,0.24)"
                                    >
                                        <Box
                                            height="120px"
                                            display="flex"
                                            align_items="center"
                                            justify_content="center"
                                            style={format!("background: linear-gradient(135deg, {}, {});", theme.colors.secondary_container, theme.colors.primary_container)}
                                        >
                                            <span class="material-symbols-outlined" style={format!("font-size: 40px; color: {}; opacity: 0.6;", theme.colors.on_secondary_container)}>
                                                {"accessibility_new"}
                                            </span>
                                        </Box>
                                        <Box class={post_content.get_class_name().to_string()}>
                                            <Box class={post_title.get_class_name().to_string()} color="on-surface">
                                                {"Building Accessible UIs"}
                                            </Box>
                                            <Box class={post_meta.get_class_name().to_string()} color="on-surface-variant">
                                                <Avatar initials="SC" size="xs" />
                                                <Box tag={BoxTag::Span}>{"Sarah Chen"}</Box>
                                                <Box tag={BoxTag::Span} style="margin: 0 2px;">{"·"}</Box>
                                                <Box tag={BoxTag::Span}>{"5 min read"}</Box>
                                            </Box>
                                            <Box class={post_body.get_class_name().to_string()} color="on-surface-variant">
                                                {"Learn how Material RS makes it easy to create inclusive interfaces that work for everyone."}
                                            </Box>
                                            <Chip label="Accessibility" chip_type={ChipType::Filter} />
                                        </Box>
                                    </Box>

                                    // Post 2
                                    <Box
                                        border_radius="20px"
                                        overflow="hidden"
                                        bg="surface-container"
                                        box_shadow="0 1px 3px rgba(0,0,0,0.12), 0 1px 2px rgba(0,0,0,0.24)"
                                    >
                                        <Box
                                            height="120px"
                                            display="flex"
                                            align_items="center"
                                            justify_content="center"
                                            style={format!("background: linear-gradient(135deg, {}, {});", theme.colors.tertiary_container, theme.colors.secondary_container)}
                                        >
                                            <span class="material-symbols-outlined" style={format!("font-size: 40px; color: {}; opacity: 0.6;", theme.colors.on_tertiary_container)}>
                                                {"palette"}
                                            </span>
                                        </Box>
                                        <Box class={post_content.get_class_name().to_string()}>
                                            <Box class={post_title.get_class_name().to_string()} color="on-surface">
                                                {"Theme Customization Guide"}
                                            </Box>
                                            <Box class={post_meta.get_class_name().to_string()} color="on-surface-variant">
                                                <Avatar initials="AK" size="xs" />
                                                <Box tag={BoxTag::Span}>{"Alex Kim"}</Box>
                                                <Box tag={BoxTag::Span} style="margin: 0 2px;">{"·"}</Box>
                                                <Box tag={BoxTag::Span}>{"8 min read"}</Box>
                                            </Box>
                                            <Box class={post_body.get_class_name().to_string()} color="on-surface-variant">
                                                {"Customize your Material 3 theme with dynamic color, tonal palettes, and more."}
                                            </Box>
                                            <Chip label="Theming" chip_type={ChipType::Filter} />
                                        </Box>
                                    </Box>

                                    // Post 3
                                    <Box
                                        border_radius="20px"
                                        overflow="hidden"
                                        bg="surface-container"
                                        box_shadow="0 1px 3px rgba(0,0,0,0.12), 0 1px 2px rgba(0,0,0,0.24)"
                                    >
                                        <Box
                                            height="120px"
                                            display="flex"
                                            align_items="center"
                                            justify_content="center"
                                            style={format!("background: linear-gradient(135deg, {}, {});", theme.colors.primary_container, theme.colors.tertiary_container)}
                                        >
                                            <span class="material-symbols-outlined" style={format!("font-size: 40px; color: {}; opacity: 0.6;", theme.colors.on_primary_container)}>
                                                {"widgets"}
                                            </span>
                                        </Box>
                                        <Box class={post_content.get_class_name().to_string()}>
                                            <Box class={post_title.get_class_name().to_string()} color="on-surface">
                                                {"Component Composition Patterns"}
                                            </Box>
                                            <Box class={post_meta.get_class_name().to_string()} color="on-surface-variant">
                                                <Avatar initials="MR" size="xs" />
                                                <Box tag={BoxTag::Span}>{"Mike Rivera"}</Box>
                                                <Box tag={BoxTag::Span} style="margin: 0 2px;">{"·"}</Box>
                                                <Box tag={BoxTag::Span}>{"12 min read"}</Box>
                                            </Box>
                                            <Box class={post_body.get_class_name().to_string()} color="on-surface-variant">
                                                {"Advanced techniques for composing complex UIs from Material RS building blocks."}
                                            </Box>
                                            <Chip label="Advanced" chip_type={ChipType::Filter} />
                                        </Box>
                                    </Box>
                                </Box>

                                // ── Sidebar Content (stacked below grid) ──
                                <Box class={sidebar_grid.get_class_name().to_string()}>

                                    // Categories Card
                                    <Box class={sidebar_card.get_class_name().to_string()} bg="surface-container">
                                        <Box class={sidebar_card_title.get_class_name().to_string()} color="on-surface">
                                            {"Categories"}
                                        </Box>
                                        { for categories.iter().map(|(name, count)| {
                                            let name_owned = name.to_string();
                                            let name_for_key = name_owned.clone();
                                            let count_val = *count;
                                            html! {
                                                <Box key={name_for_key} class={category_item.get_class_name().to_string()}>
                                                    <Box tag={BoxTag::Span} class={category_name.get_class_name().to_string()} color="on-surface">
                                                        { Html::from(name_owned) }
                                                    </Box>
                                                    <Box
                                                        tag={BoxTag::Span}
                                                        class={category_count.get_class_name().to_string()}
                                                        color="on-secondary-container"
                                                        bg="secondary-container"
                                                    >{ Html::from(count_val.to_string()) }</Box>
                                                </Box>
                                            }
                                        })}
                                    </Box>

                                    // Popular Tags Card
                                    <Box class={sidebar_card.get_class_name().to_string()} bg="surface-container">
                                        <Box class={sidebar_card_title.get_class_name().to_string()} color="on-surface">
                                            {"Popular Tags"}
                                        </Box>
                                        <Box class={tags_wrap.get_class_name().to_string()}>
                                            { for popular_tags.iter().map(|tag| {
                                                let tag = tag.to_string();
                                                let key = tag.clone();
                                                html! {
                                                    <Chip key={key} label={tag} chip_type={ChipType::Filter} />
                                                }
                                            })}
                                        </Box>
                                    </Box>

                                    // Newsletter Signup Card
                                    <Box class={sidebar_card.get_class_name().to_string()} bg="surface-container">
                                        <Box class={sidebar_card_title.get_class_name().to_string()} color="on-surface">
                                            {"Newsletter"}
                                        </Box>
                                        <Box class={newsletter_desc.get_class_name().to_string()} color="on-surface-variant">
                                            {"Stay updated with the latest articles, tutorials, and releases from the Material RS community."}
                                        </Box>
                                        <Box class={newsletter_field.get_class_name().to_string()}>
                                            <TextField
                                                label="Email address"
                                                value={(*email_value).clone()}
                                                onchange={email_onchange}
                                            />
                                        </Box>
                                        <Button variant={ButtonVariant::Filled} label="Subscribe" icon="mail" />
                                    </Box>
                                </Box>
                            </Box>
                        </Box>
                    </Box>
                </Demo>
            </Section>

            <Section title="Architecture">
                <Box class={desc_style.get_class_name().to_string()}>
                    {"This blog layout demonstrates Material Design 3 patterns: a centered TopAppBar with search and avatar, \
                      elevated Cards with gradient header areas, Chip components for tags, Avatar for author avatars, \
                      TextField and Button for newsletter signup, and a responsive grid layout for recent posts."}
                </Box>

                <Demo title="Key Components Used">
                    <Box class={grid_auto.get_class_name().to_string()}>
                        { for [
                            ("TopAppBar", "Global navigation with search and avatar actions"),
                            ("Card", "Elevated cards for featured and recent posts"),
                            ("Chip", "Tag filters for post categories"),
                            ("Avatar", "Author initials with circular theme styling"),
                            ("TextField", "Email input for newsletter signup"),
                            ("Button", "Subscribe CTA with filled variant"),
                            ("Typography", "Title, body, and caption text styles"),
                        ].iter().map(|(name, desc)| {
                            html! {
                                <Box class={card_item.get_class_name().to_string()}>
                                    <Box tag={BoxTag::Span} class={card_name.get_class_name().to_string()}>{ Html::from(name.to_string()) }</Box>
                                    <Box tag={BoxTag::Span} class={card_desc.get_class_name().to_string()}>{ Html::from(desc.to_string()) }</Box>
                                </Box>
                            }
                        })}
                    </Box>
                </Demo>
            </Section>
        </>
    }
}
