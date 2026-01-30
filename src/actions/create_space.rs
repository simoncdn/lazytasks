use crate::{app::App, db::repositories::SpaceRepository, models};

pub fn create_space(app: &mut App, title: String) {
    let new_space = models::Space::new(title);
    let space_id = new_space.id.to_string();

    if let Err(e) = SpaceRepository::create(&app.db.connection, &new_space) {
        app.error = Some(e.to_string());
        return;
    };

    let is_first_space = app.spaces.is_empty();
    app.spaces.push(new_space);

    if is_first_space {
        app.state.spaces_tree_state.select(vec![space_id]);
    }
}
