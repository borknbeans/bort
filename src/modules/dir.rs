use crate::{PromptArgs, config::Config, modules::Module};

pub(crate) struct DirModule;

impl Module for DirModule {
    fn name(&self) -> &'static str {
        "dir" 
    }

    fn format_prompt(&self, prompt: String, prompt_args: &PromptArgs, _config: &Config) -> String {
        let mut dir = prompt_args.curr_dir.clone();
        if dir.starts_with(&prompt_args.home_dir) {
            dir = dir.replace(&prompt_args.home_dir, "~");
        }

        prompt.replace(&self.module_name(), &dir)
    }
}
