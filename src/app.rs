use std::io::Result;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{self, event::Event},
    layout::{Constraint, Direction, Layout},
};

use crate::models;
use crate::{components, state};

pub struct App {
    pub exit: bool,
    pub tasks: Vec<models::task::Task>,
    pub state: state::AppState,
}

impl App {
    pub fn new() -> Self {
        let mut tasks: Vec<models::task::Task> = Vec::new();
        let task_one = models::task::Task::new(
            0,
            "Sed ut perspiciatis unde omnis".to_string(),
            "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.".to_string(),
        );
        let task_two = models::task::Task::new(
            1,
            "Lorem Ipsum standard".to_string(),
            "But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system".to_string(),
        );
        let task_three = models::task::Task::new(
            2,
            "De Finibus Bonorum et Malorum".to_string(),
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec eu congue augue. Integer felis risus, sagittis sit amet pretium eu.".to_string(),
        );
        tasks.push(task_one);
        tasks.push(task_two);
        tasks.push(task_three);

        let state = state::AppState::new();

        return App {
            exit: false,
            tasks,
            state,
        };
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            let event = ratatui::crossterm::event::read()?;
            self.handle_key_event(&event);
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(frame.area());

        components::tasks::render(frame, layout[0], self);
        components::main_view::render(frame, layout[1], self);

        if self.state.show_popup {
            components::create_task::render(frame, &self.state.input);
        }
    }

    fn handle_key_event(&mut self, event: &Event) {
        if let crossterm::event::Event::Key(key) = event {
            if self.state.show_popup {
                match key.code {
                    crossterm::event::KeyCode::Esc => {
                        self.state.toggle_popup();
                        self.state.reset_input()
                    }
                    crossterm::event::KeyCode::Enter => {
                        let new_task = models::task::Task::new(
                            self.tasks.len(),
                            self.state.input.value().to_string(),
                            "".to_string(),
                        );
                        self.tasks.push(new_task);
                        self.state.toggle_popup();
                        self.state.reset_input()
                    }
                    _ => {
                        self.state.handle_event(&event);
                    }
                }
            } else {
                match key.code {
                    crossterm::event::KeyCode::Char('c') => self.state.toggle_popup(),
                    crossterm::event::KeyCode::Char('q') => self.exit = true,
                    crossterm::event::KeyCode::Char('d') => {
                        let current_task = self.state.tasks_list_state.selected().unwrap();
                        self.tasks.remove(current_task);
                    }
                    crossterm::event::KeyCode::Char('j') => {
                        self.state.select_next_task(self.tasks.len());
                    }
                    crossterm::event::KeyCode::Char('k') => {
                        self.state.select_previous_task();
                    }
                    _ => {}
                }
            }
        }
    }
}
