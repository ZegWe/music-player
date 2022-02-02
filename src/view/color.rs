use lazy_static::lazy_static;
use regex::Regex;
use tui::style::Color;

use crate::config::InitTheme;

lazy_static! {
    static ref COLOR_HEX_REGEX: Regex =
        Regex::new(r"#(:?[0-9a-fA-F]{2})(:?[0-9a-fA-F]{2})(:?[0-9a-fA-F]{2})").unwrap();
}

fn parse_hex_color(color: &str) -> Color {
    let capture = COLOR_HEX_REGEX.captures(color).unwrap();

    Color::Rgb(
        u8::from_str_radix(capture.get(1).unwrap().as_str(), 16)
            .ok()
            .unwrap(),
        u8::from_str_radix(capture.get(2).unwrap().as_str(), 16)
            .ok()
            .unwrap(),
        u8::from_str_radix(capture.get(3).unwrap().as_str(), 16)
            .ok()
            .unwrap(),
    )
}

pub struct Theme {
    pub list_title_color: Color,
    pub list_title_page_color: Color,
    pub list_boder_color: Color,
    pub search_boder_color: Color,
    pub search_icon_color: Color,
    pub list_music_color: Color,
    pub list_folder_color: Color,
    pub list_folder_icon_color: Color,
}

impl Theme {
    pub fn new(init_theme: InitTheme) -> Theme {
        Theme {
            list_title_color: parse_hex_color(&init_theme.list_title_color),
            list_title_page_color: parse_hex_color(&init_theme.list_title_page_color),
            list_music_color: parse_hex_color(&init_theme.list_music_color),
            list_folder_color: parse_hex_color(&init_theme.list_folder_color),
            list_folder_icon_color: parse_hex_color(&init_theme.list_folder_icon_color),
            list_boder_color: parse_hex_color(&init_theme.list_boder_color),
            search_boder_color: parse_hex_color(&init_theme.search_boder_color),
            search_icon_color: parse_hex_color(&init_theme.search_icon_color),
        }
    }
}
