use serde::{Deserialize, Serialize};
use std::fs;
use dirs::home_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub banner: Option<String>,
    pub color: Option<String>,
    pub banner_path: Option<String>,
    pub no_ascii: Option<bool>,

    // pub seperator: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            banner: Some("batman".to_string()),
            color: Some("white".to_string()),   // Standardfarbe
            banner_path: None,
            no_ascii: Some(false),
        }
    }
}


pub fn load_config() -> Config {
    let config_dir = home_dir().unwrap_or_default().join(".config").join("ferrofetch");
    let config_path = config_dir.join("config.toml");

    if !config_path.exists() {
        // Create directory and standart config if not already did
        let _ = fs::create_dir_all(&config_dir);
        let default_config = toml::to_string_pretty(&Config::default()).unwrap();
        let _ = fs::write(&config_path, default_config);
    }

    let content = fs::read_to_string(&config_path).unwrap_or_default();
    toml::from_str(&content).unwrap_or_else(|_| Config::default())
}