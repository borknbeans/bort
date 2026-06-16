pub(crate) mod dir;

pub(crate) trait Module {
    fn name(&self) -> String;

    fn module_name(&self) -> String {
        format!("[{}]", self.name())
    }

     fn format(&self) -> String;
}
