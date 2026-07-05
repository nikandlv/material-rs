//! Material Design 3 Dialog
//!
//! A modal dialog with title, content, and action buttons. Supports nested dialogs
//! with dynamic z-index stacking and maps shape and elevation design tokens.

use std::sync::atomic::{AtomicU32, Ordering};
use stylist::yew::use_style;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::components::box_layout::Box;
use crate::components::button::{Button, ButtonVariant};
use crate::components::icon::Icon;
use crate::components::typography::{Typography, TypographyVariant};
use crate::theme::Theme;
use crate::theme::elevation::ElevationLevel;
use crate::utils::dynamic_style::dynamic_style;
use crate::utils::portal::Portal;

static Z_INDEX_COUNTER: AtomicU32 = AtomicU32::new(11000);

#[derive(Properties, PartialEq)]
pub struct DialogProps {
    /// Whether the dialog is open.
    #[prop_or(false)]
    pub open: bool,

    /// Dialog title.
    #[prop_or_default]
    pub title: String,

    /// Optional icon above the title.
    #[prop_or_default]
    pub icon: String,

    /// Dismiss callback (scrim click or close button).
    #[prop_or_default]
    pub on_close: Callback<()>,

    /// Confirm button label.
    #[prop_or("OK".into())]
    pub confirm_label: String,

    /// Dismiss button label.
    #[prop_or("Cancel".into())]
    pub dismiss_label: String,

    /// Confirm callback.
    #[prop_or_default]
    pub on_confirm: Callback<()>,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,

    /// Dialog content.
    #[prop_or_default]
    pub children: Children,
}

#[component]
pub fn Dialog(props: &DialogProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let local_z_index = use_state(|| 11000u32);

    let opacity = if props.open { "1" } else { "0" };
    let transform = if props.open { "scale(1)" } else { "scale(0.92)" };
    let pointer_events = if props.open { "auto" } else { "none" };
    let visibility = if props.open { "visible" } else { "hidden" };

    {
        let local_z_index = local_z_index.clone();
        let open = props.open;

        use_effect_with(open, move |open| {
            if *open {
                let next_z = Z_INDEX_COUNTER.fetch_add(10, Ordering::SeqCst);
                local_z_index.set(next_z);
            }
            || ()
        });
    }

    let on_scrim_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |e: MouseEvent| {
            if e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                .is_some_and(|el| el.tag_name() == "DIV")
            {
                on_close.emit(());
            }
        })
    };

    let on_confirm = {
        let on_confirm = props.on_confirm.clone();
        let on_close = props.on_close.clone();
        Callback::from(move |_: MouseEvent| {
            on_confirm.emit(());
            on_close.emit(());
        })
    };

    let on_dismiss = {
        let on_close = props.on_close.clone();
        Callback::from(move |_: MouseEvent| {
            on_close.emit(());
        })
    };

    let overlay_style = use_style!(
        r#"
        position: fixed;
        inset: 0;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 24px;
        transition: visibility 250ms;
        "#,
    );

    let scrim_style = use_style!(
        r#"
        position: absolute;
        inset: 0;
        background-color: rgba(0, 0, 0, 0.4);
        backdrop-filter: blur(12px);
        -webkit-backdrop-filter: blur(12px);
        transition: opacity 250ms cubic-bezier(0.2, 0, 0, 1);
        "#,
    );

    let surface_style = use_style!(
        r#"
        position: relative;
        z-index: 1;
        width: 100%;
        max-width: 560px;
        padding: 24px;
        transition: transform 250ms cubic-bezier(0.34, 1.56, 0.64, 1), opacity 200ms;
        "#,
    );

    let content_style = use_style!(
        r#"
        margin-bottom: 24px;
        font-size: 14px;
        font-weight: 400;
        line-height: 20px;
        "#,
    );

    // ── Dynamic style classes ──
    let overlay_inline_class = dynamic_style(format!(
        "z-index: {}; pointer-events: {}; visibility: {};",
        *local_z_index, pointer_events, visibility
    ));

    let scrim_inline_class = dynamic_style(format!(
        "opacity: {};",
        opacity
    ));

    let surface_inline_class = dynamic_style(format!(
        "border-radius: {}; background-color: {}; color: {}; \
         font-family: {}, sans-serif; box-shadow: {}; \
         transform: {}; opacity: {};",
        theme.shapes.extra_large,
        theme.colors.surface_container_high,
        theme.colors.on_surface,
        theme.font_family,
        theme.elevation(ElevationLevel::Level3).box_shadow,
        transform,
        opacity
    ));

    let title_color_class = dynamic_style(format!(
        "margin: 0 0 16px; text-align: start; color: {};",
        theme.colors.on_surface
    ));

    let content_color_class = dynamic_style(format!(
        "color: {};",
        theme.colors.on_surface_variant
    ));

    let component_override = theme.component_style("Dialog").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <Portal id={format!("dialog-{}", props.id)}>
            <div
                class={yew::classes![overlay_style.get_class_name().to_string(), &props.class, overlay_inline_class, &component_override]}
                id={props.id.clone()}
                role="alertdialog"
                aria-modal="true"
                aria-label={props.title.clone()}
            >
                <div
                    class={yew::classes![scrim_style.get_class_name().to_string(), scrim_inline_class]}
                    onclick={on_scrim_click}
                />

                <div
                    class={yew::classes![surface_style.get_class_name().to_string(), surface_inline_class]}
                >
                    if !props.icon.is_empty() {
                        <Box display="flex" justify_content="flex-start" margin_bottom="16px">
                            <Icon
                                name={props.icon.clone()}
                                size="24px"
                                color={theme.colors.on_surface_variant.clone()}
                            />
                        </Box>
                    }

                    if !props.title.is_empty() {
                        <Typography tag="h2" variant={TypographyVariant::HeadlineSmall}
                            class={title_color_class}>
                            { html! { { &props.title } } }
                        </Typography>
                    }

                    <div class={yew::classes![content_style.get_class_name().to_string(), content_color_class]}>
                        { props.children.clone() }
                    </div>

                    <Box display="flex" justify_content="flex-end" gap="8px">
                        <Button
                            label={props.dismiss_label.clone()}
                            variant={ButtonVariant::Text}
                            onclick={on_dismiss}
                        />
                        <Button
                            label={props.confirm_label.clone()}
                            variant={ButtonVariant::Filled}
                            onclick={on_confirm}
                        />
                    </Box>
                </div>
            </div>
        </Portal>
    }
}
