use crate::app::App;

pub fn open_delete_modal(app: &mut App) {
    if app.selected_tasks.is_empty() {
        if let Some(task_index) = app.state.get_selected_panel_state().selected() {
            let task_id = app.get_current_tasks()[task_index].id;
            app.state.open_delete_task(vec![task_id]);
        }
    } else {
        app.state.open_delete_task(app.selected_tasks.clone());
    }
}
