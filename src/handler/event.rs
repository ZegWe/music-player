use crossterm::event::{self, Event, KeyCode};

use crate::app::App;

pub fn handle_event(app: &mut App, music_database: &str) -> bool {
    let mut is_loop = true;

    //Handle input
    if let Event::Key(key) = event::read().unwrap() {
        match key.code {
            KeyCode::Char('q') => is_loop = false,
            KeyCode::Char('g') => app.move_select_top(),
            KeyCode::Char('G') => app.move_select_bottom(),
            KeyCode::Down => app.move_select_down(1),
            KeyCode::Char('j') => app.move_select_down(1),
            KeyCode::Char('J') => app.move_select_down(5),
            KeyCode::Up => app.move_select_up(1),
            KeyCode::Char('k') => app.move_select_up(1),
            KeyCode::Char('K') => app.move_select_up(5),
            KeyCode::Char('n') => app.next_page(),
            KeyCode::Char('N') => app.previous_page(),
            KeyCode::Char('l') => app.open_folder(),
            KeyCode::Char('h') => app.back_previous_folder(music_database),
            KeyCode::Enter => app.playing_music(),
            _ => {}
        }
    }

    is_loop
}
