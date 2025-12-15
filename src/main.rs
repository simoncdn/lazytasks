use std::io::Result;

mod app;
mod components;
mod models;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = app::App::new();
    let result = app.run(&mut terminal);

    ratatui::restore();

    result
}
