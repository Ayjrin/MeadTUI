use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::models::{Ingredient, IngredientType, LogEntry, Mead, MeadStatus};
use crate::widgets::InputField;

// Nord-adjacent color palette
const NORD_FROST: Color = Color::Rgb(136, 192, 208);    // #88C0D0
const NORD_BLUE: Color = Color::Rgb(0, 103, 230);       // #0067E6
const NORD_CYAN: Color = Color::Rgb(0, 255, 255);       // #00FFFF
const NORD_BG: Color = Color::Rgb(46, 52, 64);          // #2E3440
const NORD_WHITE: Color = Color::Rgb(255, 255, 255);    // #FFFFFF
const NORD_GRAY: Color = Color::Rgb(76, 86, 106);       // #4C566A

/// Field indices for navigation in detail view
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DetailField {
    Name = 0,
    Status,
    CurrentGravity,
    YanAdded,
    Notes,
}

impl DetailField {
    fn from_index(i: usize) -> Self {
        match i {
            0 => DetailField::Name,
            1 => DetailField::Status,
            2 => DetailField::CurrentGravity,
            3 => DetailField::YanAdded,
            _ => DetailField::Notes,
        }
    }

    fn count() -> usize {
        5
    }
}

/// Mead detail view state
pub struct MeadDetailView {
    /// The mead being viewed/edited
    pub mead: Option<Mead>,
    /// Ingredients for this mead
    pub ingredients: Vec<Ingredient>,
    /// Log entries for this mead
    pub log_entries: Vec<LogEntry>,
    /// Whether data needs refresh
    pub needs_refresh: bool,
    /// Current field being edited
    pub current_field: usize,
    /// Whether currently editing
    pub editing: bool,
    /// Editable fields
    pub name_input: InputField,
    pub current_gravity_input: InputField,
    pub yan_added_input: InputField,
    pub notes_input: InputField,
    /// Current status (for cycling)
    pub current_status: MeadStatus,
    /// Log entry input
    pub log_input: InputField,
    /// Whether showing log input
    pub show_log_input: bool,
    /// Ingredient input fields
    pub ingredient_name_input: InputField,
    pub ingredient_amount_input: InputField,
    pub ingredient_unit_input: InputField,
    pub selected_ingredient_type: IngredientType,
    /// Whether showing ingredient input
    pub show_ingredient_input: bool,
    /// Current ingredient input field (0-3)
    pub ingredient_field: usize,
}

impl MeadDetailView {
    pub fn new() -> Self {
        Self {
            mead: None,
            ingredients: Vec::new(),
            log_entries: Vec::new(),
            needs_refresh: true,
            current_field: 0,
            editing: false,
            name_input: InputField::new("Name"),
            current_gravity_input: InputField::new("Current Gravity"),
            yan_added_input: InputField::new("YAN Added"),
            notes_input: InputField::new("Notes"),
            current_status: MeadStatus::Planning,
            log_input: InputField::new("Log Entry"),
            show_log_input: false,
            ingredient_name_input: InputField::new("Ingredient Name"),
            ingredient_amount_input: InputField::new("Amount"),
            ingredient_unit_input: InputField::new("Unit").with_value("oz"),
            selected_ingredient_type: IngredientType::Fruit,
            show_ingredient_input: false,
            ingredient_field: 0,
        }
    }

    pub fn set_mead(&mut self, mead: Mead, ingredients: Vec<Ingredient>, log_entries: Vec<LogEntry>) {
        self.name_input.set_value(&mead.name);
        self.current_gravity_input.set_value(format!("{:.3}", mead.current_gravity));
        self.yan_added_input.set_value(format!("{:.0}", mead.yan_added));
        self.notes_input.set_value(&mead.notes);
        self.current_status = mead.status.clone();
        self.mead = Some(mead);
        self.ingredients = ingredients;
        self.log_entries = log_entries;
        self.needs_refresh = false;
    }

    pub fn next_field(&mut self) {
        if self.show_log_input {
            return;
        }
        if self.show_ingredient_input {
            self.ingredient_field = (self.ingredient_field + 1) % 4;
            self.update_ingredient_focus();
            return;
        }
        self.set_field_focus(false);
        self.editing = false;
        self.current_field = (self.current_field + 1) % DetailField::count();
        self.set_field_focus(true);
    }

    pub fn previous_field(&mut self) {
        if self.show_log_input {
            return;
        }
        if self.show_ingredient_input {
            if self.ingredient_field == 0 {
                self.ingredient_field = 3;
            } else {
                self.ingredient_field -= 1;
            }
            self.update_ingredient_focus();
            return;
        }
        self.set_field_focus(false);
        self.editing = false;
        if self.current_field == 0 {
            self.current_field = DetailField::count() - 1;
        } else {
            self.current_field -= 1;
        }
        self.set_field_focus(true);
    }

