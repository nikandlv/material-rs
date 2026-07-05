//! Material Design 3 Split Button
//!
//! A button that combines a primary action button with a dropdown trigger.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::button::{Button, ButtonVariant};
use crate::components::icon::Icon;
use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;

#[derive(Properties, PartialEq)]
pub struct SplitButtonProps {
    /// Label for the main button action.
    pub label: String,

    /// Icon for the main button action.
    #[prop_or_default]
    pub icon: String,

    /// Main button variant.
    #[prop_or(ButtonVariant::Filled)]
    pub variant: ButtonVariant,

    /// Callback when the main action button is clicked.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    /// Callback when the dropdown trigger button is clicked.
    #[prop_or_default]
    pub on_dropdown_click: Callback<MouseEvent>,

    /// Whether the button is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn SplitButton(props: &SplitButtonProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let separator_color = if props.disabled {
        "rgba(0, 0, 0, 0.12)"
    } else {
        match props.variant {
            ButtonVariant::Filled => &theme.colors.on_primary,
            ButtonVariant::FilledTonal => &theme.colors.on_secondary_container,
            ButtonVariant::FilledTertiary => &theme.colors.on_tertiary,
            ButtonVariant::Outlined => &theme.colors.outline,
            _ => &theme.colors.primary,
        }
    };

    let wrapper_style = use_style!(
        r#"
        display: inline-flex;
        align-items: center;
        border-radius: 20px;
        overflow: hidden;
        "#,
    );

    let separator_style = use_style!(
        r#"
        width: 1px;
        height: 24px;
        opacity: 0.38;
        z-index: 2;
        flex-shrink: 0;
        "#,
    );

    let action_style = use_style!(
        r#"
        border-end-end-radius: 0 !important;
        border-start-end-radius: 0 !important;
        margin-inline-end: 0;
        border-inline-end: none;
        "#,
    );

    let trigger_style = use_style!(
        r#"
        border-end-start-radius: 0 !important;
        border-start-start-radius: 0 !important;
        margin-inline-start: 0;
        padding: 0 10px;
        width: 40px;
        min-width: 40px;
        border-inline-start: none;
        "#,
    );

    let arrow_style = use_style!(
        r#"
        font-size: 20px;
        display: flex;
        align-items: center;
        justify-content: center;
        "#,
    );

    let wrapper_dynamic_class = dynamic_style(format!("box-shadow: {};", if matches!(props.variant, ButtonVariant::Elevated) { "0 1px 3px rgba(0,0,0,0.2)" } else { "none" }));
    let separator_dynamic_class = dynamic_style(format!("background-color: {};", separator_color));

    let component_override = theme.component_style("SplitButton").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div
            class={yew::classes![wrapper_style.get_class_name().to_string(), &props.class, wrapper_dynamic_class, &component_override]}
            id={props.id.clone()}
        >
            <Button
                label={props.label.clone()}
                icon={props.icon.clone()}
                variant={props.variant}
                disabled={props.disabled}
                onclick={props.onclick.clone()}
                class={action_style.get_class_name().to_string()}
            />

            <div class={yew::classes![separator_style.get_class_name().to_string(), separator_dynamic_class]} />

            <Button
                variant={props.variant}
                disabled={props.disabled}
                onclick={props.on_dropdown_click.clone()}
                class={trigger_style.get_class_name().to_string()}
            >
                <Icon name="arrow_drop_down" class={arrow_style.get_class_name().to_string()} />
            </Button>
        </div>
    }
}
