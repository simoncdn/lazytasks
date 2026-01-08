pub mod about;
pub mod active_tasks;
pub mod archived_tasks;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{app::App, components::workspace::sidebar, state::PanelState};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let archived_tasks_contraint = if app.state.active_panel == PanelState::ArchivedTasks {
        Constraint::Percentage(50)
    } else {
        Constraint::Percentage(24)
    };

    let sidebar = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(6),
            Constraint::Fill(1),
            archived_tasks_contraint,
        ])
        .split(area);

    sidebar::about::render(frame, sidebar[0], app);
    sidebar::active_tasks::render(frame, sidebar[1], app);
    sidebar::archived_tasks::render(frame, sidebar[2], app);
}
