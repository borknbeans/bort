use std::time::SystemTime;

use chrono::{DateTime, Local};

use crate::{PromptArgs, config::Config, modules::Module};

pub(crate) struct TimeModule;

impl Module for TimeModule {
    fn name(&self) -> &'static str {
        "time"
    }

    fn format_prompt(&self, prompt: String, _prompt_args: &PromptArgs, _config: &Config) -> String {
        let system_time = SystemTime::now();
        let datetime: DateTime<Local> = system_time.into();
        let format = "%H:%M";
        
        prompt.replace(&self.module_name(), &datetime.format(format).to_string())
        // println!("%F{{#689d6a}}{}%f{:>width$}\n> ", dir, datetime.format("%H:%M")); 
    }
}
