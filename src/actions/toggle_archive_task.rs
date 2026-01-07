use chrono::Utc;
use uuid::Uuid;

use crate::app::App;

pub fn toggle_archive_task(app: &mut App, option_idx: Option<usize>, task_ids: Vec<Uuid>) {
    if option_idx == Some(0) {
        app.tasks.iter_mut().for_each(|task| {
            if task_ids.contains(&task.id) {
                task.archived = !task.archived;
                task.updated_at = Some(Utc::now());
            }
        });
        app.storage.save(&app.tasks);
        app.selected_tasks.clear();

        let count = app.get_current_tasks().len();
        if let Some(idx) = app.state.get_selected_panel_state().selected() {
            if idx >= count {
                app.state
                    .get_selected_panel_state()
                    .select(count.checked_sub(1));
            }
        }
    }
}
