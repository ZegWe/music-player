use std::io;

use app::App;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use exitfailure::ExitFailure;
use handler::event::handle_event;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use view::handle_theme;

mod app;
mod config;
mod file_ops;
mod view;
mod music;
mod handler;

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

        if !handle_event(&mut app, &init_config.music_database) {
            break;
        };
    }

    disable_raw_mode()?;

    Ok(())
}
