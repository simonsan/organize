pub mod file;

pub enum Action {
    Echo,
    Shell,
}

enum FileAction {
    Move,
    Rename,
    Copy,
    Delete,
    Trash,
}

enum ConflictOption {
    Overwrite,
    Skip,
    Rename,
}
