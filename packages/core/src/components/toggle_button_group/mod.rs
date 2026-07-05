//! Material Design 3 Toggle Button Group
//!
//! A set of toggle buttons where one or more can be selected.
//! Supports single-select and multi-select modes with expressive MD3 styling.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::components::ripple::Ripple;
use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

/// A single item in the toggle button group.
#[derive(Clone, PartialEq)]
pub struct ToggleButtonItem {
    /// Unique key identifying this button.
    pub key: String,
    /// Display label text.
    pub label: String,
    /// Optional leading icon (Material Symbols ligature name).
    pub icon: String,
}

/// Selection mode for the toggle button group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleGroupMode {
    /// Only one button can be selected at a time.
    #[default]
    Single,
    /// Multiple buttons can be selected simultaneously.
    Multi,
}

#[derive(Properties, PartialEq)]
pub struct ToggleButtonGroupProps {
    /// The buttons to render.
    pub items: Vec<ToggleButtonItem>,

    /// Currently selected key(s). For single mode: one key. For multi mode: zero or more keys.
    #[prop_or_default]
    pub selected: Vec<String>,

    /// Selection mode.
    #[prop_or_default]
    pub mode: ToggleGroupMode,

    /// Callback when selection changes. Emits the new list of selected keys.
    pub onchange: Callback<Vec<String>>,

    /// Whether the entire group is disabled.
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
pub fn ToggleButtonGroup(props: &ToggleButtonGroupProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let group_style = use_style!(
        r#"
        display: inline-flex;
        border: 1px solid ${border};
        border-radius: ${radius};
        overflow: hidden;
        font-family: ${font_family}, sans-serif;
        "#,
        border = theme.colors.outline,
        radius = theme.shapes.extra_small,
        font_family = theme.font_family,
    );

    let item_style = use_style!(
        r#"
        position: relative;
        overflow: hidden;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        height: 40px;
        padding: 0 16px;
        border: none;
        outline: none;
        cursor: pointer;
        font-family: ${font_family}, sans-serif;
        font-size: 14px;
        font-weight: 500;
        letter-spacing: 0.1px;
        white-space: nowrap;
        user-select: none;
        -webkit-user-select: none;
        transition: background-color 200ms cubic-bezier(0.2, 0, 0, 1),
                    color 200ms cubic-bezier(0.2, 0, 0, 1),
                    transform 150ms cubic-bezier(0.34, 1.56, 0.64, 1);

        &:hover {
            transform: scale(1.02);
        }

        &:active {
            transform: scale(0.97);
        }

        &:focus-visible {
            outline: 2px solid ${focus_ring};
            outline-offset: -2px;
            z-index: 1;
        }

        &:disabled {
            cursor: default;
            transform: none;
        }
        "#,
        font_family = theme.font_family,
        focus_ring = theme.colors.primary,
    );

    let icon_span_style = use_style!(
        r#"
        position: relative;
        z-index: 1;
        pointer-events: none;
        font-size: 18px;
        "#,
    );

    let label_span_style = use_style!(
        r#"
        position: relative;
        z-index: 1;
        pointer-events: none;
        "#,
    );

    let divider_style = use_style!(
        r#"
        width: 1px;
        align-self: stretch;
        margin: 4px 0;
        flex-shrink: 0;
        "#,
    );

    let component_override = theme.component_style("ToggleButtonGroup").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    let items_html = props.items.iter().enumerate().map(|(idx, item)| {
        let is_selected = props.selected.contains(&item.key);
        let key = item.key.clone();

        let on_click = {
            let onchange = props.onchange.clone();
            let selected = props.selected.clone();
            let mode = props.mode;
            let disabled = props.disabled;
            let key = key.clone();
            Callback::from(move |_: MouseEvent| {
                if disabled {
                    return;
                }
                let mut next = selected.clone();
                match mode {
                    ToggleGroupMode::Single => {
                        if next.contains(&key) {
                            next.clear();
                        } else {
                            next.clear();
                            next.push(key.clone());
                        }
                    }
                    ToggleGroupMode::Multi => {
                        if next.contains(&key) {
                            next.retain(|k| k != &key);
                        } else {
                            next.push(key.clone());
                        }
                    }
                }
                onchange.emit(next);
            })
        };

        let (bg, fg) = if props.disabled {
            let on_surf_12 = with_alpha(&theme.colors.on_surface, 0.12).unwrap_or_default();
            let on_surf_38 = with_alpha(&theme.colors.on_surface, 0.38).unwrap_or_default();
            (on_surf_12, on_surf_38)
        } else if is_selected {
            (
                theme.colors.secondary_container.clone(),
                theme.colors.on_secondary_container.clone(),
            )
        } else {
            ("transparent".into(), theme.colors.on_surface.clone())
        };

        let ripple_color = if props.disabled {
            "transparent".into()
        } else if is_selected {
            with_alpha(&theme.colors.on_secondary_container, 0.12).unwrap_or_default()
        } else {
            with_alpha(&theme.colors.on_surface, 0.12).unwrap_or_default()
        };

        let item_bg_class = dynamic_style(format!("background-color: {}; color: {};", bg, fg));
        let divider_bg_class = dynamic_style(format!("background-color: {};", theme.colors.outline_variant));

        html! {
            <>
                <button
                    key={item.key.clone()}
                    class={yew::classes![item_style.get_class_name().to_string(), item_bg_class]}
                    onclick={on_click}
                    disabled={props.disabled}
                    aria-pressed={if is_selected { "true" } else { "false" }}
                >
                    <Ripple color={ripple_color} />
                    if !item.icon.is_empty() {
                        <Icon name={item.icon.clone()} class={icon_span_style.get_class_name().to_string()} />
                    }
                    <span class={label_span_style.get_class_name().to_string()}>{ &item.label }</span>
                </button>
                if idx < props.items.len() - 1 {
                    <div class={yew::classes![divider_style.get_class_name().to_string(), divider_bg_class]} />
                }
            </>
        }
    });

    html! {
        <div
            class={yew::classes![group_style.get_class_name().to_string(), &props.class, &component_override]}
            id={props.id.clone()}
            role="group"
        >
            { for items_html }
        </div>
    }
}
