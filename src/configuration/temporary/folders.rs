use crate::configuration::{
    folders::Folder,
    temporary::{
        options::TemporaryOptions,
        rules::TemporaryRule,
        TemporaryConfigElement,
    },
};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct TemporaryFolder {
    pub path: PathBuf,
    pub options: Option<TemporaryOptions>,
}

impl Default for TemporaryFolder {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            options: Some(Default::default()),
        }
    }
}

impl TemporaryConfigElement<Folder> for TemporaryFolder {
    fn unwrap(self) -> Folder {
        Folder {
            path: self.path,
            options: self.options.unwrap().unwrap(),
        }
    }

    fn fill(self, parent_rule: &TemporaryRule) -> Self {
        let default: TemporaryOptions = Default::default();
        let parent_options = parent_rule.options.clone().unwrap_or_default();
        let options = default + parent_options.clone() + self.options.unwrap_or_else(|| parent_options);

        TemporaryFolder {
            path: self.path,
            options: Some(options),
        }
    }
}
