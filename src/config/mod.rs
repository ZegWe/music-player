use exitfailure::ExitFailure;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitConfig {
    pub music_database: String,
    pub theme: InitTheme,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitTheme {
    pub list_title_color: String,
    pub list_title_page_color: String,
    pub list_border_color: String,
    pub list_music_color: String,
    pub list_folder_color: String,
    pub list_icon_color: String,
    pub list_selected_color: String,
    pub search_border_color: String,
    pub search_icon_color: String,
    pub search_font_color: String,
    pub command_font_color: String,
    pub command_border_color: String,
    pub music_pic_color1: String,
    pub music_pic_color2: String,
    pub usage_color_left: String,
    pub usage_color_right: String,
    pub cut_off_rule_color: String,
    pub play_music_list_title_color: String,
    pub play_music_list_border_color: String,
    pub play_music_list_id_color: String,
    pub play_music_list_duration_color: String,
    pub play_music_list_name_color: String,
    pub play_music_list_artist_color: String,
    pub play_music_list_album_color: String,
    pub play_music_list_header_color: String,
    pub playing_music_border_color: String,
    pub playing_music_name_color: String,
    pub gauge_color: String,
    pub gauge_border_color: String,
    pub gauge_label_color: String,
}

pub fn init() -> Result<InitConfig, ExitFailure> {
    match dirs::home_dir() {
        Some(home_path) => {
            let mut pathbuf = std::path::PathBuf::new();
            pathbuf.push(home_path.to_str().unwrap());
            pathbuf.push(".config");
            pathbuf.push("music_player");
            pathbuf.push("config.yml");
            let file = match std::fs::File::open(pathbuf) {
                Ok(file) => file,
                Err(_) => panic!("Configuration file not found"),
            };
            let init_config:InitConfig = serde_yaml::from_reader(file)?;

            Ok(init_config)
        }
        None => panic!("The path error"),
    }
}
