mod lib;

use std::{
    env,
    path::{
        Path,
        PathBuf,
    },
};

pub trait Expand<T> {
    fn expand(&self) -> T;
}

impl Expand<PathBuf> for PathBuf {
    fn expand(&self) -> PathBuf {
        self.components()
            .map(|comp| {
                let path: &Path = comp.as_ref();
                let path = path.to_str().unwrap();
                if path.starts_with('$') {
                    env::var(path.replace('$', ""))
                        .unwrap_or_else(|_| panic!("error: environment variable '{}' could not be found", path))
                } else {
                    path.to_string()
                }
            })
            .collect()
    }
}
