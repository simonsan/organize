use crate::config::actions::file::File;
use crate::config::actions::Action;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Pattern {
    pub(crate) regex: String,
    pub(crate) new_folder: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Fields {
    pub(crate) new_folder: String,
    pub(crate) patterns: Option<Vec<Pattern>>,
}

struct Folder {
    path: String,
    watch: Option<bool>,
    recursive: Option<bool>,
}

pub struct Rule {
    folders: Vec<Folder>,
    actions: Vec<Action>,
    watch: Option<bool>,
    recursive: Option<bool>,
}
//
// impl <'a> Rule {
//     pub fn get_file_dst(&self, file: &File) -> String {
//         match &self.fields.patterns {
//             Some(patterns) => {
//                 for pattern in patterns {
//                     if file.matches_pattern(&pattern) {
//                         return pattern.new_folder.to_owned();
//                     }
//                 }
//                 self.fields.new_folder.to_owned()
//             }
//             None => self.fields.new_folder.to_owned(),
//         }
//     }
// }
