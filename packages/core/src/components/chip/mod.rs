//! Material Design 3 Chip
//!
//! Supports Assist, Filter, and Input chip types.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::components::ripple::Ripple;
use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

/// Chip type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChipType {
    /// Standard action chip.
    #[default]
    Assist,
    /// Selection/filter chip.
    Filter,
    /// Input chip with remove button.
    Input,
}

#[derive(Properties, PartialEq)]
pub struct ChipProps {
    /// Chip label.
    pub label: String,

    /// Chip type.
    #[prop_or_default]
    pub chip_type: ChipType,

    /// Leading icon.
    #[prop_or_default]
    pub icon: String,

    /// Whether the chip is selected (for Filter type).
    #[prop_or(false)]
    pub selected: bool,

    /// Whether the chip is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// Callback when selected state toggles.
    #[prop_or_default]
    pub on_select: Callback<bool>,

    /// Callback when remove is clicked (for Input type).
    #[prop_or_default]
    pub on_remove: Callback<()>,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn Chip(props: &ChipProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let on_click = {
        let on_select = props.on_select.clone();
        let selected = props.selected;
        let disabled = props.disabled;
        let chip_type = props.chip_type;
        Callback::from(move |_: MouseEvent| {
            if !disabled
                && matches!(chip_type, ChipType::Filter) {
                    on_select.emit(!selected);
                }
        })
    };

    let on_remove_click = {
        let on_remove = props.on_remove.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_remove.emit(());
        })
    };

    let (bg, label_color, _border_color, outline_style, ripple_color) = if props.disabled {
        let on_surface_12 = with_alpha(&theme.colors.on_surface, 0.12).unwrap_or_default();
        let on_surface_38 = with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default();
        (
            on_surface_12.clone(),
            on_surface_38.clone(),
            on_surface_38,
            "none".into(),
            on_surface_12,
        )
    } else if matches!(props.chip_type, ChipType::Filter) && props.selected {
        (
            with_alpha(&theme.colors.secondary_container, 0.5).unwrap_or_default(),
            theme.colors.on_secondary_container.clone(),
            theme.colors.secondary_container.clone(),
            "1px solid ".to_owned() + &theme.colors.secondary_container,
            with_alpha(&theme.colors.on_secondary_container, 0.12).unwrap_or_default(),
        )
    } else {
        let outline_var = theme.colors.outline.clone();
        (
            "transparent".into(),
            theme.colors.on_surface.clone(),
            outline_var.clone(),
            "1px solid ".to_owned() + &outline_var,
            with_alpha(&theme.colors.primary, 0.12).unwrap_or_default(),
        )
    };

    let chip_style = use_style!(r#"
        position: relative;
        overflow: hidden;
        display: inline-flex;
        align-items: center;
        gap: 8px;
        height: 32px;
        padding: 0 12px;
        border-radius: ${radius};
        border: none;
        outline: none;
        font-family: ${font_family}, sans-serif;
        font-size: 14px;
        font-weight: 600;
        letter-spacing: 0.1px;
        cursor: ${cursor};
        white-space: nowrap;
        user-select: none;
        -webkit-user-select: none;
        transition: transform 150ms cubic-bezier(0.34, 1.56, 0.64, 1), background-color 200ms, border-color 200ms;

        &:hover {
            transform: ${hover_transform};
        }

        &:active {
            transform: ${active_transform};
        }

        &:focus-visible {
            outline: 2px solid ${focus_ring};
            outline-offset: 2px;
        }
        "#,
        radius = theme.shapes.small,
        font_family = theme.font_family,
        cursor = if props.disabled { "default" } else { "pointer" },
        hover_transform = if props.disabled { "none" } else { "scale(1.03) translateY(-0.5px)" },
        active_transform = if props.disabled { "none" } else { "scale(0.97)" },
        focus_ring = theme.colors.primary,
    );

    let icon_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        font-size: 18px;
        position: relative;
        z-index: 1;
        pointer-events: none;
        "#,
    );

    let label_style = use_style!(
        r#"
        position: relative;
        z-index: 1;
        pointer-events: none;
        "#,
    );

    let remove_btn_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        justify-content: center;
        width: 18px;
        height: 18px;
        border: none;
        background: none;
        cursor: pointer;
        position: relative;
        z-index: 1;
        font-size: 18px;
        padding: 0;
        border-radius: 50%;
        "#,
    );

    let check_icon = if matches!(props.chip_type, ChipType::Filter) && props.selected && !props.disabled {
        html! {
            <svg width="18" height="18" viewBox="0 0 18 18" style="position: relative; z-index: 1;">
                <path d="M4.5 9.5L7.5 12.5L13.5 6" fill="none"
                    stroke={theme.colors.on_secondary_container.clone()}
                    stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
        }
    } else {
        html! {}
    };

    let chip_dynamic_class = dynamic_style(format!("background-color: {}; color: {}; {};", bg, label_color, outline_style));

    let remove_btn_color_class = dynamic_style(format!("color: {};", if props.disabled {
        with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default()
    } else {
        theme.colors.on_surface_variant.clone()
    }));

    let component_override = theme.component_style("Chip").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div
            class={yew::classes![chip_style.get_class_name().to_string(), &props.class, chip_dynamic_class, &component_override]}
            id={props.id.clone()}
            onclick={on_click}
            role={if matches!(props.chip_type, ChipType::Filter) { Some("checkbox") } else { Some("option") }}
            aria-checked={if matches!(props.chip_type, ChipType::Filter) { if props.selected { Some("true") } else { Some("false") } } else { None }}
            aria-selected={if props.selected { "true" } else { "false" }}
            tabindex={if props.disabled { None } else { Some("0") }}
        >
            <Ripple color={ripple_color} />
            if !props.icon.is_empty() {
                <span class={icon_style.get_class_name().to_string()}>
                    <Icon name={props.icon.clone()} size="18px" />
                </span>
            }
            { check_icon }
            <span class={label_style.get_class_name().to_string()}>{ &props.label }</span>
            if matches!(props.chip_type, ChipType::Input) {
                <button
                    onclick={on_remove_click}
                    class={yew::classes![remove_btn_style.get_class_name().to_string(), remove_btn_color_class]}
                    aria-label="Remove"
                >
                    { "\u{00D7}" }
                </button>
            }
        </div>
    }
}
