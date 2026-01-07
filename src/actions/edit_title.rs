use chrono::Utc;
use uuid::Uuid;

use crate::app::App;

pub fn edit_title(app: &mut App, task_id: Uuid, title: String) {
    if !title.is_empty() {
        if let Some(task) = app.tasks.iter_mut().find(|task| task.id == task_id) {
            task.title = title.to_string();
            task.updated_at = Some(Utc::now());
        }
        app.storage.save(&app.tasks);
    }
}
