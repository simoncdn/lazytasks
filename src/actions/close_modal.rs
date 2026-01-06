use crate::app::App;

pub fn close_modal(app: &mut App) {
    app.state.close_modal();
}
