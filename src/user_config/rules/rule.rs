use crate::user_config::rules::{
    actions::Actions,
    filters::Filters,
    folder::Folder,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rule {
    pub actions: Actions,
    pub filters: Filters,
    pub folders: Vec<Folder>,
}
