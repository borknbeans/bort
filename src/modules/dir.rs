use crate::modules::Module;

pub(crate) struct Dir {
    pub(crate) home_dir: String,
    pub(crate) full_dir: String,
}

impl Module for Dir {
    fn name(&self) -> String {
        "dir".to_string() 
    }

    fn format(&self) -> String {
        let mut dir = self.full_dir.clone();
        if self.full_dir.starts_with(&self.home_dir) {
            dir = dir.replace(&self.home_dir, "~");
        }

        dir
    }
}
