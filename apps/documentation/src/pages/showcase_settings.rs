use yew::prelude::*;
use material_rs::components::*;
use material_rs::components::box_layout::{Box, BoxTag};
use material_rs::theme::Theme;
use stylist::yew::use_style;
use super::{Section, Demo};

#[derive(Clone, PartialEq)]
struct SettingsSection {
    key: &'static str,
    icon: &'static str,
    label: &'static str,
}

#[function_component]
pub fn ShowcaseSettingsPage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let active_section = use_state(|| "profile".to_string());

    let full_name = use_state(|| "John Doe".to_string());
    let email = use_state(|| "john@example.com".to_string());
    let bio = use_state(|| "Product designer and developer".to_string());
    let email_notifications = use_state(|| true);
    let push_notifications = use_state(|| false);
    let weekly_digest = use_state(|| true);
    let dark_mode = use_state(|| false);

    let sections = [
        SettingsSection { key: "profile", icon: "person", label: "Profile" },
        SettingsSection { key: "notifications", icon: "notifications", label: "Notifications" },
        SettingsSection { key: "appearance", icon: "palette", label: "Appearance" },
        SettingsSection { key: "privacy", icon: "lock", label: "Privacy" },
        SettingsSection { key: "security", icon: "shield", label: "Security" },
        SettingsSection { key: "about", icon: "info", label: "About" },
    ];

    // Static CSS classes via use_style!
    let desc_style = use_style!(r#"font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;"#);
    let app_body = use_style!(r#"display: flex; flex: 1; overflow: hidden;"#);
    let sidebar_inner = use_style!(r#"padding: 4px 0;"#);
    let card_body = use_style!(r#"padding: 24px;"#);
    let avatar_wrap = use_style!(r#"display: flex; justify-content: center; margin-bottom: 24px;"#);
    let field_wrap = use_style!(r#"margin-bottom: 20px;"#);
    let bio_wrap = use_style!(r#"margin-bottom: 24px;"#);
    let switch_list = use_style!(r#"display: flex; flex-direction: column; gap: 8px;"#);
    let actions_row = use_style!(r#"display: flex; gap: 12px; margin-top: 24px; justify-content: flex-end;"#);

    html! {
        <>
            <Section title="Settings Page Showcase">
                <Box class={desc_style.get_class_name().to_string()}>
                    {"A full-page settings interface built with Material Design 3 components. \
                      This showcases a sidebar navigation pattern with form controls including \
                      TextFields, TextAreas, Switches, and Buttons."}
                </Box>
            </Section>

            <Demo title="Settings Layout">
                <Box
                    width="100%"
                    border_radius="16px"
                    overflow="hidden"
                    border={format!("1px solid {}", theme.colors.outline_variant)}
                    display="flex"
                    height="680px"
                >
                    // ── Top App Bar ──
                    <Box display="flex" flex_direction="column" flex="1" overflow="hidden">
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
                            <Box font_size="22px" font_weight="400" margin_left="16px" color="on-surface">
                                {"Settings"}
                            </Box>
                        </Box>

                        <Box class={app_body.get_class_name().to_string()}>
                            // ── Left Sidebar ──
                            <Box
                                width="280px"
                                flex_shrink="0"
                                bg="surface-container-low"
                                overflow_y="auto"
                                padding="8px 12px"
                                border_right={format!("1px solid {}", theme.colors.outline_variant)}
                            >
                                <Box class={sidebar_inner.get_class_name().to_string()}>
                                { for sections.iter().map(|section| {
                                    let is_active = *active_section == *section.key;
                                    let section_key = section.key.to_string();
                                    let active_section = active_section.clone();
                                    let item_bg = if is_active {
                                        format!("background-color: {}; border-radius: 28px;", theme.colors.secondary_container)
                                    } else {
                                        "background-color: transparent; border-radius: 28px;".to_string()
                                    };
                                    let text_color = if is_active {
                                        &theme.colors.on_secondary_container
                                    } else {
                                        &theme.colors.on_surface_variant
                                    };
                                    let on_click = {
                                        let active_section = active_section.clone();
                                        let section_key = section_key.clone();
                                        Callback::from(move |_: MouseEvent| {
                                            active_section.set(section_key.clone());
                                        })
                                    };
                                    html! {
                                        <Box
                                            key={section_key}
                                            display="flex"
                                            align_items="center"
                                            padding="0 16px"
                                            height="48px"
                                            cursor="pointer"
                                            margin="2px 0"
                                            transition="background-color 200ms"
                                            style={item_bg}
                                            onclick={on_click}
                                        >
                                            <span class="material-symbols-outlined" style={format!("font-size: 22px; margin-right: 16px; color: {};", text_color)}>
                                                { section.icon }
                                            </span>
                                            <Box tag={BoxTag::Span} font_size="14px" font_weight={if is_active { "600" } else { "500" }.to_string()} color={text_color.to_string()}>
                                                { Html::from(section.label.to_string()) }
                                            </Box>
                                        </Box>
                                    }
                                })}
                                </Box>
                            </Box>

                            // ── Right Content Area ──
                            <Box flex="1" overflow_y="auto" bg="surface" padding="24px 32px">
                                <Card variant={CardVariant::Elevated}>
                                    <Box class={card_body.get_class_name().to_string()}>
                                        // ── Profile Section ──
                                        <Box tag={BoxTag::Span} font_size="16px" font_weight="500" color="on-surface" margin="0 0 20px 0">
                                            {"Profile"}
                                        </Box>

                                        // Avatar
                                        <Box class={avatar_wrap.get_class_name().to_string()}>
                                            <Avatar initials="JD" size="xl" />
                                        </Box>

                                        // Full Name
                                        <Box class={field_wrap.get_class_name().to_string()}>
                                            <TextField
                                                label="Full Name"
                                                variant={TextFieldVariant::Outlined}
                                                value={(*full_name).clone()}
                                                onchange={Callback::from({
                                                    let full_name = full_name.clone();
                                                    move |v: String| full_name.set(v)
                                                })}
                                                leading_icon="person"
                                            />
                                        </Box>

                                        // Email
                                        <Box class={field_wrap.get_class_name().to_string()}>
                                            <TextField
                                                label="Email"
                                                variant={TextFieldVariant::Outlined}
                                                value={(*email).clone()}
                                                onchange={Callback::from({
                                                    let email = email.clone();
                                                    move |v: String| email.set(v)
                                                })}
                                                leading_icon="mail"
                                                input_type="email"
                                            />
                                        </Box>

                                        // Bio
                                        <Box class={bio_wrap.get_class_name().to_string()}>
                                            <TextArea
                                                label="Bio"
                                                variant={TextFieldVariant::Outlined}
                                                value={(*bio).clone()}
                                                onchange={Callback::from({
                                                    let bio = bio.clone();
                                                    move |v: String| bio.set(v)
                                                })}
                                                rows={3}
                                            />
                                        </Box>

                                        <Divider />

                                        // ── Notifications Section ──
                                        <Box tag={BoxTag::Span} font_size="16px" font_weight="500" color="on-surface" margin="24px 0 16px 0">
                                            {"Notifications"}
                                        </Box>

                                        <Box class={switch_list.get_class_name().to_string()}>
                                            <Switch
                                                label="Email notifications"
                                                checked={*email_notifications}
                                                onchange={Callback::from({
                                                    let email_notifications = email_notifications.clone();
                                                    move |v: bool| email_notifications.set(v)
                                                })}
                                            />
                                            <Switch
                                                label="Push notifications"
                                                checked={*push_notifications}
                                                onchange={Callback::from({
                                                    let push_notifications = push_notifications.clone();
                                                    move |v: bool| push_notifications.set(v)
                                                })}
                                            />
                                            <Switch
                                                label="Weekly digest"
                                                checked={*weekly_digest}
                                                onchange={Callback::from({
                                                    let weekly_digest = weekly_digest.clone();
                                                    move |v: bool| weekly_digest.set(v)
                                                })}
                                            />
                                        </Box>

                                        <Divider />

                                        // ── Appearance Section ──
                                        <Box tag={BoxTag::Span} font_size="16px" font_weight="500" color="on-surface" margin="24px 0 16px 0">
                                            {"Appearance"}
                                        </Box>

                                        <Switch
                                            label="Dark mode"
                                            checked={*dark_mode}
                                            onchange={Callback::from({
                                                let dark_mode = dark_mode.clone();
                                                move |v: bool| dark_mode.set(v)
                                            })}
                                        />

                                        <Divider />

                                        // ── Action Buttons ──
                                        <Box class={actions_row.get_class_name().to_string()}>
                                            <Button
                                                label="Cancel"
                                                variant={ButtonVariant::Text}
                                            />
                                            <Button
                                                label="Save Changes"
                                                variant={ButtonVariant::Filled}
                                            />
                                        </Box>
                                    </Box>
                                </Card>
                            </Box>
                        </Box>
                    </Box>
                </Box>
            </Demo>
        </>
    }
}
