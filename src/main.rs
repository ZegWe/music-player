use std::io;
use std::time::{Duration, Instant};

use app::App;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use exitfailure::ExitFailure;
use handler::event::handle_event;
use rodio::{OutputStream, Sink};
use tui::backend::CrosstermBackend;
use tui::Terminal;
use view::handle_theme;

mod app;
mod config;
mod file_ops;
mod handler;
mod music;
mod view;
mod utils;

fn main() -> Result<(), ExitFailure> {
    let init_config = config::init()?;
    let theme = handle_theme(init_config.theme);

    // Initialize terminal
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    // Initialize App state
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let mut sink = Sink::try_new(&stream_handle)?; // Music player
    let mut app = App::new(&mut terminal, &init_config.music_database, &mut sink)?;

    let tick_rate = Duration::from_secs(1);
    let mut last_tick = Instant::now();
    loop {
        app.update_window_height();
        view::draw(&mut app, &theme)?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if !handle_event(&mut app, &init_config.music_database)? {
                break;
            };
        }

        if last_tick.elapsed() >= tick_rate {
            app.check_music_list();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;

    Ok(())
}
