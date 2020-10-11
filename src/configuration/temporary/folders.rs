use crate::configuration::{
    folders::Folder,
    temporary::{
        options::TemporaryOptions,
        rules::TemporaryRule,
        TemporaryConfigElement,
    },
};
use serde::Deserialize;
use std::{
    env,
    path::{
        Path,
        PathBuf,
    },
};

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

impl TemporaryFolder {
    pub fn expand_env_vars(&self) -> PathBuf {
        let components = self.path.components();
        let mut new_path = PathBuf::new();

        for component in components.into_iter() {
            let component: &Path = component.as_ref();
            if component.to_str().unwrap().starts_with('$') {
                let env_var = env::var(component.to_str().unwrap());
                if let Ok(env_var) = env_var {
                    println!("{}", env_var);
                    new_path.push(env_var);
                } else {
                    panic!(format!("an environment variable ({}) was found in the configuration file but it couldn't be read. Are you sure it exists?", new_path.display()))
                }
            } else {
                new_path.push(component);
            }
        }
        new_path
    }
}
