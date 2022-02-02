use exitfailure::ExitFailure;
use tui::layout::{Layout, Direction, Constraint};

use crate::app::App;

pub fn draw(app: &mut App) -> Result<(), ExitFailure> {
    app.terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(f.size());
    })?;

    Ok(())
}
