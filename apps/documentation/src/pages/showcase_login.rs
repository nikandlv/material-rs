use yew::prelude::*;
use stylist::yew::use_style;
use material_rs::components::*;
use material_rs::components::box_layout::Box;
use material_rs::theme::Theme;
use super::{Section, CodeBlock};

#[function_component]
pub fn ShowcaseLoginPage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let email = use_state(String::new);
    let password = use_state(String::new);
    let remember_me = use_state(|| false);

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |val: String| email.set(val))
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |val: String| password.set(val))
    };

    let on_remember_me = {
        let remember_me = remember_me.clone();
        Callback::from(move |v: bool| remember_me.set(v))
    };

    let on_sign_in = {
        let email = email.clone();
        let password = password.clone();
        let remember_me = *remember_me;
        Callback::from(move |_: MouseEvent| {
            let _ = (*email).clone();
            let _ = (*password).clone();
            let _ = remember_me;
        })
    };

    // Demo styles
    let demo_outer_style = use_style!(
        r#"
        border-radius: 16px;
        overflow: hidden;
        border: 1px solid ${border_color};
        margin-bottom: 32px;
        "#,
        border_color = theme.colors.outline_variant,
    );

    let chrome_bar_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        padding: 10px 16px;
        background-color: ${bg};
        border-bottom: 1px solid ${border_color};
        gap: 8px;
        "#,
        bg = theme.colors.surface_container_high,
        border_color = theme.colors.outline_variant,
    );

    let dots_style = use_style!(r#"display: flex; gap: 6px;"#);

    let url_bar_style = use_style!(
        r#"
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 5px 12px;
        border-radius: 8px;
        background-color: ${bg};
        margin: 0 40px;
        font-size: 12px;
        color: ${color};
        "#,
        bg = theme.colors.surface_container_lowest,
        color = theme.colors.on_surface_variant,
    );

    let login_bg_style = use_style!(
        r#"
        min-height: 520px;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 40px 24px;
        background: linear-gradient(135deg, ${c1} 0%, ${c2} 50%, ${c3} 100%);
        "#,
        c1 = theme.colors.primary_container,
        c2 = theme.colors.tertiary_container,
        c3 = theme.colors.secondary_container,
    );

    let card_style = use_style!(
        r#"
        width: 100%;
        max-width: 380px;
        background-color: ${bg};
        border-radius: 28px;
        padding: 36px 28px;
        box-shadow: 0 8px 32px rgba(0,0,0,0.12);
        "#,
        bg = theme.colors.surface,
    );

    let logo_circle_style = use_style!(
        r#"
        width: 56px;
        height: 56px;
        border-radius: 16px;
        background-color: ${bg};
        display: flex;
        align-items: center;
        justify-content: center;
        "#,
        bg = theme.colors.primary_container,
    );

    let logo_icon_style = use_style!(
        r#"
        font-size: 28px;
        color: ${color};
        "#,
        color = theme.colors.on_primary_container,
    );

    let title_style = use_style!(
        r#"
        text-align: center;
        margin-bottom: 28px;
        "#,
    );

    let h1_style = use_style!(
        r#"
        font-family: ${font};
        font-size: 28px;
        font-weight: 400;
        color: ${color};
        margin: 0 0 4px 0;
        line-height: 1.3;
        "#,
        font = theme.font_family,
        color = theme.colors.on_surface,
    );

    let subtitle_style = use_style!(
        r#"
        font-family: ${font};
        font-size: 14px;
        color: ${color};
        margin: 0;
        line-height: 1.5;
        "#,
        font = theme.font_family,
        color = theme.colors.on_surface_variant,
    );

    let forgot_link_style = use_style!(
        r#"
        color: ${color};
        font-size: 14px;
        text-decoration: none;
        font-family: ${font};
        cursor: pointer;
        "#,
        color = theme.colors.primary,
        font = theme.font_family,
    );

    let divider_row_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        gap: 16px;
        margin: 24px 0;
        "#,
    );

    let divider_line_style = use_style!(
        r#"
        flex: 1;
        height: 1px;
        background-color: ${color};
        "#,
        color = theme.colors.outline_variant,
    );

    let divider_text_style = use_style!(
        r#"
        font-size: 12px;
        color: ${color};
        white-space: nowrap;
        font-family: ${font};
        "#,
        color = theme.colors.on_surface_variant,
        font = theme.font_family,
    );

    let signup_style = use_style!(
        r#"
        text-align: center;
        font-size: 14px;
        font-family: ${font};
        color: ${color};
        "#,
        font = theme.font_family,
        color = theme.colors.on_surface_variant,
    );

    let signup_link_style = use_style!(
        r#"
        color: ${color};
        text-decoration: none;
        font-weight: 500;
        cursor: pointer;
        "#,
        color = theme.colors.primary,
    );

    let code = r##"use yew::prelude::*;
use stylist::yew::use_style;
use material_rs::components::*;
use material_rs::components::box_layout::{Box, BoxTag};
use material_rs::theme::Theme;

