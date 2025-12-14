use std::io::Result;

use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders},
};

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.area());

    let main_title = "Context View";
    let main_view = Block::new()
        .title(main_title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let task_title = "Tasks";
    let tasks_view = Block::new()
        .title(task_title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(tasks_view, layout[0]);
    frame.render_widget(main_view, layout[1]);
}
