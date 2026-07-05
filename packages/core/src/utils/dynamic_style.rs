//! Dynamic style utility for creating runtime CSS with `stylist::Style`.
//!
//! Unlike `use_style!` which only accepts static strings, this function
//! accepts dynamic `String` values interpolated into CSS, making it
//! perfect for theme-dependent colors, computed sizes, and conditional styles.
//!
//! # Example
//!
//! ```ignore
//! let bg = theme.colors.primary.clone();
//! let class = dynamic_style(format!(
//!     "background-color: {}; padding: {}px; border-radius: {}px;",
//!     bg, padding, radius
//! ));
//!
//! html! { <div class={class}>{"Hello"}</div> }
//! ```

use stylist::Style;

/// Takes a CSS string and returns a class name ready to use.
///
/// Unlike `use_style!` (static only), this works with dynamic `String` values
/// like theme colors, computed dimensions, or conditional CSS.
///
/// Returns the `stylist` class name string (e.g., `"stylist-xyz123"`).
/// Returns an empty string if the CSS cannot be parsed.
pub fn dynamic_style(css: String) -> String {
    match Style::new(css.clone()) {
        Ok(style) => style.get_class_name().to_string(),
        Err(_) => {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::warn_1(
                &format!(
                    "dynamic_style: invalid CSS ignored: {}",
                    &css[..css.len().min(80)]
                )
                .into(),
            );
            String::new()
        }
    }
}
