//! Material Design 3 Expressive Alert Box
//!
//! A prominent, dismissible alert banner following MD3 expressive design language.
//! Uses filled color containers, prominent icons, and bold typography.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::button::{Button, ButtonVariant};
use crate::components::icon::Icon;
use crate::components::icon_button::{IconButton, IconButtonVariant};
use crate::components::typography::{Typography, TypographyVariant};
use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

/// Alert severity level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertSeverity {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Properties, PartialEq)]
pub struct AlertBoxProps {
    pub message: String,

    #[prop_or_default]
    pub severity: AlertSeverity,

    #[prop_or_default]
    pub title: String,

    #[prop_or(true)]
    pub open: bool,

    #[prop_or(true)]
    pub dismissible: bool,

    #[prop_or_default]
    pub on_dismiss: Callback<()>,

    #[prop_or_default]
    pub action_label: String,

    #[prop_or_default]
    pub on_action: Callback<()>,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub class: String,

    /// Whether the alert box takes full width of its parent.
    #[prop_or(true)]
    pub full_width: bool,
}

#[component]
pub fn AlertBox(props: &AlertBoxProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    if !props.open {
        return html! {};
    }

    let (icon_name, container_bg, icon_color, _action_bg, _action_fg) = match props.severity {
        AlertSeverity::Info => (
            "info",
            with_alpha(&theme.colors.primary_container, 0.6).unwrap_or_default(),
            theme.colors.on_primary_container.clone(),
            with_alpha(&theme.colors.primary, 0.12).unwrap_or_default(),
            theme.colors.primary.clone(),
        ),
        AlertSeverity::Success => (
            "check_circle",
            with_alpha(&theme.colors.tertiary_container, 0.6).unwrap_or_default(),
            theme.colors.on_tertiary_container.clone(),
            with_alpha(&theme.colors.tertiary, 0.12).unwrap_or_default(),
            theme.colors.tertiary.clone(),
        ),
        AlertSeverity::Warning => (
            "warning",
            with_alpha("#E8A317", 0.15).unwrap_or_default(),
            "#E8A317".into(),
            with_alpha("#E8A317", 0.12).unwrap_or_default(),
            "#E8A317".into(),
        ),
        AlertSeverity::Error => (
            "error",
            with_alpha(&theme.colors.error_container, 0.6).unwrap_or_default(),
            theme.colors.on_error_container.clone(),
            with_alpha(&theme.colors.error, 0.12).unwrap_or_default(),
            theme.colors.error.clone(),
        ),
    };

    let on_dismiss = {
        let cb = props.on_dismiss.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };

    let on_action = {
        let cb = props.on_action.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };

    let alert_style = use_style!(
        r#"
        position: relative;
        display: flex;
        align-items: flex-start;
        gap: 16px;
        padding: 16px 20px;
        width: ${width};
        border-radius: ${radius};
        font-family: ${font_family}, sans-serif;
        box-shadow: 0 2px 6px rgba(0,0,0,0.08);
        "#,
        width = if props.full_width { "100%" } else { "auto" },
        radius = theme.shapes.extra_large,
        font_family = theme.font_family,
    );

    let icon_circle_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        justify-content: center;
        width: 40px;
        height: 40px;
        border-radius: 50%;
        flex-shrink: 0;
        margin-top: 2px;
        "#,
    );

    let content_style = use_style!(
        r#"
        flex: 1;
        min-width: 0;
        display: flex;
        flex-direction: column;
        "#,
    );

    let container_bg_class = dynamic_style(format!("background-color: {};", container_bg));
    let icon_circle_bg_class = dynamic_style(format!("background-color: {};", with_alpha(&icon_color, 0.15).unwrap_or_default()));
    let title_color_class = dynamic_style(format!("color: {}; margin-bottom: 4px;", icon_color));
    let message_color_class = dynamic_style(format!("color: {};", theme.colors.on_surface));

    let component_override = theme.component_style("AlertBox").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div
            class={yew::classes![alert_style.get_class_name().to_string(), &props.class, container_bg_class, &component_override]}
            id={props.id.clone()}
            role="alert"
        >
            <div class={yew::classes![icon_circle_style.get_class_name().to_string(), icon_circle_bg_class]}>
                <Icon name={icon_name.to_string()} size="22px" color={icon_color.clone()} />
            </div>

            <div class={content_style.get_class_name().to_string()}>
                if !props.title.is_empty() {
                    <Typography variant={TypographyVariant::TitleMedium}
                         class={title_color_class}>
                        { html! { { &props.title } } }
                    </Typography>
                }
                <Typography variant={TypographyVariant::BodyMedium}
                     class={message_color_class}>
                    { html! { { &props.message } } }
                </Typography>
            </div>

            if !props.action_label.is_empty() {
                <Button
                    variant={ButtonVariant::Filled}
                    label={props.action_label.clone()}
                    onclick={on_action}
                />
            }

            if props.dismissible {
                <IconButton
                    icon="close"
                    variant={IconButtonVariant::Standard}
                    label="Dismiss"
                    onclick={on_dismiss}
                />
            }
        </div>
    }
}
