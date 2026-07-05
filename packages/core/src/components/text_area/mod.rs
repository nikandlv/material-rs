//! Material Design 3 Text Area Component
//!
//! A multiline text input field with floating label, matching the TextField aesthetics.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::text_field::TextFieldVariant;
use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

#[derive(Properties, PartialEq)]
pub struct TextAreaProps {
    /// Floating label text.
    pub label: String,

    /// Text field variant: Filled or Outlined. Defaults to Filled.
    #[prop_or_default]
    pub variant: TextFieldVariant,

    /// Current text value.
    pub value: String,

    /// Callback when value changes.
    pub onchange: Callback<String>,

    /// Placeholder text shown when empty and focused.
    #[prop_or_default]
    pub placeholder: String,

    /// Helper text shown below the text area.
    #[prop_or_default]
    pub helper_text: String,

    /// Error text shown when in error state.
    #[prop_or_default]
    pub error_text: String,

    /// Whether the text area is in error state.
    #[prop_or(false)]
    pub error: bool,

    /// Whether the text area is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// Standard row height. Defaults to 3.
    #[prop_or(3)]
    pub rows: u32,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn TextArea(props: &TextAreaProps) -> Html {
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
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            onchange.emit(input.value());
        })
    };

    let is_error = props.error;
    let label_floated = *focused || !props.value.is_empty() || !props.placeholder.is_empty();

    let (container_color, label_color, text_color, indicator_color) = if props.disabled {
        let on_surf_38 = with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default();
        let on_surf_12 = with_alpha(&theme.colors.on_surface, 0.12).unwrap_or_default();
        (
            if matches!(props.variant, TextFieldVariant::Filled) { on_surf_12 } else { "transparent".into() },
            on_surf_38.clone(),
            on_surf_38,
            "transparent".into(),
        )
    } else if is_error {
        (
            if matches!(props.variant, TextFieldVariant::Filled) { theme.colors.surface_container_highest.clone() } else { "transparent".into() },
            theme.colors.error.clone(),
            theme.colors.on_surface.clone(),
            theme.colors.error.clone(),
        )
    } else if *focused {
        (
            if matches!(props.variant, TextFieldVariant::Filled) { theme.colors.surface_container_highest.clone() } else { "transparent".into() },
            theme.colors.primary.clone(),
            theme.colors.on_surface.clone(),
            theme.colors.primary.clone(),
        )
    } else {
        (
            if matches!(props.variant, TextFieldVariant::Filled) { theme.colors.surface_container_highest.clone() } else { "transparent".into() },
            theme.colors.on_surface_variant.clone(),
            theme.colors.on_surface.clone(),
            theme.colors.on_surface_variant.clone(),
        )
    };

    let indicator_height = if *focused && !props.disabled { "2px" } else { "1px" };

    let wrapper_style = use_style!(
        r#"
        display: flex;
        flex-direction: column;
        width: 100%;
        font-family: ${font_family}, sans-serif;
        "#,
        font_family = theme.font_family,
    );

    let container_style = use_style!(
        r#"
        position: relative;
        display: flex;
        width: 100%;
        box-sizing: border-box;
        transition: background-color 200ms cubic-bezier(0.2, 0, 0, 1);
        "#,
    );

    let label_css_style = use_style!(
        r#"
        position: absolute;
        inset-inline-start: 16px;
        font-weight: 500;
        padding: 0 4px;
        transition: top 200ms cubic-bezier(0.2, 0, 0, 1), font-size 200ms cubic-bezier(0.2, 0, 0, 1), color 200ms;
        pointer-events: none;
        z-index: 3;
        transform-origin: left center;
        "#,
    );

    let input_style = use_style!(
        r#"
        flex: 1;
        min-height: 80px;
        padding: 16px 16px 12px 16px;
        border: none;
        outline: none;
        background: transparent;
        font-size: 16px;
        resize: vertical;
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
        text_color = text_color,
        caret_color = theme.colors.primary,
    );

    let helper_style = use_style!(
        r#"
        padding: 4px 16px 0 16px;
        font-size: 12px;
        line-height: 16px;
        "#,
    );

    let label_top = if label_floated { "8px" } else { "20px" };
    let label_bg = if matches!(props.variant, TextFieldVariant::Outlined) && label_floated {
        theme.colors.surface.clone()
    } else {
        "transparent".to_string()
    };
    let font_size_label = if label_floated { "12px" } else { "16px" };
    let label_transform = if label_floated && matches!(props.variant, TextFieldVariant::Outlined) {
        "translateY(-50%)"
    } else {
        "none"
    };

    let border_color = if *focused { indicator_color.clone() } else { theme.colors.outline.clone() };
    let border_width = if *focused { "2px" } else { "1px" };

    let _input_padding_top = if matches!(props.variant, TextFieldVariant::Filled) { "24px" } else { "16px" };

    let shown_helper = if is_error && !props.error_text.is_empty() {
        &props.error_text
    } else {
        &props.helper_text
    };

    // ── Dynamic style classes ──
    let container_inline_class = dynamic_style(format!(
        "border-radius: {}; background-color: {}; {}",
        match props.variant {
            TextFieldVariant::Filled | TextFieldVariant::Dense => "4px 4px 0 0",
            _ => "4px",
        },
        container_color,
        if matches!(props.variant, TextFieldVariant::Outlined) {
            format!("border: {} solid {}; border-radius: 4px; position: absolute; inset: 0; pointer-events: none; transition: border 200ms cubic-bezier(0.2, 0, 0, 1);", border_width, border_color)
        } else { String::new() }
    ));

    let label_class = dynamic_style(format!(
        "top: {}; transform: {}; font-size: {}; color: {}; background: {};",
        label_top, label_transform, font_size_label, label_color, label_bg
    ));

    let textarea_inline_class = dynamic_style(format!(
        "font-family: {}, sans-serif; color: {}; caret-color: {};",
        theme.font_family, text_color, theme.colors.primary
    ));

    let indicator_class = dynamic_style(format!(
        "position: absolute; bottom: 0; inset-inline-start: 0; inset-inline-end: 0; height: {}; background-color: {}; transition: height 200ms cubic-bezier(0.2, 0, 0, 1); border-radius: 1px 1px 0 0; z-index: 2;",
        indicator_height, indicator_color
    ));

    let helper_color_class = dynamic_style(format!(
        "color: {}; font-family: {}, sans-serif;",
        if is_error { theme.colors.error.clone() } else { theme.colors.on_surface_variant.clone() },
        theme.font_family
    ));

    let component_override = theme.component_style("TextArea").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div class={yew::classes![wrapper_style.get_class_name().to_string(), &props.class, &component_override]}
             id={props.id.clone()}
             >
            <div class={yew::classes![container_style.get_class_name().to_string(), container_inline_class]}>
                if !props.label.is_empty() {
                    <label class={yew::classes![label_css_style.get_class_name().to_string(), label_class]}>
                        { &props.label }
                    </label>
                } else if !label_floated && !props.placeholder.is_empty() {
                    <label class={yew::classes![label_css_style.get_class_name().to_string(), label_class]}>
                        { &props.placeholder }
                    </label>
                }

                <textarea
                    rows={props.rows.to_string()}
                    value={props.value.clone()}
                    placeholder={if label_floated { props.placeholder.clone() } else { String::new() }}
                    disabled={props.disabled}
                    onfocus={on_focus}
                    onblur={on_blur}
                    oninput={on_input}
                    class={yew::classes![input_style.get_class_name().to_string(), textarea_inline_class]}
                    aria-label={props.label.clone()}
                    aria-invalid={if is_error { "true" } else { "false" }}
                />

                if matches!(props.variant, TextFieldVariant::Filled) {
                    <div class={indicator_class} />
                }
            </div>

            if !shown_helper.is_empty() {
                <div class={helper_style.get_class_name().to_string()}>
                    <span class={helper_color_class}>
                        { shown_helper }
                    </span>
                </div>
            }
        </div>
    }
}
