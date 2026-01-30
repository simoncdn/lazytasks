use crate::{app::App, state::PanelState};

pub fn open_create_task_modal(app: &mut App) {
    if app.state.active_panel == PanelState::ActiveTasks {
        let space_id = app.state.spaces_tree_state.selected()[0].clone();
        app.state.open_create_task(space_id)
    }
}
