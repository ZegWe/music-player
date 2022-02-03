use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

use crate::file_ops::DirectoryItem;

use super::color::Theme;

struct Display {
    from: usize,
    to: usize,
    page: (usize, usize),
}

impl Display {
    pub fn new(height: usize, content_len: usize, selected_index: usize) -> Display {
        // show items length
        let display_pages = (content_len as f32 / (height as f32)).ceil() as usize;

        let mut from = 0;
        let mut to = 0;
        let mut page = 0;
        for i in 0..display_pages {
            if selected_index < (i + 1) * height {
                from = i * height;
                to = (i * height) + height;
                page = i + 1;
                break;
            }
        }
        if to >= content_len {
            to = content_len;
        }

        Display {
            from,
            to,
            page: (page, display_pages),
        }
    }
}

pub fn draw_music_list<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    theme: &Theme,
    window_height: usize,
    files: &Vec<DirectoryItem>,
    selected_index: &Option<usize>,
    search_string: &str,
) {
    let selected_index = selected_index.unwrap();
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

    if files.len() != 0 {
        //Convert DirectoryItems to Text
        for i in display.from..display.to {
            match &files[i] {
                DirectoryItem::File(path) => {
                    let name = get_file_name(path);

                    music_names.push(get_spans(
                        "  ",
                        &name,
                        theme.list_icon_color,
                        theme.list_music_color,
                    ));
                }
                DirectoryItem::Directory(path) => {
                    let name = get_file_name(path);

                    music_names.push(get_spans(
                        "  ",
                        &name,
                        theme.list_icon_color,
                        theme.list_folder_color,
                    ));
                }
            }
        }

        // Set style for selected music
        let remove_num = selected_index - ((display.page.0 - 1) * window_height);
        let names =
            music_names.remove(remove_num);
        let mut icon_name: Vec<String> = names
            .0
            .iter()
            .map(|span| span.content.to_string())
            .collect();
        icon_name[1].insert_str(0, "");
        music_names.insert(
            remove_num,
            get_spans(
                &icon_name[0],
                &icon_name[1],
                theme.list_icon_color,
                theme.list_selected_color,
            ),
        );

        //Create the list chunks
        let inner_rect = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(3)])
            .split(inner_rect);

        // Display search block
        let text = Text::from(Spans::from(vec![
            Span::styled("  ", Style::default().fg(theme.search_icon_color)),
            Span::styled(search_string, Style::default().fg(Color::White)),
        ]));
        let search = Paragraph::new(text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(theme.search_border_color)),
        );
        frame.render_widget(search, chunks[0]);

        // Display musics and folders
        frame.render_widget(Paragraph::new(music_names), chunks[1]);
    }
}

fn get_file_name(path: &str) -> &str {
    let str = path.split("\\").collect::<Vec<&str>>();
    str.last().unwrap()
}

fn get_spans<'a>(icon: &'a str, name: &'a str, icon_color: Color, name_color: Color) -> Spans<'a> {
    Spans::from(vec![
        Span::styled(icon, Style::default().fg(icon_color)),
        Span::styled(format!("{}\n", name), Style::default().fg(name_color)),
    ])
}
