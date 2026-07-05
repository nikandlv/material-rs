//! Material Design 3 Select Box
//!
//! A dropdown selection field matching MD3 Text Field aesthetics.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::components::menu::Menu;
use crate::components::text_field::TextFieldVariant;
use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

#[derive(Clone, Properties, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[derive(Properties, PartialEq)]
pub struct SelectProps {
    /// Select input label.
    pub label: String,

    /// Currently selected value.
    pub value: String,

    /// List of options to choose from.
    pub options: Vec<SelectOption>,

    /// Callback when value changes.
    pub onchange: Callback<String>,

    /// Optional leading icon name.
    #[prop_or_default]
    pub leading_icon: String,

    /// Whether the select is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// Text field variant for styling.
    #[prop_or_default]
    pub variant: TextFieldVariant,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,

    /// Whether the select takes full width of its parent.
    #[prop_or(true)]
    pub full_width: bool,
}

#[component]
pub fn Select(props: &SelectProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");
    let menu_open = use_state(|| false);
    let focused = use_state(|| false);

    let toggle_menu = {
        let menu_open = menu_open.clone();
        let disabled = props.disabled;
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                menu_open.set(!*menu_open);
            }
        })
    };

    let close_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_: ()| menu_open.set(false))
    };

    let on_focus = {
        let focused = focused.clone();
        Callback::from(move |_: FocusEvent| focused.set(true))
    };

    let on_blur = {
        let focused = focused.clone();
        Callback::from(move |_: FocusEvent| focused.set(false))
    };

    let selected_label = props
        .options
        .iter()
        .find(|opt| opt.value == props.value)
        .map(|opt| opt.label.clone())
        .unwrap_or_else(|| props.value.clone());

    let has_value = !props.value.is_empty();
    let label_floated = *focused || has_value;

    // Color states matching TextField variants
    let (container_color, label_color, text_color, indicator_color) = if props.disabled {
        let on_surf_38 = with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default();
        let on_surf_12 = with_alpha(&theme.colors.on_surface, 0.12).unwrap_or_default();
        (
            if matches!(props.variant, TextFieldVariant::Filled) {
                on_surf_12
            } else {
                "transparent".into()
            },
            on_surf_38.clone(),
            on_surf_38,
            "transparent".into(),
        )
    } else if *focused {
        (
            if matches!(props.variant, TextFieldVariant::Filled) {
                theme.colors.surface_container_highest.clone()
            } else {
                "transparent".into()
            },
            theme.colors.primary.clone(),
            theme.colors.on_surface.clone(),
            theme.colors.primary.clone(),
        )
    } else {
        (
            match props.variant {
                TextFieldVariant::Filled => theme.colors.surface_container_highest.clone(),
                _ => "transparent".into(),
            },
            theme.colors.on_surface_variant.clone(),
            theme.colors.on_surface.clone(),
            theme.colors.on_surface_variant.clone(),
        )
    };

    let indicator_height = if *focused && !props.disabled {
        "2px"
    } else {
        "1px"
    };

    let container_style = use_style!(
        r#"
        position: relative;
        display: flex;
        align-items: center;
        width: 100%;
        height: 56px;
        border-radius: ${radius};
        background-color: ${container_color};
        cursor: ${cursor};
        box-sizing: border-box;
        font-family: ${font_family}, sans-serif;
        transition: background-color 200ms cubic-bezier(0.2, 0, 0, 1);
        "#,
        container_color = container_color,
        cursor = if props.disabled { "default" } else { "pointer" },
        font_family = theme.font_family,
        radius = match props.variant {
            TextFieldVariant::Filled => "4px 4px 0 0",
            TextFieldVariant::Outlined => "4px",
            _ => "4px",
        },
    );

    let border_style = match props.variant {
        TextFieldVariant::Outlined => {
            let border_color = if *focused {
                indicator_color.clone()
            } else {
                theme.colors.outline.clone()
            };
            format!("border: 1px solid {}; border-radius: 4px;", border_color)
        }
        TextFieldVariant::Underline => {
            let border_color = if *focused {
                indicator_color.clone()
            } else {
                theme.colors.outline.clone()
            };
            format!("border-bottom: 1px solid {};", border_color)
        }
        _ => String::new(),
    };

    let has_leading = !props.leading_icon.is_empty();
    let label_left = if has_leading { "48px" } else { "16px" };
    let label_top = if label_floated { "8px" } else { "18px" };
    let label_size = if label_floated { "12px" } else { "16px" };

    // Convert SelectOption items into Menu MenuItem parameters
    let menu_items: Vec<crate::components::menu::MenuItem> = props
        .options
        .iter()
        .map(|opt| crate::components::menu::MenuItem {
            key: opt.value.clone(),
            label: opt.label.clone(),
            icon: String::new(),
            trailing_text: String::new(),
            disabled: false,
        })
        .collect();

    // ── Dynamic style classes ──
    let root_class = dynamic_style(format!(
        "position: relative; display: flex; flex-direction: column; width: {};",
        if props.full_width { "100%" } else { "auto" }
    ));

    let label_class = dynamic_style(format!(
        "position: absolute; inset-inline-start: {}; top: {}; font-size: {}; font-weight: 500; \
         color: {}; pointer-events: none; transition: top 200ms cubic-bezier(0.2, 0, 0, 1), \
         font-size 200ms cubic-bezier(0.2, 0, 0, 1); z-index: 2;",
        label_left, label_top, label_size, label_color
    ));

    let value_display_class = dynamic_style(format!(
        "flex: 1; padding-block-start: {}; padding-inline-end: 16px; padding-block-end: 8px; padding-inline-start: {}; font-size: 16px; color: {}; \
         white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
        if label_floated { "24px" } else { "16px" },
        if has_leading { "4px" } else { "16px" },
        text_color
    ));

    let chevron_class = dynamic_style(format!(
        "margin-inline-end: 12px; \
         transform: {}; transition: transform 250ms cubic-bezier(0.2, 0, 0, 1);",
        if *menu_open {
            "rotate(180deg)"
        } else {
            "rotate(0)"
        }
    ));

    let indicator_class = dynamic_style(format!(
        "position: absolute; bottom: 0; inset-inline-start: 0; inset-inline-end: 0; height: {}; background-color: {}; \
         transition: height 200ms cubic-bezier(0.2, 0, 0, 1); z-index: 2;",
        indicator_height, indicator_color
    ));

    let component_override = theme.component_style("Select").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div
            class={yew::classes!["md-select-container", &props.class, root_class, &component_override]}
            id={props.id.clone()}
        >
            <div
                class={container_style.get_class_name().to_string()}
                style={border_style}
                onclick={toggle_menu}
                onfocus={on_focus}
                onblur={on_blur}
                tabindex={if props.disabled { "-1" } else { "0" }}
                id={format!("{}-anchor", props.id)}
            >
                // Floating Label
                if !props.label.is_empty() {
                    <label class={label_class}>
                        { &props.label }
                    </label>
                }

                // Leading Icon
                if has_leading {
                    <span style="margin: 0 12px;">
                        <Icon
                            name={props.leading_icon.clone()}
                            color={if props.disabled { with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default() } else { theme.colors.on_surface_variant.clone() }}
                        />
                    </span>
                }

                // Value Display
                <div class={value_display_class}>
                    { selected_label }
                </div>

                // Trailing dropdown chevron
                <span class={chevron_class}>
                    <Icon
                        name="arrow_drop_down"
                        color={if props.disabled { with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default() } else { theme.colors.on_surface_variant.clone() }}
                    />
                </span>

                // Bottom active line indicator (only for Filled variant)
                if matches!(props.variant, TextFieldVariant::Filled) {
                    <div class={indicator_class} />
                }
            </div>

            // Option selection menu
            <Menu
                open={*menu_open}
                on_close={close_menu}
                anchor_id={format!("{}-anchor", props.id)}
                items={menu_items}
                on_select={props.onchange.clone()}
                match_anchor_width={true}
            />
        </div>
    }
}
