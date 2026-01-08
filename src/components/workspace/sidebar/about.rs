use ratatui::{
    Frame,
    layout::Rect,
    style::Color,
    widgets::{Block, BorderType, Borders},
};

use crate::{app::App, state::PanelState};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let is_active = app.state.active_modal.is_none() && app.state.active_panel == PanelState::About;

    let border_color = if is_active {
        Color::Green
    } else {
        Color::White
    };

    let overview = Block::new()
        .title(" About ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_color);

    frame.render_widget(overview, area);
}
