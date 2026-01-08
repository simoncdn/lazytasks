pub mod about;
pub mod active_tasks;
pub mod archived_tasks;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{app::App, components::workspace::sidebar};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let sidebar = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(6),
            Constraint::Percentage(70),
            Constraint::Percentage(24),
        ])
        .split(area);

    sidebar::about::render(frame, sidebar[0], app);
    sidebar::active_tasks::render(frame, sidebar[1], app);
    sidebar::archived_tasks::render(frame, sidebar[2], app);
}
