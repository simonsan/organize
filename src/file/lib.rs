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
    fn test_temporary_files() -> Result<(), Error> {
        let crdownload = File::from("$HOME/Downloads/unsplash.jpg.crdownload");
        let tmp = File::from("$HOME/Downloads/unsplash.jpg.tmp");
        let part = File::from("$HOME/Downloads/unsplash.jpg.part");
        let download = File::from("$HOME/Downloads/unsplash.jpg.crdownload");
        let filters = Filters::default();
        for file in [crdownload, tmp, part, download].iter() {
            if file.matches_filters(&filters) {
                return Err(Error::new(ErrorKind::Other, "temporary file matched filters"));
            }
        }
        Ok(())
    }

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
    fn test_filters_filename_startswith() -> Result<(), Error> {
        let file = File::from("/home/cabero/Documents/matricula.pdf");
        let mut filters = Filters::default();
        filters.filename.startswith = "matricula".into();
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
    #[test]
    fn test_filters_filename_contains() -> Result<(), Error> {
        let file = File::from("/home/cabero/Documents/matricula.pdf");
        let mut filters = Filters::default();
        filters.filename.contains = "icula".into();
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
    #[test]
    fn test_filters_filename_endswith() -> Result<(), Error> {
        let file = File::from("/home/cabero/Documents/matricula.pdf");
        let mut filters = Filters::default();
        filters.filename.contains = "ula.pdf".into();
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
