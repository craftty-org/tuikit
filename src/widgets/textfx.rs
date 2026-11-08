//! Text helpers for TUI rendering.

/// Braille spinner frames (10-frame cycle).
pub const SPINNER_FRAMES: [&str; 10] = [
    "\u{280b}", "\u{2819}", "\u{2839}", "\u{2838}", "\u{283c}",
    "\u{2834}", "\u{2826}", "\u{2827}", "\u{2807}", "\u{280f}",
];

/// Return the spinner glyph for `frame`.
pub fn spinner_glyph(frame: usize) -> &'static str {
    SPINNER_FRAMES[(frame / 2) % SPINNER_FRAMES.len()]
}

/// Cubic ease-out (`t` in `0..=1`).
pub fn cubic_ease_out(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

/// Cubic ease-in (`t` in `0..=1`).
pub fn cubic_ease_in(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * t
}

/// Word-wrap text to `max_chars`, preserving input words.
///
/// Long words are kept intact (a word longer than `max_chars` stays on its own line).
pub fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    if max_chars == 0 {
        return vec![text.to_string()];
    }
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split(' ') {
        if current.len() + word.len() + 1 > max_chars && !current.is_empty() {
            lines.push(std::mem::take(&mut current));
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

/// Amount of right padding to reach `width` columns.
pub fn row_pad(content: &str, width: u16) -> usize {
    (width as usize).saturating_sub(content.chars().count())
}

/// Marquee a long single-line label when `active`, otherwise truncate.
///
/// If the text fits, it is padded to `max_width` (consistent cell width).
/// If it does not fit and is not active, it truncates with `…`.
/// When active, it cycles through offsets and holds briefly at the start before looping.
pub fn scroll_label(text: &str, max_width: usize, frame: usize, active: bool) -> String {
    scroll_text(text, max_width, frame, active)
}

/// Build a scrolling label with configurable hold/speed behavior.
pub fn scroll_text(text: &str, max_width: usize, frame: usize, active: bool) -> String {
    if max_width == 0 {
        return String::new();
    }
    if text.chars().count() <= max_width {
        return format!("{:<width$}", text, width = max_width);
    }
    if !active {
        let truncated: String = text.chars().take(max_width.saturating_sub(1)).collect();
        return format!("{truncated}…");
    }
    const HOLD_FRAMES: usize = 300;
    const SPEED: usize = 6;
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let hold_steps = HOLD_FRAMES / SPEED;
    let step = frame / SPEED;
    let pos = step % (n + hold_steps).max(1);
    let offset = if pos < n { pos } else { 0 };
    let scrolled: String = chars.iter().skip(offset).chain(chars.iter().take(offset)).collect();
    scrolled.chars().take(max_width).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scroll_behaves() {
        assert_eq!(scroll_text("ab", 4, 0, true), "ab  ");
        let out = scroll_text("a long label that scrolls", 10, 60, true);
        assert!(out.chars().count() <= 10);
        let out = scroll_text("a long label that scrolls", 10, 0, false);
        assert!(out.chars().count() <= 10);
    }

    #[test]
    fn wrap_keeps_lines() {
        assert_eq!(wrap_text("hello world", 5), vec!["hello", "world"]);
    }
}
