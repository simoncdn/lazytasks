use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let task_title = " Tasks ";
    let list_items = app
        .tasks
        .iter()
        .map(|task| ListItem::new(task.title.clone()));

    let tasks_view = List::new(list_items).block(
        Block::new()
            .title(task_title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(tasks_view, area);
}
