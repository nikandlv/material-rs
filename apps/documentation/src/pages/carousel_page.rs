use yew::prelude::*;
use material_rs::components::*;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn CarouselPage() -> Html {
    let carousel_props = vec![
        PropRow { name: "items".into(), r#type: "Vec<CarouselItem>".into(), default_value: "required".into(), description: "Items to display in the carousel.".into() },
        PropRow { name: "size".into(), r#type: "CarouselSize".into(), default_value: "Medium".into(), description: "Size variant: Small (132x80), Medium (282x180), or Large (328x240).".into() },
        PropRow { name: "show_indicators".into(), r#type: "bool".into(), default_value: "true".into(), description: "Show dot indicators below the carousel.".into() },
        PropRow { name: "auto_center".into(), r#type: "bool".into(), default_value: "true".into(), description: "Auto-scroll to center the active item after selection.".into() },
        PropRow { name: "on_select".into(), r#type: "Callback<String>".into(), default_value: "none".into(), description: "Callback fired when an item is clicked, passing the item key.".into() },
    ];

    let medium_items = vec![
        CarouselItem { key: "1".into(), children: html! { <span style="color: white; font-weight: 500;">{"Album 1"}</span> }, image: String::new() },
        CarouselItem { key: "2".into(), children: html! { <span style="color: white; font-weight: 500;">{"Album 2"}</span> }, image: String::new() },
        CarouselItem { key: "3".into(), children: html! { <span style="color: white; font-weight: 500;">{"Album 3"}</span> }, image: String::new() },
        CarouselItem { key: "4".into(), children: html! { <span style="color: white; font-weight: 500;">{"Album 4"}</span> }, image: String::new() },
        CarouselItem { key: "5".into(), children: html! { <span style="color: white; font-weight: 500;">{"Album 5"}</span> }, image: String::new() },
    ];

    let small_items = vec![
        CarouselItem { key: "1".into(), children: html! { <span style="color: white; font-size: 12px;">{"1"}</span> }, image: String::new() },
        CarouselItem { key: "2".into(), children: html! { <span style="color: white; font-size: 12px;">{"2"}</span> }, image: String::new() },
        CarouselItem { key: "3".into(), children: html! { <span style="color: white; font-size: 12px;">{"3"}</span> }, image: String::new() },
        CarouselItem { key: "4".into(), children: html! { <span style="color: white; font-size: 12px;">{"4"}</span> }, image: String::new() },
        CarouselItem { key: "5".into(), children: html! { <span style="color: white; font-size: 12px;">{"5"}</span> }, image: String::new() },
    ];

    let large_items = vec![
        CarouselItem { key: "1".into(), children: html! { <span style="color: white; font-weight: 600; font-size: 18px;">{"Featured 1"}</span> }, image: String::new() },
        CarouselItem { key: "2".into(), children: html! { <span style="color: white; font-weight: 600; font-size: 18px;">{"Featured 2"}</span> }, image: String::new() },
        CarouselItem { key: "3".into(), children: html! { <span style="color: white; font-weight: 600; font-size: 18px;">{"Featured 3"}</span> }, image: String::new() },
        CarouselItem { key: "4".into(), children: html! { <span style="color: white; font-weight: 600; font-size: 18px;">{"Featured 4"}</span> }, image: String::new() },
    ];

    html! {
        <>
            <Section title="Carousel">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"A horizontal scrolling container for browsing items with snap behavior, \
                      optional indicators, and dynamic color transitions. Supports small (compact), \
                      medium (card), and large (hero) container types."}
                </p>

                // ── Medium Carousel ──
                <Demo title="Medium (Default)">
                    <Carousel items={medium_items.clone()} />
                </Demo>
                <CodeBlock code={"let items = vec![\n    CarouselItem { key: \"1\".into(), children: html! { <span>{\"Album 1\"}</span> }, image: String::new() },\n    CarouselItem { key: \"2\".into(), children: html! { <span>{\"Album 2\"}</span> }, image: String::new() },\n    CarouselItem { key: \"3\".into(), children: html! { <span>{\"Album 3\"}</span> }, image: String::new() },\n];\n\n<Carousel items={items} />".to_string()} language={"rust".to_string()} />

                // ── Small Carousel ──
                <Demo title="Small">
                    <Carousel items={small_items} size={CarouselSize::Small} />
                </Demo>
                <CodeBlock code={"<Carousel items={items} size={CarouselSize::Small} />".to_string()} language={"rust".to_string()} />

                // ── Large Carousel ──
                <Demo title="Large">
                    <Carousel items={large_items} size={CarouselSize::Large} />
                </Demo>
                <CodeBlock code={"<Carousel items={items} size={CarouselSize::Large} />".to_string()} language={"rust".to_string()} />

                // ── With Indicators ──
                <Demo title="With Indicators (Default)">
                    <Carousel items={medium_items.clone()} show_indicators={true} />
                </Demo>
                <CodeBlock code={"<Carousel items={items} show_indicators={true} />".to_string()} language={"rust".to_string()} />

                // ── Without Indicators ──
                <Demo title="Without Indicators">
                    <Carousel items={medium_items} show_indicators={false} />
                </Demo>
                <CodeBlock code={"<Carousel items={items} show_indicators={false} />".to_string()} language={"rust".to_string()} />

                <PropTable rows={carousel_props} />
            </Section>
        </>
    }
}
