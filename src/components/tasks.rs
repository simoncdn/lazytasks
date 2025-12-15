use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let task_title = " Tasks ";
    let list_items = app
        .tasks
        .iter()
        .map(|task| ListItem::new(task.title.clone()));

    let tasks_view = List::new(list_items)
        .block(
            Block::new()
                .title(task_title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Color::Green),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_stateful_widget(tasks_view, area, &mut app.state.tasks_list_state);
}
