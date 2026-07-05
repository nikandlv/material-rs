//! Material Design 3 Accordion (Expansion Panel)
//!
//! A toggleable panel that expands to show more details, following MD3 specs.
//! Content stays in the DOM at all times — animated via max-height + opacity.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::components::typography::{Typography, TypographyVariant};
use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;

#[derive(Properties, PartialEq)]
pub struct AccordionProps {
    pub title: String,

    #[prop_or_default]
    pub icon: String,

    #[prop_or(false)]
    pub expanded: bool,

    #[prop_or_default]
    pub ontoggle: Callback<bool>,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,
}

#[component]
pub fn Accordion(props: &AccordionProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let is_expanded = use_state(|| props.expanded);
    let has_toggled = use_state(|| false);
    let content_ref = use_node_ref();
    // Stores the measured content height in pixels for precise max-height animation
    let content_height = use_state(|| 0u32);

    // Measure content height after mount and on each render so it stays accurate
    {
        let content_ref = content_ref.clone();
        let content_height = content_height.clone();
        use_effect_with((), move |_| {
            if let Some(el) = content_ref.cast::<web_sys::HtmlElement>() {
                let h = el.scroll_height();
                if h > 0 {
                    content_height.set(h as u32);
                }
            }
            || ()
        });
    }

    // Sync with external prop
    {
        let is_expanded = is_expanded.clone();
        let prop_expanded = props.expanded;
        use_effect_with(prop_expanded, move |expanded| {
            is_expanded.set(*expanded);
            || ()
        });
    }

    let on_header_click = {
        let is_expanded = is_expanded.clone();
        let has_toggled = has_toggled.clone();
        let ontoggle = props.ontoggle.clone();
        Callback::from(move |_: MouseEvent| {
            has_toggled.set(true);
            let next = !*is_expanded;
            is_expanded.set(next);
            ontoggle.emit(next);
        })
    };

    let card_bg = theme.colors.surface_container_low.clone();
    let border_color = theme.colors.outline_variant.clone();
    let text_color = theme.colors.on_surface.clone();

    let indicator_rotation = if *is_expanded { "rotate(180deg)" } else { "rotate(0)" };

    // No transition on first paint — only after user interaction
    let transition = if *has_toggled {
        "max-height 300ms cubic-bezier(0.4, 0, 0.2, 1), opacity 250ms cubic-bezier(0.4, 0, 0.2, 1), padding 300ms cubic-bezier(0.4, 0, 0.2, 1)"
    } else {
        "none"
    };

    let chevron_transition = if *has_toggled {
        "transform 300ms cubic-bezier(0.4, 0, 0.2, 1)"
    } else {
        "none"
    };

    let header_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 100%;
        padding: 16px;
        border: none;
        background: none;
        cursor: pointer;
        font-family: inherit;
        font-size: 16px;
        font-weight: 500;
        color: inherit;
        outline: none;
        -webkit-tap-highlight-color: transparent;
        position: relative;
        "#,
    );

    let header_content_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        gap: 12px;
        pointer-events: none;
        "#,
    );

    let icon_style = use_style!(
        r#"
        pointer-events: none;
        "#,
    );

    let content_body_style = use_style!(
        r#"
        font-size: 14px;
        line-height: 20px;
        "#,
    );

    let container_style = use_style!(
        r#"
        border-radius: ${radius};
        border: 1px solid ${border_color};
        background-color: ${bg};
        color: ${color};
        font-family: ${font_family}, sans-serif;
        margin-bottom: 8px;
        overflow: hidden;
        "#,
        radius = theme.shapes.medium.clone(),
        border_color = border_color,
        bg = card_bg,
        color = text_color,
        font_family = theme.font_family,
    );

    // Use measured height + small buffer so content isn't clipped
    let max_h = if *is_expanded && *content_height > 0 {
        format!("{}px", *content_height + 32)
    } else if *is_expanded {
        "500px".to_string()
    } else {
        "0px".to_string()
    };

    let component_override = theme.component_style("Accordion").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div
            class={yew::classes![container_style.get_class_name().to_string(), &props.class, &component_override]}
            id={props.id.clone()}
        >
            <button
                onclick={on_header_click}
                class={header_style.get_class_name().to_string()}
            >
                <div class={header_content_style.get_class_name().to_string()}>
                    if !props.icon.is_empty() {
                        <Icon
                            name={props.icon.clone()}
                            size="24px"
                            color={theme.colors.on_surface_variant.clone()}
                            class={icon_style.get_class_name().to_string()}
                        />
                    }
                    <Typography variant={TypographyVariant::TitleMedium} tag="span" style="font-weight: 500;">{ html! { { &props.title } } }</Typography>
                </div>

                <div class={dynamic_style(format!(
                    "transform: {}; transition: {}; pointer-events: none; display: inline-flex; align-items: center; justify-content: center;",
                    indicator_rotation, chevron_transition
                ))}>
                    <Icon
                        name="expand_more"
                        size="24px"
                        color={theme.colors.on_surface_variant.clone()}
                    />
                </div>
            </button>

            // Content wrapper — always in DOM, animated via max-height
            <div
                ref={content_ref}
                class={dynamic_style(format!(
                    "max-height: {}; opacity: {}; padding: {}; overflow: hidden; \
                     transition: {};",
                    max_h,
                    if *is_expanded { "1" } else { "0" },
                    if *is_expanded { "0 16px 16px 16px" } else { "0 16px" },
                    transition,
                ))}
            >
                <div class={yew::classes![content_body_style.get_class_name().to_string(), dynamic_style(format!("color: {};", theme.colors.on_surface_variant))]}>
                    { props.children.clone() }
                </div>
            </div>
        </div>
    }
}
