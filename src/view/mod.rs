use exitfailure::ExitFailure;
use tui::layout::{Constraint, Direction, Layout};

use crate::app::App;
use crate::config::InitTheme;

use self::color::Theme;
use self::music_list::draw_music_list;
mod color;
mod music_list;

pub fn handle_theme(init_theme: InitTheme) -> Theme {
    Theme::new(init_theme)
}

pub fn draw(app: &mut App, theme: &Theme) -> Result<(), ExitFailure> {
    let search_string = app.get_search_string();
    app.terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(f.size());

        draw_music_list(
            f,
            chunks[0],
            theme,
            app.window_height as usize,
            &app.directory_contents,
            &app.selection_index,
            &search_string,
        );
    })?;

    Ok(())
}
