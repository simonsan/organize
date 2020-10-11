use regex::Regex;

#[derive(Debug, Clone)]
pub struct Filters {
    pub regex: Regex,
    pub filename: String,
    pub extensions: Vec<String>,
}
