//! Basic RGB color parsing and math.

/// 8-bit RGB color.
pub type Rgb8 = [u8; 3];

/// Parse a `#rrggbb` (or `rrggbb`) hex string into an RGB triple.
pub fn parse_hex_rgb(s: &str) -> Option<Rgb8> {
    let h = s.trim().strip_prefix('#').unwrap_or(s.trim());
    if h.len() != 6 {
        return None;
    }
    let mut out = [0u8; 3];
    for (i, slot) in out.iter_mut().enumerate() {
        *slot = u8::from_str_radix(&h[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(out)
}

/// Format an RGB triple as `#rrggbb`.
pub fn to_hex_rgb(c: Rgb8) -> String {
    format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2])
}

/// Linear blend of `a` toward `b` by `t` (`0.0..=1.0`).
pub fn blend_rgb(a: Rgb8, b: Rgb8, t: f64) -> Rgb8 {
    let t = t.clamp(0.0, 1.0);
    let mix = |x: u8, y: u8| (x as f64 + (y as f64 - x as f64) * t).round() as u8;
    [mix(a[0], b[0]), mix(a[1], b[1]), mix(a[2], b[2])]
}

/// Relative luminance (sRGB), used by WCAG contrast calculations.
pub fn relative_luminance(c: Rgb8) -> f64 {
    let chan = |v: u8| {
        let v = v as f64 / 255.0;
        if v <= 0.03928 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * chan(c[0]) + 0.7152 * chan(c[1]) + 0.0722 * chan(c[2])
}

/// WCAG 2.1 contrast ratio between two RGB colors.
pub fn contrast_ratio(a: Rgb8, b: Rgb8) -> f64 {
    let (la, lb) = (relative_luminance(a), relative_luminance(b));
    let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hex() {
        assert_eq!(parse_hex_rgb("#7aa2f7"), Some([0x7a, 0xa2, 0xf7]));
        assert_eq!(parse_hex_rgb("7aa2f7"), Some([0x7a, 0xa2, 0xf7]));
        assert_eq!(parse_hex_rgb("nope"), None);
    }

    #[test]
    fn roundtrips_hex() {
        let c = [1, 2, 3];
        assert_eq!(parse_hex_rgb(&to_hex_rgb(c)), Some(c));
    }
}
