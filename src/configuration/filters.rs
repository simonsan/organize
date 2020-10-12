use regex::Regex;

#[derive(Debug, Clone)]
pub struct Filters {
    pub regex: Regex,
    pub filename: String,
    pub extensions: Vec<String>,
}

#[allow(clippy::trivial_regex)]
impl Default for Filters {
    fn default() -> Self {
        Filters {
            regex: Regex::new("").unwrap(),
            filename: String::new(),
            extensions: Vec::new(),
        }
    }
}
