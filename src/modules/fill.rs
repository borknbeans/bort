use regex::Regex;
use serde::Deserialize;

use crate::{PromptArgs, config::{Config, SharedModuleConfig}, modules::Module};


pub(crate) struct FillModule;

#[derive(Deserialize)]
#[serde(default)]
pub(crate) struct FillModuleConfig {
    pub(crate) char: char,

    #[serde(flatten)]
    pub(crate) shared: SharedModuleConfig,
}

impl Module for FillModule {
    fn name(&self) -> &'static str {
        "fill"
    }

    fn format_prompt(&self, prompt: String, prompt_args: &PromptArgs, config: &Config) -> String {
        let my_config = if let Some(fill_config) = &config.fill {
            fill_config
        } else {
            &FillModuleConfig::default()
        };

        // Ignore color formatting when calculating the fill amount
        let regex = Regex::new(r"\{[^\}]*\}|%[fF]").unwrap();
        let raw_prompt = regex.replace_all(&prompt, ""); 

        let module_idx = raw_prompt.find(&self.module_name());

        if let Some(idx) = module_idx {
            let mut length_before = 0;
            for i in (0..idx).rev() {
                if raw_prompt.chars().nth(i) == Some('\n') {
                    break;
                }
                length_before += 1;
            }

            let mut length_after = 0;
            for i in idx + self.module_name().len()..raw_prompt.len() {
                if raw_prompt.chars().nth(i) == Some('\n') {
                    break;
                }
                length_after += 1;
            }

            // Create fill with spaces to start
            let mut fill = format!("{:>width$}", "", width = prompt_args.terminal_width - length_before - length_after);
            // Replace it with our characters next
            fill = fill.replace(" ", &my_config.char.to_string());
            if let Some(color) = my_config.shared.color.clone() {
                fill = self.color(fill, color);
            }
            return prompt.replace(&self.module_name(), &fill);
        }

        prompt
    }
}

impl Default for FillModuleConfig {
    fn default() -> Self {
        Self { char: ' ', shared: Default::default() }
    }
}
