pub enum Filters {
    Regex,
    Filename,
    LastModified,
}

enum Filename {
    StartsWith,
    Contains,
    EndsWith,
    CaseSensitive,
}
