//! CSS class name and style utilities.

/// Merge multiple optional style strings into one, filtering empty values.
pub fn merge_styles(styles: &[&str]) -> String {
    styles
        .iter()
        .filter(|s| !s.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generate an inline `style` attribute string from key-value pairs.
///
/// # Example
///
/// ```ignore
/// let style = style_map! {
///     "display" => "flex",
///     "padding" => "8px",
///     "color" => &theme.colors.primary,
/// };
/// ```
#[macro_export]
macro_rules! style_map {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut parts = Vec::new();
        $(
            let val: &str = $value;
            if !val.is_empty() {
                parts.push(format!("{}: {}", $key, val));
            }
        )*
        parts.join("; ")
    }};
}

/// Combine two CSS class names with a space separator.
/// Returns `primary` if `secondary` is empty.
pub fn combine_classes(primary: &str, secondary: &str) -> String {
    if secondary.is_empty() {
        primary.to_string()
    } else {
        format!("{} {}", primary, secondary)
    }
}

/// Create a CSS `transition` value.
pub fn transition(property: &str, duration_ms: u32, easing: &str) -> String {
    format!("{} {}ms {}", property, duration_ms, easing)
}

/// Create a CSS `transition` for multiple properties.
pub fn transitions(pairs: &[(&str, u32, &str)]) -> String {
    pairs
        .iter()
        .map(|(prop, dur, ease)| format!("{} {}ms {}", prop, dur, ease))
        .collect::<Vec<_>>()
        .join(", ")
}