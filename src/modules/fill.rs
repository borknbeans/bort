use regex::Regex;

use crate::{PromptArgs, config::Config, modules::Module};


pub(crate) struct FillModule;

impl Module for FillModule {
    fn name(&self) -> &'static str {
        "fill"
    }

    fn format_prompt(&self, prompt: String, prompt_args: &PromptArgs, _config: &Config) -> String {
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

            let fill = format!("{:>width$}", "", width = prompt_args.terminal_width - length_before - length_after);
            return prompt.replace(&self.module_name(), &fill);
        }

        prompt
    }
}
