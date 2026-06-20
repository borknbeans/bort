
pub(crate) mod dir;
pub(crate) mod fill;
pub(crate) mod indicator;
pub(crate) mod time;

pub(crate) fn format_modules(prompt_args: crate::PromptArgs, config: crate::config::Config) -> String {
    let mut prompt = config.format.clone();

    prompt = dir::DirModule.format_prompt(prompt, &prompt_args, &config);
    prompt = time::TimeModule.format_prompt(prompt, &prompt_args, &config);
    prompt = indicator::IndicatorModule.format_prompt(prompt, &prompt_args, &config);

    // Fill should always go last to make sure it formats properly
    prompt = fill::FillModule.format_prompt(prompt, &prompt_args, &config);

    prompt
}

pub(crate) trait Module {
    fn name(&self) -> &'static str;

    fn module_name(&self) -> String {
        format!("[{}]", self.name())
    }

    fn color(&self, input: String, color: String) -> String {
        format!("%F{{{}}}{}%f", color, input)
    }

     fn format_prompt(&self, prompt: String, prompt_args: &crate::PromptArgs, config: &crate::config::Config) -> String;
}