    fn update_ingredient_focus(&mut self) {
        self.ingredient_name_input.set_focused(self.ingredient_field == 0);
        self.ingredient_amount_input.set_focused(self.ingredient_field == 1);
        self.ingredient_unit_input.set_focused(self.ingredient_field == 2);
        // Field 3 is type selector
    }

    fn set_field_focus(&mut self, focused: bool) {
        match DetailField::from_index(self.current_field) {
            DetailField::Name => self.name_input.set_focused(focused),
            DetailField::Status => {}
            DetailField::CurrentGravity => self.current_gravity_input.set_focused(focused),
            DetailField::YanAdded => self.yan_added_input.set_focused(focused),
            DetailField::Notes => self.notes_input.set_focused(focused),
        }
    }

    fn get_current_field_mut(&mut self) -> Option<&mut InputField> {
        if self.show_log_input {
            return Some(&mut self.log_input);
        }
        if self.show_ingredient_input {
            return match self.ingredient_field {
                0 => Some(&mut self.ingredient_name_input),
                1 => Some(&mut self.ingredient_amount_input),
                2 => Some(&mut self.ingredient_unit_input),
                _ => None,
            };
        }
        match DetailField::from_index(self.current_field) {
            DetailField::Name => Some(&mut self.name_input),
            DetailField::Status => None,
            DetailField::CurrentGravity => Some(&mut self.current_gravity_input),
            DetailField::YanAdded => Some(&mut self.yan_added_input),
            DetailField::Notes => Some(&mut self.notes_input),
        }
    }

    pub fn is_editing(&self) -> bool {
        self.editing
    }

    pub fn toggle_edit(&mut self) {
        let field = DetailField::from_index(self.current_field);
        if field == DetailField::Status {
            // Cycle status instead of editing
            self.current_status = self.current_status.next();
        } else {
            self.editing = !self.editing;
        }
    }

    pub fn cancel_edit(&mut self) {
        self.editing = false;
    }

    pub fn insert_char(&mut self, c: char) {
        if self.show_ingredient_input && self.ingredient_field == 3 {
            // Type selector - ignore char input
            return;
        }
        if let Some(field) = self.get_current_field_mut() {
            field.insert_char(c);
        }
    }

    pub fn delete_char(&mut self) {
        if let Some(field) = self.get_current_field_mut() {
            field.delete_char();
        }
    }

