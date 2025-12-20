use ratatui::{crossterm::event::Event, widgets::ListState};
use tui_input::{Input, backend::crossterm::EventHandler};

/// The application global state
pub struct AppState {
    /// State of the task list (selected, scroll)
    pub tasks_list_state: ListState,

    /// State of opening a modal
    pub show_popup: bool,

    /// State of the creating task input
    pub input: Input,
}

impl AppState {
    pub fn new() -> Self {
        let mut tasks_list_state = ListState::default();
        tasks_list_state.select(Some(0));

        let mut delete_task_list_state = ListState::default();
        delete_task_list_state.select(Some(0));

        AppState {
            tasks_list_state: tasks_list_state,
            show_popup: false,
            input: Input::default(),
        }
    }

    pub fn select_next_task(&mut self, tasks_count: usize) {
        let current_task = self.tasks_list_state.selected();
        if current_task < Some(tasks_count - 1) {
            self.tasks_list_state.select_next();
        } else {
            self.tasks_list_state.select_first();
        }
    }

    pub fn select_previous_task(&mut self) {
        let current_task = self.tasks_list_state.selected();
        if current_task > Some(0) {
            self.tasks_list_state.select_previous();
        } else {
            self.tasks_list_state.select_last();
        }
    }

    pub fn toggle_popup(&mut self) {
        self.show_popup = !self.show_popup
    }

    /// handle key event to fill input
    pub fn handle_event(&mut self, event: &Event) {
        self.input.handle_event(event);
    }

    pub fn reset_input(&mut self) {
        self.input.reset();
    }
}
