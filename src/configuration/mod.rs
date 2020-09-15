pub mod actions;
pub mod conflicts;
pub mod filters;
pub mod folders;
pub mod options;

use crate::configuration::{
    actions::Actions,
    filters::Filters,
    folders::Folder,
    options::Options,
};
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
