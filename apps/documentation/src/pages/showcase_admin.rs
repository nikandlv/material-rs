use yew::prelude::*;
use stylist::yew::use_style;
use material_rs::components::*;
use material_rs::components::box_layout::Box;
use material_rs::theme::Theme;
use super::{Section, Demo, CodeBlock};

#[function_component]
pub fn ShowcaseAdminPage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let active_key = use_state(|| "dashboard".to_string());
    let drawer_open = use_state(|| true);

    let drawer_items = vec![
        DrawerItem {
            key: "dashboard".into(),
            icon: "dashboard".into(),
            label: "Dashboard".into(),
            section: "".into(),
            active: *active_key == "dashboard",
        },
        DrawerItem {
            key: "users".into(),
            icon: "group".into(),
            label: "Users".into(),
            section: "".into(),
            active: *active_key == "users",
        },
        DrawerItem {
            key: "products".into(),
            icon: "inventory_2".into(),
            label: "Products".into(),
            section: "".into(),
            active: *active_key == "products",
        },
        DrawerItem {
            key: "orders".into(),
            icon: "shopping_bag".into(),
            label: "Orders".into(),
            section: "".into(),
            active: *active_key == "orders",
        },
        DrawerItem {
            key: "analytics".into(),
            icon: "analytics".into(),
            label: "Analytics".into(),
            section: "".into(),
            active: *active_key == "analytics",
        },
        DrawerItem {
            key: "settings".into(),
            icon: "settings".into(),
            label: "Settings".into(),
            section: "".into(),
            active: *active_key == "settings",
        },
    ];

    let on_select = {
        let active_key = active_key.clone();
        Callback::from(move |key: String| active_key.set(key))
    };

    let on_drawer_toggle = {
        let drawer_open = drawer_open.clone();
        Callback::from(move |_: MouseEvent| drawer_open.set(!*drawer_open))
    };

    let green = "#2e7d32";
    let red = "#c62828";

    // ── All use_style! hooks at top level ──
    let desc_style = use_style!(
        r#"
        font-size: 14px;
        line-height: 1.6;
        color: var(--md-sys-color-on-surface-variant);
        margin-bottom: 24px;
        "#
    );

    let outer_style = use_style!(
        r#"
        border-radius: 16px;
        overflow: hidden;
        border: 1px solid ${border};
        background-color: ${bg};
        height: 700px;
        display: flex;
        "#,
        border = theme.colors.outline_variant,
        bg = theme.colors.surface,
    );

    let main_style = use_style!(
        r#"
        flex: 1;
        display: flex;
        flex-direction: column;
        overflow: hidden;
        "#
    );

    let actions_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        gap: 4px;
        "#
    );

    let scrollable_style = use_style!(
        r#"
        flex: 1;
        overflow-y: auto;
        padding: 24px;
        "#
    );

    let card_body_style = use_style!(r#"padding: 24px;"#);
    let welcome_style = use_style!(r#"margin-bottom: 24px;"#);

    let welcome_sub_style = use_style!(
        r#"
        color: var(--md-sys-color-on-surface-variant);
        margin-top: 4px;
        "#
    );

    let grid_4_style = use_style!(
        r#"
        display: grid;
        grid-template-columns: repeat(4, 1fr);
        gap: 16px;
        margin-bottom: 24px;
        "#
    );

    let stat_card_style = use_style!(r#"padding: 16px;"#);

    let stat_row_style = use_style!(
        r#"
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        "#
    );

    let stat_label_style = use_style!(
        r#"
        color: ${color};
        "#,
        color = theme.colors.on_surface_variant,
    );

    let stat_value_style = use_style!(r#"font-weight: 700;"#);

    // Icon circle styles (one per container color)
    let icon_circle_primary = use_style!(
        r#"
        width: 40px;
        height: 40px;
        border-radius: 50%;
        background-color: ${bg};
        display: flex;
        align-items: center;
        justify-content: center;
        "#,
        bg = theme.colors.primary_container,
    );

    let icon_circle_secondary = use_style!(
        r#"
        width: 40px;
        height: 40px;
        border-radius: 50%;
        background-color: ${bg};
        display: flex;
        align-items: center;
        justify-content: center;
        "#,
        bg = theme.colors.secondary_container,
    );

    let icon_circle_tertiary = use_style!(
        r#"
        width: 40px;
        height: 40px;
        border-radius: 50%;
        background-color: ${bg};
        display: flex;
        align-items: center;
        justify-content: center;
        "#,
        bg = theme.colors.tertiary_container,
    );

    let icon_circle_error = use_style!(
        r#"
        width: 40px;
        height: 40px;
        border-radius: 50%;
        background-color: ${bg};
        display: flex;
        align-items: center;
        justify-content: center;
        "#,
        bg = theme.colors.error_container,
    );

    let trend_row_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        gap: 4px;
        margin-top: 8px;
        "#
    );

    let trend_green = use_style!(
        r#"
        color: ${color};
        "#,
        color = green,
    );

    let trend_red = use_style!(
        r#"
        color: ${color};
        "#,
        color = red,
    );

    let from_last_style = use_style!(
        r#"
        color: ${color};
        "#,
        color = theme.colors.on_surface_variant,
    );

    let grid_2_style = use_style!(
        r#"
        display: grid;
        grid-template-columns: 1.4fr 1fr;
        gap: 24px;
        margin-bottom: 24px;
        "#
    );

    let table_card_style = use_style!(r#"padding: 20px;"#);

    let table_header_style = use_style!(
        r#"
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 16px;
        "#
    );

    let activity_title_style = use_style!(r#"margin-bottom: 16px;"#);

    let activity_list_style = use_style!(
        r#"
        display: flex;
        flex-direction: column;
        "#
    );

    // Activity icon circle styles
    let act_icon_primary = use_style!(
        r#"
        width: 36px;
        height: 36px;
        border-radius: 50%;
        background-color: ${bg};
        display: flex;
        align-items: center;
        justify-content: center;
        "#,
        bg = theme.colors.primary_container,
    );

    let act_icon_secondary = use_style!(
        r#"
        width: 36px;
        height: 36px;
        border-radius: 50%;
        background-color: ${bg};
        display: flex;
        align-items: center;
        justify-content: center;
        "#,
        bg = theme.colors.secondary_container,
    );

    let act_icon_tertiary = use_style!(
        r#"
        width: 36px;
        height: 36px;
        border-radius: 50%;
        background-color: ${bg};
        display: flex;
        align-items: center;
        justify-content: center;
        "#,
        bg = theme.colors.tertiary_container,
    );

    let act_icon_error = use_style!(
        r#"
        width: 36px;
        height: 36px;
        border-radius: 50%;
        background-color: ${bg};
        display: flex;
        align-items: center;
        justify-content: center;
        "#,
        bg = theme.colors.error_container,
    );

    let time_label_style = use_style!(
        r#"
        color: ${color};
        white-space: nowrap;
        "#,
        color = theme.colors.on_surface_variant,
    );

    let metrics_title_style = use_style!(r#"margin-bottom: 20px;"#);

    let grid_3_style = use_style!(
        r#"
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 24px;
        "#
    );

    let metric_header_style = use_style!(
        r#"
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 8px;
        "#
    );

    let metric_label_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        gap: 8px;
        "#
    );

    let metric_pct_primary = use_style!(
        r#"
        color: ${color};
        "#,
        color = theme.colors.primary,
    );

    let metric_pct_secondary = use_style!(
        r#"
        color: ${color};
        "#,
        color = theme.colors.secondary,
    );

    let metric_pct_tertiary = use_style!(
        r#"
        color: ${color};
        "#,
        color = theme.colors.tertiary,
    );

    html! {
        <>
            <Section title="Admin Dashboard">
                <p class={desc_style.get_class_name().to_string()}>
                    {"A full-page admin dashboard with a persistent navigation drawer, stat cards, \
                      data table, activity feed, and progress indicators."}
                </p>
            </Section>

            <Demo title="Full Dashboard Layout">
                <Box class={outer_style.get_class_name().to_string()}>
                    // ── Persistent Navigation Drawer ──
                    <NavigationDrawer
                        variant={DrawerVariant::Persistent}
                        open={*drawer_open}
                        headline="Material RS Admin"
                        items={drawer_items}
                        on_select={on_select}
                    />

                    // ── Main Content ──
                    <Box class={main_style.get_class_name().to_string()}>
                        // ── Top App Bar ──
                        <TopAppBar
                            title="Dashboard"
                            variant={TopAppBarVariant::Small}
                            nav_icon="menu"
                            position={TopAppBarPosition::Static}
                            on_nav_click={on_drawer_toggle}
                            actions={
                                html! {
                                    <Box class={actions_style.get_class_name().to_string()}>
                                        <IconButton icon="search" variant={IconButtonVariant::Standard} label="Search" />
                                        <Badge count={3}>
                                            <IconButton icon="notifications" variant={IconButtonVariant::Standard} label="Notifications" />
                                        </Badge>
                                        <Avatar icon="person" size="32px" shape={AvatarShape::Circle} />
                                    </Box>
                                }
                            }
                        />

                        // ── Scrollable Content ──
                        <Box class={scrollable_style.get_class_name().to_string()}>
                            <Card variant={CardVariant::Elevated}>
                                <Box class={card_body_style.get_class_name().to_string()}>
                                    // ── Welcome Header ──
                                    <Box class={welcome_style.get_class_name().to_string()}>
                                        <Typography variant={TypographyVariant::HeadlineSmall} tag="h2">
                                            {"Welcome back, Admin"}
                                        </Typography>
                                        <Typography variant={TypographyVariant::BodyMedium} tag="p" class={welcome_sub_style.get_class_name().to_string()}>
                                            {"Here's what's happening with your platform today."}
                                        </Typography>
                                    </Box>

                                    // ── Stat Cards Grid ──
                                    <Box class={grid_4_style.get_class_name().to_string()}>
                                        // Users Card
                                        <Card variant={CardVariant::Elevated}>
                                            <Box class={stat_card_style.get_class_name().to_string()}>
                                                <Box class={stat_row_style.get_class_name().to_string()}>
                                                    <Box>
                                                        <Typography variant={TypographyVariant::LabelMedium} tag="p" class={stat_label_style.get_class_name().to_string()}>
                                                            {"Total Users"}
                                                        </Typography>
                                                        <Typography variant={TypographyVariant::HeadlineMedium} tag="h3" class={stat_value_style.get_class_name().to_string()}>
                                                            {"12,345"}
                                                        </Typography>
                                                    </Box>
                                                    <Box class={icon_circle_primary.get_class_name().to_string()}>
                                                        <Icon name="group" size="24px" color={theme.colors.on_primary_container.clone()} />
                                                    </Box>
                                                </Box>
                                                <Box class={trend_row_style.get_class_name().to_string()}>
                                                    <Icon name="trending_up" size="16px" color={green.to_string()} />
                                                    <Typography variant={TypographyVariant::LabelSmall} tag="span" class={trend_green.get_class_name().to_string()}>
                                                        {"+12.5%"}
                                                    </Typography>
                                                    <Typography variant={TypographyVariant::LabelSmall} tag="span" class={from_last_style.get_class_name().to_string()}>
                                                        {"from last month"}
                                                    </Typography>
                                                </Box>
                                            </Box>
                                        </Card>

                                        // Revenue Card
                                        <Card variant={CardVariant::Elevated}>
                                            <Box class={stat_card_style.get_class_name().to_string()}>
                                                <Box class={stat_row_style.get_class_name().to_string()}>
                                                    <Box>
                                                        <Typography variant={TypographyVariant::LabelMedium} tag="p" class={stat_label_style.get_class_name().to_string()}>
                                                            {"Revenue"}
                                                        </Typography>
                                                        <Typography variant={TypographyVariant::HeadlineMedium} tag="h3" class={stat_value_style.get_class_name().to_string()}>
                                                            {"$45,231"}
                                                        </Typography>
                                                    </Box>
                                                    <Box class={icon_circle_secondary.get_class_name().to_string()}>
                                                        <Icon name="attach_money" size="24px" color={theme.colors.on_secondary_container.clone()} />
                                                    </Box>
                                                </Box>
                                                <Box class={trend_row_style.get_class_name().to_string()}>
                                                    <Icon name="trending_up" size="16px" color={green.to_string()} />
                                                    <Typography variant={TypographyVariant::LabelSmall} tag="span" class={trend_green.get_class_name().to_string()}>
                                                        {"+8.2%"}
                                                    </Typography>
                                                    <Typography variant={TypographyVariant::LabelSmall} tag="span" class={from_last_style.get_class_name().to_string()}>
                                                        {"from last month"}
                                                    </Typography>
                                                </Box>
                                            </Box>
                                        </Card>

                                        // Orders Card
                                        <Card variant={CardVariant::Elevated}>
                                            <Box class={stat_card_style.get_class_name().to_string()}>
                                                <Box class={stat_row_style.get_class_name().to_string()}>
                                                    <Box>
                                                        <Typography variant={TypographyVariant::LabelMedium} tag="p" class={stat_label_style.get_class_name().to_string()}>
                                                            {"Orders"}
                                                        </Typography>
                                                        <Typography variant={TypographyVariant::HeadlineMedium} tag="h3" class={stat_value_style.get_class_name().to_string()}>
                                                            {"1,234"}
                                                        </Typography>
                                                    </Box>
                                                    <Box class={icon_circle_tertiary.get_class_name().to_string()}>
                                                        <Icon name="shopping_bag" size="24px" color={theme.colors.on_tertiary_container.clone()} />
                                                    </Box>
                                                </Box>
                                                <Box class={trend_row_style.get_class_name().to_string()}>
                                                    <Icon name="trending_up" size="16px" color={green.to_string()} />
                                                    <Typography variant={TypographyVariant::LabelSmall} tag="span" class={trend_green.get_class_name().to_string()}>
                                                        {"+5.1%"}
                                                    </Typography>
                                                    <Typography variant={TypographyVariant::LabelSmall} tag="span" class={from_last_style.get_class_name().to_string()}>
                                                        {"from last month"}
                                                    </Typography>
                                                </Box>
                                            </Box>
                                        </Card>

                                        // Conversion Card
                                        <Card variant={CardVariant::Elevated}>
                                            <Box class={stat_card_style.get_class_name().to_string()}>
                                                <Box class={stat_row_style.get_class_name().to_string()}>
                                                    <Box>
                                                        <Typography variant={TypographyVariant::LabelMedium} tag="p" class={stat_label_style.get_class_name().to_string()}>
                                                            {"Conversion"}
                                                        </Typography>
                                                        <Typography variant={TypographyVariant::HeadlineMedium} tag="h3" class={stat_value_style.get_class_name().to_string()}>
                                                            {"3.24%"}
                                                        </Typography>
                                                    </Box>
                                                    <Box class={icon_circle_error.get_class_name().to_string()}>
                                                        <Icon name="analytics" size="24px" color={theme.colors.on_error_container.clone()} />
                                                    </Box>
                                                </Box>
                                                <Box class={trend_row_style.get_class_name().to_string()}>
                                                    <Icon name="trending_down" size="16px" color={red.to_string()} />
                                                    <Typography variant={TypographyVariant::LabelSmall} tag="span" class={trend_red.get_class_name().to_string()}>
                                                        {"-0.4%"}
                                                    </Typography>
                                                    <Typography variant={TypographyVariant::LabelSmall} tag="span" class={from_last_style.get_class_name().to_string()}>
                                                        {"from last month"}
                                                    </Typography>
                                                </Box>
                                            </Box>
                                        </Card>
                                    </Box>

                                    // ── Two Column Layout ──
                                    <Box class={grid_2_style.get_class_name().to_string()}>
                                        // ── Recent Orders Table ──
                                        <Card variant={CardVariant::Elevated}>
                                            <Box class={table_card_style.get_class_name().to_string()}>
                                                <Box class={table_header_style.get_class_name().to_string()}>
                                                    <Typography variant={TypographyVariant::TitleMedium} tag="h3">
                                                        {"Recent Orders"}
                                                    </Typography>
                                                    <Button label="View All" variant={ButtonVariant::Text} />
                                                </Box>
                                                <Table
                                                    headers={vec![
                                                        "Order".to_string(),
                                                        "Customer".to_string(),
                                                        "Status".to_string(),
                                                        "Amount".to_string(),
                                                    ]}
                                                    rows={vec![
                                                        TableRow {
                                                            id: "ord-001".into(),
                                                            cells: vec![
                                                                "#ORD-7352".into(),
                                                                "Alex Johnson".into(),
                                                                "Delivered".into(),
                                                                "$1,250.00".into(),
                                                            ],
                                                        },
                                                        TableRow {
                                                            id: "ord-002".into(),
                                                            cells: vec![
                                                                "#ORD-7351".into(),
                                                                "Sarah Chen".into(),
                                                                "Processing".into(),
                                                                "$890.50".into(),
                                                            ],
                                                        },
                                                        TableRow {
                                                            id: "ord-003".into(),
                                                            cells: vec![
                                                                "#ORD-7350".into(),
                                                                "Mike Rivera".into(),
                                                                "Delivered".into(),
                                                                "$2,100.00".into(),
                                                            ],
                                                        },
                                                        TableRow {
                                                            id: "ord-004".into(),
                                                            cells: vec![
                                                                "#ORD-7349".into(),
                                                                "Emma Wilson".into(),
                                                                "Cancelled".into(),
                                                                "$450.00".into(),
                                                            ],
                                                        },
                                                        TableRow {
                                                            id: "ord-005".into(),
                                                            cells: vec![
                                                                "#ORD-7348".into(),
                                                                "James Park".into(),
                                                                "Processing".into(),
                                                                "$3,400.00".into(),
                                                            ],
                                                        },
                                                        TableRow {
                                                            id: "ord-007".into(),
                                                            cells: vec![
                                                                "#ORD-7347".into(),
                                                                "Lisa Brown".into(),
                                                                "Delivered".into(),
                                                                "$675.25".into(),
                                                            ],
                                                        },
                                                    ]}
                                                />
                                            </Box>
                                        </Card>

                                        // ── Activity Feed ──
                                        <Card variant={CardVariant::Elevated}>
                                            <Box class={table_card_style.get_class_name().to_string()}>
                                                <Typography variant={TypographyVariant::TitleMedium} tag="h3" class={activity_title_style.get_class_name().to_string()}>
                                                    {"Recent Activity"}
                                                </Typography>
                                                <Box class={activity_list_style.get_class_name().to_string()}>
                                                    <ListItem
                                                        headline="New user registered"
                                                        supporting_text="Alex Johnson created an account"
                                                        leading={html! {
                                                            <Box class={act_icon_primary.get_class_name().to_string()}>
                                                                <Icon name="person_add" size="20px" color={theme.colors.on_primary_container.clone()} />
                                                            </Box>
                                                        }}
                                                        trailing={html! {
                                                            <Typography variant={TypographyVariant::LabelSmall} tag="span" class={time_label_style.get_class_name().to_string()}>
                                                                {"2 min ago"}
                                                            </Typography>
                                                        }}
                                                        interactive={false}
                                                    />
                                                    <ListItem
                                                        headline="Order #ORD-7352 delivered"
                                                        supporting_text="Alex Johnson received their package"
                                                        leading={html! {
                                                            <Box class={act_icon_secondary.get_class_name().to_string()}>
                                                                <Icon name="local_shipping" size="20px" color={theme.colors.on_secondary_container.clone()} />
                                                            </Box>
                                                        }}
                                                        trailing={html! {
                                                            <Typography variant={TypographyVariant::LabelSmall} tag="span" class={time_label_style.get_class_name().to_string()}>
                                                                {"18 min ago"}
                                                            </Typography>
                                                        }}
                                                        interactive={false}
                                                    />
                                                    <ListItem
                                                        headline="Payment received"
                                                        supporting_text="$890.50 from Sarah Chen"
                                                        leading={html! {
                                                            <Box class={act_icon_tertiary.get_class_name().to_string()}>
                                                                <Icon name="payments" size="20px" color={theme.colors.on_tertiary_container.clone()} />
                                                            </Box>
                                                        }}
                                                        trailing={html! {
                                                            <Typography variant={TypographyVariant::LabelSmall} tag="span" class={time_label_style.get_class_name().to_string()}>
                                                                {"1 hour ago"}
                                                            </Typography>
                                                        }}
                                                        interactive={false}
                                                    />
                                                    <ListItem
                                                        headline="New review posted"
                                                        supporting_text="Mike Rivera left a 5-star review"
                                                        leading={html! {
                                                            <Box class={act_icon_primary.get_class_name().to_string()}>
                                                                <Icon name="star" size="20px" color={theme.colors.on_primary_container.clone()} />
                                                            </Box>
                                                        }}
                                                        trailing={html! {
                                                            <Typography variant={TypographyVariant::LabelSmall} tag="span" class={time_label_style.get_class_name().to_string()}>
                                                                {"3 hours ago"}
                                                            </Typography>
                                                        }}
                                                        interactive={false}
                                                    />
                                                    <ListItem
                                                        headline="System alert"
                                                        supporting_text="CPU usage exceeded 90%"
                                                        leading={html! {
                                                            <Box class={act_icon_error.get_class_name().to_string()}>
                                                                <Icon name="warning" size="20px" color={theme.colors.on_error_container.clone()} />
                                                            </Box>
                                                        }}
                                                        trailing={html! {
                                                            <Typography variant={TypographyVariant::LabelSmall} tag="span" class={time_label_style.get_class_name().to_string()}>
                                                                {"5 hours ago"}
                                                            </Typography>
                                                        }}
                                                        interactive={false}
                                                    />
                                                </Box>
                                            </Box>
                                        </Card>
                                    </Box>

                                    // ── Progress Metrics ──
                                    <Card variant={CardVariant::Elevated}>
                                        <Box class={table_card_style.get_class_name().to_string()}>
                                            <Typography variant={TypographyVariant::TitleMedium} tag="h3" class={metrics_title_style.get_class_name().to_string()}>
                                                {"Performance Metrics"}
                                            </Typography>
                                            <Box class={grid_3_style.get_class_name().to_string()}>
                                                // Server Uptime
                                                <Box>
                                                    <Box class={metric_header_style.get_class_name().to_string()}>
                                                        <Box class={metric_label_style.get_class_name().to_string()}>
                                                            <Icon name="dns" size="20px" color={theme.colors.primary.clone()} />
                                                            <Typography variant={TypographyVariant::LabelLarge} tag="span">
                                                                {"Server Uptime"}
                                                            </Typography>
                                                        </Box>
                                                        <Typography variant={TypographyVariant::LabelLarge} tag="span" class={metric_pct_primary.get_class_name().to_string()}>
                                                            {"99.8%"}
                                                        </Typography>
                                                    </Box>
                                                    <ProgressIndicator progress_type={ProgressType::Linear} value={Some(0.998)} />
                                                </Box>

                                                // Storage Usage
                                                <Box>
                                                    <Box class={metric_header_style.get_class_name().to_string()}>
                                                        <Box class={metric_label_style.get_class_name().to_string()}>
                                                            <Icon name="cloud" size="20px" color={theme.colors.secondary.clone()} />
                                                            <Typography variant={TypographyVariant::LabelLarge} tag="span">
                                                                {"Storage"}
                                                            </Typography>
                                                        </Box>
                                                        <Typography variant={TypographyVariant::LabelLarge} tag="span" class={metric_pct_secondary.get_class_name().to_string()}>
                                                            {"68%"}
                                                        </Typography>
                                                    </Box>
                                                    <ProgressIndicator progress_type={ProgressType::Linear} value={Some(0.68)} />
                                                </Box>

                                                // Bandwidth
                                                <Box>
                                                    <Box class={metric_header_style.get_class_name().to_string()}>
                                                        <Box class={metric_label_style.get_class_name().to_string()}>
                                                            <Icon name="cell_tower" size="20px" color={theme.colors.tertiary.clone()} />
                                                            <Typography variant={TypographyVariant::LabelLarge} tag="span">
                                                                {"Bandwidth"}
                                                            </Typography>
                                                        </Box>
                                                        <Typography variant={TypographyVariant::LabelLarge} tag="span" class={metric_pct_tertiary.get_class_name().to_string()}>
                                                            {"42%"}
                                                        </Typography>
                                                    </Box>
                                                    <ProgressIndicator progress_type={ProgressType::Linear} value={Some(0.42)} />
                                                </Box>
                                            </Box>
                                        </Box>
                                    </Card>
                                </Box>
                            </Card>
                        </Box>
                    </Box>
                </Box>
            </Demo>

            <Section title="Code Structure">
                <p class={desc_style.get_class_name().to_string()}>
                    {"This dashboard combines a Persistent NavigationDrawer with TopAppBar, \
                      Card for content containers, Table for data display, ListItem for activity feed, \
                      ProgressIndicator for metrics, Avatar for user display, Badge for notifications, \
                      Icon for all icons, and Typography for text."}
                </p>
                <CodeBlock code={DASHBOARD_CODE.to_string()} language={"rust".to_string()} />
            </Section>
        </>
    }
}

const DASHBOARD_CODE: &str = r##"// Admin Dashboard with Persistent Navigation Drawer

use stylist::yew::use_style;
use material_rs::components::*;
use material_rs::components::box_layout::{Box, BoxTag};

let active_key = use_state(|| "dashboard".to_string());
let drawer_open = use_state(|| true);

// Navigation drawer with active state tracking
<NavigationDrawer
    variant={DrawerVariant::Persistent}
    open={*drawer_open}
    headline="Material RS Admin"
    items={vec![
        DrawerItem { key: "dashboard".into(), icon: "dashboard".into(), label: "Dashboard".into(), section: "".into(), active: *active_key == "dashboard" },
        DrawerItem { key: "users".into(), icon: "group".into(), label: "Users".into(), section: "".into(), active: *active_key == "users" },
        // ... more items
    ]}
    on_select={Callback::from(move |key| active_key.set(key))}
/>

// TopAppBar with avatar and notification badge
<TopAppBar
    title="Dashboard"
    variant={TopAppBarVariant::Small}
    nav_icon="menu"
    on_nav_click={on_drawer_toggle}
    actions={html! {
        <Box display="flex" align_items="center" gap="4px">
            <IconButton icon="search" variant={IconButtonVariant::Standard} />
            <Badge count={3}>
                <IconButton icon="notifications" variant={IconButtonVariant::Standard} />
            </Badge>
            <Avatar icon="person" size="32px" />
        </Box>
    }}
/>

// Stat card with Icon circle and Typography
<Card variant={CardVariant::Elevated}>
    <Typography variant={TypographyVariant::LabelMedium}>{"Total Users"}</Typography>
    <Typography variant={TypographyVariant::HeadlineMedium}>{"12,345"}</Typography>
    <Icon name="trending_up" size="16px" color="#2e7d32" />
</Card>

// Activity feed using ListItem
<ListItem
    headline="New user registered"
    supporting_text="Alex Johnson created an account"
    leading={html! { <Icon name="person_add" size="20px" /> }}
    trailing={html! { <span>{"2 min ago"}</span> }}
/>

// Progress metrics
<ProgressIndicator progress_type={ProgressType::Linear} value={Some(0.998)} />"##;
