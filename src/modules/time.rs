use std::time::SystemTime;

use chrono::{DateTime, Local};
use serde::Deserialize;

use crate::{PromptArgs, config::{Config, SharedModuleConfig}, modules::Module};

pub(crate) struct TimeModule;

#[derive(Deserialize)]
#[serde(default)]
pub(crate) struct TimeModuleConfig {
    pub(crate) format: String,

    #[serde(flatten)]
    pub(crate) shared: SharedModuleConfig,
}

impl Module for TimeModule {
    fn name(&self) -> &'static str {
        "time"
    }

    fn format_prompt(&self, prompt: String, _prompt_args: &PromptArgs, config: &Config) -> String {
        let my_config = if let Some(time_config) = &config.time {
            time_config
        } else {
            &TimeModuleConfig::default()
        };

        let system_time = SystemTime::now();
        let datetime: DateTime<Local> = system_time.into();
        let mut res = datetime.format(&my_config.format).to_string();

        if let Some(color) = my_config.shared.color.clone() {
            res = self.color(res, color);
        }
        
        prompt.replace(&self.module_name(), &res) 
    }
}

impl Default for TimeModuleConfig {
    fn default() -> Self {
        Self {
            format: "%H:%M".to_string(),
            shared: SharedModuleConfig::default(),
        }
    }
}
