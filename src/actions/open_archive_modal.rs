use crate::{app::App, state::PanelState};

pub fn open_archive_modal(app: &mut App) {
    let is_archived = app.state.active_panel == PanelState::ArchivedTasks;

    if app.selected_tasks.is_empty() {
        if let Some(task_index) = app.state.get_selected_panel_state().selected() {
            let task_id = app.get_current_tasks()[task_index].id;
            app.state.open_archived_task(vec![task_id], is_archived)
        }
    } else {
        app.state
            .open_archived_task(app.selected_tasks.clone(), is_archived);
    }
}
