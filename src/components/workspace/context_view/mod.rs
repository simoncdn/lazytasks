pub mod about;
pub mod task_view;

use ratatui::{Frame, layout::Rect};

use crate::{app::App, components::workspace::context_view, models::task::Task, state::PanelState};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    match app.state.active_panel {
        PanelState::ActiveTasks => {
            let title = String::from(" Task details ");
            let tasks: Vec<&Task> = app.tasks.iter().filter(|t| !t.archived).collect();
            let current_list = &app.state.active_tasks_state;
            context_view::task_view::render(frame, area, title, current_list, tasks);
        }
        PanelState::ArchivedTasks => {
            let title = String::from(" Task details ");
            let tasks: Vec<&Task> = app.tasks.iter().filter(|t| t.archived).collect();
            let current_list = &app.state.archived_tasks_state;
            context_view::task_view::render(frame, area, title, current_list, tasks);
        }
        PanelState::About => {
            let title = String::from(" About ");
            context_view::about::render(frame, area, title);
        }
    };
}
