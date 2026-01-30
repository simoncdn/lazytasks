use chrono::Utc;
use uuid::Uuid;

use crate::{
    app::App,
    db::repositories::{SpaceRepository, TaskRepository},
};

pub fn archive_space(app: &mut App, option_idx: Option<usize>, space_id: Uuid) {
    if option_idx != Some(0) {
        return;
    }

    let space = match app.spaces.iter_mut().find(|s| s.id == space_id) {
        Some(s) => s,
        None => return,
    };

    space.archived = !space.archived;
    space.archived_at = if space.archived {
        Some(Utc::now())
    } else {
        None
    };
    space.updated_at = Some(Utc::now());

    if let Err(e) = SpaceRepository::update(&app.db.connection, space) {
        app.error = Some(e.to_string());
        return;
    }

    let is_archived = space.archived;

    for task in app.tasks.iter_mut().filter(|t| t.space_id == Some(space_id)) {
        task.archived = is_archived;
        task.archived_at = if is_archived { Some(Utc::now()) } else { None };
        task.updated_at = Some(Utc::now());

        if let Err(e) = TaskRepository::update(&app.db.connection, task) {
            app.error = Some(e.to_string());
            return;
        }
    }

    app.selected_tasks.clear();
    app.state.spaces_tree_state.select_first();
}
