use crate::{app::App, state::PanelState};

pub fn open_priority_modal(app: &mut App) {
    let is_active_tasks = app.state.active_panel == PanelState::ActiveTasks;

    if is_active_tasks {
        if app.selected_tasks.is_empty() {
            if let Some(task_index) = app
                .state
                .get_selected_panel_state()
                .and_then(|s| s.selected())
            {
                let task_id = app.get_current_tasks()[task_index].id;
                app.state.open_priority_task(vec![task_id]);
            }
        } else {
            app.state.open_priority_task(app.selected_tasks.clone());
        }
    }
}
