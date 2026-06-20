use serde::Deserialize;

use crate::modules;

#[derive(Deserialize, Default)]
pub(crate) struct Config {
    pub(crate) format: String,
    pub(crate) dir: Option<modules::dir::DirModuleConfig>,
    pub(crate) time: Option<modules::time::TimeModuleConfig>,
}

#[derive(Deserialize, Default)]
pub(crate) struct SharedModuleConfig {
    pub(crate) color: Option<String>,
}

pub(crate) fn parse_config(home_dir: &str) -> Config {
    let config_path = format!("{}/.config/bort.toml", home_dir);
    let config_file = std::fs::read_to_string(config_path).unwrap_or_default();
    let config: Config = toml::from_str(&config_file).unwrap_or_default();
    config
}
