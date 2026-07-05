use yew::prelude::*;
use material_rs::components::{ImageList, ImageListItem, ImageListVariant};
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn ImageListPage() -> Html {
    let item_props = vec![
        PropRow { name: "src".into(), r#type: "String".into(), default_value: "(required)".into(), description: "URL of the image to display.".into() },
        PropRow { name: "alt".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Alt text for the image (accessibility).".into() },
        PropRow { name: "caption".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Text displayed below the image when show_captions is true.".into() },
        PropRow { name: "header".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Text displayed as an overlay header when show_headers is true.".into() },
        PropRow { name: "cols".into(), r#type: "u32".into(), default_value: "1".into(), description: "Number of columns this item spans (used in Quilted variant).".into() },
        PropRow { name: "rows".into(), r#type: "u32".into(), default_value: "1".into(), description: "Number of rows this item spans (used in Quilted variant).".into() },
    ];

    let list_props = vec![
        PropRow { name: "items".into(), r#type: "Vec<ImageListItem>".into(), default_value: "(required)".into(), description: "Vector of image items to display.".into() },
        PropRow { name: "variant".into(), r#type: "ImageListVariant".into(), default_value: "Standard".into(), description: "Layout variant: Standard, Quilted, Woven, or Masonry.".into() },
        PropRow { name: "columns".into(), r#type: "u32".into(), default_value: "3".into(), description: "Number of columns in the grid. Masonry uses column-count, others use grid.".into() },
        PropRow { name: "gap".into(), r#type: "u32".into(), default_value: "4".into(), description: "Gap between items in pixels.".into() },
        PropRow { name: "show_captions".into(), r#type: "bool".into(), default_value: "false".into(), description: "Whether to display caption text below each image.".into() },
        PropRow { name: "show_headers".into(), r#type: "bool".into(), default_value: "false".into(), description: "Whether to display header overlay text on each image.".into() },
        PropRow { name: "standard_height".into(), r#type: "bool".into(), default_value: "false".into(), description: "When true, forces all items to a 4:3 aspect ratio.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    html! {
        <>
            <Section title="Image List / Gallery">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Image lists display a collection of images in a responsive grid. They support \
                      four layout variants — Standard (uniform grid), Quilted (variable span sizes), \
                      Woven (alternating column spans), and Masonry (staggered vertical columns). \
                      Each item can have captions and header overlays, with hover zoom and shadow effects."}
                </p>

                // ── Standard Grid with Captions ──
                <Demo title="Standard Grid with Captions">
                    <ImageList items={vec![
                        ImageListItem { src: "https://picsum.photos/seed/img1/400/400".into(), alt: "Mountain Landscape".into(), caption: "Mountain Landscape".into(), header: String::new(), cols: 1, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/img2/400/400".into(), alt: "Ocean Sunset".into(), caption: "Ocean Sunset".into(), header: String::new(), cols: 1, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/img3/400/400".into(), alt: "City Skyline".into(), caption: "City Skyline".into(), header: String::new(), cols: 1, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/img4/400/400".into(), alt: "Forest Trail".into(), caption: "Forest Trail".into(), header: String::new(), cols: 1, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/img5/400/400".into(), alt: "Desert Dunes".into(), caption: "Desert Dunes".into(), header: String::new(), cols: 1, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/img6/400/400".into(), alt: "Northern Lights".into(), caption: "Northern Lights".into(), header: String::new(), cols: 1, rows: 1 },
                    ]} columns={3} show_captions={true} />
                </Demo>
                <CodeBlock code={"<ImageList\n    items={vec![\n        ImageListItem { src: \"https://picsum.photos/seed/img1/400/400\".into(), alt: \"Mountain Landscape\".into(), caption: \"Mountain Landscape\".into(), header: String::new(), cols: 1, rows: 1 },\n        ImageListItem { src: \"https://picsum.photos/seed/img2/400/400\".into(), alt: \"Ocean Sunset\".into(), caption: \"Ocean Sunset\".into(), header: String::new(), cols: 1, rows: 1 },\n        // ... more items\n    ]}\n    columns={3}\n    show_captions={true}\n/>".to_string()} language={"rust".to_string()} />

                // ── Masonry Layout ──
                <Demo title="Masonry Layout">
                    <ImageList items={vec![
                        ImageListItem { src: "https://picsum.photos/seed/mas1/400/300".into(), alt: "Photo 1".into(), caption: String::new(), header: String::new(), cols: 1, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/mas2/400/500".into(), alt: "Photo 2".into(), caption: String::new(), header: String::new(), cols: 1, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/mas3/400/400".into(), alt: "Photo 3".into(), caption: String::new(), header: String::new(), cols: 1, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/mas4/400/350".into(), alt: "Photo 4".into(), caption: String::new(), header: String::new(), cols: 1, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/mas5/400/450".into(), alt: "Photo 5".into(), caption: String::new(), header: String::new(), cols: 1, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/mas6/400/380".into(), alt: "Photo 6".into(), caption: String::new(), header: String::new(), cols: 1, rows: 1 },
                    ]} variant={ImageListVariant::Masonry} columns={3} />
                </Demo>
                <CodeBlock code={"<ImageList\n    items={vec![\n        ImageListItem { src: \"https://picsum.photos/seed/mas1/400/300\".into(), alt: \"Photo 1\".into(), caption: String::new(), header: String::new(), cols: 1, rows: 1 },\n        // ... more items\n    ]}\n    variant={ImageListVariant::Masonry}\n    columns={3}\n/>".to_string()} language={"rust".to_string()} />

                // ── Quilted Layout ──
                <Demo title="Quilted Layout">
                    <ImageList items={vec![
                        ImageListItem { src: "https://picsum.photos/seed/qui1/400/400".into(), alt: "Photo 1".into(), caption: String::new(), header: String::new(), cols: 2, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/qui2/400/400".into(), alt: "Photo 2".into(), caption: String::new(), header: String::new(), cols: 1, rows: 2 },
                        ImageListItem { src: "https://picsum.photos/seed/qui3/400/400".into(), alt: "Photo 3".into(), caption: String::new(), header: String::new(), cols: 1, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/qui4/400/400".into(), alt: "Photo 4".into(), caption: String::new(), header: String::new(), cols: 1, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/qui5/400/400".into(), alt: "Photo 5".into(), caption: String::new(), header: String::new(), cols: 1, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/qui6/400/400".into(), alt: "Photo 6".into(), caption: String::new(), header: String::new(), cols: 1, rows: 1 },
                    ]} variant={ImageListVariant::Quilted} columns={3} />
                </Demo>
                <CodeBlock code={"<ImageList\n    items={vec![\n        ImageListItem { src: \"...\".into(), alt: \"Photo 1\".into(), caption: String::new(), header: String::new(), cols: 2, rows: 1 },\n        ImageListItem { src: \"...\".into(), alt: \"Photo 2\".into(), caption: String::new(), header: String::new(), cols: 1, rows: 2 },\n        // ... more items\n    ]}\n    variant={ImageListVariant::Quilted}\n    columns={3}\n/>".to_string()} language={"rust".to_string()} />

                // ── With Headers ──
                <Demo title="With Header Overlays">
                    <ImageList items={vec![
                        ImageListItem { src: "https://picsum.photos/seed/hdr1/400/400".into(), alt: "Photo 1".into(), caption: String::new(), header: "Travel".into(), cols: 1, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/hdr2/400/400".into(), alt: "Photo 2".into(), caption: String::new(), header: "Nature".into(), cols: 1, rows: 1 },
                        ImageListItem { src: "https://picsum.photos/seed/hdr3/400/400".into(), alt: "Photo 3".into(), caption: String::new(), header: "Architecture".into(), cols: 1, rows: 1 },
                    ]} columns={3} show_headers={true} />
                </Demo>
                <CodeBlock code={"<ImageList\n    items={vec![\n        ImageListItem { src: \"...\".into(), alt: \"Photo 1\".into(), caption: String::new(), header: \"Travel\".into(), cols: 1, rows: 1 },\n        // ... more items\n    ]}\n    columns={3}\n    show_headers={true}\n/>".to_string()} language={"rust".to_string()} />

                // ── Props Tables ──
                <PropTable rows={list_props} />
                <PropTable rows={item_props} />
            </Section>
        </>
    }
}
