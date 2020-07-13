#[derive(PartialEq)]
pub enum Action {
    Echo,
    Shell,
}

#[derive(PartialEq)]
enum FileAction {
    Move,
    Rename,
    Copy,
    Delete,
    Trash,
}

#[derive(PartialEq)]
pub enum ConflictOption {
    Overwrite,
    Skip,
    Rename,
}
