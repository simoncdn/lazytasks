use crate::{app::App, state::PanelState};

pub fn open_edit_title_modal(app: &mut App) {
    if let Some(task_index) = app.state.get_selected_panel_state().selected()
        && app.state.active_panel == PanelState::ActiveTasks
    {
        let task = &app.get_current_tasks()[task_index];
        app.state.open_edit_task(task.id, task.title.clone());
    }
}
