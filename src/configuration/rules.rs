use crate::configuration::{
    actions::Actions,
    filters::Filters,
    folders::Folder,
    options::Options,
};

#[derive(Debug, Clone)]
pub struct Rule {
    pub actions: Actions,
    pub filters: Filters,
    pub folders: Vec<Folder>,
    pub options: Options,
}
