use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
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
    "===============   ===============", "", " ",
    "Move selection down            ", "[j]", " ",
    "Move selection up              ", "[k]", " ",
    "Move selection to the top      ", "[g]", " ",
    "Move selection to the bottom   ", "[G]", " ",
    "Next page                      ", "[n]", " ",
    "Previous page                  ", "[N]", " ",
    "Open folder                    ", "[l]", " ",
    "Back previous folder           ", "[h]",
];

pub fn draw_play_music_list<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    theme: &Theme,
    music_list: &mut Vec<Music>,
) {
    let mut title_spans = Vec::new();
    // let mut title_spans = Spans::default();
    if music_list.len() > 0 {
        title_spans.push(Span::styled(
            " Play list ",
            Style::default().fg(theme.play_music_list_title_color),
        ));
        title_spans.push(Span::styled(" | ", Style::default().fg(Color::Yellow)));
        title_spans.push(Span::styled(
            format!(" Total {} music ", music_list.len()),
            Style::default().fg(theme.play_music_list_title_color),
        ));
    };

    // Play music list block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title_spans)
        .style(Style::default().fg(theme.play_music_list_border_color));
    frame.render_widget(block, area);

    if music_list.len() > 0 {
        draw_play_list(frame, &area, theme, music_list);
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
    let usage_split = USAGE.split(|s| *s == " ");
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

fn draw_play_list<B: Backend>(
    frame: &mut Frame<B>,
    area: &Rect,
    theme: &Theme,
    music_list: &mut Vec<Music>,
) {
    //Create the list chunks
    let inner_rect = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Percentage(20),
            Constraint::Percentage(35),
        ])
        .split(inner_rect);

    let mut names: Vec<Spans> = Vec::new();
    let mut artists: Vec<Spans> = Vec::new();
    let mut albums: Vec<Spans> = Vec::new();
    for (i, music) in music_list.iter().enumerate() {
        let s = music.total_duration.as_secs() % 60;
        let m = music.total_duration.as_secs() / 60;
        names.push(Spans::from(vec![
            Span::styled(
                format!("{}.", i + 1),
                Style::default().fg(theme.play_music_list_id_color),
            ),
            Span::styled(
                format!("[{:0>2}m {:2}s]", m, s),
                Style::default().fg(theme.play_music_list_duration_color),
            ),
            Span::styled(
                &music.name,
                Style::default().fg(theme.play_music_list_name_color),
            ),
        ]));

        artists.push(Spans::from(vec![Span::styled(
            &music.artist,
            Style::default().fg(theme.play_music_list_artist_color),
        )]));

        albums.push(Spans::from(vec![Span::styled(
            &music.album,
            Style::default().fg(theme.play_music_list_album_color),
        )]));
    }

    frame.render_widget(Paragraph::new(names), chunks[0]);
    frame.render_widget(Paragraph::new(artists), chunks[1]);
    frame.render_widget(Paragraph::new(albums), chunks[2]);
}
