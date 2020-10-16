mod lib;

use serde_yaml::to_string;
use std::{
    env,
    io::Error,
    path::{
        Path,
        PathBuf,
    },
};

pub trait Expandable {
    fn fullpath(&self) -> Result<Self, Error>;
    fn expand_user(&self) -> Self;
    fn expand_vars(&self) -> Self;
    fn expand_placeholders(&self, path: &Path) -> Self;
}

impl Expandable for PathBuf {
    fn fullpath(&self) -> Result<Self, Error> {
        Ok(self.expand_user().expand_vars().canonicalize()?)
    }

    fn expand_user(&self) -> Self {
        let str = self.to_str().unwrap().to_string();
        Self::from(str.replace("~", "$HOME"))
    }

    fn expand_vars(&self) -> Self {
        self.components()
            .map(|component| {
                let component: &Path = component.as_ref();
                let component = component.to_str().unwrap();
                if component.starts_with('$') {
                    env::var(component.replace('$', ""))
                        .unwrap_or_else(|_| panic!("error: environment variable '{}' could not be found", component))
                } else {
                    component.to_string()
                }
            })
            .collect()
    }

    fn expand_placeholders(&self, path: &Path) -> Self {
        let as_str = self.to_str().unwrap().to_string();
    }
}
