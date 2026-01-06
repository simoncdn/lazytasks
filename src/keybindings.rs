use chrono::Utc;
use ratatui::crossterm::{self, event::Event};

use ratatui::DefaultTerminal;
use tui_input::backend::crossterm::EventHandler;

use crate::actions;
use crate::{app::App, models, state::ModalState};

pub fn handle_key_event(app: &mut App, event: &Event, terminal: &mut DefaultTerminal) {
    if let crossterm::event::Event::Key(key) = event {
        match &mut app.state.active_modal {
            Some(ModalState::CreateTask { input }) => match key.code {
                crossterm::event::KeyCode::Esc => actions::close_modal(app),
                crossterm::event::KeyCode::Enter => {
                    let title = input.value().trim();
                    if !title.is_empty() {
                        let new_task = models::task::Task::new(input.value());
                        app.tasks.push(new_task);
                        app.storage.save(&app.tasks);
                    }

                    let new_index = app.active_tasks().len() - 1;

                    app.state.active_tasks_state.select(Some(new_index));
                    app.state.close_modal();
                }
                _ => {
                    input.handle_event(&event);
                }
            },
            Some(ModalState::EditTask { task_id, input }) => match key.code {
                crossterm::event::KeyCode::Esc => actions::close_modal(app),
                crossterm::event::KeyCode::Enter => {
                    let new_title = input.value().trim();
                    if !new_title.is_empty() {
                        if let Some(task) = app.tasks.iter_mut().find(|task| task.id == *task_id) {
                            task.title = new_title.to_string();
                            task.updated_at = Some(Utc::now());
                        }
                        app.storage.save(&app.tasks);
                    }
                    app.state.close_modal();
                }
                _ => {
                    input.handle_event(&event);
                }
            },
            Some(ModalState::ArchivedTask {
                task_ids,
                selected_option,
                is_archived: _,
            }) => match key.code {
                crossterm::event::KeyCode::Esc => actions::close_modal(app),
                crossterm::event::KeyCode::Enter => {
                    let current_option_index = selected_option.selected();

                    if current_option_index == Some(0) {
                        app.tasks.iter_mut().for_each(|task| {
                            if task_ids.contains(&task.id) {
                                task.archived = !task.archived;
                                task.updated_at = Some(Utc::now());
                            }
                        });
                        app.storage.save(&app.tasks);
                        app.selected_tasks.clear();

                        let count = app.get_current_tasks().len();
                        if let Some(idx) = app.state.get_selected_panel_state().selected() {
                            if idx >= count {
                                app.state
                                    .get_selected_panel_state()
                                    .select(count.checked_sub(1));
                            }
                        }
                    }

                    app.state.close_modal();
                }
                crossterm::event::KeyCode::Char('j') => {
                    selected_option.select_next();
                }
                crossterm::event::KeyCode::Char('k') => {
                    selected_option.select_previous();
                }
                _ => {}
            },
            Some(ModalState::DeleteTask {
                task_ids,
                selected_option,
            }) => match key.code {
                crossterm::event::KeyCode::Esc => actions::close_modal(app),
                crossterm::event::KeyCode::Enter => {
                    let current_option_index = selected_option.selected();

                    if current_option_index == Some(0) {
                        app.tasks.retain(|t| !task_ids.contains(&t.id));
                        app.selected_tasks.clear();
                        app.storage.save(&app.tasks);

                        let count = app.get_current_tasks().len();
                        if let Some(idx) = app.state.get_selected_panel_state().selected() {
                            if idx >= count {
                                app.state
                                    .get_selected_panel_state()
                                    .select(count.checked_sub(1));
                            }
                        }
                    }

                    app.state.close_modal();
                }
                crossterm::event::KeyCode::Char('j') => {
                    selected_option.select_next();
                }
                crossterm::event::KeyCode::Char('k') => {
                    selected_option.select_previous();
                }
                _ => {}
            },
            None => match key.code {
                crossterm::event::KeyCode::Char('a') => actions::open_archive_modal(app),
                crossterm::event::KeyCode::Char('c') => actions::open_create_modal(app),
                crossterm::event::KeyCode::Char('e') => actions::open_edit_title_modal(app),
                crossterm::event::KeyCode::Char('E') => actions::edit_task(app, terminal),
                crossterm::event::KeyCode::Char('y') => actions::toggle_task_completion(app),
                crossterm::event::KeyCode::Char('q') => actions::quit(app),
                crossterm::event::KeyCode::Char('d') => actions::open_delete_modal(app),
                crossterm::event::KeyCode::Char('j') => actions::select_next_task(app),
                crossterm::event::KeyCode::Char('k') => actions::select_previous_task(app),
                crossterm::event::KeyCode::Char(' ') => actions::toggle_task_selection(app),
                crossterm::event::KeyCode::Tab => actions::switch_panel(app),
                _ => {}
            },
        }
    }
}