    pub fn delete_char_forward(&mut self) {
        if let Some(field) = self.get_current_field_mut() {
            field.delete_char_forward();
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.show_ingredient_input && self.ingredient_field == 3 {
            // Cycle ingredient type
            self.selected_ingredient_type = match self.selected_ingredient_type {
                IngredientType::Fruit => IngredientType::Other,
                IngredientType::Spice => IngredientType::Fruit,
                IngredientType::Nutrient => IngredientType::Spice,
                IngredientType::Adjunct => IngredientType::Nutrient,
                IngredientType::Other => IngredientType::Adjunct,
            };
            return;
        }
        if let Some(field) = self.get_current_field_mut() {
            field.move_cursor_left();
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.show_ingredient_input && self.ingredient_field == 3 {
            // Cycle ingredient type
            self.selected_ingredient_type = match self.selected_ingredient_type {
                IngredientType::Fruit => IngredientType::Spice,
                IngredientType::Spice => IngredientType::Nutrient,
                IngredientType::Nutrient => IngredientType::Adjunct,
                IngredientType::Adjunct => IngredientType::Other,
                IngredientType::Other => IngredientType::Fruit,
            };
            return;
        }
        if let Some(field) = self.get_current_field_mut() {
            field.move_cursor_right();
        }
    }

    pub fn clear_ingredient_inputs(&mut self) {
        self.ingredient_name_input.clear();
        self.ingredient_amount_input.clear();
        self.ingredient_unit_input.set_value("oz");
        self.selected_ingredient_type = IngredientType::Fruit;
        self.ingredient_field = 0;
    }

    /// Get the updated mead with current form values
    pub fn get_updated_mead(&self) -> Option<Mead> {
        self.mead.as_ref().map(|m| {
            let mut updated = m.clone();
            updated.name = self.name_input.get_value().to_string();
            updated.current_gravity = self.current_gravity_input.get_f64().unwrap_or(m.current_gravity);
            updated.yan_added = self.yan_added_input.get_f64().unwrap_or(m.yan_added);
            updated.notes = self.notes_input.get_value().to_string();
            updated.status = self.current_status.clone();
            updated
        })
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Min(15),    // Content
                Constraint::Length(3),  // Controls
            ])
            .split(area);

        // Title
        let title_text = self.mead.as_ref()
            .map(|m| format!("{} - {}", m.name, m.status.as_str()))
            .unwrap_or_else(|| "Mead Details".to_string());
        
        let title = Paragraph::new(Line::from(Span::styled(
            title_text,
            Style::default()
                .fg(NORD_FROST)
                .add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(NORD_FROST))
                .border_set(border::ROUNDED),
        );
        frame.render_widget(title, main_chunks[0]);

        // Content area - split into left (details) and right (logs/ingredients)
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[1]);

        // Left side - mead details
        self.render_details(frame, content_chunks[0]);

        // Right side - logs and ingredients
        self.render_logs_and_ingredients(frame, content_chunks[1]);

        // Controls
        let controls = if self.show_log_input {
            Line::from(vec![
                Span::styled("Type", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(" log entry  ", Style::default().fg(NORD_WHITE)),
                Span::styled("Enter", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(" Save  ", Style::default().fg(NORD_WHITE)),
                Span::styled("Esc", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(" Cancel", Style::default().fg(NORD_WHITE)),
            ])
        } else if self.show_ingredient_input {
            Line::from(vec![
                Span::styled("Tab", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(" Next field  ", Style::default().fg(NORD_WHITE)),
                Span::styled("Enter", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(" Save  ", Style::default().fg(NORD_WHITE)),
                Span::styled("Esc", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(" Cancel", Style::default().fg(NORD_WHITE)),
            ])
        } else {
            Line::from(vec![
                Span::styled("Tab/Arrows", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(" Navigate  ", Style::default().fg(NORD_WHITE)),
                Span::styled("Type", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(" Edit  ", Style::default().fg(NORD_WHITE)),
                Span::styled("l", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(" Log  ", Style::default().fg(NORD_WHITE)),
                Span::styled("i", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(" Ingredient  ", Style::default().fg(NORD_WHITE)),
                Span::styled("s", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(" Save  ", Style::default().fg(NORD_WHITE)),
                Span::styled("Esc", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(" Back", Style::default().fg(NORD_WHITE)),
            ])
        };

        let controls_widget = Paragraph::new(controls)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(NORD_GRAY))
                    .border_set(border::ROUNDED),
            );
        frame.render_widget(controls_widget, main_chunks[2]);
    }

    fn render_details(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Name
                Constraint::Length(3), // Status
                Constraint::Length(3), // Current Gravity
                Constraint::Length(3), // YAN Added
                Constraint::Length(3), // Notes
                Constraint::Min(0),    // Info display
            ])
            .split(area);

        // Editable fields
        frame.render_widget(&self.name_input, chunks[0]);

        // Status selector
        let status_style = if self.current_field == 1 {
            Style::default().fg(NORD_CYAN)
        } else {
            Style::default().fg(NORD_GRAY)
        };
        let status_block = Block::default()
            .title(Span::styled(" Status (Enter to cycle) ", 
                if self.current_field == 1 {
                    Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(NORD_FROST)
                }
            ))
            .borders(Borders::ALL)
            .border_style(status_style)
            .border_set(border::ROUNDED);
        
        let status_text = Paragraph::new(format!("  {}", self.current_status.as_str()))
            .style(Style::default().fg(NORD_WHITE))
            .block(status_block);
        frame.render_widget(status_text, chunks[1]);

        frame.render_widget(&self.current_gravity_input, chunks[2]);
        frame.render_widget(&self.yan_added_input, chunks[3]);
        frame.render_widget(&self.notes_input, chunks[4]);

        // Static info display
        if let Some(mead) = &self.mead {
            let info_lines = vec![
                Line::from(vec![
                    Span::styled("Start Date: ", Style::default().fg(NORD_GRAY)),
                    Span::styled(&mead.start_date, Style::default().fg(NORD_WHITE)),
                ]),
                Line::from(vec![
                    Span::styled("Honey: ", Style::default().fg(NORD_GRAY)),
                    Span::styled(format!("{} ({:.1} lbs)", &mead.honey_type, mead.honey_amount_lbs), Style::default().fg(NORD_WHITE)),
                ]),
                Line::from(vec![
                    Span::styled("Yeast: ", Style::default().fg(NORD_GRAY)),
                    Span::styled(&mead.yeast_strain, Style::default().fg(NORD_WHITE)),
                ]),
                Line::from(vec![
                    Span::styled("OG: ", Style::default().fg(NORD_GRAY)),
                    Span::styled(format!("{:.3}", mead.starting_gravity), Style::default().fg(NORD_WHITE)),
                    Span::styled("  Target ABV: ", Style::default().fg(NORD_GRAY)),
                    Span::styled(format!("{:.1}%", mead.target_abv), Style::default().fg(NORD_WHITE)),
                ]),
                Line::from(vec![
                    Span::styled("Volume: ", Style::default().fg(NORD_GRAY)),
                    Span::styled(format!("{:.1} gal", mead.volume_gallons), Style::default().fg(NORD_WHITE)),
                    Span::styled("  YAN Req: ", Style::default().fg(NORD_GRAY)),
                    Span::styled(format!("{:.0} ppm", mead.yan_required), Style::default().fg(NORD_WHITE)),
                ]),
            ];
            
            let info = Paragraph::new(info_lines)
                .block(
                    Block::default()
                        .title(Span::styled(" Original Values ", Style::default().fg(NORD_FROST)))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(NORD_GRAY))
                        .border_set(border::ROUNDED),
                );
            frame.render_widget(info, chunks[5]);
        }
    }

    fn render_logs_and_ingredients(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Ingredients section
        self.render_ingredients(frame, chunks[0]);

        // Log entries section
        self.render_logs(frame, chunks[1]);
    }

    fn render_ingredients(&self, frame: &mut Frame, area: Rect) {
        if self.show_ingredient_input {
            // Show ingredient input form
            let input_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3), // Name
                    Constraint::Length(3), // Amount
                    Constraint::Length(3), // Unit
                    Constraint::Length(3), // Type
                ])
                .split(area);

            let block = Block::default()
                .title(Span::styled(" Add Ingredient ", Style::default().fg(NORD_FROST)))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(NORD_BLUE))
                .border_set(border::ROUNDED);
            frame.render_widget(block, area);

            frame.render_widget(&self.ingredient_name_input, input_chunks[0]);
            frame.render_widget(&self.ingredient_amount_input, input_chunks[1]);
            frame.render_widget(&self.ingredient_unit_input, input_chunks[2]);

            // Type selector
            let type_style = if self.ingredient_field == 3 {
                Style::default().fg(NORD_CYAN)
            } else {
                Style::default().fg(NORD_GRAY)
            };
            let type_block = Block::default()
                .title(Span::styled(" Type (Left/Right to change) ", 
                    if self.ingredient_field == 3 {
                        Style::default().fg(NORD_CYAN)
                    } else {
                        Style::default().fg(NORD_FROST)
                    }
                ))
                .borders(Borders::ALL)
                .border_style(type_style)
                .border_set(border::ROUNDED);
            let type_text = Paragraph::new(format!("  {}", self.selected_ingredient_type.as_str()))
                .style(Style::default().fg(NORD_WHITE))
                .block(type_block);
            frame.render_widget(type_text, input_chunks[3]);
        } else {
            // Show ingredients list
            let items: Vec<ListItem> = self.ingredients
                .iter()
                .map(|ing| {
                    ListItem::new(Line::from(vec![
                        Span::styled(
                            format!("[{}] ", ing.ingredient_type.as_str()),
                            Style::default().fg(NORD_CYAN),
                        ),
                        Span::styled(format!("{} - {:.1} {}", ing.name, ing.amount, ing.unit), Style::default().fg(NORD_WHITE)),
                    ]))
                })
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .title(Span::styled(format!(" Ingredients ({}) ", self.ingredients.len()), Style::default().fg(NORD_FROST)))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(NORD_BLUE))
                        .border_set(border::ROUNDED),
                );
            frame.render_widget(list, area);
        }
    }

    fn render_logs(&self, frame: &mut Frame, area: Rect) {
        if self.show_log_input {
            // Show log input
            let input_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(area);

            let block = Block::default()
                .title(Span::styled(" Add Log Entry ", Style::default().fg(NORD_FROST)))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(NORD_FROST))
                .border_set(border::ROUNDED);
            frame.render_widget(block, area);

            frame.render_widget(&self.log_input, input_chunks[0]);
        } else {
            // Show log entries
            let items: Vec<ListItem> = self.log_entries
                .iter()
                .map(|entry| {
                    ListItem::new(Line::from(vec![
                        Span::styled(
                            format!("[{}] ", entry.timestamp.format("%Y-%m-%d %H:%M")),
                            Style::default().fg(NORD_GRAY),
                        ),
                        Span::styled(&entry.entry_text, Style::default().fg(NORD_WHITE)),
                    ]))
                })
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .title(Span::styled(format!(" Log Entries ({}) ", self.log_entries.len()), Style::default().fg(NORD_FROST)))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(NORD_FROST))
                        .border_set(border::ROUNDED),
                );
            frame.render_widget(list, area);
        }
    }
}

impl Default for MeadDetailView {
    fn default() -> Self {
        Self::new()
    }
}

