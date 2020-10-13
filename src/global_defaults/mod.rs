mod lib;

use crate::config_directory;
use std::collections::HashMap;
use std::fs;
use yaml_rust::YamlEmitter;
use std::fs::File;
use serde::{
    Deserialize,
    Serialize
};

#[derive(Deserialize, Serialize, Clone, Debug)]
struct FiltersDefaults {
    case_sensitive: Option<bool>,
}

impl Default for FiltersDefaults {
    fn default() -> Self {
        Self {
            case_sensitive: Some(false)
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct FoldersDefaults {
    recursive: Option<bool>,
    watch: Option<bool>
}


impl Default for FoldersDefaults {
    fn default() -> Self {
        Self {
            recursive: Some(false),
            watch: Some(true)
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GlobalDefaults {
    filters: Option<FiltersDefaults>,
    folders: Option<FoldersDefaults>,
}

impl Default for GlobalDefaults {
    fn default() -> Self {
        GlobalDefaults {
            folders: Some(Default::default()),
            filters: Some(Default::default()),
        }
    }
}


impl GlobalDefaults {
    pub fn new() -> GlobalDefaults {
        let path = config_directory().join("settings.yml");
        match fs::read_to_string(&path) {
           Ok(content) => {
               let settings: Self = serde_yaml::from_str(&content)
                   .expect("could not read settings.yml");  // todo improve error message
               settings
           },
           Err(_) => {
               let file = File::create(&path).unwrap();
               let default = Self::default();
               serde_yaml::to_writer(file, &default).unwrap();
               default
           }
        }
    }
}