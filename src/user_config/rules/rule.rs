use crate::user_config::rules::{actions::Actions, filters::Filters, folder::WrappedFolder};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    pub actions: Actions,
    pub filters: Filters,
    pub folders: Vec<WrappedFolder>,
}
