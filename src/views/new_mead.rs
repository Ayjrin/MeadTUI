use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::models::{Mead, MeadStatus};
use crate::widgets::InputField;

// Nord-adjacent color palette
const NORD_FROST: Color = Color::Rgb(136, 192, 208);    // #88C0D0
const NORD_BLUE: Color = Color::Rgb(0, 103, 230);       // #0067E6
const NORD_CYAN: Color = Color::Rgb(0, 255, 255);       // #00FFFF
const NORD_BG: Color = Color::Rgb(46, 52, 64);          // #2E3440
const NORD_WHITE: Color = Color::Rgb(255, 255, 255);    // #FFFFFF
const NORD_GRAY: Color = Color::Rgb(76, 86, 106);       // #4C566A

/// Field indices for navigation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NewMeadField {
    Name = 0,
    StartDate,
    HoneyType,
    HoneyAmount,
    YeastStrain,
    TargetAbv,
    StartingGravity,
    VolumeGallons,
    YanRequired,
    Notes,
    Submit,
}

impl NewMeadField {
    fn from_index(i: usize) -> Self {
        match i {
            0 => NewMeadField::Name,
            1 => NewMeadField::StartDate,
            2 => NewMeadField::HoneyType,
            3 => NewMeadField::HoneyAmount,
            4 => NewMeadField::YeastStrain,
            5 => NewMeadField::TargetAbv,
            6 => NewMeadField::StartingGravity,
            7 => NewMeadField::VolumeGallons,
            8 => NewMeadField::YanRequired,
            9 => NewMeadField::Notes,
            _ => NewMeadField::Submit,
        }
    }

    fn count() -> usize {
        11
    }
}

/// New mead form view state
pub struct NewMeadView {
    /// Input fields
    pub name: InputField,
    pub start_date: InputField,
    pub honey_type: InputField,
    pub honey_amount: InputField,
    pub yeast_strain: InputField,
    pub target_abv: InputField,
    pub starting_gravity: InputField,
    pub volume_gallons: InputField,
    pub yan_required: InputField,
    pub notes: InputField,
    /// Currently selected field
    pub current_field: usize,
    /// Whether currently editing a field
    pub editing: bool,
}

