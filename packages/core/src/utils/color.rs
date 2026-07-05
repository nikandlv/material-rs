//! Color utility functions for manipulating hex colors.

/// Parse a hex color string ("#RRGGBB" or "#RGB") into (r, g, b) u8 values.
pub fn parse_hex(hex: &str) -> Option<(u8, u8, u8)> {
    let trimmed = hex.trim_start_matches('#');
    let (r, g, b) = match trimmed.len() {
        6 => {
            let r = u8::from_str_radix(&trimmed[0..2], 16).ok()?;
            let g = u8::from_str_radix(&trimmed[2..4], 16).ok()?;
            let b = u8::from_str_radix(&trimmed[4..6], 16).ok()?;
            (r, g, b)
        }
        3 => {
            let r = u8::from_str_radix(&trimmed[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&trimmed[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&trimmed[2..3].repeat(2), 16).ok()?;
            (r, g, b)
        }
        _ => return None,
    };
    Some((r, g, b))
}

/// Convert (r, g, b) to a hex color string.
pub fn to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

/// Blend two hex colors by a given ratio (0.0 = color1, 1.0 = color2).
pub fn blend(hex1: &str, hex2: &str, ratio: f64) -> Option<String> {
    let (r1, g1, b1) = parse_hex(hex1)?;
    let (r2, g2, b2) = parse_hex(hex2)?;
    let ratio = ratio.clamp(0.0, 1.0);

    let r = (r1 as f64 * (1.0 - ratio) + r2 as f64 * ratio).round() as u8;
    let g = (g1 as f64 * (1.0 - ratio) + g2 as f64 * ratio).round() as u8;
    let b = (b1 as f64 * (1.0 - ratio) + b2 as f64 * ratio).round() as u8;

    Some(to_hex(r, g, b))
}

/// Apply an alpha (opacity) to a hex color, returning an `rgba()` CSS string.
pub fn with_alpha(hex: &str, alpha: f64) -> Option<String> {
    let (r, g, b) = parse_hex(hex)?;
    let a = alpha.clamp(0.0, 1.0);
    Some(format!("rgba({}, {}, {}, {:.2})", r, g, b, a))
}

/// Calculate the relative luminance of a color (WCAG 2.1).
pub fn luminance(hex: &str) -> Option<f64> {
    let (r, g, b) = parse_hex(hex)?;
    let r = linearize(r);
    let g = linearize(g);
    let b = linearize(b);
    Some(0.2126 * r + 0.7152 * g + 0.0722 * b)
}

/// Determine if a color is "light" (luminance > 0.179).
pub fn is_light(hex: &str) -> bool {
    luminance(hex).is_none_or(|l| l > 0.179)
}

/// Lighten a hex color by a percentage (0.0–1.0).
pub fn lighten(hex: &str, amount: f64) -> Option<String> {
    blend(hex, "#FFFFFF", amount)
}

/// Darken a hex color by a percentage (0.0–1.0).
pub fn darken(hex: &str, amount: f64) -> Option<String> {
    blend(hex, "#000000", amount)
}

fn linearize(c: u8) -> f64 {
    let s = c as f64 / 255.0;
    if s <= 0.04045 {
        s / 12.92
    } else {
        ((s + 0.055) / 1.055).powf(2.4)
    }
}