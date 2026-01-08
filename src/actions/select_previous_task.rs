use crate::{app::App, state::PanelState};

pub fn select_previous_task(app: &mut App) {
    match app.state.active_panel {
        PanelState::ActiveTasks => app.state.select_previous_task(app.active_tasks().len()),
        PanelState::ArchivedTasks => app.state.select_previous_task(app.archived_tasks().len()),
        PanelState::About => app.state.select_previous_task(app.archived_tasks().len()),
    }
}
