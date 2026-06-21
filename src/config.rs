use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Default)]
pub struct Config {
    pub banner: Option<String>,
    pub color: Option<String>,
    pub banner_path: Option<String>,
}

pub fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("ferrofetch");
    path.push("config.toml");
    path
}

pub fn load_config() -> Config {
    let path = get_config_path();
    
    if !path.exists() {
        return Config::default();
    }

    match fs::read_to_string(&path) {
        Ok(content) => {
            match toml::from_str(&content) {
                Ok(cfg) => cfg,
                Err(e) => {
                    println!("Warning: config.toml has an Error: {}", e);
                    Config::default()
                }
            }
        }
        Err(_) => Config::default(),
    }
}