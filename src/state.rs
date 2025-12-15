use ratatui::widgets::ListState;

/// The application global state
pub struct AppState {
    /// State of the task list (selected, scroll)
    pub tasks_list_state: ListState,
}

impl AppState {
    pub fn new() -> Self {
        let mut tasks_list_state = ListState::default();
        tasks_list_state.select(Some(0));

        AppState {
            tasks_list_state: tasks_list_state,
        }
    }

    // pub fn select_next_task(&mut self) {
    //     let current_task = self.tasks_list_state.s
    //
    // }
}
