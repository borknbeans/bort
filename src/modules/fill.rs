use crate::{PromptArgs, config::Config, modules::Module};


pub(crate) struct FillModule;

impl Module for FillModule {
    fn name(&self) -> &'static str {
        "fill"
    }

    fn format_prompt(&self, prompt: String, prompt_args: &PromptArgs, config: &Config) -> String {
        let module_idx = prompt.find(&self.module_name());

        if let Some(idx) = module_idx {
            let mut length_before = 0;
            for i in (0..idx).rev() {
                if prompt.chars().nth(i) == Some('\n') {
                    break;
                }
                length_before += 1;
            }

            let mut length_after = 0;
            for i in idx + self.module_name().len()..prompt.len() {
                if prompt.chars().nth(i) == Some('\n') {
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
