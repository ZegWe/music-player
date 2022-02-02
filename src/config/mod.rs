use exitfailure::ExitFailure;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitConfig {
    pub music_database: String,
}

pub fn init() -> Result<InitConfig, ExitFailure> {
    match dirs::home_dir() {
        Some(home_path) => {
            let config_path = format!(
                "{}\\.config\\music_player\\config.yml",
                home_path.to_str().unwrap()
            );

            let file = std::fs::File::open(config_path)?;
            let init_config: InitConfig = serde_yaml::from_reader(file)?;

            Ok(init_config)
        }
        None => panic!("The path error"),
    }
}
