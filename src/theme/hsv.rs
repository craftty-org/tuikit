//! HSV conversion and hue utilities.

use super::color::Rgb8;

/// Convert RGB to HSV (`h` in degrees `[0, 360)`, `s`/`v` in `[0, 1]`).
pub fn rgb_to_hsv(c: Rgb8) -> [f32; 3] {
    let r = c[0] as f32 / 255.0;
    let g = c[1] as f32 / 255.0;
    let b = c[2] as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    let h = if d <= 0.0 {
        0.0
    } else if (max - r).abs() < f32::EPSILON {
        ((g - b) / d).rem_euclid(6.0)
    } else if (max - g).abs() < f32::EPSILON {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    } * 60.0;
    let s = if max <= 0.0 { 0.0 } else { d / max };
    [h, s, max]
}

/// Convert HSV to RGB (`h` in degrees).
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Rgb8 {
    let h = h.rem_euclid(360.0);
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    [
        ((r + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((g + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((b + m) * 255.0).round().clamp(0.0, 255.0) as u8,
    ]
}

/// Circular hue distance in degrees (`[0, 180]`).
pub fn hue_distance(a: f32, b: f32) -> f32 {
    let d = (a - b).abs().rem_euclid(360.0);
    if d > 180.0 { 360.0 - d } else { d }
}

/// Shift hue by `turns` of the color wheel (1.0 = 360°), preserving saturation floor and
/// slightly lifting value for visibility.
pub fn shift_hue(c: Rgb8, turns: f32) -> Rgb8 {
    let [h, s, v] = rgb_to_hsv(c);
    hsv_to_rgb(h + 360.0 * turns, s.max(0.25), (v * 1.15).min(1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsv_roundtrip_basic() {
        let c = [10, 200, 30];
        let [h, s, v] = rgb_to_hsv(c);
        let back = hsv_to_rgb(h, s, v);
        // Allow tiny rounding differences.
        for i in 0..3 {
            assert!((c[i] as i16 - back[i] as i16).abs() <= 1);
        }
    }
}
