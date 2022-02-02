use std::io;

use app::App;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use exitfailure::ExitFailure;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use view::handle_theme;

mod app;
mod config;
mod file_ops;
mod view;

fn main() -> Result<(), ExitFailure> {
    let init_config = config::init()?;
    let theme = handle_theme(init_config.theme);

    // Initialize terminal
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    //Initialize App state
    let mut app = App::new(&mut terminal, &init_config.music_database)?;

    //Main application loop
    loop {
        app.update_window_height();
        view::draw(&mut app, &theme)?;

        //Handle input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('g') => app.move_select_top(),
                KeyCode::Char('G') => app.move_select_bottom(),
                KeyCode::Down => app.move_select_down(1),
                KeyCode::Char('j') => app.move_select_down(1),
                KeyCode::Char('J') => app.move_select_down(5),
                KeyCode::Up => app.move_select_up(1),
                KeyCode::Char('k') => app.move_select_up(1),
                KeyCode::Char('K') => app.move_select_up(5),
                KeyCode::Char(']') => app.next_page(),
                KeyCode::Char('[') => app.previous_page(),
                KeyCode::Enter => app.open_folder(),
                KeyCode::Char('l') => app.open_folder(),
                KeyCode::Char('h') => app.back_previous_folder(&init_config.music_database),
                _ => {}
            }
        }
    }

    disable_raw_mode()?;

    Ok(())
}
