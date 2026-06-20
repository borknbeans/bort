use crate::modules::Module;


pub(crate) struct IndicatorModule;

impl Module for IndicatorModule {
    fn name(&self) -> &'static str {
        "indicator"
    }

    fn format_prompt(&self, prompt: String, prompt_args: &crate::PromptArgs, config: &crate::config::Config) -> String {
        prompt.replace(&self.module_name(), ">")
    }
}
