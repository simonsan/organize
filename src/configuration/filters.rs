use serde::Deserialize;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Filters {
    regex: Option<String>,
    filename: Option<String>,
}
