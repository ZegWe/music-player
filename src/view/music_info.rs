use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

use crate::music::Music;

use super::color::Theme;

#[rustfmt::skip]
const MUSIC_PLAYER_PIC: &[&str] = &[
    "███╗   ███╗","██╗   ██╗","███████╗","██╗"," ██████╗", "",
    "████╗ ████║","██║   ██║","██╔════╝","██║","██╔════╝", "",
    "██╔████╔██║","██║   ██║","███████╗","██║","██║     ", "",
    "██║╚██╔╝██║","██║   ██║","╚════██║","██║","██║     ", "",
    "██║ ╚═╝ ██║","╚██████╔╝","███████║","██║","╚██████╗", "",
    "╚═╝     ╚═╝"," ╚═════╝ ","╚══════╝","╚═╝"," ╚═════╝",
];
#[rustfmt::skip]
const USAGE: &[&str] = &[
    "                              ", "   ", "",
    "Move selection down            ", "[j]", "",
    "Move selection top             ", "[k]", "",
    "Move selection to the top      ", "[g]", "",
    "Move selection to the bottom   ", "[G]", "",
    "Next page                      ", "[n]", "",
    "Previous page                  ", "[N]", "",
    "Open folder                    ", "[l]", "",
    "Back previous folder           ", "[h]",
];

pub fn draw_music_info<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    theme: &Theme,
    music_info: &Option<Music>,
) {
    // Music info block
    let mut title = " Music info ";
    if *music_info == None {
        title = "";
    }
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Spans::from(vec![Span::styled(
            title,
            Style::default().fg(theme.info_title_color),
        )]))
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(theme.info_border_color));
    frame.render_widget(block, area);

    if let Some(_) = music_info {
    } else {
        draw_home_page(frame, &area, theme);
    }
}

fn draw_home_page<B: Backend>(frame: &mut Frame<B>, area: &Rect, theme: &Theme) {
    let inner_rect = Rect::new(area.x + 1, area.y + 2, area.width - 2, area.height - 3);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(3)])
        .split(inner_rect);

    // Display music picture
    let mut pic: Vec<Spans> = Vec::new();
    let pic_split = MUSIC_PLAYER_PIC.split(|s| s.len() == 0);
    for line in pic_split {
        let mut spans: Vec<Span> = Vec::new();
        for (i, item) in line.iter().enumerate() {
            let mut color = theme.music_pic_color1;
            if i % 2 == 0 {
                color = theme.music_pic_color2;
            }
            spans.push(Span::styled(*item, Style::default().fg(color)))
        }
        pic.push(Spans::from(spans));
    }
    frame.render_widget(Paragraph::new(pic).alignment(Alignment::Center), chunks[0]);

    // Display usage
    let mut usage: Vec<Spans> = Vec::new();
    let usage_split = USAGE.split(|s| s.len() == 0);
    for line in usage_split {
        usage.push(Spans::from(vec![
            Span::styled(line[0], Style::default().fg(theme.usage_color_left)),
            Span::styled(line[1], Style::default().fg(theme.usage_color_right)),
        ]))
    }

    frame.render_widget(
        Paragraph::new(usage).alignment(Alignment::Center),
        chunks[1],
    );
}
