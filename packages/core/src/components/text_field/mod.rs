//! Material Design 3 Text Field
//!
//! Supports Filled and Outlined text fields with floating labels, leading/trailing icons,
//! and helper/error text support.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

/// Text field variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextFieldVariant {
    #[default]
    Filled,
    Outlined,
    /// No background, no border - clean minimal style.
    Plain,
    /// Bottom border only indicator style.
    Underline,
    /// Compact height variant (40px instead of 56px).
    Dense,
}

#[derive(Properties, PartialEq)]
pub struct TextFieldProps {
    /// Floating label text.
    pub label: String,

    /// Text field variant.
    #[prop_or_default]
    pub variant: TextFieldVariant,

    /// Current text value.
    pub value: String,

    /// Callback when value changes.
    pub onchange: Callback<String>,

    /// Placeholder text shown when empty and focused.
    #[prop_or_default]
    pub placeholder: String,

    /// Helper text shown below the text field.
    #[prop_or_default]
    pub helper_text: String,

    /// Error text shown when in error state.
    #[prop_or_default]
    pub error_text: String,

    /// Whether the text field is in error state.
    #[prop_or(false)]
    pub error: bool,

    /// Whether the text field is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// Leading icon name (Google Font Icon ligature).
    #[prop_or_default]
    pub leading_icon: String,

    /// Trailing icon name (Google Font Icon ligature).
    #[prop_or_default]
    pub trailing_icon: String,

    /// Input type (text, password, email, etc.).
    #[prop_or("text".to_owned())]
    pub input_type: String,

    /// Whether the text field is required.
    #[prop_or(false)]
    pub required: bool,

    /// Maximum length constraint.
    #[prop_or(0)]
    pub max_length: usize,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,

    /// Whether the text field takes full width of its parent.
    #[prop_or(true)]
    pub full_width: bool,
}

