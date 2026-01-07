use crate::app::App;

pub fn switch_panel(app: &mut App) {
    app.selected_tasks.clear();
    app.state.toggle_active_panel();
}
