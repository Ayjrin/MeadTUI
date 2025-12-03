use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

// Nord-adjacent color palette
const NORD_FROST: Color = Color::Rgb(136, 192, 208);    // #88C0D0
const NORD_CYAN: Color = Color::Rgb(0, 255, 255);       // #00FFFF
const NORD_BG: Color = Color::Rgb(46, 52, 64);          // #2E3440
const NORD_WHITE: Color = Color::Rgb(255, 255, 255);    // #FFFFFF
const NORD_GRAY: Color = Color::Rgb(76, 86, 106);       // #4C566A

/// A text input field widget
#[derive(Debug, Clone)]
pub struct InputField {
    /// The label for this field
    pub label: String,
    /// The current text value
    pub value: String,
    /// Cursor position in the text
    pub cursor: usize,
    /// Whether this field is currently focused
    pub focused: bool,
    /// Placeholder text when empty
    pub placeholder: String,
}

impl InputField {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: String::new(),
            cursor: 0,
            focused: false,
            placeholder: String::new(),
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor = self.value.len();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Insert a character at the cursor position
    pub fn insert_char(&mut self, c: char) {
        self.value.insert(self.cursor, c);
        self.cursor += 1;
    }

    /// Delete the character before the cursor (backspace)
    pub fn delete_char(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.value.remove(self.cursor);
        }
    }

    /// Delete the character at the cursor (delete key)
    pub fn delete_char_forward(&mut self) {
        if self.cursor < self.value.len() {
            self.value.remove(self.cursor);
        }
    }

    /// Move cursor left
    pub fn move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Move cursor right
    pub fn move_cursor_right(&mut self) {
        if self.cursor < self.value.len() {
            self.cursor += 1;
        }
    }

    /// Move cursor to start
    pub fn move_cursor_start(&mut self) {
        self.cursor = 0;
    }

    /// Move cursor to end
    pub fn move_cursor_end(&mut self) {
        self.cursor = self.value.len();
    }

    /// Clear the field
    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor = 0;
    }

    /// Get the value as a string
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Set the value
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
        self.cursor = self.value.len();
    }

    /// Parse the value as f64
    pub fn get_f64(&self) -> Option<f64> {
        self.value.parse().ok()
    }

    /// Set focus state
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }
}

impl Widget for &InputField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = if self.focused {
            Style::default().fg(NORD_CYAN)
        } else {
            Style::default().fg(NORD_GRAY)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(Span::styled(
                format!(" {} ", self.label),
                if self.focused {
                    Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(NORD_FROST)
                },
            ));

        let inner = block.inner(area);
        block.render(area, buf);

        // Render the text content
        let display_text = if self.value.is_empty() && !self.focused {
            Line::from(Span::styled(
                &self.placeholder,
                Style::default().fg(NORD_GRAY),
            ))
        } else if self.focused {
            // Show cursor
            let before_cursor: String = self.value.chars().take(self.cursor).collect();
            let cursor_char = self.value.chars().nth(self.cursor).unwrap_or(' ');
            let after_cursor: String = self.value.chars().skip(self.cursor + 1).collect();

            Line::from(vec![
                Span::styled(before_cursor, Style::default().fg(NORD_WHITE)),
                Span::styled(
                    cursor_char.to_string(),
                    Style::default().bg(NORD_CYAN).fg(NORD_BG),
                ),
                Span::styled(after_cursor, Style::default().fg(NORD_WHITE)),
            ])
        } else {
            Line::from(Span::styled(self.value.as_str(), Style::default().fg(NORD_WHITE)))
        };

        Paragraph::new(display_text).render(inner, buf);
    }
}

