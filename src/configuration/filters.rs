use serde::Deserialize;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Filters {
    pub regex: Option<String>,
    pub filename: Option<String>,
    pub extension: Option<String>,
}
