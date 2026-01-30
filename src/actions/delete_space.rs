use uuid::Uuid;

use crate::{app::App, db::repositories::SpaceRepository};

pub fn delete_space(app: &mut App, option_idx: Option<usize>, space_id: Uuid) {
    if option_idx != Some(0) {
        return;
    }

    if let Err(e) = SpaceRepository::delete(&app.db.connection, &space_id) {
        app.error = Some(e.to_string());
        return;
    }

    app.tasks.retain(|t| t.space_id != Some(space_id));
    app.spaces.retain(|s| s.id != space_id);
    app.selected_tasks.clear();
    app.state.spaces_tree_state.select_first();
}
