use crate::configuration::{
    actions::Actions,
    filters::Filters,
    folders::Folder,
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
