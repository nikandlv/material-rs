//! # material-rs
//!
//! A Material Design 3 UI component library for Rust/Yew.
//!
//! ## Quick Start
//!
//! ```ignore
//! use material_rs::theme::{MaterialThemeProvider, default_light_theme};
//! use material_rs::components::Button;
//!
//! #[component]
//! fn App() -> Html {
//!     html! {
//!         <MaterialThemeProvider theme={default_light_theme()}>
//!             <Button label="Click me" />
//!         </MaterialThemeProvider>
//!     }
//! }
//! ```
//!
//! ## Module Structure
//!
//! - **`theme`** — Full MD3 design token system (color, typography, elevation, shape, motion)
//! - **`components`** — All MD3 UI components
//! - **`utils`** — Color manipulation, CSS class helpers, accessibility utilities

pub mod components;
pub mod theme;
pub mod utils;

// Convenience re-exports
pub use theme::{
    MaterialThemeProvider, Theme, ThemeMode, ThemeBuilder, Direction,
    default_light_theme, default_dark_theme, default_theme,
};