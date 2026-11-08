//! Small text normalization and formatting helpers.

/// Collapse whitespace (ASCII/Unicode whitespace via `split_whitespace`) into single spaces.
pub fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A pragmatic HTML-tag stripper.
///
/// This removes content between `<` and `>` and decodes a few common entities. It is **not** a
/// full HTML parser and should not be used for security-sensitive sanitization.
pub fn strip_html_basic(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    let out = out
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'");
    normalize_whitespace(&out)
}

/// Truncate a string to `max_chars` Unicode scalar values, appending `suffix` if truncated.
pub fn truncate_chars(text: &str, max_chars: usize, suffix: &str) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let keep = max_chars.saturating_sub(suffix.chars().count());
    let mut out = text.chars().take(keep).collect::<String>();
    out.push_str(suffix);
    out
}

/// Format a count in a compact human-readable form (`1.2k`, `3.4M`).
pub fn format_compact_count(value: u64) -> String {
    match value {
        0..=999 => value.to_string(),
        1_000..=999_999 => format!("{:.1}k", value as f64 / 1_000.0),
        _ => format!("{:.1}M", value as f64 / 1_000_000.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_basic_html() {
        assert_eq!(strip_html_basic("<b>Hello</b> &amp; world"), "Hello & world");
    }

    #[test]
    fn truncates_by_chars() {
        assert_eq!(truncate_chars("abcdef", 4, "…"), "abc…");
        assert_eq!(truncate_chars("abc", 4, "…"), "abc");
    }

    #[test]
    fn formats_counts() {
        assert_eq!(format_compact_count(999), "999");
        assert_eq!(format_compact_count(1_234), "1.2k");
    }
}
