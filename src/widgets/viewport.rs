//! Viewport and list-row mapping helpers.

/// Keep `selected` visible within a window of `visible` rows starting at `offset`.
pub fn step_viewport(offset: usize, selected: usize, visible: usize, total: usize) -> (usize, usize) {
    if total <= visible || visible == 0 {
        return (0, total);
    }
    let max_start = total - visible;
    let mut o = offset.min(max_start);
    if selected < o {
        o = selected;
    } else if selected >= o + visible {
        o = (selected + 1).saturating_sub(visible).min(max_start);
    }
    (o, (o + visible).min(total))
}

/// A vertical list window with optional fixed prefix rows (cursor-space aware).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListRows {
    /// Fixed rows above scrolling items (counted in cursor space).
    pub prefix_rows: usize,
    /// First scrolling item's absolute index.
    pub offset: usize,
    /// Number of scrolling items available.
    pub len: usize,
}

impl ListRows {
    pub fn new(prefix_rows: usize, offset: usize, len: usize) -> Self {
        Self { prefix_rows, offset, len }
    }

    /// `(screen y, absolute item index)` for rows that fit.
    pub fn entries(self, first_y: u16, capacity: usize) -> Vec<(u16, usize)> {
        let actions = self.prefix_rows.min(capacity);
        let tracks = self.len.min(capacity.saturating_sub(actions));
        let mut rows = Vec::with_capacity(actions + tracks);
        for i in 0..actions {
            rows.push((first_y.saturating_add(i as u16), i));
        }
        for i in 0..tracks {
            rows.push((
                first_y.saturating_add((actions + i) as u16),
                self.prefix_rows + self.offset + i,
            ));
        }
        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_keeps_selected_visible() {
        assert_eq!(step_viewport(0, 5, 3, 10), (3, 6));
        assert_eq!(step_viewport(8, 2, 3, 10), (2, 5));
    }

    #[test]
    fn rows_map_prefix_and_items() {
        let rows = ListRows::new(2, 5, 3).entries(10, 20);
        assert_eq!(rows, vec![(10, 0), (11, 1), (12, 7), (13, 8), (14, 9)]);
        let tight = ListRows::new(2, 0, 5).entries(3, 3);
        assert_eq!(tight, vec![(3, 0), (4, 1), (5, 2)]);
    }
}
