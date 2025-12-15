use std::io::Result;

use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::components;
use crate::models;

pub struct App {
    pub tasks: Vec<models::task::Task>,
}

impl App {
    pub fn new() -> Self {
        let mut tasks: Vec<models::task::Task> = Vec::new();
        let task_one = models::task::Task::new(
            "0".to_string(),
            "task-01".to_string(),
            "Lorem ipsum dolor".to_string(),
        );
        let task_two = models::task::Task::new(
            "1".to_string(),
            "task-02".to_string(),
            "Lorem ipsum dolor".to_string(),
        );
        let task_three = models::task::Task::new(
            "2".to_string(),
            "task-03".to_string(),
            "Lorem ipsum dolor".to_string(),
        );
        tasks.push(task_one);
        tasks.push(task_two);
        tasks.push(task_three);

        return App { tasks };
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            if matches!(event::read()?, Event::Key(_)) {
                break Ok(());
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(frame.area());

        components::tasks::render(frame, layout[0], self);
        components::main_view::render(frame, layout[1]);
    }
}
