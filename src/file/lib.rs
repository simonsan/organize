#[cfg(test)]
mod tests {
    use crate::{
        file::File,
        user_config::rules::filters::Filters,
    };
    use std::io::{
        Error,
        ErrorKind,
    };

    #[test]
    fn test_filters_extensions() -> Result<(), Error> {
        let file = File::from("/home/cabero/Documents/matricula.pdf");
        let mut filters = Filters::default();
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
        let file = File::from("/home/cabero/Documents/matricula.pdf");
        let filters = Filters::default();
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
        let file = File::from("/home/cabero/Documents/matricula.pdf");
        let mut filters = Filters::default();
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