#[function_component]
pub fn ShowcaseLoginPage() -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let email = use_state(String::new);
    let password = use_state(String::new);
    let remember_me = use_state(|| false);

    let login_bg_style = use_style!(
        r#"
        min-height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
        background: linear-gradient(135deg, ${c1} 0%, ${c2} 50%, ${c3} 100%);
        padding: 24px;
        "#,
        c1 = theme.colors.primary_container,
        c2 = theme.colors.tertiary_container,
        c3 = theme.colors.secondary_container,
    );

    let card_style = use_style!(
        r#"
        width: 100%;
        max-width: 420px;
        background-color: ${bg};
        border-radius: 28px;
        padding: 40px 32px;
        box-shadow: 0 8px 32px rgba(0,0,0,0.12);
        "#,
        bg = theme.colors.surface,
    );

    html! {
        <Box class={login_bg_style.get_class_name().to_string()}>
            <Box class={card_style.get_class_name().to_string()}>
                <Box display="flex" justify_content="center" margin_bottom="24px">
                    <Box class={logo_circle_style.get_class_name().to_string()}>
                        <span class="material-symbols-outlined" style={format!("font-size: 28px; color: {};", theme.colors.on_primary_container)}>{"palette"}</span>
                    </Box>
                </Box>
                // ... rest of login form
            </Box>
        </Box>
    }
}"##.to_string();

    html! {
        <>
            <Section title="Login Page Showcase">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"A full-page login screen built with Material Design 3 components. This showcase demonstrates how text fields, buttons, checkboxes, and dividers come together to form a polished, production-ready authentication flow."}
                </p>
            </Section>

            // ── Full Page Login Demo ──
            <Box class={demo_outer_style.get_class_name().to_string()}>
                // Simulated browser chrome
                <Box class={chrome_bar_style.get_class_name().to_string()}>
                    <Box class={dots_style.get_class_name().to_string()}>
                        <Box width="10px" height="10px" border_radius="50%" bg="#FF5F57" />
                        <Box width="10px" height="10px" border_radius="50%" bg="#FEBC2E" />
                        <Box width="10px" height="10px" border_radius="50%" bg="#28C840" />
                    </Box>
                    <Box class={url_bar_style.get_class_name().to_string()}>
                        {"app.example.com/login"}
                    </Box>
                    <Box width="46px" />
                </Box>

                // Login page content
                <Box class={login_bg_style.get_class_name().to_string()}>
                    <Box class={card_style.get_class_name().to_string()}>
                        // Material Logo
                        <Box display="flex" justify_content="center" margin_bottom="24px">
                            <Box class={logo_circle_style.get_class_name().to_string()}>
                                <span class={yew::classes!("material-symbols-outlined", logo_icon_style.get_class_name().to_string())}>{"palette"}</span>
                            </Box>
                        </Box>

                        // Welcome title
                        <Box class={title_style.get_class_name().to_string()}>
                            <h1 class={h1_style.get_class_name().to_string()}>{"Welcome back"}</h1>
                            <p class={subtitle_style.get_class_name().to_string()}>{"Sign in to your account"}</p>
                        </Box>

                        // Email field
                        <Box margin_bottom="20px">
                            <TextField
                                label="Email"
                                variant={TextFieldVariant::Filled}
                                value={(*email).clone()}
                                onchange={on_email_change.clone()}
                                leading_icon="mail"
                                input_type="email"
                            />
                        </Box>

                        // Password field
                        <Box margin_bottom="16px">
                            <TextField
                                label="Password"
                                variant={TextFieldVariant::Filled}
                                value={(*password).clone()}
                                onchange={on_password_change.clone()}
                                leading_icon="lock"
                                trailing_icon="visibility"
                                input_type="password"
                            />
                        </Box>

                        // Remember me + Forgot password
                        <Box display="flex" justify_content="space-between" align_items="center" margin_bottom="24px">
                            <Checkbox
                                label="Remember me"
                                checked={*remember_me}
                                onchange={on_remember_me.clone()}
                            />
                            <a class={forgot_link_style.get_class_name().to_string()}>{"Forgot password?"}</a>
                        </Box>

                        // Sign In button
                        <Button
                            label="Sign In"
                            variant={ButtonVariant::Filled}
                            full_width={true}
                            onclick={on_sign_in.clone()}
                        />

                        // Divider
                        <Box class={divider_row_style.get_class_name().to_string()}>
                            <Box class={divider_line_style.get_class_name().to_string()} />
                            <span class={divider_text_style.get_class_name().to_string()}>{"Or sign in with"}</span>
                            <Box class={divider_line_style.get_class_name().to_string()} />
                        </Box>

                        // Social sign-in buttons
                        <Box display="flex" gap="12px" margin_bottom="28px">
                            <Button label="Google" variant={ButtonVariant::Outlined} full_width={true} />
                            <Button label="Apple" variant={ButtonVariant::Outlined} full_width={true} />
                        </Box>

                        // Sign up link
                        <Box class={signup_style.get_class_name().to_string()}>
                            {"Don't have an account? "}
                            <a class={signup_link_style.get_class_name().to_string()}>{"Sign up"}</a>
                        </Box>
                    </Box>
                </Box>
            </Box>

            // ── Code ──
            <CodeBlock code={code} language={"rust".to_string()} />
        </>
    }
}
