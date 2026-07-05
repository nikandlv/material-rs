use yew::prelude::*;
use yew_router::prelude::*;
use material_rs::components::{Typography, TypographyVariant, Button, ButtonVariant};
use crate::routes::Route;

#[function_component]
pub fn NotFoundPage() -> Html {
    let navigator = use_navigator().unwrap();
    html! {
        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 60vh; gap: 16px;">
            <Typography tag="h1" variant={TypographyVariant::DisplayLarge}>{"404"}</Typography>
            <Typography tag="p" variant={TypographyVariant::BodyLarge}>{"Page not found"}</Typography>
            <Button label="Go Home" variant={ButtonVariant::Filled} onclick={Callback::from(move |_: MouseEvent| navigator.push(&Route::Landing))} />
        </div>
    }
}
