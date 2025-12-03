use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};

use crate::db::Database;
use crate::models::{Ingredient, LogEntry};
use crate::views::{MainMenuView, MeadDetailView, MeadListView, NewMeadView};

/// The current view/screen being displayed
#[derive(Debug, Clone, PartialEq)]
pub enum View {
    MainMenu,
    MeadList,
    NewMead,
    MeadDetail(i64), // mead id
}

/// The main application state
pub struct App {
    /// Current view
    pub current_view: View,
    /// Database connection
    pub db: Database,
    /// Whether the app should exit
    pub should_exit: bool,
    /// Main menu view state
    pub main_menu: MainMenuView,
    /// Mead list view state
    pub mead_list: MeadListView,
    /// New mead form state
    pub new_mead: NewMeadView,
    /// Mead detail view state
    pub mead_detail: MeadDetailView,
    /// Status message to display
    pub status_message: Option<String>,
}

impl App {
    /// Create a new app instance
    pub fn new() -> io::Result<Self> {
        let db = Database::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        
        Ok(Self {
            current_view: View::MainMenu,
            db,
            should_exit: false,
            main_menu: MainMenuView::new(),
            mead_list: MeadListView::new(),
            new_mead: NewMeadView::new(),
            mead_detail: MeadDetailView::new(),
            status_message: None,
        })
    }

    /// Main application loop
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Render the current view
    fn draw(&mut self, frame: &mut Frame) {
        match &self.current_view {
            View::MainMenu => self.main_menu.render(frame, &self.status_message),
            View::MeadList => {
                // Load meads if needed
                if self.mead_list.needs_refresh {
                    if let Ok(meads) = self.db.get_all_meads() {
                        self.mead_list.set_meads(meads);
                    }
                }
                self.mead_list.render(frame);
            }
            View::NewMead => self.new_mead.render(frame),
            View::MeadDetail(id) => {
                // Load mead data if needed
                if self.mead_detail.needs_refresh {
                    if let Ok(Some(mead)) = self.db.get_mead(*id) {
                        let ingredients = self.db.get_ingredients(*id).unwrap_or_default();
                        let log_entries = self.db.get_log_entries(*id).unwrap_or_default();
                        self.mead_detail.set_mead(mead, ingredients, log_entries);
                    }
                }
                self.mead_detail.render(frame);
            }
        }
    }