#[component]
pub fn TextField(props: &TextFieldProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let focused = use_state(|| false);

    let on_focus = {
        let focused = focused.clone();
        Callback::from(move |_: FocusEvent| focused.set(true))
    };

    let on_blur = {
        let focused = focused.clone();
        Callback::from(move |_: FocusEvent| focused.set(false))
    };

    let on_input = {
        let onchange = props.onchange.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            onchange.emit(input.value());
        })
    };

    let is_error = props.error;
    let label_floated = *focused || !props.value.is_empty() || !props.placeholder.is_empty();

    let is_dense = matches!(props.variant, TextFieldVariant::Dense);
    let container_height = if is_dense { "40px" } else { "56px" };

    // Color states
    let (container_color, label_color, text_color, indicator_color, icon_color) = if props.disabled {
        let on_surf_38 = with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default();
        let on_surf_12 = with_alpha(&theme.colors.on_surface, 0.12).unwrap_or_default();
        (
            if matches!(props.variant, TextFieldVariant::Filled) { on_surf_12 } else { "transparent".into() },
            on_surf_38.clone(),
            on_surf_38.clone(),
            "transparent".into(),
            on_surf_38,
        )
    } else if is_error {
        (
            if matches!(props.variant, TextFieldVariant::Filled) { theme.colors.surface_container_highest.clone() } else { "transparent".into() },
            theme.colors.error.clone(),
            theme.colors.on_surface.clone(),
            theme.colors.error.clone(),
            theme.colors.error.clone(),
        )
    } else if *focused {
        (
            if matches!(props.variant, TextFieldVariant::Filled) { theme.colors.surface_container_highest.clone() } else { "transparent".into() },
            theme.colors.primary.clone(),
            theme.colors.on_surface.clone(),
            theme.colors.primary.clone(),
            theme.colors.on_surface_variant.clone(),
        )
    } else {
        (
            match props.variant {
                TextFieldVariant::Filled | TextFieldVariant::Dense => theme.colors.surface_container_highest.clone(),
                _ => "transparent".into(),
            },
            theme.colors.on_surface_variant.clone(),
            theme.colors.on_surface.clone(),
            theme.colors.on_surface_variant.clone(),
            theme.colors.on_surface_variant.clone(),
        )
    };

    let indicator_height = if *focused && !props.disabled { "2px" } else { "1px" };

    // ── Static CSS classes ──

    let outer_style = use_style!(
        r#"
        display: flex;
        flex-direction: column;
        font-family: ${font_family}, sans-serif;
        "#,
        font_family = theme.font_family,
    );

    let container_style = use_style!(
        r#"
        position: relative;
        display: flex;
        align-items: center;
        width: 100%;
        height: ${height};
        border-radius: ${radius};
        overflow: visible;
        transition: background-color 200ms cubic-bezier(0.2, 0, 0, 1);
        "#,
        height = container_height,
        radius = match props.variant {
            TextFieldVariant::Filled | TextFieldVariant::Dense => "4px 4px 0 0",
            TextFieldVariant::Outlined => "4px",
            TextFieldVariant::Plain => "4px",
            TextFieldVariant::Underline => "0",
        },
    );

    let input_style = use_style!(
        r#"
        flex: 1;
        height: 100%;
        border: none;
        outline: none;
        background: transparent;
        font-family: ${font_family}, sans-serif;
        font-size: 16px;
        position: relative;
        z-index: 1;

        &:-webkit-autofill,
        &:-webkit-autofill:hover,
        &:-webkit-autofill:focus,
        &:-webkit-autofill:active {
            -webkit-box-shadow: 0 0 0 1000px transparent inset !important;
            -webkit-text-fill-color: ${text_color} !important;
            caret-color: ${caret_color} !important;
            transition: background-color 5000s ease-in-out 0s;
        }
        "#,
        font_family = theme.font_family,
        text_color = text_color,
        caret_color = theme.colors.primary,
    );

    let icon_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        justify-content: center;
        width: 24px;
        height: 24px;
        margin: 0 12px;
        flex-shrink: 0;
        font-size: 24px;
        position: relative;
        z-index: 1;
        "#,
    );

    let helper_style = use_style!(
        r#"
        display: flex;
        justify-content: space-between;
        padding: 4px 16px 0 16px;
        font-size: 12px;
        font-family: ${font_family}, sans-serif;
        line-height: 16px;
        "#,
        font_family = theme.font_family,
    );

    // ── Dynamic inline styles (colors change with state) ──

    let has_leading = !props.leading_icon.is_empty();
    let input_padding_left = if has_leading { "4px" } else { "16px" };
    let input_padding_right = if !props.trailing_icon.is_empty() { "4px" } else { "16px" };

    let (label_top, label_left, label_translate_y, label_bg) = match props.variant {
        TextFieldVariant::Filled | TextFieldVariant::Dense => {
            let top = if label_floated {
                if is_dense { "4px" } else { "8px" }
            } else {
                if is_dense { "10px" } else { "18px" }
            };
            let left = if has_leading { "48px" } else { "16px" };
            (top.to_owned(), left.to_owned(), "none", "transparent".to_string())
        }
        TextFieldVariant::Plain => {
            let top = if label_floated { "-8px" } else { "12px" };
            let left = if has_leading { "48px" } else { "0px" };
            (top.to_owned(), left.to_owned(), "none", theme.colors.surface.clone())
        }
        TextFieldVariant::Underline | TextFieldVariant::Outlined => {
            let top = if label_floated { "0px" } else { "50%" };
            let ty = if label_floated { "none" } else { "translateY(-50%)" };
            let left = if has_leading { "48px" } else {
                if label_floated { "12px" } else { "16px" }
            };
            let bg = if label_floated { theme.colors.surface.clone() } else { "transparent".into() };
            (top.to_owned(), left.to_owned(), ty, bg)
        }
    };

    let font_size_label = if label_floated { "12px" } else { "16px" };
    let label_transform = if label_translate_y == "none" {
        if matches!(props.variant, TextFieldVariant::Outlined) {
            "translateY(-50%)"
        } else {
            "none"
        }
    } else {
        label_translate_y
    };

    let input_padding_top = match props.variant {
        TextFieldVariant::Filled => "24px",
        TextFieldVariant::Dense => "16px",
        TextFieldVariant::Plain => "8px",
        _ => "16px",
    };

    let counter_text = if props.max_length > 0 {
        Some(format!("{}/{}", props.value.len(), props.max_length))
    } else {
        None
    };

    let shown_helper = if is_error && !props.error_text.is_empty() {
        &props.error_text
    } else {
        &props.helper_text
    };

    let outlined_border_color = if *focused { indicator_color.clone() } else { theme.colors.outline.clone() };
    let underline_border_color = outlined_border_color.clone();

    // ── Dynamic style classes ──
    let container_bg_class = dynamic_style(format!(
        "background-color: {};", container_color
    ));

    let outlined_border_class = dynamic_style(format!(
        "position: absolute; inset: 0; border: 1px solid {}; border-radius: 4px; pointer-events: none; z-index: 1;",
        outlined_border_color
    ));

    let underline_border_class = dynamic_style(format!(
        "position: absolute; bottom: 0; inset-inline-start: 0; inset-inline-end: 0; border-bottom: 1px solid {}; pointer-events: none; z-index: 1;",
        underline_border_color
    ));

    let label_class = dynamic_style(format!(
        "position: absolute; inset-inline-start: {}; top: {}; transform: {}; font-size: {}; font-weight: 500; color: {}; background: {}; padding: 0 4px; transition: top 200ms cubic-bezier(0.2, 0, 0, 1), font-size 200ms cubic-bezier(0.2, 0, 0, 1), inset-inline-start 200ms cubic-bezier(0.2, 0, 0, 1), color 200ms; pointer-events: none; z-index: 3; transform-origin: left center;",
        label_left, label_top, label_transform, font_size_label, label_color, label_bg
    ));

    let input_inline_class = dynamic_style(format!(
        "padding-block-start: {}; padding-inline-end: {}; padding-block-end: 8px; padding-inline-start: {}; color: {}; caret-color: {};",
        input_padding_top, input_padding_right, input_padding_left, text_color, theme.colors.primary
    ));

    let indicator_class = dynamic_style(format!(
        "position: absolute; bottom: 0; inset-inline-start: 0; inset-inline-end: 0; height: {}; background-color: {}; transition: height 200ms cubic-bezier(0.2, 0, 0, 1); border-radius: 1px 1px 0 0; z-index: 2;",
        indicator_height, indicator_color
    ));

    let helper_color_class = dynamic_style(format!(
        "color: {};",
        if is_error { theme.colors.error.clone() } else { theme.colors.on_surface_variant.clone() }
    ));

    let counter_color_class = dynamic_style(format!(
        "color: {}; margin-inline-start: auto;",
        theme.colors.on_surface_variant
    ));

    let component_override = theme.component_style("TextField").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div class={yew::classes![outer_style.get_class_name().to_string(), &props.class, &component_override]}
             id={props.id.clone()}
             style={if props.full_width { "width: 100%;" } else { "width: auto;" }}>
            <div class={yew::classes![container_style.get_class_name().to_string(), container_bg_class]}>
                // Border style (only for Outlined and Underline)
                if matches!(props.variant, TextFieldVariant::Outlined) {
                    <div class={outlined_border_class} />
                }
                if matches!(props.variant, TextFieldVariant::Underline) {
                    <div class={underline_border_class} />
                }

                // Label
                if !props.label.is_empty() {
                    <label class={label_class}>{ &props.label }</label>
                } else if !label_floated && !props.placeholder.is_empty() {
                    <label class={label_class}>{ &props.placeholder }</label>
                }

                if !props.leading_icon.is_empty() {
                    <Icon name={props.leading_icon.clone()} color={icon_color.clone()} class={icon_style.get_class_name().to_string()} />
                }

                <input
                    type={props.input_type.clone()}
                    value={props.value.clone()}
                    placeholder={if label_floated { props.placeholder.clone() } else { String::new() }}
                    disabled={props.disabled}
                    onfocus={on_focus}
                    onblur={on_blur}
                    oninput={on_input}
                    class={yew::classes![input_style.get_class_name().to_string(), input_inline_class]}
                    aria-label={props.label.clone()}
                    aria-invalid={if is_error { "true" } else { "false" }}
                    required={props.required}
                    maxlength={if props.max_length > 0 { Some(props.max_length.to_string()) } else { None }}
                />

                if !props.trailing_icon.is_empty() {
                    <Icon name={props.trailing_icon.clone()} color={icon_color.clone()} class={icon_style.get_class_name().to_string()} />
                }

                // Indicator (Filled / Dense)
                if matches!(props.variant, TextFieldVariant::Filled | TextFieldVariant::Dense) {
                    <div class={indicator_class} />
                }
            </div>
            // Helper/Error message
            if !shown_helper.is_empty() || counter_text.is_some() {
                <div class={helper_style.get_class_name().to_string()}>
                    <span class={helper_color_class}>
                        { shown_helper }
                    </span>
                    if let Some(counter) = counter_text {
                        <span class={counter_color_class}>
                            { counter }
                        </span>
                    }
                </div>
            }
        </div>
    }
}
