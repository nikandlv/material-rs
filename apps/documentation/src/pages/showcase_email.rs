use yew::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
use material_rs::components::*;
use material_rs::components::box_layout::{Box, BoxTag};
use material_rs::theme::Theme;
use stylist::yew::use_style;
use super::{Section, Demo};

#[derive(Clone, PartialEq)]
struct Email {
    id: usize,
    sender: String,
    sender_initials: String,
    subject: String,
    preview: String,
    time: String,
    starred: bool,
    unread: bool,
    important: bool,
}

#[function_component]
pub fn ShowcaseEmailPage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let emails = use_state(|| {
        vec![
            Email { id: 1, sender: "Material Design 3".into(), sender_initials: "MD".into(), subject: "New components released in Material You".into(), preview: "We're excited to announce the latest set of Material Design 3 components including updated chips, cards...".into(), time: "2m".into(), starred: true, unread: true, important: true },
            Email { id: 2, sender: "Quarterly Report".into(), sender_initials: "QR".into(), subject: "Q4 results are in — great quarter!".into(), preview: "The quarterly business review shows strong growth across all divisions with revenue up 23% year over...".into(), time: "1h".into(), starred: false, unread: true, important: false },
            Email { id: 3, sender: "Material RS Team".into(), sender_initials: "MT".into(), subject: "Welcome to Material RS".into(), preview: "Thank you for joining the Material RS community. We're thrilled to have you on board and look forward...".into(), time: "3h".into(), starred: false, unread: false, important: false },
            Email { id: 4, sender: "Manager".into(), sender_initials: "MG".into(), subject: "Meeting Tomorrow at 10am".into(), preview: "Don't forget the 10am sync meeting tomorrow. We'll be discussing the sprint roadmap and upcoming...".into(), time: "5h".into(), starred: false, unread: true, important: false },
            Email { id: 5, sender: "Dev Team".into(), sender_initials: "DT".into(), subject: "Sprint 24 completed successfully".into(), preview: "Sprint 24 has been completed with all planned stories delivered. The release candidate is now in QA...".into(), time: "1d".into(), starred: true, unread: false, important: true },
            Email { id: 6, sender: "Design System".into(), sender_initials: "DS".into(), subject: "Color token updates for 2025".into(), preview: "We've updated the color token palette to align with the latest Material You guidelines. Please review...".into(), time: "2d".into(), starred: false, unread: false, important: false },
        ]
    });

    let active_folder = use_state(|| "inbox".to_string());

    let on_star_toggle = {
        let emails = emails.clone();
        Callback::from(move |id: usize| {
            let mut new: Vec<Email> = (*emails).clone();
            if let Some(e) = new.iter_mut().find(|e| e.id == id) {
                e.starred = !e.starred;
            }
            emails.set(new);
        })
    };

    let on_folder_select = {
        let active_folder = active_folder.clone();
        Callback::from(move |folder: String| {
            active_folder.set(folder);
        })
    };

    let folders = [
        ("inbox", "inbox", "Inbox", "3"),
        ("star", "star", "Starred", "12"),
        ("send", "send", "Sent", ""),
        ("draft", "draft", "Drafts", "2"),
        ("report", "report", "Spam", ""),
        ("delete", "delete", "Trash", ""),
    ];

    // Static CSS classes via use_style!
    let desc_style = use_style!(r#"font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;"#);
    let topbar_actions = use_style!(r#"display: flex; align-items: center; gap: 4px;"#);
    let sidebar_inner = use_style!(r#"padding: 4px 0 12px 0;"#);
    let folder_list = use_style!(r#"padding: 4px 0;"#);
    let folder_item = use_style!(r#"display: flex; align-items: center; padding: 0 12px; height: 48px; cursor: pointer; margin: 2px 0; transition: background-color 200ms; border-radius: 28px;"#);
    let email_item = use_style!(r#"display: flex; align-items: flex-start; padding: 12px 16px; cursor: pointer; border-bottom: 1px solid var(--md-sys-color-outline-variant); transition: background-color 150ms;"#);
    let star_btn = use_style!(r#"cursor: pointer; padding: 0 12px; flex-shrink: 0; display: flex; align-items: center;"#);
    let avatar_wrap = use_style!(r#"flex-shrink: 0; margin-right: 16px;"#);
    let email_content = use_style!(r#"flex: 1; min-width: 0; display: flex; flex-direction: column; gap: 2px;"#);
    let sender_row = use_style!(r#"display: flex; align-items: center; gap: 8px;"#);
    let sender_name = use_style!(r#"font-size: 14px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;"#);
    let email_meta = use_style!(r#"flex-shrink: 0; display: flex; flex-direction: column; align-items: flex-end; gap: 8px; margin-left: 16px;"#);
    let hover_actions = use_style!(r#"display: flex; gap: 2px; opacity: 0; transition: opacity 200ms;"#);
    let footer_text = use_style!(r#"flex: 1; display: flex; align-items: center; justify-content: center; font-size: 14px;"#);
    let grid_auto = use_style!(r#"display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 12px;"#);
    let card_item = use_style!(r#"padding: 16px; border-radius: 12px; border: 1px solid var(--md-sys-color-outline-variant); background-color: var(--md-sys-color-surface-container-low);"#);
    let card_name = use_style!(r#"font-size: 14px; font-weight: 600; display: block; margin-bottom: 4px; color: var(--md-sys-color-on-surface);"#);
    let card_desc = use_style!(r#"font-size: 12px; line-height: 1.4; color: var(--md-sys-color-on-surface-variant);"#);

    html! {
        <>
            <Section title="Email App Showcase">
                <Box class={desc_style.get_class_name().to_string()}>
                    {"A full-page email client demo built entirely with Material RS components. \
                      This showcases how TopAppBar, List, Avatar, Badge, Chip, IconButton, and Divider \
                      come together to create a realistic Material Design 3 application."}
                </Box>

                <Demo title="Interactive Email Client">
                    <Box
                        width="100%"
                        border_radius="16px"
                        overflow="hidden"
                        border={format!("1px solid {}", theme.colors.outline_variant)}
                        display="flex"
                        flex_direction="column"
                        height="680px"
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
                                {"Inbox"}
                            </Box>
                            <Box class={topbar_actions.get_class_name().to_string()}>
                                <IconButton icon="search" variant={IconButtonVariant::Standard} label="Search" />
                                <IconButton icon="edit" variant={IconButtonVariant::Standard} label="Compose" />
                                <Box margin_left="8px">
                                    <Avatar initials="U" size="sm" />
                                </Box>
                            </Box>
                        </Box>

                        // ── Body: Sidebar + Email List ──
                        <Box display="flex" flex="1" overflow="hidden">

                            // ── Left Sidebar ──
                            <Box
                                tag={BoxTag::Nav}
                                width="240px"
                                flex_shrink="0"
                                bg="surface-container-low"
                                overflow_y="auto"
                                padding="8px 12px"
                                border_right={format!("1px solid {}", theme.colors.outline_variant)}
                            >
                                <Box class={sidebar_inner.get_class_name().to_string()}>
                                    <Button variant={ButtonVariant::Filled} label="Compose" icon="edit" />
                                </Box>
                                <Divider />
                                <Box class={folder_list.get_class_name().to_string()}>
                                { for folders.iter().map(|(key, icon, label, count)| {
                                    let is_active = *active_folder == *key;
                                    let folder_key = key.to_string();
                                    let on_folder_select = on_folder_select.clone();
                                    let item_bg = if is_active {
                                        format!("background-color: {};", theme.colors.secondary_container)
                                    } else {
                                        "background-color: transparent;".to_string()
                                    };
                                    let text_color = if is_active {
                                        &theme.colors.on_secondary_container
                                    } else {
                                        &theme.colors.on_surface_variant
                                    };
                                    let fw = if is_active { "600" } else { "500" };
                                    let label_owned = label.to_string();
                                    let count_owned = count.to_string();
                                    let folder_key_for_key = folder_key.clone();
                                    html! {
                                        <Box
                                            key={folder_key_for_key}
                                            class={folder_item.get_class_name().to_string()}
                                            style={item_bg}
                                            onclick={Callback::from(move |_: MouseEvent| {
                                                on_folder_select.emit(folder_key.clone());
                                            })}
                                        >
                                            <span class="material-symbols-outlined"
                                                style={format!("font-size: 22px; margin-right: 16px; color: {};", text_color)}
                                            >{ icon }</span>
                                            <Box tag={BoxTag::Span} flex="1" font_size="14px" font_weight={fw} color={text_color.to_string()}>
                                                { Html::from(label_owned) }
                                            </Box>
                                            if !count.is_empty() {
                                                <Box tag={BoxTag::Span} font_size="12px" font_weight="500" color={text_color.to_string()}>
                                                    { Html::from(count_owned) }
                                                </Box>
                                            }
                                        </Box>
                                    }
                                })}
                                </Box>
                            </Box>

                            // ── Email List ──
                            <Box tag={BoxTag::Main} flex="1" overflow_y="auto" bg="surface" display="flex" flex_direction="column">
                                // ── List Header ──
                                <Box
                                    display="flex"
                                    align_items="center"
                                    padding="12px 16px"
                                    border_bottom="1px solid var(--md-sys-color-outline-variant)"
                                    bg="surface"
                                >
                                    <Checkbox checked={false} />
                                    <Box tag={BoxTag::Span} font_size="14px" font_weight="500" color="on-surface" margin_left="16px">
                                        {"Primary"}
                                    </Box>
                                    <Box tag={BoxTag::Span} font_size="12px" color="on-surface-variant" padding="2px 8px" border_radius="12px" bg="surface-container" margin_left="12px">
                                        {"Promotions"}
                                    </Box>
                                    <Box tag={BoxTag::Span} font_size="12px" color="on-surface-variant" padding="2px 8px" border_radius="12px" bg="surface-container" margin_left="8px">
                                        {"Social"}
                                    </Box>
                                </Box>

                                // ── Email Items ──
                                { for emails.iter().map(|email| {
                                    let email_id = email.id;
                                    let on_star = {
                                        let on_star_toggle = on_star_toggle.clone();
                                        Callback::from(move |_: MouseEvent| {
                                            on_star_toggle.emit(email_id);
                                        })
                                    };
                                    let star_icon = if email.starred { "star" } else { "star_border" };
                                    let star_color = if email.starred { &theme.colors.primary } else { &theme.colors.on_surface_variant };
                                    let sender_weight = if email.unread { "700" } else { "500" };
                                    let subject_weight = if email.unread { "600" } else { "400" };
                                    let sender_owned = email.sender.clone();
                                    let subject_owned = email.subject.clone();
                                    let preview_owned = email.preview.clone();
                                    let time_owned = email.time.clone();
                                    let initials = email.sender_initials.clone();
                                    let important = email.important;

                                    html! {
                                        <Box
                                            key={email_id.to_string()}
                                            class={email_item.get_class_name().to_string()}
                                            onmouseenter={Callback::from(move |_e: MouseEvent| {
                                                #[cfg(target_arch = "wasm32")]
                                                {
                                                    let target = _e.target().unwrap();
                                                    let el: web_sys::HtmlElement = target.unchecked_into();
                                                    let _ = el.style().set_property("background-color", "var(--md-sys-color-surface-container-highest)");
                                                }
                                            })}
                                            onmouseleave={Callback::from(move |_e: MouseEvent| {
                                                #[cfg(target_arch = "wasm32")]
                                                {
                                                    let target = _e.target().unwrap();
                                                    let el: web_sys::HtmlElement = target.unchecked_into();
                                                    let _ = el.style().set_property("background-color", "transparent");
                                                }
                                            })}
                                        >
                                            <Checkbox checked={false} />
                                            <Box class={star_btn.get_class_name().to_string()} onclick={on_star}>
                                                <span class="material-symbols-outlined" style={format!("font-size: 20px; color: {};", star_color)}>{ star_icon }</span>
                                            </Box>
                                            <Box class={avatar_wrap.get_class_name().to_string()}>
                                                <Avatar initials={initials} size="sm" />
                                            </Box>
                                            <Box class={email_content.get_class_name().to_string()}>
                                                <Box class={sender_row.get_class_name().to_string()}>
                                                    <Box tag={BoxTag::Span} class={sender_name.get_class_name().to_string()} font_weight={sender_weight} color="on-surface">
                                                        { Html::from(sender_owned) }
                                                    </Box>
                                                    if important {
                                                        <Chip label="Important" chip_type={ChipType::Filter} icon="priority_high" />
                                                    }
                                                </Box>
                                                <Box tag={BoxTag::Span} font_size="14px" font_weight={subject_weight} color="on-surface" white_space="nowrap" overflow="hidden" style="text-overflow: ellipsis;">
                                                    { Html::from(subject_owned) }
                                                </Box>
                                                <Box tag={BoxTag::Span} font_size="13px" color="on-surface-variant" white_space="nowrap" overflow="hidden" style="text-overflow: ellipsis;">
                                                    { Html::from(preview_owned) }
                                                </Box>
                                            </Box>
                                            <Box class={email_meta.get_class_name().to_string()}>
                                                <Box tag={BoxTag::Span} font_size="12px" color="on-surface-variant">
                                                    { Html::from(time_owned) }
                                                </Box>
                                                <Box class={hover_actions.get_class_name().to_string()}>
                                                    <IconButton icon="archive" variant={IconButtonVariant::Standard} label="Archive" />
                                                    <IconButton icon="delete" variant={IconButtonVariant::Standard} label="Delete" />
                                                </Box>
                                            </Box>
                                        </Box>
                                    }
                                })}

                                <Box class={footer_text.get_class_name().to_string()} color="on-surface-variant">
                                    {"All messages loaded"}
                                </Box>
                            </Box>
                        </Box>
                    </Box>
                </Demo>
            </Section>

            <Section title="Architecture">
                <Box class={desc_style.get_class_name().to_string()}>
                    {"This email client demonstrates several Material Design 3 patterns working together: \
                      a TopAppBar for global navigation, a persistent NavigationDrawer-style sidebar with active \
                      state tracking, an interactive email list with avatars and badges, and theme-aware surfaces \
                      that adapt to light and dark mode."}
                </Box>

                <Demo title="Key Components Used">
                    <Box class={grid_auto.get_class_name().to_string()}>
                        { for [
                            ("TopAppBar", "Global navigation bar with compose and search actions"),
                            ("Avatar", "Sender initials in circular shape with theme colors"),
                            ("Badge", "Unread count indicator on folder items"),
                            ("Chip", "Important tag on priority emails"),
                            ("ListItem", "Folder navigation items with active state"),
                            ("IconButton", "Star toggle, archive, and delete actions"),
                            ("Divider", "Visual separators between sidebar sections"),
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
