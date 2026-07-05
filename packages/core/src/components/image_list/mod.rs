//! Material Design 3 Expressive Image List / Gallery
//!
//! A responsive grid of images supporting standard, quilted, woven, and masonry layouts.
//! Follows MD3 expressive design with hover zoom, gap animations, and overlay support.

use stylist::yew::use_style;
use yew::prelude::*;

use crate::theme::Theme;
use crate::utils::dynamic_style::dynamic_style;

/// Layout variant for the image list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ImageListVariant {
    #[default]
    Standard,
    Quilted,
    Woven,
    Masonry,
}

/// A single item in the image list.
#[derive(Clone, PartialEq)]
pub struct ImageListItem {
    pub src: String,
    pub alt: String,
    pub caption: String,
    pub header: String,
    pub cols: u32,
    pub rows: u32,
}

#[derive(Properties, PartialEq)]
pub struct ImageListProps {
    pub items: Vec<ImageListItem>,

    #[prop_or_default]
    pub variant: ImageListVariant,

    #[prop_or(3)]
    pub columns: u32,

    #[prop_or(4)]
    pub gap: u32,

    #[prop_or(false)]
    pub show_captions: bool,

    #[prop_or(false)]
    pub show_headers: bool,

    #[prop_or(false)]
    pub standard_height: bool,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn ImageList(props: &ImageListProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let grid_style = use_style!(
        r#"
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
        gap: ${gap}px;
        width: 100%;

        @media (max-width: 600px) {
            grid-template-columns: repeat(2, 1fr);
        }

        @media (max-width: 380px) {
            grid-template-columns: 1fr;
        }
        "#,
        gap = props.gap,
    );

    let masonry_style = use_style!(
        r#"
        column-count: ${columns};
        column-gap: ${gap}px;
        width: 100%;

        @media (max-width: 900px) {
            column-count: 2;
        }

        @media (max-width: 500px) {
            column-count: 1;
        }
        "#,
        columns = props.columns,
        gap = props.gap,
    );

    let container_class = if props.variant == ImageListVariant::Masonry {
        masonry_style.get_class_name().to_string()
    } else {
        grid_style.get_class_name().to_string()
    };

    let item_style = use_style!(
        r#"
        position: relative;
        overflow: hidden;
        border-radius: ${radius};
        cursor: pointer;
        transition: box-shadow 200ms cubic-bezier(0.2, 0, 0, 1);
        isolation: isolate;

        &:hover {
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
            z-index: 2;
        }

        &:hover .image-overlay {
            opacity: 1;
        }

        &:hover img {
            transform: scale(1.05);
        }
        "#,
        radius = theme.shapes.small,
    );

    let wrapper_style = use_style!(
        r#"
        width: 100%;
        font-family: ${font_family}, sans-serif;
        "#,
        font_family = theme.font_family,
    );

    let img_style = use_style!(
        r#"
        width: 100%;
        height: 100%;
        display: block;
        object-fit: cover;
        transition: transform 200ms cubic-bezier(0.2, 0, 0, 1);
        "#,
    );

    let header_overlay_style = use_style!(
        r#"
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        padding: 8px 12px;
        background: linear-gradient(to bottom, rgba(0,0,0,0.6), transparent);
        opacity: 0;
        transition: opacity 200ms;
        pointer-events: none;
        "#,
    );

    let header_text_style = use_style!(
        r#"
        color: white;
        font-size: 14px;
        font-weight: 500;
        text-shadow: 0 1px 2px rgba(0,0,0,0.3);
        "#,
    );

    let caption_style = use_style!(
        r#"
        padding: 6px 2px 2px;
        font-size: 12px;
        line-height: 16px;
        font-weight: 500;
        letter-spacing: 0.4px;
        "#,
    );

    let component_override = theme.component_style("ImageList").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    let items_html = props.items.iter().enumerate().map(|(idx, item)| {
        let grid_col = match props.variant {
            ImageListVariant::Quilted => {
                if item.cols > 1 || item.rows > 1 {
                    format!("grid-column: span {}; grid-row: span {};", item.cols, item.rows)
                } else {
                    String::new()
                }
            }
            ImageListVariant::Woven => {
                if idx % 5 == 0 || idx % 7 == 0 {
                    "grid-column: span 2;".into()
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        };

        let aspect_style = if props.standard_height {
            "aspect-ratio: 4 / 3;".into()
        } else if props.variant == ImageListVariant::Masonry {
            let ratios = ["3/4", "4/3", "1/1", "3/4", "4/5"];
            format!("aspect-ratio: {};", ratios[idx % ratios.len()])
        } else {
            "aspect-ratio: 1 / 1;".into()
        };

        let masonry_item_style = if props.variant == ImageListVariant::Masonry {
            "break-inside: avoid; margin-bottom: 4px;".to_string()
        } else {
            String::new()
        };

        let item_dynamic_class = dynamic_style(format!("{} {}", grid_col, masonry_item_style));
        let aspect_dynamic_class = dynamic_style(aspect_style.to_string());

        html! {
            <div key={idx}>
                <div
                    class={yew::classes![item_style.get_class_name().to_string(), item_dynamic_class]}
                >
                    <div class={aspect_dynamic_class}>
                        <img
                            src={item.src.clone()}
                            alt={item.alt.clone()}
                            loading="lazy"
                            class={img_style.get_class_name().to_string()}
                        />
                    </div>

                    if props.show_headers && !item.header.is_empty() {
                        <div class={yew::classes!["image-overlay", header_overlay_style.get_class_name().to_string()]}>
                            <span class={header_text_style.get_class_name().to_string()}>
                                { &item.header }
                            </span>
                        </div>
                    }
                </div>

                if props.show_captions && !item.caption.is_empty() {
                    <div class={yew::classes![caption_style.get_class_name().to_string(), dynamic_style(format!("color: {};", theme.colors.on_surface_variant))]}>
                        { &item.caption }
                    </div>
                }
            </div>
        }
    });

    html! {
        <div
            class={yew::classes![wrapper_style.get_class_name().to_string(), container_class, &props.class, &component_override]}
            id={props.id.clone()}
            role="list"
        >
            { for items_html }
        </div>
    }
}
