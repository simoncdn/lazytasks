pub mod active_tasks;
pub mod archived_tasks;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{app::App, components::sidebar};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let sidebar = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    sidebar::active_tasks::render(frame, sidebar[0], app, &app.active_tasks());
    sidebar::archived_tasks::render(frame, sidebar[1], app, &app.archived_tasks());
}
