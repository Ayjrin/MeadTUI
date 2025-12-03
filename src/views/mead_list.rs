use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table},
};

use crate::models::Mead;

// Nord-adjacent color palette
const NORD_FROST: Color = Color::Rgb(136, 192, 208);    // #88C0D0
const NORD_BLUE: Color = Color::Rgb(0, 103, 230);       // #0067E6
const NORD_CYAN: Color = Color::Rgb(0, 255, 255);       // #00FFFF
const NORD_BG: Color = Color::Rgb(46, 52, 64);          // #2E3440
const NORD_WHITE: Color = Color::Rgb(255, 255, 255);    // #FFFFFF
const NORD_GRAY: Color = Color::Rgb(76, 86, 106);       // #4C566A

/// Mead list view state
pub struct MeadListView {
    /// List of meads
    pub meads: Vec<Mead>,
    /// Currently selected index
    pub selected: usize,
    /// Whether the list needs to be refreshed from DB
    pub needs_refresh: bool,
}

impl MeadListView {
    pub fn new() -> Self {
        Self {
            meads: Vec::new(),
            selected: 0,
            needs_refresh: true,
        }
    }

    pub fn set_meads(&mut self, meads: Vec<Mead>) {
        self.meads = meads;
        self.needs_refresh = false;
        // Ensure selected index is valid
        if self.selected >= self.meads.len() && !self.meads.is_empty() {
            self.selected = self.meads.len() - 1;
        }
    }

    pub fn next(&mut self) {
        if !self.meads.is_empty() {
            self.selected = (self.selected + 1) % self.meads.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.meads.is_empty() {
            if self.selected == 0 {
                self.selected = self.meads.len() - 1;
            } else {
                self.selected -= 1;
            }
        }
    }

    pub fn get_selected(&self) -> Option<&Mead> {
        self.meads.get(self.selected)
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Min(10),    // Table
                Constraint::Length(3),  // Controls
            ])
            .split(area);

        // Title
        let title = Paragraph::new(Line::from(vec![
            Span::styled(
                "Current Meads",
                Style::default()
                    .fg(NORD_FROST)
                    .add_modifier(Modifier::BOLD),
            ),
        ]))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(NORD_FROST))
                .border_set(border::ROUNDED),
        );
        frame.render_widget(title, chunks[0]);

        // Mead table/list
        if self.meads.is_empty() {
            let empty_msg = Paragraph::new("No meads yet! Press Esc to go back and create one.")
                .alignment(Alignment::Center)
                .style(Style::default().fg(NORD_GRAY))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(NORD_BLUE))
                        .border_set(border::ROUNDED),
                );
            frame.render_widget(empty_msg, chunks[1]);
        } else {
            let header = Row::new(vec![
                "Name",
                "Status",
                "Start Date",
                "Honey",
                "Yeast",
                "OG",
                "Current",
            ])
            .style(
                Style::default()
                    .fg(NORD_CYAN)
                    .add_modifier(Modifier::BOLD),
            )
            .height(1);

            let rows: Vec<Row> = self
                .meads
                .iter()
                .enumerate()
                .map(|(i, mead)| {
                    let style = if i == self.selected {
                        Style::default()
                            .fg(NORD_BG)
                            .bg(NORD_CYAN)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(NORD_WHITE)
                    };

                    Row::new(vec![
                        mead.name.clone(),
                        mead.status.as_str().to_string(),
                        mead.start_date.clone(),
                        mead.honey_type.clone(),
                        mead.yeast_strain.clone(),
                        format!("{:.3}", mead.starting_gravity),
                        format!("{:.3}", mead.current_gravity),
                    ])
                    .style(style)
                    .height(1)
                })
                .collect();

            let table = Table::new(
                rows,
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(12),
                    Constraint::Percentage(12),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                ],
            )
            .header(header)
            .block(
                Block::default()
                    .title(Span::styled(
                        format!(" {} meads ", self.meads.len()),
                        Style::default().fg(NORD_FROST),
                    ))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(NORD_BLUE))
                    .border_set(border::ROUNDED),
            );

            frame.render_widget(table, chunks[1]);
        }

        // Controls
        let controls = Line::from(vec![
            Span::styled("Up/Down", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(" Navigate  ", Style::default().fg(NORD_WHITE)),
            Span::styled("Enter", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(" View Details  ", Style::default().fg(NORD_WHITE)),
            Span::styled("d", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(" Delete  ", Style::default().fg(NORD_WHITE)),
            Span::styled("Esc", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(" Back", Style::default().fg(NORD_WHITE)),
        ]);

        let controls_widget = Paragraph::new(controls)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(NORD_GRAY))
                    .border_set(border::ROUNDED),
            );

        frame.render_widget(controls_widget, chunks[2]);
    }
}

impl Default for MeadListView {
    fn default() -> Self {
        Self::new()
    }
}

