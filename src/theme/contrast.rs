//! WCAG-oriented foreground selection helpers.

use super::color::{blend_rgb, contrast_ratio, relative_luminance, Rgb8};

const MIN_LEGIBLE_RATIO: f64 = 4.5;
const MIN_DISTINCT_RATIO: f64 = 2.0;

/// Choose a foreground color that is legible on `background`.
///
/// If `preferred` already meets WCAG AA contrast for normal text, it is preserved. Otherwise a
/// near-black or near-white fallback is selected.
pub fn readable_foreground(preferred: Rgb8, background: Rgb8) -> Rgb8 {
    if contrast_ratio(preferred, background) >= MIN_LEGIBLE_RATIO {
        return preferred;
    }
    let black = [0u8, 0, 0];
    let white = [255u8, 255, 255];
    let (black_ratio, white_ratio) = (
        contrast_ratio(black, background),
        contrast_ratio(white, background),
    );
    if black_ratio >= MIN_LEGIBLE_RATIO && black_ratio <= white_ratio {
        black
    } else if white_ratio >= MIN_LEGIBLE_RATIO {
        white
    } else if black_ratio >= white_ratio {
        black
    } else {
        white
    }
}

/// Choose a foreground that stands out as a highlight on `background`.
///
/// Preserves `preferred` when it is legible and sufficiently distinct. If legible but too close,
/// nudges it away from the background along the luminance axis to preserve hue where possible.
pub fn standout_foreground(preferred: Rgb8, background: Rgb8) -> Rgb8 {
    let ratio = contrast_ratio(preferred, background);
    if ratio < MIN_LEGIBLE_RATIO {
        return readable_foreground(preferred, background);
    }
    if ratio >= MIN_DISTINCT_RATIO {
        return preferred;
    }
    let towards_light = relative_luminance(preferred) >= relative_luminance(background);
    let target = if towards_light { [255, 255, 255] } else { [0, 0, 0] };
    for step in 1..=16u32 {
        let t = f64::from(step) / 16.0;
        let candidate = blend_rgb(preferred, target, t);
        if contrast_ratio(candidate, background) >= MIN_DISTINCT_RATIO {
            return candidate;
        }
    }
    target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_contrasting_foreground() {
        let fg = [0xcd, 0xd6, 0xf4];
        let bg = [0x1f, 0x23, 0x35];
        assert_eq!(readable_foreground(fg, bg), fg);
    }

    #[test]
    fn corrects_low_contrast() {
        let same = [0xe6, 0xe6, 0xe6];
        assert!(contrast_ratio(readable_foreground(same, same), same) >= 4.5);
    }
}
