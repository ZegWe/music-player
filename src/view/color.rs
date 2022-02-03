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
    pub list_border_color: Color,
    pub list_music_color: Color,
    pub list_folder_color: Color,
    pub list_icon_color: Color,
    pub list_selected_color: Color,
    pub search_border_color: Color,
    pub search_icon_color: Color,
    pub info_title_color: Color,
    pub info_border_color: Color,
    pub music_pic_color1: Color,
    pub music_pic_color2: Color,
    pub usage_color_left: Color,
    pub usage_color_right: Color
}

impl Theme {
    pub fn new(init_theme: InitTheme) -> Theme {
        Theme {
            list_title_color: parse_hex_color(&init_theme.list_title_color),
            list_title_page_color: parse_hex_color(&init_theme.list_title_page_color),
            list_music_color: parse_hex_color(&init_theme.list_music_color),
            list_folder_color: parse_hex_color(&init_theme.list_folder_color),
            list_icon_color: parse_hex_color(&init_theme.list_icon_color),
            list_selected_color: parse_hex_color(&init_theme.list_selected_color),
            list_border_color: parse_hex_color(&init_theme.list_border_color),
            search_border_color: parse_hex_color(&init_theme.search_border_color),
            search_icon_color: parse_hex_color(&init_theme.search_icon_color),
            info_title_color: parse_hex_color(&init_theme.info_title_color),
            info_border_color: parse_hex_color(&init_theme.info_border_color),
            music_pic_color1: parse_hex_color(&init_theme.music_pic_color1),
            music_pic_color2: parse_hex_color(&init_theme.music_pic_color2),
            usage_color_left: parse_hex_color(&init_theme.usage_color_left),
            usage_color_right: parse_hex_color(&init_theme.usage_color_right),
        }
    }
}
