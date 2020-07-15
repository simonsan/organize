pub mod actions;
mod filters;
mod folders;
pub mod options;

use crate::configuration::actions::Actions;
use crate::configuration::filters::Filters;
use crate::configuration::folders::Folder;
use crate::configuration::options::Options;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    pub actions: Actions,
    pub filters: Option<Filters>,
    pub folders: Vec<Folder>,
    pub options: Option<Options>,
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            actions: Default::default(),
            filters: None,
            folders: Default::default(),
            options: Some(Default::default()),
        }
    }
}
