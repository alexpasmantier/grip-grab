use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};

use tui_input::backend::crossterm::EventHandler;

use crate::app::{self, App};

pub fn handle_key_event(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.should_quit = true;
        }
        KeyCode::Tab => {
            app.next_block();
        }
        KeyCode::BackTab => {
            app.previous_block();
        }
        _ => {}
    }
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
        KeyCode::Up | KeyCode::Char('k') => {}
        KeyCode::Down | KeyCode::Char('j') => {}
        _ => {}
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
