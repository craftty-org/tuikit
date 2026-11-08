//! Progress bar helpers (style-agnostic).

/// Build a text progress bar of `width` cells for `ratio` (`0.0..=1.0`).
pub fn progress_bar_text(ratio: f64, width: usize, filled: char, empty: char) -> String {
    if width == 0 {
        return String::new();
    }
    let ratio = ratio.clamp(0.0, 1.0);
    let filled_n = (ratio * width as f64).round() as usize;
    let empty_n = width - filled_n.min(width);
    let mut out = String::with_capacity(width);
    for _ in 0..filled_n.min(width) {
        out.push(filled);
    }
    for _ in 0..empty_n {
        out.push(empty);
    }
    out
}

/// Clamp helper for ratio values.
pub fn clamp_ratio(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bar_ratios() {
        assert_eq!(progress_bar_text(0.0, 4, '#', '-'), "----");
        assert_eq!(progress_bar_text(1.0, 4, '#', '-'), "####");
        assert_eq!(progress_bar_text(0.5, 4, '#', '-').chars().count(), 4);
    }
}
