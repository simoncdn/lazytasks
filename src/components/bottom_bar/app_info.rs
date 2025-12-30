use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::Paragraph,
};

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn render(frame: &mut Frame, area: Rect) {
    let app_info = Paragraph::new(APP_VERSION).alignment(Alignment::Right);

    frame.render_widget(app_info, area);
}