impl NewMeadView {
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        Self {
            name: InputField::new("Name").with_placeholder("My First Mead"),
            start_date: InputField::new("Start Date").with_value(now.format("%Y-%m-%d").to_string()),
            honey_type: InputField::new("Honey Type").with_placeholder("Wildflower, Clover, etc."),
            honey_amount: InputField::new("Honey (lbs)").with_value("3.0"),
            yeast_strain: InputField::new("Yeast Strain").with_placeholder("Lalvin 71B, D47, etc."),
            target_abv: InputField::new("Target ABV %").with_value("14.0"),
            starting_gravity: InputField::new("Starting Gravity").with_value("1.100"),
            volume_gallons: InputField::new("Volume (gallons)").with_value("1.0"),
            yan_required: InputField::new("YAN Required (ppm)").with_value("200"),
            notes: InputField::new("Notes").with_placeholder("Any additional notes..."),
            current_field: 0,
            editing: false,
        }
    }

    pub fn next_field(&mut self) {
        self.set_field_focus(false);
        self.editing = false;
        self.current_field = (self.current_field + 1) % NewMeadField::count();
        self.set_field_focus(true);
    }

    pub fn previous_field(&mut self) {
        self.set_field_focus(false);
        self.editing = false;
        if self.current_field == 0 {
            self.current_field = NewMeadField::count() - 1;
        } else {
            self.current_field -= 1;
        }
        self.set_field_focus(true);
    }

    fn set_field_focus(&mut self, focused: bool) {
        let field = NewMeadField::from_index(self.current_field);
        match field {
            NewMeadField::Name => self.name.set_focused(focused),
            NewMeadField::StartDate => self.start_date.set_focused(focused),
            NewMeadField::HoneyType => self.honey_type.set_focused(focused),
            NewMeadField::HoneyAmount => self.honey_amount.set_focused(focused),
            NewMeadField::YeastStrain => self.yeast_strain.set_focused(focused),
            NewMeadField::TargetAbv => self.target_abv.set_focused(focused),
            NewMeadField::StartingGravity => self.starting_gravity.set_focused(focused),
            NewMeadField::VolumeGallons => self.volume_gallons.set_focused(focused),
            NewMeadField::YanRequired => self.yan_required.set_focused(focused),
            NewMeadField::Notes => self.notes.set_focused(focused),
            NewMeadField::Submit => {}
        }
    }

    fn get_current_field_mut(&mut self) -> Option<&mut InputField> {
        match NewMeadField::from_index(self.current_field) {
            NewMeadField::Name => Some(&mut self.name),
            NewMeadField::StartDate => Some(&mut self.start_date),
            NewMeadField::HoneyType => Some(&mut self.honey_type),
            NewMeadField::HoneyAmount => Some(&mut self.honey_amount),
            NewMeadField::YeastStrain => Some(&mut self.yeast_strain),
            NewMeadField::TargetAbv => Some(&mut self.target_abv),
            NewMeadField::StartingGravity => Some(&mut self.starting_gravity),
            NewMeadField::VolumeGallons => Some(&mut self.volume_gallons),
            NewMeadField::YanRequired => Some(&mut self.yan_required),
            NewMeadField::Notes => Some(&mut self.notes),
            NewMeadField::Submit => None,
        }
    }

    pub fn is_editing(&self) -> bool {
        self.editing
    }

    pub fn is_on_submit(&self) -> bool {
        NewMeadField::from_index(self.current_field) == NewMeadField::Submit
    }

    pub fn toggle_edit(&mut self) {
        if !self.is_on_submit() {
            self.editing = !self.editing;
        }
    }

    pub fn cancel_edit(&mut self) {
        self.editing = false;
    }

    pub fn insert_char(&mut self, c: char) {
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
        if let Some(field) = self.get_current_field_mut() {
            field.move_cursor_left();
        }
    }

    pub fn move_cursor_right(&mut self) {
        if let Some(field) = self.get_current_field_mut() {
            field.move_cursor_right();
        }
    }

    pub fn move_cursor_start(&mut self) {
        if let Some(field) = self.get_current_field_mut() {
            field.move_cursor_start();
        }
    }

    pub fn move_cursor_end(&mut self) {
        if let Some(field) = self.get_current_field_mut() {
            field.move_cursor_end();
        }
    }

    /// Build a Mead struct from the form data
    pub fn build_mead(&self) -> Mead {
        Mead {
            name: self.name.get_value().to_string(),
            start_date: self.start_date.get_value().to_string(),
            honey_type: self.honey_type.get_value().to_string(),
            honey_amount_lbs: self.honey_amount.get_f64().unwrap_or(0.0),
            yeast_strain: self.yeast_strain.get_value().to_string(),
            target_abv: self.target_abv.get_f64().unwrap_or(14.0),
            starting_gravity: self.starting_gravity.get_f64().unwrap_or(1.100),
            current_gravity: self.starting_gravity.get_f64().unwrap_or(1.100),
            volume_gallons: self.volume_gallons.get_f64().unwrap_or(1.0),
            yan_required: self.yan_required.get_f64().unwrap_or(0.0),
            yan_added: 0.0,
            status: MeadStatus::Primary,
            notes: self.notes.get_value().to_string(),
            ..Default::default()
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Min(20),    // Form
                Constraint::Length(3),  // Controls
            ])
            .split(area);

        // Title
        let title = Paragraph::new(Line::from(vec![
            Span::styled(
                "New Mead",
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

        // Form layout - two columns
        let form_area = chunks[1];
        let form_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(form_area);

        // Left column fields
        let left_fields = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Name
                Constraint::Length(3), // Start Date
                Constraint::Length(3), // Honey Type
                Constraint::Length(3), // Honey Amount
                Constraint::Length(3), // Yeast Strain
                Constraint::Min(0),
            ])
            .split(form_columns[0]);

        // Right column fields
        let right_fields = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Target ABV
                Constraint::Length(3), // Starting Gravity
                Constraint::Length(3), // Volume
                Constraint::Length(3), // YAN Required
                Constraint::Length(3), // Notes
                Constraint::Length(3), // Submit button
                Constraint::Min(0),
            ])
            .split(form_columns[1]);

        // Render left column
        frame.render_widget(&self.name, left_fields[0]);
        frame.render_widget(&self.start_date, left_fields[1]);
        frame.render_widget(&self.honey_type, left_fields[2]);
        frame.render_widget(&self.honey_amount, left_fields[3]);
        frame.render_widget(&self.yeast_strain, left_fields[4]);

        // Render right column
        frame.render_widget(&self.target_abv, right_fields[0]);
        frame.render_widget(&self.starting_gravity, right_fields[1]);
        frame.render_widget(&self.volume_gallons, right_fields[2]);
        frame.render_widget(&self.yan_required, right_fields[3]);
        frame.render_widget(&self.notes, right_fields[4]);

        // Submit button
        let is_submit_selected = self.current_field == NewMeadField::Submit as usize;
        let submit_style = if is_submit_selected {
            Style::default()
                .fg(NORD_BG)
                .bg(NORD_CYAN)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(NORD_FROST)
        };

        let submit_btn = Paragraph::new("[ Create Mead ]")
            .alignment(Alignment::Center)
            .style(submit_style)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(if is_submit_selected {
                        Style::default().fg(NORD_CYAN)
                    } else {
                        Style::default().fg(NORD_GRAY)
                    })
                    .border_set(border::ROUNDED),
            );
        frame.render_widget(submit_btn, right_fields[5]);

        // Controls
        let controls = Line::from(vec![
            Span::styled("Tab/Arrows", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(" Navigate  ", Style::default().fg(NORD_WHITE)),
            Span::styled("Type", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(" to edit  ", Style::default().fg(NORD_WHITE)),
            Span::styled("Enter", Style::default().fg(NORD_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(" Submit  ", Style::default().fg(NORD_WHITE)),
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

impl Default for NewMeadView {
    fn default() -> Self {
        Self::new()
    }
}

