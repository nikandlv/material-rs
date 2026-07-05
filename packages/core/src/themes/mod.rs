use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    // Structural layout elements
    pub font_family: String,
    pub background: String,
    pub on_background: String,

    // Primary Interactive Element Tokens (like your primary button)
    pub primary: String,
    pub on_primary: String,

    // Secondary / Surface elements
    pub surface: String,
    pub on_surface: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub theme: Theme,
}

#[component]
pub fn MaterialThemeProvider(props: &Props) -> Html {
    html! {
        <ContextProvider<Theme> context={props.theme.clone()}>
            {props.children.clone()}
        </ContextProvider<Theme>>
    }
}

pub fn default_theme() -> Theme {
    Theme {
        font_family: "Roboto, system-ui".to_owned(),

        // Classic M3 Baseline Tokens (Purple theme baseline)
        background: "#FEF7FF".to_owned(),
        on_background: "#1D1B20".to_owned(),

        primary: "#6750A4".to_owned(), // Deep purple button background
        on_primary: "#FFFFFF".to_owned(), // White text inside primary buttons

        surface: "#F7F2FA".to_owned(),
        on_surface: "#1D1B20".to_owned(),
    }
}
