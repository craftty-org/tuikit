//! Palette extraction from image bytes.

use super::color::{to_hex_rgb, Rgb8};
use super::hsv::{hue_distance, rgb_to_hsv, shift_hue};

/// A small extracted palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette {
    pub primary: Rgb8,
    pub secondary: Rgb8,
    pub tertiary: Rgb8,
}

/// Extract a palette from encoded image bytes supported by the `image` crate.
/// Returns `None` when decoding fails or no usable colors are found.
pub fn extract_palette(bytes: &[u8]) -> Option<Palette> {
    let img = image::load_from_memory(bytes).ok()?;
    let small = img.resize(64, 64, image::imageops::FilterType::Triangle);
    let rgba = small.to_rgba8();

    let mut pixels = Vec::with_capacity(rgba.width() as usize * rgba.height() as usize * 3);
    for px in rgba.pixels() {
        if px.0[3] > 32 {
            pixels.extend_from_slice(&px.0[..3]);
        }
    }
    if pixels.len() < 3 {
        return None;
    }

    let palette = match color_thief::get_palette(&pixels, color_thief::ColorFormat::Rgb, 10, 8) {
        Ok(p) if !p.is_empty() => p,
        _ => return None,
    };

    let rgb = |c: &color_thief::Color| [c.r, c.g, c.b];
    let primary = rgb(&palette[0]);
    let primary_hsv = rgb_to_hsv(primary);
    let mut secondary = None;
    let mut tertiary = None;
    let mut best_score = -1.0f32;
    for c in palette.iter().skip(1) {
        let v = rgb(c);
        let hsv = rgb_to_hsv(v);
        let score = hue_distance(primary_hsv[0], hsv[0]) + 2.0 * hsv[1];
        if score > best_score && hsv[1] > 0.12 {
            best_score = score;
            tertiary = secondary.take();
            secondary = Some(v);
        } else if tertiary.is_none() {
            tertiary = Some(v);
        }
    }

    let fallback = |c: Rgb8| {
        let shifted = shift_hue(c, 0.33);
        [
            ((c[0] as f32 + shifted[0] as f32) / 2.0) as u8,
            ((c[1] as f32 + shifted[1] as f32) / 2.0) as u8,
            ((c[2] as f32 + shifted[2] as f32) / 2.0) as u8,
        ]
    };
    let secondary = secondary.unwrap_or_else(|| fallback(primary));
    let tertiary = tertiary.unwrap_or_else(|| fallback(secondary));
    Some(Palette {
        primary,
        secondary,
        tertiary,
    })
}

/// Convenience: extract palette and format as `#rrggbb` strings.
pub fn extract_palette_hex(bytes: &[u8]) -> Option<(String, String, String)> {
    extract_palette(bytes).map(|p| (to_hex_rgb(p.primary), to_hex_rgb(p.secondary), to_hex_rgb(p.tertiary)))
}
