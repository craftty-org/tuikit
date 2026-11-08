//! Single-line text input state.

use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum InputAction {
    #[default]
    None,
    Changed,
    Submitted,
    Cancelled,
}

/// Generic single-line input with cursor and horizontal scrolling.
#[derive(Debug, Clone, Default)]
pub struct InputState {
    pub value: String,
    pub cursor: usize, // byte index into `value`
}

impl InputState {
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        let cursor = value.len();
        Self { value, cursor }
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor = 0;
    }

    /// Handle a key event. Callers should map `InputAction` to app behavior.
    pub fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers, kind: KeyEventKind) -> InputAction {
        if !matches!(kind, KeyEventKind::Press | KeyEventKind::Repeat) {
            return InputAction::None;
        }
        match code {
            KeyCode::Enter => InputAction::Submitted,
            KeyCode::Esc => InputAction::Cancelled,
            KeyCode::Backspace => {
                if self.cursor > 0 && !self.value.is_empty() {
                    // Move to previous char boundary.
                    let prev = prev_boundary(&self.value, self.cursor);
                    self.value.replace_range(prev..self.cursor, "");
                    self.cursor = prev;
                    InputAction::Changed
                } else {
                    InputAction::None
                }
            }
            KeyCode::Delete => {
                if self.cursor < self.value.len() {
                    let next = next_boundary(&self.value, self.cursor);
                    self.value.replace_range(self.cursor..next, "");
                    InputAction::Changed
                } else {
                    InputAction::None
                }
            }
            KeyCode::Left => {
                self.cursor = prev_boundary(&self.value, self.cursor);
                InputAction::None
            }
            KeyCode::Right => {
                self.cursor = next_boundary(&self.value, self.cursor);
                InputAction::None
            }
            KeyCode::Home => {
                self.cursor = 0;
                InputAction::None
            }
            KeyCode::End => {
                self.cursor = self.value.len();
                InputAction::None
            }
            KeyCode::Char('u') if modifiers.contains(KeyModifiers::CONTROL) => {
                self.clear();
                InputAction::Changed
            }
            KeyCode::Char('w') if modifiers.contains(KeyModifiers::CONTROL) => {
                if self.cursor == 0 {
                    return InputAction::None;
                }
                let before_owned = self.value[..self.cursor].to_string();
                let trimmed = before_owned.trim_end().to_string();
                self.value.replace_range(..self.cursor, &trimmed);
                self.cursor = trimmed.len();
                InputAction::Changed
            }
            KeyCode::Char(c) if !modifiers.contains(KeyModifiers::CONTROL) && !modifiers.contains(KeyModifiers::ALT) => {
                self.value.insert(self.cursor, c);
                self.cursor += c.len_utf8();
                InputAction::Changed
            }
            _ => InputAction::None,
        }
    }

    /// Visible slice for a viewport of `width` columns (chooses a window around the cursor).
    pub fn visible(&self, width: usize) -> (String, usize) {
        if width == 0 || self.value.is_empty() {
            return (String::new(), 0);
        }
        // Operate on chars for simpler width handling.
        let chars: Vec<char> = self.value.chars().collect();
        let cursor_chars = self.value[..self.cursor].chars().count();
        let start = if cursor_chars <= width {
            0
        } else {
            cursor_chars - width + 1
        };
        let end = (start + width).min(chars.len());
        let vis: String = chars[start..end].iter().collect();
        (vis, cursor_chars - start)
    }
}

fn prev_boundary(s: &str, idx: usize) -> usize {
    if idx == 0 {
        return 0;
    }
    let mut i = idx - 1;
    while i > 0 && !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}

fn next_boundary(s: &str, idx: usize) -> usize {
    if idx >= s.len() {
        return s.len();
    }
    let mut i = idx + 1;
    while i < s.len() && !s.is_char_boundary(i) {
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types_and_edits() {
        let mut inp = InputState::new("hi");
        assert_eq!(inp.handle_key(KeyCode::Char('!'), KeyModifiers::NONE, KeyEventKind::Press), InputAction::Changed);
        assert_eq!(inp.value, "hi!");
        assert_eq!(inp.handle_key(KeyCode::Backspace, KeyModifiers::NONE, KeyEventKind::Press), InputAction::Changed);
        assert_eq!(inp.value, "hi");
        assert_eq!(inp.handle_key(KeyCode::Left, KeyModifiers::NONE, KeyEventKind::Press), InputAction::None);
    }

    #[test]
    fn visible_window() {
        let inp = InputState::new("abcdef");
        let (vis, pos) = inp.visible(3);
        assert!(vis.chars().count() <= 3);
        assert!(pos <= 3);
    }
}
