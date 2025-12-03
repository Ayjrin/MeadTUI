use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

// Nord-adjacent color palette
const NORD_FROST: Color = Color::Rgb(136, 192, 208);    // #88C0D0
const NORD_BLUE: Color = Color::Rgb(0, 103, 230);       // #0067E6
const NORD_CYAN: Color = Color::Rgb(0, 255, 255);       // #00FFFF
const NORD_BG: Color = Color::Rgb(46, 52, 64);          // #2E3440
const NORD_WHITE: Color = Color::Rgb(255, 255, 255);    // #FFFFFF
const NORD_GRAY: Color = Color::Rgb(76, 86, 106);       // #4C566A

/// Main menu view state
pub struct MainMenuView {
    /// Currently selected menu item
    pub selected: usize,
    /// Menu options
    options: Vec<&'static str>,
}

impl MainMenuView {
    pub fn new() -> Self {
        Self {
            selected: 0,
            options: vec!["Current Meads", "New Mead"],
        }
    }

    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % self.options.len();
    }

    pub fn previous(&mut self) {
        if self.selected == 0 {
            self.selected = self.options.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn render(&self, frame: &mut Frame, status_message: &Option<String>) {
        let area = frame.area();

        // Create main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(8), // Logo/title
                Constraint::Min(10),   // Menu
                Constraint::Length(3), // Status bar
                Constraint::Length(3), // Controls
            ])
            .split(area);

        // Render title/logo
        let title = vec![
            Line::from(""),
            Line::from(Span::styled(
                " MEAD TRACKER ",
                Style::default()
                    .fg(NORD_FROST)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Track your mead brewing journey",
                Style::default().fg(NORD_GRAY),
            )),
        ];

        let title_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(NORD_FROST))
            .border_set(border::ROUNDED);

        let title_widget = Paragraph::new(title)
            .alignment(Alignment::Center)
            .block(title_block);

        frame.render_widget(title_widget, chunks[0]);

        // Render menu
        let items: Vec<ListItem> = self
            .options
            .iter()
            .enumerate()
            .map(|(i, opt)| {
                let style = if i == self.selected {
                    Style::default()
                        .fg(NORD_BG)
                        .bg(NORD_CYAN)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(NORD_WHITE)
                };

                let prefix = if i == self.selected { "> " } else { "  " };
                ListItem::new(Line::from(format!("{}{}", prefix, opt))).style(style)
            })
            .collect();

        let menu_block = Block::default()
            .title(Span::styled(
                " Menu ",
                Style::default()
                    .fg(NORD_CYAN)
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(NORD_BLUE))
            .border_set(border::ROUNDED);

        let menu = List::new(items).block(menu_block);

        // Center the menu horizontally
        let menu_area = centered_rect(40, 100, chunks[1]);
        frame.render_widget(menu, menu_area);

        // Render status message if any
        let status_text = status_message.as_ref().map(|s| s.as_str()).unwrap_or("");

        let status = Paragraph::new(status_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(NORD_FROST))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(NORD_GRAY))
                    .border_set(border::ROUNDED),
            );

        frame.render_widget(status, chunks[2]);

        // Render controls
        let controls = Line::from(vec![
            Span::styled(
                "Up/Down",
                Style::default()
                    .fg(NORD_CYAN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Navigate  ", Style::default().fg(NORD_WHITE)),
            Span::styled(
                "Enter",
                Style::default()
                    .fg(NORD_CYAN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Select  ", Style::default().fg(NORD_WHITE)),
            Span::styled(
                "q",
                Style::default()
                    .fg(NORD_CYAN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Quit", Style::default().fg(NORD_WHITE)),
        ]);

        let controls_widget = Paragraph::new(controls).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(NORD_GRAY))
                .border_set(border::ROUNDED),
        );

        frame.render_widget(controls_widget, chunks[3]);
    }
}

impl Default for MainMenuView {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
