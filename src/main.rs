use std::env;
use std::io;

use app::App;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use exitfailure::ExitFailure;
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod app;
mod config;
mod file_ops;
mod view;

fn main() -> Result<(), ExitFailure> {
    let init_config = config::init()?;
    env::set_current_dir(init_config.music_database)?;

    // Initialize terminal
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    //Initialize App state
    let mut app = App::new(&mut terminal)?;

    //Main application loop
    loop {
        app.update_window_height();

        //Handle input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    disable_raw_mode()?;

    Ok(())
}
