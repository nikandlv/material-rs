//! Material Design 3 Stepper Component
//!
//! A progress tracker representing sequence steps, showing labels, active step indicator, and checked items.
//! Features micro-animations for active steps and filling connectors.
//! Responsive: labels hide on small screens, circles shrink, connectors reduce.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::components::typography::{Typography, TypographyVariant};
use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;

#[derive(Clone, PartialEq, Properties)]
pub struct Step {
    pub label: String,
    #[prop_or_default]
    pub optional_text: String,
}

#[derive(Properties, PartialEq)]
pub struct StepperProps {
    /// Vector of steps.
    pub steps: Vec<Step>,

    /// Index of the current active step (0-indexed).
    #[prop_or(0)]
    pub active_step: usize,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn Stepper(props: &StepperProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let stepper_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        flex-wrap: nowrap;
        box-sizing: border-box;
        width: 100%;
        overflow-x: hidden;
        max-width: 100dvw;
        padding: 16px;
        font-family: ${font_family}, sans-serif;

        &::-webkit-scrollbar { height: 0; }
        "#,
        font_family = theme.font_family,
    );

    let step_item_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        gap: 12px;
        flex-shrink: 0;
        "#,
    );

    let circle_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        justify-content: center;
        width: 32px;
        height: 32px;
        border-radius: 50%;
        flex-shrink: 0;
        font-size: 14px;
        font-weight: 600;
        transition: transform 250ms cubic-bezier(0.34, 1.56, 0.64, 1),
                    background-color 250ms,
                    border-color 250ms,
                    color 200ms;

        @media (max-width: 480px) {
            width: 28px;
            height: 28px;
            font-size: 12px;
        }
        "#,
    );

    let label_style = use_style!(
        r#"
        display: flex;
        flex-direction: column;

        @media (max-width: 480px) {
            display: none;
        }
        "#,
    );

    let connector_style = use_style!(
        r#"
        flex: 1;
        height: 2px;
        min-width: 16px;
        transition: background-color 350ms cubic-bezier(0.2, 0, 0, 1);

        @media (max-width: 480px) {
            min-width: 8px;
        }
        "#,
    );

    let optional_text_style = use_style!(
        r#"
        font-size: 11px;
        "#,
    );

    let component_override = theme.component_style("Stepper").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    let items_html = props.steps.iter().enumerate().map(|(idx, step)| {
        let is_active = idx == props.active_step;
        let is_completed = idx < props.active_step;
        let is_first = idx == 0;

        let (circle_bg, circle_color, border_style, text_weight) = if is_completed {
            (
                theme.colors.primary.clone(),
                theme.colors.on_primary.clone(),
                "none".to_string(),
                "600".to_string()
            )
        } else if is_active {
            (
                theme.colors.primary_container.clone(),
                theme.colors.on_primary_container.clone(),
                format!("2px solid {}", theme.colors.primary),
                "600".to_string()
            )
        } else {
            (
                "transparent".to_string(),
                theme.colors.on_surface_variant.clone(),
                format!("1px solid {}", theme.colors.outline),
                "500".to_string()
            )
        };

        let transform = if is_active { "scale(1.15)" } else { "scale(1)" };

        let circle_class = dynamic_style(format!(
            "background-color: {}; border: {}; transform: {};",
            circle_bg, border_style, transform
        ));
        let span_color_class = dynamic_style(format!("color: {};", circle_color));
        let typo_class = dynamic_style(format!("font-weight: {}; color: {}; transition: color 250ms cubic-bezier(0.2, 0, 0, 1);", text_weight,
            if is_active { &theme.colors.primary } else { &theme.colors.on_surface }));
        let opt_color_class = dynamic_style(format!("color: {};", theme.colors.on_surface_variant));
        let connector_bg_class = dynamic_style(format!(
            "background-color: {};",
            if is_completed { &theme.colors.primary } else { &theme.colors.outline_variant }
        ));
        let step_margin_class = dynamic_style(if is_first { "margin-inline-start: 0;".to_string() } else { "margin-inline-start: 16px;".to_string() });

        html! {
            <>
                <div class={yew::classes![step_item_style.get_class_name().to_string(), step_margin_class]}>
                    <div class={yew::classes![circle_style.get_class_name().to_string(), circle_class]}>
                        if is_completed {
                            <Icon name="check" size="18px" color={circle_color.to_string()} />
                        } else {
                            <span class={span_color_class}>{ idx + 1 }</span>
                        }
                    </div>

                    <div class={label_style.get_class_name().to_string()}>
                        <Typography variant={TypographyVariant::BodyMedium}
                              tag="span"
                              class={typo_class}>
                            { html! { { &step.label } } }
                        </Typography>
                        if !step.optional_text.is_empty() {
                            <span class={yew::classes![optional_text_style.get_class_name().to_string(), opt_color_class]}>
                                { html! { { &step.optional_text } } }
                            </span>
                        }
                    </div>
                </div>

                if idx < props.steps.len() - 1 {
                    <div class={yew::classes![connector_style.get_class_name().to_string(), connector_bg_class]} />
                }
            </>
        }
    });

    html! {
        <div
            class={yew::classes![stepper_style.get_class_name().to_string(), &props.class, &component_override]}
            id={props.id.clone()}
        >
            { for items_html }
        </div>
    }
}
