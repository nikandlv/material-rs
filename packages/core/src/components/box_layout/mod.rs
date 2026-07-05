//! Material Design 3 Box Component
//!
//! A utility layout component equivalent to MUI Box that maps style properties
//! dynamically into clean inline CSS styles. Supports all HTML event handlers,
//! ARIA attributes, and refs for full flexibility.

use yew::prelude::*;

use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BoxTag {
    #[default]
    Div,
    Section,
    Article,
    Nav,
    Header,
    Footer,
    Aside,
    Main,
    Ul,
    Ol,
    Li,
    Figure,
    FigCaption,
    Details,
    Summary,
    Span,
    Pre,
    Code,
}

impl BoxTag {
    pub fn as_str(&self) -> &'static str {
        match self {
            BoxTag::Div => "div",
            BoxTag::Section => "section",
            BoxTag::Article => "article",
            BoxTag::Nav => "nav",
            BoxTag::Header => "header",
            BoxTag::Footer => "footer",
            BoxTag::Aside => "aside",
            BoxTag::Main => "main",
            BoxTag::Ul => "ul",
            BoxTag::Ol => "ol",
            BoxTag::Li => "li",
            BoxTag::Figure => "figure",
            BoxTag::FigCaption => "figcaption",
            BoxTag::Details => "details",
            BoxTag::Summary => "summary",
            BoxTag::Span => "span",
            BoxTag::Pre => "pre",
            BoxTag::Code => "code",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct BoxProps {
    /// HTML element tag. Defaults to "div".
    #[prop_or_default]
    pub tag: BoxTag,

    // Layout
    #[prop_or_default] pub display: Option<String>,
    #[prop_or_default] pub flex_direction: Option<String>,
    #[prop_or_default] pub justify_content: Option<String>,
    #[prop_or_default] pub align_items: Option<String>,
    #[prop_or_default] pub align_self: Option<String>,
    #[prop_or_default] pub align_content: Option<String>,
    #[prop_or_default] pub gap: Option<String>,
    #[prop_or_default] pub row_gap: Option<String>,
    #[prop_or_default] pub column_gap: Option<String>,
    #[prop_or_default] pub flex_wrap: Option<String>,
    #[prop_or_default] pub flex_grow: Option<String>,
    #[prop_or_default] pub flex_shrink: Option<String>,
    #[prop_or_default] pub flex_basis: Option<String>,
    #[prop_or_default] pub flex: Option<String>,
    #[prop_or_default] pub order: Option<String>,

    // Grid
    #[prop_or_default] pub grid_template_columns: Option<String>,
    #[prop_or_default] pub grid_template_rows: Option<String>,
    #[prop_or_default] pub grid_column: Option<String>,
    #[prop_or_default] pub grid_row: Option<String>,
    #[prop_or_default] pub grid_area: Option<String>,

    // Spacing (Margins)
    #[prop_or_default] pub margin: Option<String>,
    #[prop_or_default] pub margin_top: Option<String>,
    #[prop_or_default] pub margin_right: Option<String>,
    #[prop_or_default] pub margin_bottom: Option<String>,
    #[prop_or_default] pub margin_left: Option<String>,
    #[prop_or_default] pub mx: Option<String>,
    #[prop_or_default] pub my: Option<String>,

    // Spacing (Paddings)
    #[prop_or_default] pub padding: Option<String>,
    #[prop_or_default] pub padding_top: Option<String>,
    #[prop_or_default] pub padding_right: Option<String>,
    #[prop_or_default] pub padding_bottom: Option<String>,
    #[prop_or_default] pub padding_left: Option<String>,
    #[prop_or_default] pub px: Option<String>,
    #[prop_or_default] pub py: Option<String>,

    // Dimensions
    #[prop_or_default] pub width: Option<String>,
    #[prop_or_default] pub height: Option<String>,
    #[prop_or_default] pub min_width: Option<String>,
    #[prop_or_default] pub min_height: Option<String>,
    #[prop_or_default] pub max_width: Option<String>,
    #[prop_or_default] pub max_height: Option<String>,

    // Positioning
    #[prop_or_default] pub position: Option<String>,
    #[prop_or_default] pub top: Option<String>,
    #[prop_or_default] pub right: Option<String>,
    #[prop_or_default] pub bottom: Option<String>,
    #[prop_or_default] pub left: Option<String>,
    #[prop_or_default] pub inset: Option<String>,
    #[prop_or_default] pub z_index: Option<String>,

    // Border & Shape
    #[prop_or_default] pub border: Option<String>,
    #[prop_or_default] pub border_top: Option<String>,
    #[prop_or_default] pub border_bottom: Option<String>,
    #[prop_or_default] pub border_left: Option<String>,
    #[prop_or_default] pub border_right: Option<String>,
    #[prop_or_default] pub border_radius: Option<String>,
    #[prop_or_default] pub box_shadow: Option<String>,
    #[prop_or_default] pub outline: Option<String>,

    // Typography
    #[prop_or_default] pub font_size: Option<String>,
    #[prop_or_default] pub font_weight: Option<String>,
    #[prop_or_default] pub line_height: Option<String>,
    #[prop_or_default] pub letter_spacing: Option<String>,
    #[prop_or_default] pub text_align: Option<String>,
    #[prop_or_default] pub text_decoration: Option<String>,
    #[prop_or_default] pub text_transform: Option<String>,
    #[prop_or_default] pub white_space: Option<String>,
    #[prop_or_default] pub word_break: Option<String>,

    // Visuals & Interactivity
    #[prop_or_default] pub bg: Option<String>,
    #[prop_or_default] pub color: Option<String>,
    #[prop_or_default] pub opacity: Option<String>,
    #[prop_or_default] pub overflow: Option<String>,
    #[prop_or_default] pub overflow_x: Option<String>,
    #[prop_or_default] pub overflow_y: Option<String>,
    #[prop_or_default] pub cursor: Option<String>,
    #[prop_or_default] pub user_select: Option<String>,
    #[prop_or_default] pub pointer_events: Option<String>,
    #[prop_or_default] pub visibility: Option<String>,
    #[prop_or_default] pub transition: Option<String>,
    #[prop_or_default] pub transform: Option<String>,
    #[prop_or_default] pub backdrop_filter: Option<String>,
    #[prop_or_default] pub object_fit: Option<String>,

    // Standard HTML
    #[prop_or_default] pub style: String,
    #[prop_or_default] pub id: String,
    #[prop_or_default] pub class: String,
    #[prop_or_default] pub role: String,
    #[prop_or_default] pub tabindex: Option<String>,
    #[prop_or_default] pub aria_label: Option<String>,
    #[prop_or_default] pub aria_labelledby: Option<String>,
    #[prop_or_default] pub aria_describedby: Option<String>,
    #[prop_or_default] pub aria_hidden: Option<bool>,
    #[prop_or_default] pub aria_expanded: Option<bool>,
    #[prop_or_default] pub aria_selected: Option<bool>,
    #[prop_or_default] pub aria_disabled: Option<bool>,
    #[prop_or_default] pub aria_controls: Option<String>,
    #[prop_or_default] pub aria_live: Option<String>,
    #[prop_or_default] pub dir: Option<String>,
    #[prop_or_default] pub draggable: Option<bool>,

    // Event handlers
    #[prop_or_default] pub onclick: Callback<MouseEvent>,
    #[prop_or_default] pub ondblclick: Callback<MouseEvent>,
    #[prop_or_default] pub onmousedown: Callback<MouseEvent>,
    #[prop_or_default] pub onmouseup: Callback<MouseEvent>,
    #[prop_or_default] pub onmouseenter: Callback<MouseEvent>,
    #[prop_or_default] pub onmouseleave: Callback<MouseEvent>,
    #[prop_or_default] pub onmousemove: Callback<MouseEvent>,
    #[prop_or_default] pub onkeydown: Callback<KeyboardEvent>,
    #[prop_or_default] pub onkeyup: Callback<KeyboardEvent>,
    #[prop_or_default] pub onkeypress: Callback<KeyboardEvent>,
    #[prop_or_default] pub onfocus: Callback<FocusEvent>,
    #[prop_or_default] pub onblur: Callback<FocusEvent>,
    #[prop_or_default] pub onscroll: Callback<Event>,
    #[prop_or_default] pub oninput: Callback<InputEvent>,
    #[prop_or_default] pub onchange: Callback<Event>,
    #[prop_or_default] pub ontouchstart: Callback<TouchEvent>,
    #[prop_or_default] pub ontouchend: Callback<TouchEvent>,
    #[prop_or_default] pub ontouchmove: Callback<TouchEvent>,
    #[prop_or_default] pub ondrag: Callback<DragEvent>,
    #[prop_or_default] pub ondragstart: Callback<DragEvent>,
    #[prop_or_default] pub ondragend: Callback<DragEvent>,
    #[prop_or_default] pub ondragover: Callback<DragEvent>,
    #[prop_or_default] pub ondragenter: Callback<DragEvent>,
    #[prop_or_default] pub ondragleave: Callback<DragEvent>,
    #[prop_or_default] pub ondrop: Callback<DragEvent>,

    // Ref
    #[prop_or_default]
    pub node_ref: NodeRef,

    // Children
    #[prop_or_default]
    pub children: Children,
}

fn add_prop(css: &mut Vec<String>, name: &str, opt: &Option<String>) {
    if let Some(val) = opt {
        css.push(format!("{}: {};", name, val));
    }
}

#[component]
pub fn Box(props: &BoxProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    // Resolve color/bg values if they reference theme colors
    let resolved_color = props.color.as_ref().map(|c| {
        match c.as_str() {
            "primary" => theme.colors.primary.clone(),
            "secondary" => theme.colors.secondary.clone(),
            "tertiary" => theme.colors.tertiary.clone(),
            "error" => theme.colors.error.clone(),
            "surface" => theme.colors.on_surface.clone(),
            "on-surface" => theme.colors.on_surface.clone(),
            "on-surface-variant" => theme.colors.on_surface_variant.clone(),
            "outline" => theme.colors.outline.clone(),
            "outline-variant" => theme.colors.outline_variant.clone(),
            other => other.to_owned(),
        }
    });

    let resolved_bg = props.bg.as_ref().map(|b| {
        match b.as_str() {
            "primary" => theme.colors.primary.clone(),
            "secondary" => theme.colors.secondary.clone(),
            "tertiary" => theme.colors.tertiary.clone(),
            "error" => theme.colors.error.clone(),
            "primary-container" => theme.colors.primary_container.clone(),
            "secondary-container" => theme.colors.secondary_container.clone(),
            "tertiary-container" => theme.colors.tertiary_container.clone(),
            "error-container" => theme.colors.error_container.clone(),
            "surface" => theme.colors.surface.clone(),
            "surface-container" => theme.colors.surface_container.clone(),
            "surface-container-low" => theme.colors.surface_container_low.clone(),
            "surface-container-high" => theme.colors.surface_container_high.clone(),
            "surface-container-highest" => theme.colors.surface_container_highest.clone(),
            "surface-variant" => theme.colors.surface_variant.clone(),
            other => other.to_owned(),
        }
    });

    let mut css = Vec::new();

    // Layout
    add_prop(&mut css, "display", &props.display);
    add_prop(&mut css, "flex-direction", &props.flex_direction);
    add_prop(&mut css, "justify-content", &props.justify_content);
    add_prop(&mut css, "align-items", &props.align_items);
    add_prop(&mut css, "align-self", &props.align_self);
    add_prop(&mut css, "align-content", &props.align_content);
    add_prop(&mut css, "gap", &props.gap);
    add_prop(&mut css, "row-gap", &props.row_gap);
    add_prop(&mut css, "column-gap", &props.column_gap);
    add_prop(&mut css, "flex-wrap", &props.flex_wrap);
    add_prop(&mut css, "flex-grow", &props.flex_grow);
    add_prop(&mut css, "flex-shrink", &props.flex_shrink);
    add_prop(&mut css, "flex-basis", &props.flex_basis);
    add_prop(&mut css, "flex", &props.flex);
    add_prop(&mut css, "order", &props.order);

    // Grid
    add_prop(&mut css, "grid-template-columns", &props.grid_template_columns);
    add_prop(&mut css, "grid-template-rows", &props.grid_template_rows);
    add_prop(&mut css, "grid-column", &props.grid_column);
    add_prop(&mut css, "grid-row", &props.grid_row);
    add_prop(&mut css, "grid-area", &props.grid_area);

    // Margins
    add_prop(&mut css, "margin", &props.margin);
    add_prop(&mut css, "margin-top", &props.margin_top);
    add_prop(&mut css, "margin-right", &props.margin_right);
    add_prop(&mut css, "margin-bottom", &props.margin_bottom);
    add_prop(&mut css, "margin-left", &props.margin_left);
    if let Some(m) = &props.mx {
        css.push(format!("margin-left: {}; margin-right: {};", m, m));
    }
    if let Some(m) = &props.my {
        css.push(format!("margin-top: {}; margin-bottom: {};", m, m));
    }

    // Paddings
    add_prop(&mut css, "padding", &props.padding);
    add_prop(&mut css, "padding-top", &props.padding_top);
    add_prop(&mut css, "padding-right", &props.padding_right);
    add_prop(&mut css, "padding-bottom", &props.padding_bottom);
    add_prop(&mut css, "padding-left", &props.padding_left);
    if let Some(p) = &props.px {
        css.push(format!("padding-left: {}; padding-right: {};", p, p));
    }
    if let Some(p) = &props.py {
        css.push(format!("padding-top: {}; padding-bottom: {};", p, p));
    }

    // Dimensions
    add_prop(&mut css, "width", &props.width);
    add_prop(&mut css, "height", &props.height);
    add_prop(&mut css, "min-width", &props.min_width);
    add_prop(&mut css, "min-height", &props.min_height);
    add_prop(&mut css, "max-width", &props.max_width);
    add_prop(&mut css, "max-height", &props.max_height);

    // Positioning
    add_prop(&mut css, "position", &props.position);
    add_prop(&mut css, "top", &props.top);
    add_prop(&mut css, "right", &props.right);
    add_prop(&mut css, "bottom", &props.bottom);
    add_prop(&mut css, "left", &props.left);
    add_prop(&mut css, "inset", &props.inset);
    add_prop(&mut css, "z-index", &props.z_index);

    // Border & Shape
    add_prop(&mut css, "border", &props.border);
    add_prop(&mut css, "border-top", &props.border_top);
    add_prop(&mut css, "border-bottom", &props.border_bottom);
    add_prop(&mut css, "border-left", &props.border_left);
    add_prop(&mut css, "border-right", &props.border_right);
    add_prop(&mut css, "border-radius", &props.border_radius);
    add_prop(&mut css, "box-shadow", &props.box_shadow);
    add_prop(&mut css, "outline", &props.outline);

    // Typography
    add_prop(&mut css, "font-size", &props.font_size);
    add_prop(&mut css, "font-weight", &props.font_weight);
    add_prop(&mut css, "line-height", &props.line_height);
    add_prop(&mut css, "letter-spacing", &props.letter_spacing);
    add_prop(&mut css, "text-align", &props.text_align);
    add_prop(&mut css, "text-decoration", &props.text_decoration);
    add_prop(&mut css, "text-transform", &props.text_transform);
    add_prop(&mut css, "white-space", &props.white_space);
    add_prop(&mut css, "word-break", &props.word_break);

    // Visuals
    add_prop(&mut css, "opacity", &props.opacity);
    add_prop(&mut css, "overflow", &props.overflow);
    add_prop(&mut css, "overflow-x", &props.overflow_x);
    add_prop(&mut css, "overflow-y", &props.overflow_y);
    add_prop(&mut css, "cursor", &props.cursor);
    add_prop(&mut css, "user-select", &props.user_select);
    add_prop(&mut css, "pointer-events", &props.pointer_events);
    add_prop(&mut css, "visibility", &props.visibility);
    add_prop(&mut css, "transition", &props.transition);
    add_prop(&mut css, "transform", &props.transform);
    add_prop(&mut css, "backdrop-filter", &props.backdrop_filter);
    add_prop(&mut css, "object-fit", &props.object_fit);

    if let Some(c) = resolved_color {
        css.push(format!("color: {};", c));
    }
    if let Some(b) = resolved_bg {
        css.push(format!("background-color: {};", b));
    }

    css.push("box-sizing: border-box;".to_string());
    css.push(format!("font-family: {}, sans-serif;", theme.font_family));

    let mut final_style = css.join(" ");
    if !props.style.is_empty() {
        final_style.push(' ');
        final_style.push_str(&props.style);
    }

    let class_name = props.class.clone();
    let children = props.children.clone();
    let id = props.id.clone();
    let onclick = props.onclick.clone();
    let ondblclick = props.ondblclick.clone();
    let onmousedown = props.onmousedown.clone();
    let onmouseup = props.onmouseup.clone();
    let onmouseenter = props.onmouseenter.clone();
    let onmouseleave = props.onmouseleave.clone();
    let onmousemove = props.onmousemove.clone();
    let onkeydown = props.onkeydown.clone();
    let onkeyup = props.onkeyup.clone();
    let onkeypress = props.onkeypress.clone();
    let onfocus = props.onfocus.clone();
    let onblur = props.onblur.clone();
    let onscroll = props.onscroll.clone();
    let oninput = props.oninput.clone();
    let onchange = props.onchange.clone();
    let ontouchstart = props.ontouchstart.clone();
    let ontouchend = props.ontouchend.clone();
    let ontouchmove = props.ontouchmove.clone();
    let ondrag = props.ondrag.clone();
    let ondragstart = props.ondragstart.clone();
    let ondragend = props.ondragend.clone();
    let ondragover = props.ondragover.clone();
    let ondragenter = props.ondragenter.clone();
    let ondragleave = props.ondragleave.clone();
    let ondrop = props.ondrop.clone();
    let node_ref = props.node_ref.clone();

    let tag = props.tag.as_str();

    let role_attr = if props.role.is_empty() { None } else { Some(props.role.clone()) };

    let aria_label_val = props.aria_label.clone();
    let aria_labelledby_val = props.aria_labelledby.clone();
    let aria_describedby_val = props.aria_describedby.clone();
    let aria_controls_val = props.aria_controls.clone();
    let aria_live_val = props.aria_live.clone();

    // For semantic HTML tags, add display role via CSS
    if tag != "div" && tag != "span" {
        match tag {
            "nav" => { css.push("display: block;".to_string()); }
            "section" | "article" | "aside" | "main" | "figure" | "figcaption" => {
                css.push("display: block;".to_string());
            }
            "ul" | "ol" => {
                css.push("display: block; list-style: none; padding: 0; margin: 0;".to_string());
            }
            "li" => { css.push("display: list-item;".to_string()); }
            _ => {}
        }
        // Recompute final_style
        final_style = css.join(" ");
        if !props.style.is_empty() {
            final_style.push(' ');
            final_style.push_str(&props.style);
        }
    }

    let component_override = theme.component_style("Box").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div
            ref={node_ref}
            id={id}
            class={yew::classes![class_name, &component_override]}
            style={final_style}
            role={role_attr}
            tabindex={props.tabindex.clone()}
            dir={props.dir.clone()}
            draggable={props.draggable.map(|d| if d { "true".to_string() } else { "false".to_string() })}
            onclick={onclick}
            ondblclick={ondblclick}
            onmousedown={onmousedown}
            onmouseup={onmouseup}
            onmouseenter={onmouseenter}
            onmouseleave={onmouseleave}
            onmousemove={onmousemove}
            onkeydown={onkeydown}
            onkeyup={onkeyup}
            onkeypress={onkeypress}
            onfocus={onfocus}
            onblur={onblur}
            onscroll={onscroll}
            oninput={oninput}
            onchange={onchange}
            ontouchstart={ontouchstart}
            ontouchend={ontouchend}
            ontouchmove={ontouchmove}
            ondrag={ondrag}
            ondragstart={ondragstart}
            ondragend={ondragend}
            ondragover={ondragover}
            ondragenter={ondragenter}
            ondragleave={ondragleave}
            ondrop={ondrop}
            aria-label={aria_label_val}
            aria-labelledby={aria_labelledby_val}
            aria-describedby={aria_describedby_val}
            aria-hidden={props.aria_hidden.map(|v| v.to_string())}
            aria-expanded={props.aria_expanded.map(|v| v.to_string())}
            aria-selected={props.aria_selected.map(|v| v.to_string())}
            aria-disabled={props.aria_disabled.map(|v| v.to_string())}
            aria-controls={aria_controls_val}
            aria-live={aria_live_val}
        >
            { children }
        </div>
    }
}
