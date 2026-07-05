//! Component style overrides for global theme customization.
//!
//! Allows users to customize any component's appearance globally via the theme,
//! similar to MUI's `createTheme({ components: { MuiButton: { styleOverrides: { ... } } } })`.
//!
//! # Example
//!
//! ```ignore
//! use material_rs::theme::{ThemeBuilder, ComponentOverrides};
//!
//! let theme = ThemeBuilder::new()
//!     .mode(ThemeMode::Light)
//!     .component("Button.root", "border-radius: 24px; font-weight: 700;".into())
//!     .component("Card.root", "border-radius: 16px; box-shadow: 0 4px 12px rgba(0,0,0,0.1);".into())
//!     .component("TextField.filled", "border-radius: 8px;".into())
//!     .build();
//! ```
//!
//! # Component Keys
//!
//! Keys follow the pattern `ComponentName.slot` where slot is optional:
//!
//! - `Button.root` / `Button.label` / `Button.icon`
//! - `Card.root` / `Card.content`
//! - `TextField.filled` / `TextField.outlined`
//! - `Switch.track` / `Switch.thumb`
//! - `Checkbox.box` / `Checkbox.label`
//! - `Chip.root` / `Chip.label`
//! - `Dialog.surface` / `Dialog.title` / `Dialog.content` / `Dialog.actions`
//! - `Menu.surface` / `Menu.item`
//! - `Snackbar.surface` / `Snackbar.message`
//! - `TabBar.indicator` / `TabBar.tab`
//! - `NavigationDrawer.item` / `NavigationBar.item`
//! - And more for every component...
//!
//! Use `theme.get_component_style("Button.root")` to retrieve overrides,
//! or the convenience `theme.component_css("Button")` to get the root CSS.

use std::collections::HashMap;

/// A map of component slot keys to CSS override strings.
///
/// Keys use the format `"ComponentName.slot"` (e.g., `"Button.root"`, `"Card.content"`).
/// Values are CSS strings that get appended to the component's default styles.
pub type ComponentStyles = HashMap<String, String>;

/// Get the CSS override for a specific component slot.
///
/// Returns the CSS string if an override exists, or `None`.
///
/// # Example
///
/// ```ignore
/// let css = theme.get_component_style("Button.root");
/// if let Some(custom_css) = css {
///     // Apply custom_css to the Button's root element
/// }
/// ```
pub fn get_component_style<'a>(styles: &'a ComponentStyles, key: &str) -> Option<&'a str> {
    styles.get(key).map(|s| s.as_str())
}
