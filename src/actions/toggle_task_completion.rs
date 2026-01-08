use chrono::Utc;

use crate::app::App;

pub fn toggle_task_completion(app: &mut App) {
    if app.selected_tasks.is_empty() {
        if let Some(task_index) = app.state.get_selected_panel_state().and_then(|s| s.selected()) {
            let task = app.get_current_tasks()[task_index].clone();
            if let Some(task) = app.tasks.iter_mut().find(|t| t.id == task.id) {
                task.completed = !task.completed;
                task.updated_at = Some(Utc::now());
            }
        }
    } else {
        app.tasks.iter_mut().for_each(|t| {
            if app.selected_tasks.contains(&t.id) {
                t.completed = !t.completed;
                t.updated_at = Some(Utc::now());
            }
        });
        app.selected_tasks.clear();
    }
    app.storage.save(&app.tasks);
}
