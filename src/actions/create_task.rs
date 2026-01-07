use crate::{app::App, models};

pub fn create_task(app: &mut App, title: String) {
    let new_task = models::task::Task::new(title);
    app.tasks.push(new_task);
    app.storage.save(&app.tasks);

    let new_index = app.active_tasks().len() - 1;
    app.state.active_tasks_state.select(Some(new_index));
}
