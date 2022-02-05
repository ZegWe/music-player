use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

use crate::file_ops::DirectoryItem;
use crate::utils::split_path::split_path_to_name;

use super::color::Theme;
use super::display::Display;

pub fn draw_music_list<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    theme: &Theme,
    window_height: usize,
    files: &Vec<DirectoryItem>,
    selected_index: &Option<usize>,
    search_string: &str,
    command_string: &str,
    error: &Option<String>,
) {
    let selected_index = match selected_index {
        Some(index) => *index,
        None => 0,
    };
    // let selected_index = selected_index.unwrap();
    let display = Display::new(window_height, files.len(), selected_index);
    let mut music_names: Vec<Spans> = Vec::new();

    // List block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Spans::from(vec![
            Span::styled(" Music list ", Style::default().fg(theme.list_title_color)),
            Span::styled(
                format!("Page: {}/{} ", display.page.0, display.page.1),
                Style::default().fg(theme.list_title_page_color),
            ),
        ]))
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(theme.list_border_color));
    frame.render_widget(block, area);

    // Init music list
    if files.len() > 0 {
        //Convert DirectoryItems to Text
        for i in display.from..display.to {
            match &files[i] {
                DirectoryItem::File(path) => {
                    let name = split_path_to_name(path);

                    music_names.push(get_spans(
                        "  ".to_string(),
                        name.to_string(),
                        theme.list_icon_color,
                        theme.list_music_color,
                    ));
                }
                DirectoryItem::Directory(path) => {
                    let name = split_path_to_name(path);

                    music_names.push(get_spans(
                        "  ".to_string(),
                        name.to_string(),
                        theme.list_icon_color,
                        theme.list_folder_color,
                    ));
                }
            }
        }

        // Set style for selected music
        let remove_num = selected_index - ((display.page.0 - 1) * window_height);
        let names = music_names.remove(remove_num);
        let mut icon_name: Vec<String> = names
            .0
            .iter()
            .map(|span| span.content.to_string())
            .collect();
        icon_name[1].insert_str(0, "");
        music_names.insert(
            remove_num,
            get_spans(
                icon_name[0].to_string(),
                icon_name[1].to_string(),
                theme.list_icon_color,
                theme.list_selected_color,
            ),
        );
    }

    //Create the list chunks
    let inner_rect = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3)])
        .split(inner_rect);

    if let Some(error) = error {
        // Display error block
        draw_error(frame, chunks[0], error);
    } else if command_string.len() != 0 {
        // Display search block
        draw_command(frame, chunks[0], theme, command_string);
    } else {
        draw_search(frame, chunks[0], theme, search_string);
    }

    // Display musics and folders
    frame.render_widget(Paragraph::new(music_names), chunks[1]);
}

fn get_spans(icon: String, name: String, icon_color: Color, name_color: Color) -> Spans<'static> {
    Spans::from(vec![
        Span::styled(icon, Style::default().fg(icon_color)),
        Span::styled(format!("{}\n", name), Style::default().fg(name_color)),
    ])
}

fn draw_search<B: Backend>(frame: &mut Frame<B>, area: Rect, theme: &Theme, search_string: &str) {
    let text = Text::from(Spans::from(vec![
        Span::styled("  ", Style::default().fg(theme.search_icon_color)),
        Span::styled(search_string, Style::default().fg(theme.search_font_color)),
    ]));
    let search = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(theme.search_border_color)),
    );
    frame.render_widget(search, area);
}

fn draw_command<B: Backend>(frame: &mut Frame<B>, area: Rect, theme: &Theme, command_string: &str) {
    let text = Text::from(Spans::from(vec![Span::styled(
        command_string,
        Style::default().fg(theme.command_font_color),
    )]));
    let command_paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Command ")
            .style(Style::default().fg(theme.command_border_color)),
    );
    frame.render_widget(command_paragraph, area);
}

fn draw_error<B: Backend>(frame: &mut Frame<B>, area: Rect, error: &str) {
    let text = Spans::from(Span::styled(error, Style::default().fg(Color::LightRed)));
    let err_paragraph = Paragraph::new(text).block(
        Block::default()
            .title(Span::styled(" Error ", Style::default().fg(Color::LightRed)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::LightRed)),
    );
    frame.render_widget(err_paragraph, area);
}
