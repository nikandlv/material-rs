//! Portal utility for rendering overlays into `document.body`.
//!
//! Non-persistent overlays (dialog, menu, modal drawer, snackbar, tooltip,
//! bottom sheet) should use portals so they render above all page content
//! and aren't clipped by `overflow: hidden` ancestors.

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use yew::prelude::*;

/// Portal target — a `<div>` appended to `document.body`.
///
/// Reuses the same container element across calls (keyed by `id`) so
/// multiple portals can share one root without leaking DOM nodes.
fn get_or_create_container(id: &str) -> HtmlElement {
    let document = web_sys::window().unwrap().document().unwrap();
    if let Some(existing) = document.get_element_by_id(id) {
        existing.unchecked_into()
    } else {
        let container = document.create_element("div").unwrap();
        container.set_id(id);
        container
            .unchecked_ref::<HtmlElement>()
            .style()
            .set_property("position", "fixed")
            .unwrap();
        container
            .unchecked_ref::<HtmlElement>()
            .style()
            .set_property("inset", "0")
            .unwrap();
        container
            .unchecked_ref::<HtmlElement>()
            .style()
            .set_property("pointer-events", "none")
            .unwrap();
        container
            .unchecked_ref::<HtmlElement>()
            .style()
            .set_property("z-index", "9999")
            .unwrap();
        document
            .body()
            .unwrap()
            .append_child(&container)
            .unwrap();
        container.unchecked_into()
    }
}

/// Renders `children` into a portal attached to `document.body`.
///
/// The portal container has `position: fixed; inset: 0` so children
/// can use `position: absolute` relative to the viewport.
///
/// # Example
///
/// ```ignore
/// html! {
///     <Portal id="my-overlay">
///         <Dialog open={true} title="Hello">
///             {"Content"}
///         </Dialog>
///     </Portal>
/// }
/// ```
#[function_component]
pub fn Portal(props: &PortalProps) -> Html {
    let container = use_state(|| -> Option<HtmlElement> { None });

    {
        let container = container.clone();
        let id = props.id.clone();
        use_effect_with(id.clone(), move |id| {
            let c = get_or_create_container(id);
            container.set(Some(c));
            || ()
        });
    }

    if let Some(target) = (*container).as_ref() {
        create_portal(props.children.clone().into(), target.clone().unchecked_into::<Element>())
    } else {
        html! {}
    }
}

#[derive(Properties, PartialEq)]
pub struct PortalProps {
    /// Unique ID for the portal container (reused across renders).
    pub id: String,

    /// Content to render into the portal.
    pub children: Children,
}
