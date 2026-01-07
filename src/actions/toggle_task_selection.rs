use crate::app::App;

pub fn toggle_task_selection(app: &mut App) {
    if let Some(task_index) = app.state.get_selected_panel_state().selected() {
        let task_id = app.get_current_tasks()[task_index].id;
        if app.selected_tasks.contains(&task_id) {
            app.selected_tasks.retain(|id| *id != task_id);
        } else {
            app.selected_tasks.push(task_id);
        }
    }
}
