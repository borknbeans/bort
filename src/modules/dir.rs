use serde::Deserialize;

use crate::{PromptArgs, config::{Config, SharedModuleConfig}, modules::Module};

pub(crate) struct DirModule;

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct DirModuleConfig {
    #[serde(flatten)]
    pub(crate) shared: SharedModuleConfig,
}

impl Module for DirModule {
    fn name(&self) -> &'static str {
        "dir" 
    }

    fn format_prompt(&self, prompt: String, prompt_args: &PromptArgs, config: &Config) -> String {
        let my_config = if let Some(dir_config) = &config.dir {
            dir_config
        } else {
            &DirModuleConfig::default()
        };

        let mut dir = prompt_args.curr_dir.clone();
        if dir.starts_with(&prompt_args.home_dir) {
            dir = dir.replace(&prompt_args.home_dir, "~");
        }

        if let Some(color) = my_config.shared.color.clone() {
            dir = self.color(dir, color);
        }

        prompt.replace(&self.module_name(), &dir)
    }
}
