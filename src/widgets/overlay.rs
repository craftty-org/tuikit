//! Overlay helpers (dim scrim + centered popup rects).

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

/// Center a `width`×`height` rect inside `area` (clamped to the area).
pub fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let w = width.min(area.width);
    let h = height.min(area.height);
    Rect {
        x: area.x + area.width.saturating_sub(w) / 2,
        y: area.y + area.height.saturating_sub(h) / 2,
        width: w,
        height: h,
    }
}

/// Center a percent-sized rect inside `area`.
pub fn centered_percent(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);
    horizontal[1]
}

/// Dim all cells in `area` by multiplying RGB channels by `factor`.
pub fn dim_region(buf: &mut ratatui::buffer::Buffer, area: Rect, factor: f32) {
    let f = factor.clamp(0.0, 1.0);
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_fg(dim_color(cell.fg, f));
                cell.set_bg(dim_color(cell.bg, f));
            }
        }
    }
}

fn dim_color(c: Color, factor: f32) -> Color {
    match c {
        Color::Rgb(r, g, b) => Color::Rgb(
            (r as f32 * factor) as u8,
            (g as f32 * factor) as u8,
            (b as f32 * factor) as u8,
        ),
        other => other,
    }
}

/// Render simple centered label lines (used as a base for popups/loaders).
pub fn render_centered_text(frame: &mut ratatui::Frame<'_>, area: Rect, lines: &[String], style: Style) {
    if area.height == 0 || area.width == 0 {
        return;
    }
    let start_y = area.y + area.height.saturating_sub(lines.len() as u16) / 2;
    for (i, line) in lines.iter().enumerate() {
        let y = start_y + i as u16;
        if y >= area.bottom() {
            break;
        }
        let para = Paragraph::new(Line::from(Span::styled(line.clone(), style)))
            .alignment(Alignment::Center);
        frame.render_widget(
            para,
            Rect { y, height: 1, ..area },
        );
    }
}

/// Convenience: render a small centered popup panel text block with a title.
pub fn render_popup_text(
    frame: &mut ratatui::Frame<'_>,
    area: Rect,
    title: &str,
    body: &[String],
    title_style: Style,
    body_style: Style,
) {
    if area.height == 0 || area.width == 0 {
        return;
    }
    if !title.is_empty() {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(title.to_string(), title_style)))
                .alignment(Alignment::Center),
            Rect { height: 1, ..area },
        );
    }
    let body_y = area.y + if title.is_empty() { 0 } else { 1 };
    let body_area = Rect {
        y: body_y,
        height: area.height.saturating_sub(if title.is_empty() { 0 } else { 1 }),
        ..area
    };
    let _ = body_style;
    let _ = Modifier::empty(); // keep Modifier import simple if style already carries it
    render_centered_text(frame, body_area, body, body_style);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centers_rects() {
        let area = Rect { x: 0, y: 0, width: 100, height: 40 };
        let r = centered_rect(20, 10, area);
        assert_eq!((r.width, r.height), (20, 10));
        assert!(r.x > 0 && r.y > 0);
        let p = centered_percent(60, 50, area);
        assert!(p.width > 0 && p.height > 0 && p.width <= area.width && p.height <= area.height);
    }
}
