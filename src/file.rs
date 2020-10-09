use crate::configuration::filters::Filters;
use regex::Regex;
use std::{
    io::{
        Error,
        ErrorKind,
    },
    path::{
        Path,
        PathBuf,
    },
};

#[allow(dead_code)]
pub struct File<'a> {
    pub filename: &'a str,
    pub stem: &'a str,
    pub extension: &'a str,
    pub path: PathBuf,
    pub is_hidden: bool,
}

#[allow(dead_code)]
impl<'a> File<'a> {
    pub fn from(path: &'a Path) -> Result<Self, Error> {
        let (stem, extension) = get_stem_and_extension(path)?;
        let filename = path.file_name().unwrap().to_str().unwrap();
        Ok(File {
            filename,
            stem,
            extension,
            path: path.to_path_buf(),
            is_hidden: filename.starts_with('.'),
        })
    }

    pub fn matches_filters(&self, filters: &Filters) -> bool {
        // TODO test this function
        let path = self.path.to_str().unwrap();
        if filters.regex.is_some() {
            let regex = Regex::new(filters.regex.as_ref().unwrap()).expect("invalid regex defined in config file");
            if regex.is_match(path) {
                return true;
            }
        }
        if filters.filename.is_some() && self.filename == filters.filename.as_ref().unwrap() {
            return true;
        }
        if let Some(extensions) = &filters.extensions {
            for extension in extensions.iter() {
                if self.extension == extension {
                    return true;
                }
            }
        }
        false
    }

    // pub fn get_matching_rules(&self, rules: Rules) -> Result<Vec<&'a Rule>, Error> {
    //     let mut matching_rules = Vec::new();
    //     for rule in rules.iter() {
    //         if let Some(filters) = &rule.filters {
    //             if self.matches_filters(filters)? {
    //                 matching_rules.push(rule)
    //             }
    //         }
    //     }
    //     Ok(matching_rules)
    // }
}

/// # Arguments
/// * `path`: A reference to a std::path::PathBuf
/// # Return
/// Returns the stem and extension of `path` if they exist and can be parsed, otherwise returns an Error
pub fn get_stem_and_extension(path: &Path) -> Result<(&str, &str), Error> {
    let stem = path
        .file_stem()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "file does not have a file stem (?)"))?
        .to_str()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "cannot convert OsStr to &str"))?;
    let extension = path
        .extension()
        .unwrap_or_else(|| "".as_ref()) // some files don't have extensions
        .to_str()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "cannot convert OsStr to str"))?;

    Ok((stem, extension))
}