    /// Handle input events
    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                self.handle_key_event(key);
            }
        }
        Ok(())
    }

    /// Handle key events based on current view
    fn handle_key_event(&mut self, key: KeyEvent) {
        // Clear status message on any key press
        self.status_message = None;

        match &self.current_view {
            View::MainMenu => self.handle_main_menu_key(key),
            View::MeadList => self.handle_mead_list_key(key),
            View::NewMead => self.handle_new_mead_key(key),
            View::MeadDetail(_) => self.handle_mead_detail_key(key),
        }
    }

    /// Handle keys in main menu
    fn handle_main_menu_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_exit = true,
            KeyCode::Up | KeyCode::Char('k') => self.main_menu.previous(),
            KeyCode::Down | KeyCode::Char('j') => self.main_menu.next(),
            KeyCode::Enter => {
                match self.main_menu.selected {
                    0 => {
                        self.mead_list.needs_refresh = true;
                        self.current_view = View::MeadList;
                    }
                    1 => {
                        self.new_mead = NewMeadView::new();
                        self.current_view = View::NewMead;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    /// Handle keys in mead list
    fn handle_mead_list_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.current_view = View::MainMenu,
            KeyCode::Up | KeyCode::Char('k') => self.mead_list.previous(),
            KeyCode::Down | KeyCode::Char('j') => self.mead_list.next(),
            KeyCode::Enter => {
                if let Some(mead) = self.mead_list.get_selected() {
                    let mead_id = mead.id;
                    self.mead_detail.needs_refresh = true;
                    self.current_view = View::MeadDetail(mead_id);
                }
            }
            KeyCode::Char('d') => {
                if let Some(mead) = self.mead_list.get_selected() {
                    let mead_id = mead.id;
                    let mead_name = mead.name.clone();
                    if self.db.delete_mead(mead_id).is_ok() {
                        self.mead_list.needs_refresh = true;
                        self.status_message = Some(format!("Deleted mead: {}", mead_name));
                    }
                }
            }
            _ => {}
        }
    }

    /// Handle keys in new mead form
    fn handle_new_mead_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                if self.new_mead.is_editing() {
                    self.new_mead.cancel_edit();
                } else {
                    self.current_view = View::MainMenu;
                }
            }
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    self.new_mead.previous_field();
                } else {
                    self.new_mead.next_field();
                }
            }
            KeyCode::Up if !self.new_mead.is_editing() => {
                self.new_mead.previous_field();
            }
            KeyCode::Down if !self.new_mead.is_editing() => {
                self.new_mead.next_field();
            }
            KeyCode::Enter => {
                if self.new_mead.is_on_submit() {
                    // Save the mead
                    let mead = self.new_mead.build_mead();
                    match self.db.create_mead(&mead) {
                        Ok(_) => {
                            self.status_message = Some(format!("Created mead: {}", mead.name));
                            self.current_view = View::MainMenu;
                        }
                        Err(e) => {
                            self.status_message = Some(format!("Error: {}", e));
                        }
                    }
                } else if self.new_mead.is_editing() {
                    // Stop editing and move to next field
                    self.new_mead.next_field();
                } else {
                    self.new_mead.next_field();
                }
            }
            KeyCode::Char(c) => {
                // Start editing automatically and insert the character
                if !self.new_mead.is_on_submit() {
                    if !self.new_mead.is_editing() {
                        self.new_mead.toggle_edit();
                    }
                    self.new_mead.insert_char(c);
                }
            }
            KeyCode::Backspace => {
                if !self.new_mead.is_on_submit() {
                    if !self.new_mead.is_editing() {
                        self.new_mead.toggle_edit();
                    }
                    self.new_mead.delete_char();
                }
            }
            KeyCode::Delete => {
                if self.new_mead.is_editing() {
                    self.new_mead.delete_char_forward();
                }
            }
            KeyCode::Left => {
                if self.new_mead.is_editing() {
                    self.new_mead.move_cursor_left();
                }
            }
            KeyCode::Right => {
                if self.new_mead.is_editing() {
                    self.new_mead.move_cursor_right();
                }
            }
            KeyCode::Home => {
                if self.new_mead.is_editing() {
                    self.new_mead.move_cursor_start();
                }
            }
            KeyCode::End => {
                if self.new_mead.is_editing() {
                    self.new_mead.move_cursor_end();
                }
            }
            _ => {}
        }
    }

    /// Handle keys in mead detail view
    fn handle_mead_detail_key(&mut self, key: KeyEvent) {
        let in_input_mode = self.mead_detail.is_editing() 
            || self.mead_detail.show_log_input 
            || self.mead_detail.show_ingredient_input;

        match key.code {
            KeyCode::Esc => {
                if self.mead_detail.is_editing() {
                    self.mead_detail.cancel_edit();
                } else if self.mead_detail.show_log_input || self.mead_detail.show_ingredient_input {
                    self.mead_detail.show_log_input = false;
                    self.mead_detail.show_ingredient_input = false;
                } else {
                    self.mead_list.needs_refresh = true;
                    self.current_view = View::MeadList;
                }
            }
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    self.mead_detail.previous_field();
                } else {
                    self.mead_detail.next_field();
                }
            }
            KeyCode::Up if !in_input_mode => {
                self.mead_detail.previous_field();
            }
            KeyCode::Down if !in_input_mode => {
                self.mead_detail.next_field();
            }
            KeyCode::Char('l') if !in_input_mode => {
                self.mead_detail.show_log_input = true;
                self.mead_detail.log_input.set_focused(true);
            }
            KeyCode::Char('i') if !in_input_mode => {
                self.mead_detail.show_ingredient_input = true;
                self.mead_detail.ingredient_name_input.set_focused(true);
            }
            KeyCode::Char('s') if !in_input_mode => {
                // Save changes
                if let Some(mead) = self.mead_detail.get_updated_mead() {
                    if self.db.update_mead(&mead).is_ok() {
                        self.status_message = Some("Mead updated!".to_string());
                        self.mead_detail.needs_refresh = true;
                    }
                }
            }
            KeyCode::Enter => {
                if self.mead_detail.show_log_input {
                    // Save log entry
                    if let Some(mead) = &self.mead_detail.mead {
                        let entry = LogEntry {
                            mead_id: mead.id,
                            entry_text: self.mead_detail.log_input.get_value().to_string(),
                            ..Default::default()
                        };
                        if !entry.entry_text.is_empty() {
                            if self.db.create_log_entry(&entry).is_ok() {
                                self.mead_detail.log_input.clear();
                                self.mead_detail.show_log_input = false;
                                self.mead_detail.needs_refresh = true;
                            }
                        }
                    }
                } else if self.mead_detail.show_ingredient_input {
                    // Save ingredient
                    if let Some(mead) = &self.mead_detail.mead {
                        let ingredient = Ingredient {
                            mead_id: mead.id,
                            name: self.mead_detail.ingredient_name_input.get_value().to_string(),
                            amount: self.mead_detail.ingredient_amount_input.get_f64().unwrap_or(0.0),
                            unit: self.mead_detail.ingredient_unit_input.get_value().to_string(),
                            ingredient_type: self.mead_detail.selected_ingredient_type.clone(),
                            ..Default::default()
                        };
                        if !ingredient.name.is_empty() {
                            if self.db.create_ingredient(&ingredient).is_ok() {
                                self.mead_detail.clear_ingredient_inputs();
                                self.mead_detail.show_ingredient_input = false;
                                self.mead_detail.needs_refresh = true;
                            }
                        }
                    }
                } else {
                    // Cycle status if on status field, otherwise toggle edit
                    self.mead_detail.toggle_edit();
                }
            }
            KeyCode::Char(c) => {
                if self.mead_detail.show_log_input || self.mead_detail.show_ingredient_input {
                    self.mead_detail.insert_char(c);
                } else if !in_input_mode {
                    // Start editing automatically
                    self.mead_detail.toggle_edit();
                    if self.mead_detail.is_editing() {
                        self.mead_detail.insert_char(c);
                    }
                } else {
                    self.mead_detail.insert_char(c);
                }
            }
            KeyCode::Backspace => {
                if self.mead_detail.show_log_input || self.mead_detail.show_ingredient_input {
                    self.mead_detail.delete_char();
                } else if !self.mead_detail.is_editing() {
                    self.mead_detail.toggle_edit();
                    if self.mead_detail.is_editing() {
                        self.mead_detail.delete_char();
                    }
                } else {
                    self.mead_detail.delete_char();
                }
            }
            KeyCode::Delete if in_input_mode => {
                self.mead_detail.delete_char_forward();
            }
            KeyCode::Left if in_input_mode => {
                self.mead_detail.move_cursor_left();
            }
            KeyCode::Right if in_input_mode => {
                self.mead_detail.move_cursor_right();
            }
            _ => {}
        }
    }
}

