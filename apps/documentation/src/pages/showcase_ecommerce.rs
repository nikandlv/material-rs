use yew::prelude::*;
use material_rs::components::*;
use material_rs::components::box_layout::{Box, BoxTag};
use material_rs::theme::Theme;
use stylist::yew::use_style;
use super::{Section, Demo};

#[derive(Clone, PartialEq)]
struct Product {
    name: String,
    price: String,
    rating: f64,
    icon: String,
    color: String,
}

#[function_component]
pub fn ShowcaseEcommercePage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let active_filter = use_state(|| "all".to_string());

    let products = [
        Product { name: "Wireless Headphones".into(), price: "$79.99".into(), rating: 4.5, icon: "headphones".into(), color: theme.colors.primary_container.clone() },
        Product { name: "Smart Watch".into(), price: "$199.99".into(), rating: 4.8, icon: "watch".into(), color: theme.colors.secondary_container.clone() },
        Product { name: "USB-C Hub".into(), price: "$49.99".into(), rating: 4.2, icon: "cable".into(), color: theme.colors.tertiary_container.clone() },
        Product { name: "Mechanical Keyboard".into(), price: "$129.99".into(), rating: 4.7, icon: "keyboard".into(), color: theme.colors.primary_container.clone() },
        Product { name: "Laptop Stand".into(), price: "$59.99".into(), rating: 4.4, icon: "laptop".into(), color: theme.colors.secondary_container.clone() },
        Product { name: "Webcam HD".into(), price: "$89.99".into(), rating: 4.6, icon: "videocam".into(), color: theme.colors.tertiary_container.clone() },
    ];

    let filters = [
        ("all", "All"),
        ("electronics", "Electronics"),
        ("clothing", "Clothing"),
        ("home", "Home"),
    ];

    // Static CSS classes via use_style!
    let desc_style = use_style!(r#"font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;"#);
    let topbar_actions = use_style!(r#"display: flex; align-items: center; gap: 4px;"#);
    let product_card_body = use_style!(r#"padding: 16px;"#);
    let icon_circle = use_style!(r#"width: 80px; height: 80px; border-radius: 50%; display: flex; align-items: center; justify-content: center; margin: 0 auto 16px auto;"#);
    let star_row = use_style!(r#"display: flex; align-items: center; justify-content: center; gap: 2px; margin-bottom: 8px;"#);
    let rating_text = use_style!(r#"font-size: 12px; margin-left: 4px;"#);
    let price_text = use_style!(r#"font-size: 18px; font-weight: 600; margin: 0 0 12px 0; text-align: center;"#);
    let pagination_text = use_style!(r#"font-size: 14px;"#);
    let pagination_pages = use_style!(r#"display: flex; align-items: center; gap: 4px;"#);
    let page_btn = use_style!(r#"width: 36px; height: 36px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 14px; font-weight: 500; cursor: pointer;"#);

    let render_stars = |rating: f64| {
        let full_stars = rating.floor() as i32;
        let has_half = (rating - rating.floor()) >= 0.5;
        let mut stars = Vec::new();

        for i in 0..5 {
            if i < full_stars {
                stars.push(html! {
                    <span class="material-symbols-outlined" style="font-size: 16px; color: #FFC107; font-variation-settings: 'FILL' 1;">
                        {"star"}
                    </span>
                });
            } else if i == full_stars && has_half {
                stars.push(html! {
                    <span class="material-symbols-outlined" style="font-size: 16px; color: #FFC107; font-variation-settings: 'FILL' 1;">
                        {"star_half"}
                    </span>
                });
            } else {
                stars.push(html! {
                    <span class="material-symbols-outlined" style="font-size: 16px; color: #FFC107;">
                        {"star"}
                    </span>
                });
            }
        }
        stars
    };

    html! {
        <>
            <Section title="E-commerce Store Showcase">
                <Box class={desc_style.get_class_name().to_string()}>
                    {"A product listing page built with Material Design 3 components. \
                      This showcases TopAppBar with badge, Filter Chips, Product Cards with \
                      ratings, and pagination controls."}
                </Box>
            </Section>

            <Demo title="Online Store Layout">
                <Box
                    width="100%"
                    border_radius="16px"
                    overflow="hidden"
                    border={format!("1px solid {}", theme.colors.outline_variant)}
                    display="flex"
                    flex_direction="column"
                >
                    // ── Top App Bar ──
                    <Box
                        display="flex"
                        align_items="center"
                        padding="8px 16px"
                        height="64px"
                        bg="surface"
                        color="on-surface"
                        flex_shrink="0"
                    >
                        <IconButton icon="menu" variant={IconButtonVariant::Standard} label="Menu" />
                        <Box font_size="22px" font_weight="400" margin_left="16px" flex="1" color="on-surface">
                            {"Material RS Store"}
                        </Box>
                        <Box class={topbar_actions.get_class_name().to_string()}>
                            <IconButton icon="search" variant={IconButtonVariant::Standard} label="Search" />
                            <Badge count={3}>
                                <IconButton icon="shopping_cart" variant={IconButtonVariant::Standard} label="Cart" />
                            </Badge>
                        </Box>
                    </Box>

                    // ── Filter Chips ──
                    <Box
                        display="flex"
                        gap="8px"
                        padding="12px 16px"
                        bg="surface"
                        border_bottom={format!("1px solid {}", theme.colors.outline_variant)}
                        overflow_x="auto"
                    >
                        { for filters.iter().map(|(key, label)| {
                            let is_active = *active_filter == *key;
                            let filter_key = key.to_string();
                            let active_filter = active_filter.clone();
                            let chip_bg = if is_active {
                                format!("background-color: {}; color: {};", theme.colors.secondary_container, theme.colors.on_secondary_container)
                            } else {
                                format!("background-color: {}; color: {};", theme.colors.surface_container, theme.colors.on_surface_variant)
                            };
                            let on_click = {
                                let active_filter = active_filter.clone();
                                let filter_key = filter_key.clone();
                                Callback::from(move |_: MouseEvent| {
                                    active_filter.set(filter_key.clone());
                                })
                            };
                            html! {
                                <Box
                                    key={filter_key}
                                    display="flex"
                                    align_items="center"
                                    padding="0 16px"
                                    height="32px"
                                    border_radius="8px"
                                    cursor="pointer"
                                    font_size="14px"
                                    font_weight="500"
                                    white_space="nowrap"
                                    transition="all 200ms"
                                    style={chip_bg}
                                    onclick={on_click}
                                >
                                    { Html::from(label.to_string()) }
                                </Box>
                            }
                        })}
                    </Box>

                    // ── Product Grid ──
                    <Box
                        display="grid"
                        grid_template_columns="repeat(3, 1fr)"
                        gap="16px"
                        padding="24px"
                        bg="surface-container-low"
                    >
                        { for products.iter().map(|product| {
                            html! {
                                <Card variant={CardVariant::Elevated}>
                                    <Box class={product_card_body.get_class_name().to_string()}>
                                        // Icon circle
                                        <Box class={icon_circle.get_class_name().to_string()} style={format!("background-color: {};", product.color)}>
                                            <span class="material-symbols-outlined" style={format!("font-size: 36px; color: {};", theme.colors.on_surface)}>
                                                { &product.icon }
                                            </span>
                                        </Box>

                                        // Product name
                                        <Box tag={BoxTag::Span} font_size="14px" font_weight="500" color="on-surface" margin="0 0 8px 0" text_align="center">
                                            { Html::from(product.name.clone()) }
                                        </Box>

                                        // Star rating
                                        <Box class={star_row.get_class_name().to_string()}>
                                            { for render_stars(product.rating).into_iter() }
                                            <Box tag={BoxTag::Span} class={rating_text.get_class_name().to_string()} color="on-surface-variant">
                                                { Html::from(format!("{:.1}", product.rating)) }
                                            </Box>
                                        </Box>

                                        // Price
                                        <Box tag={BoxTag::Span} class={price_text.get_class_name().to_string()} color="on-surface">
                                            { Html::from(product.price.clone()) }
                                        </Box>

                                        // Add to Cart button
                                        <Button
                                            label="Add to Cart"
                                            variant={ButtonVariant::FilledTonal}
                                            icon="shopping_cart"
                                        />
                                    </Box>
                                </Card>
                            }
                        })}
                    </Box>

                    // ── Pagination ──
                    <Box
                        display="flex"
                        align_items="center"
                        justify_content="space-between"
                        padding="16px 24px"
                        bg="surface"
                        border_top={format!("1px solid {}", theme.colors.outline_variant)}
                    >
                        <Box tag={BoxTag::Span} class={pagination_text.get_class_name().to_string()} color="on-surface-variant">
                            {"Showing 6 of 120 products"}
                        </Box>
                        <Box class={pagination_pages.get_class_name().to_string()}>
                            <Button label="Prev" variant={ButtonVariant::Outlined} />
                            <Box class={page_btn.get_class_name().to_string()} bg="primary" color="on-primary">
                                {"1"}
                            </Box>
                            <Box class={page_btn.get_class_name().to_string()} color="on-surface-variant">
                                {"2"}
                            </Box>
                            <Box class={page_btn.get_class_name().to_string()} color="on-surface-variant">
                                {"3"}
                            </Box>
                            <Box tag={BoxTag::Span} class={pagination_text.get_class_name().to_string()} color="on-surface-variant">
                                {"..."}
                            </Box>
                            <Box class={page_btn.get_class_name().to_string()} color="on-surface-variant">
                                {"10"}
                            </Box>
                            <Button label="Next" variant={ButtonVariant::Outlined} />
                        </Box>
                    </Box>
                </Box>
            </Demo>
        </>
    }
}
