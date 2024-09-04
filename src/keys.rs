use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use tui_input::backend::crossterm::EventHandler;

use crate::app::{self, App};

pub fn handle_key_event(key: KeyEvent, app: &mut App) {
    // global key bindings
    match key {
        // quitting the app
        KeyEvent {
            code: KeyCode::Char('q') | KeyCode::Esc,
            ..
        } => {
            app.should_quit = true;
        }
        // moving between blocks
        KeyEvent {
            code: KeyCode::Tab, ..
        } => {
            app.next_block();
        }
        KeyEvent {
            code: KeyCode::BackTab,
            ..
        } => {
            app.previous_block();
        }
        KeyEvent {
            code: KeyCode::Char('k') | KeyCode::Up,
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            app.move_to_block_on_top();
        }
        KeyEvent {
            code: KeyCode::Char('j') | KeyCode::Down,
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            app.move_to_block_below();
        }
        KeyEvent {
            code: KeyCode::Char('h') | KeyCode::Left,
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            app.move_to_block_left();
        }
        KeyEvent {
            code: KeyCode::Char('l') | KeyCode::Right,
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            app.move_to_block_right();
        }
        // moving between search results
        KeyEvent {
            code: KeyCode::Char('n'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            handle_results_key_event(
                KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                    ..key
                },
                app,
            );
        }
        KeyEvent {
            code: KeyCode::Char('p'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            handle_results_key_event(
                KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                    ..key
                },
                app,
            );
        }
        _ => {}
    }

    // block specific key bindings
    match app.current_block {
        app::CurrentBlock::Search => {
            handle_search_key_event(key, app);
        }
        app::CurrentBlock::Results => {
            handle_results_key_event(key, app);
        }
        app::CurrentBlock::Preview => {
            handle_preview_key_event(key, app);
        }
    }
}

fn handle_search_key_event(key: KeyEvent, app: &mut App) {
    match key.code {
        _ => {
            app.pattern.handle_event(&Event::Key(key));
        }
    }
}

fn handle_results_key_event(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(selected_index) = app.results_list.state.selected() {
                app.results_list
                    .state
                    .select(Some((selected_index + 1) % app.results_list.results.len()));
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if let Some(selected_index) = app.results_list.state.selected() {
                if selected_index == 0 {
                    app.results_list
                        .state
                        .select(Some(app.results_list.results.len() - 1));
                } else {
                    app.results_list.state.select_previous();
                }
            }
        }
        _ => {}
    }
    if app.results_list.state.selected().is_none() {
        app.results_list.state.select_first();
    }
}

fn handle_preview_key_event(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {}
        KeyCode::Down | KeyCode::Char('j') => {}
        KeyCode::Char('d') => {}
        KeyCode::Char('u') => {}
        _ => {}
    }
}
