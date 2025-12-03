mod app;
mod db;
mod models;
mod views;
mod widgets;

use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
    let mut app = app::App::new()?;
    app.run(terminal)
}
