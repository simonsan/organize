use crate::configuration::filters::Filters;
use std::{
    io::Error,
    path::{
        Path,
        PathBuf,
    },
};

#[allow(dead_code)]
pub struct File {
    pub filename: String,
    pub stem: String,
    pub extension: String,
    pub path: PathBuf,
    pub is_hidden: bool,
}

impl From<&Path> for File {
    fn from(path: &Path) -> Self {
        let (stem, extension) = get_stem_and_extension(path).unwrap();
        let filename = String::from(path.file_name().unwrap().to_str().unwrap());
        File {
            is_hidden: filename.starts_with('.'),
            filename,
            stem,
            extension,
            path: path.to_path_buf(),
        }
    }
}

impl File {
    pub fn matches_filters(&self, filters: &Filters) -> bool {
        // TODO test this function
        let path = self.path.to_str().unwrap();
        if !filters.regex.as_str().is_empty() && filters.regex.is_match(path) {
            return true;
        }
        if !filters.filename.is_empty() && self.filename == filters.filename {
            return true;
        }
        if !filters.extensions.is_empty() {
            for extension in filters.extensions.iter() {
                if self.extension.eq(extension) {
                    return true;
                }
            }
        }
        false
    }
}

/// # Arguments
/// * `path`: A reference to a std::path::PathBuf
/// # Return
/// Returns the stem and extension of `path` if they exist and can be parsed, otherwise returns an Error
pub fn get_stem_and_extension(path: &Path) -> Result<(String, String), Error> {
    let stem = path.file_stem().unwrap().to_str().unwrap().to_owned();
    let extension = path.extension().unwrap_or_default().to_str().unwrap().to_owned();

    Ok((stem, extension))
}

#[cfg(test)]
mod tests {
    use crate::{
        configuration::filters::Filters,
        file::File,
    };
    use std::{
        io::{
            Error,
            ErrorKind,
        },
        path::Path,
    };

    #[test]
    fn test_filters_extensions() -> Result<(), Error> {
        let file = File::from(Path::new("/home/cabero/Documents/matricula.pdf"));
        let mut filters: Filters = Default::default();
        filters.extensions.push("pdf".to_string());
        if file.matches_filters(&filters) {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "file did not match the filters correctly".to_string(),
            ))
        }
    }
    #[test]
    fn test_filters_regex() -> Result<(), Error> {
        let file = File::from(Path::new("/home/cabero/Documents/matricula.pdf"));
        let filters: Filters = Default::default();
        if file.matches_filters(&filters) {
            // the default regex is an empty one, so it should match everything
            // but we check for this possibility before trying to match
            Err(Error::new(ErrorKind::Other, "file matched an empty regex".to_string()))
        } else {
            Ok(())
        }
    }
    #[test]
    fn test_filters_filename() -> Result<(), Error> {
        let file = File::from(Path::new("/home/cabero/Documents/matricula.pdf"));
        let mut filters: Filters = Default::default();
        filters.filename = "matricula.pdf".to_string();
        if file.matches_filters(&filters) {
            // the default regex is an empty one, so it should match everything
            // but we check for this possibility before trying to match
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "file did not match the filters correctly".to_string(),
            ))
        }
    }
}
